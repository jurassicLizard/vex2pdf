use crate::lib_utils::errors::Vex2PdfError;
use cyclonedx_bom::errors::{BomError, JsonReadError, XmlReadError};
use cyclonedx_bom::prelude::Bom;
use log::warn;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// Returns the application version and copyright text.
///
/// This is used both for clap's `long_version` output and for displaying
/// version information at startup.
pub const fn get_version_info() -> &'static str {
    concat!(
        "vex2pdf ", env!("CARGO_PKG_VERSION"), " - CycloneDX (VEX) to PDF Converter\n",
        "Copyright (c) 2025 Salem B. - MIT Or Apache 2.0 License"
    )
}

/// Parses an XML file into a CycloneDX Bom object.
///
/// Reads the file content and attempts to parse it as a CycloneDX 1.5 XML document.
/// Includes special handling for CycloneDX 1.6 documents by attempting to downgrade
/// them to version 1.5 by modifying the namespace.
///
/// Note: The downgrade from 1.6 to 1.5 is a compatibility feature and may not work
/// if the document uses 1.6-specific fields.
pub(crate) fn parse_vex_xml<P: AsRef<Path>>(path: P) -> Result<Bom, Box<dyn Error>> {
    // First, read the entire file content
    let content = fs::read(path)?;

    // try to parse xml bom
    match Bom::parse_from_xml_v1_5(&content[..]) {
        Ok(bom) => Ok(bom),
        Err(err) => match &err {
            XmlReadError::InvalidNamespaceError {
                expected_namespace,
                actual_namespace,
            } => {
                // check if we are dealing with a cyclonedx version > 1.5
                if let Some(actual) = actual_namespace {
                    if actual.contains("1.6") {
                        print_downgrade_warning();

                        // convert content to string to replace namespace
                        let xml_str = std::string::String::from_utf8_lossy(&content);

                        // replace the namespace
                        let modified_xml = xml_str.replace(actual, expected_namespace);

                        // Try parsing with the modified XML
                        return Ok(Bom::parse_from_xml_v1_5(modified_xml.as_bytes())?);
                    }
                }

                // if we get here we couldn't handle the namespace error
                Err(Box::new(err))
            }
            _ => Err(Box::new(err)),
        },
    }
}

/// Parses a JSON file into a CycloneDX Bom object.
///
/// Reads the file content and attempts to parse it as a CycloneDX JSON document.
/// Includes special handling for CycloneDX 1.6 documents by attempting to downgrade
/// them to version 1.5 by modifying the spec version value.
///
/// The function first tries to parse the JSON normally. If that fails due to an unsupported
/// spec version (1.6), it modifies the JSON object to use version 1.5 and tries again.
///
/// Note: The downgrade from 1.6 to 1.5 is a compatibility feature and may not work
/// if the document uses 1.6-specific fields.
pub(crate) fn parse_vex_json<P: AsRef<Path>>(path: P) -> Result<Bom, Box<dyn Error>> {
    // First, read the entire file content
    let content = fs::read(path)?;
    // Try to parse normally first
    match Bom::parse_from_json(&content[..]) {
        Ok(bom) => Ok(bom),
        Err(err) => match err {
            JsonReadError::BomError { error } => {
                match error {
                    BomError::UnsupportedSpecVersion(version) if version == "1.6" => {
                        // Parse to JSON Value
                        let mut json_value: serde_json::Value = serde_json::from_slice(&content)?;

                        print_downgrade_warning();

                        json_value["specVersion"] = serde_json::Value::String("1.5".to_string());

                        // Try parsing with modified JSON
                        Ok(Bom::parse_json_value(json_value)?)
                    }
                    _ => Err(JsonReadError::BomError { error }.into()),
                }
            }
            _ => Err(err.into()),
        },
    }
}

/// Prints a warning message about downgrading from CycloneDX 1.6 to 1.5.
///
/// Called when the parser encounters a 1.6 document and attempts to process it
/// by downgrading to version 1.5.
fn print_downgrade_warning() {
    warn!("");
    warn!("NOTE: Downgrading CycloneDX BOM from spec version 1.6 to 1.5");
    warn!("Reason: Current implementation does not yet fully support spec version 1.6");
    warn!("Warning: This compatibility mode only works for BOMs that don't utilize 1.6-specific fields");
    warn!("         Processing will fail if 1.6-specific fields are encountered");
    warn!("");
}

/// Constructs an output PDF path based on the input file path.
///
/// Creates a new path with the same base name as the input file but with a .pdf extension.
/// Used internally to determine where to save generated PDF files.
///
/// ## Arguments
/// - dest_dir : Path to build from **Optional**
/// - file_path : file path to convert from
///
/// ## Behavior
///
/// - If `dest_dir` is `None`, replaces the input file's extension with `.pdf` in the same directory
/// - If `dest_dir` is `Some(dir)` and is a directory, creates the PDF in that directory with the input file's base name
/// - If `dest_dir` is `Some(file)` and is a file, returns an error
///
/// ## Examples
///
/// ```rust
/// use std::path::PathBuf;
/// use vex2pdf::lib_utils::run_utils;
///
/// let path = PathBuf::from("/tmp/file.json");
///
/// // No dest_dir: PDF goes in same directory as input
/// assert_eq!(
///     run_utils::get_output_pdf_path(None, &path).unwrap(),
///     PathBuf::from("/tmp/file.pdf")
/// );
///
/// // dest_dir is a directory: PDF goes in that directory
/// assert_eq!(
///     run_utils::get_output_pdf_path(Some(PathBuf::from("/tmp/output").as_path()), &path).unwrap(),
///     PathBuf::from("/tmp/output/file.pdf")
/// );
/// ```
pub fn get_output_pdf_path(
    dest_dir: Option<&Path>,
    file_path: &Path,
) -> Result<PathBuf, Vex2PdfError> {
    let file_stem = file_path
        .file_stem()
        .ok_or_else(|| Vex2PdfError::InvalidFileStem(file_path.to_path_buf()))?;

    let pdf_name = format!("{}.pdf", file_stem.to_string_lossy());

    match dest_dir {
        None => Ok(file_path.with_file_name(pdf_name)),
        Some(out_dir) if out_dir.is_file() => {
            Err(Vex2PdfError::InvalidOutputDir(out_dir.to_path_buf()))
        }
        Some(out_dir) => Ok(out_dir.join(&pdf_name)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile;
    use tempfile::env::temp_dir;
    use tempfile::NamedTempFile;

    #[test]
    fn test_path_build_no_dest_dir() {
        let path = PathBuf::from("/tmp/file.json");

        // No dest_dir: PDF should be in same directory as input
        assert_eq!(
            get_output_pdf_path(None, path.as_path()).unwrap(),
            PathBuf::from("/tmp/file.pdf")
        );
    }

    #[test]
    fn test_path_build_with_dest_dir() {
        let path = PathBuf::from("/tmp/file.json");

        // dest_dir provided: PDF should be in dest_dir with input's base name
        assert_eq!(
            get_output_pdf_path(
                Some(PathBuf::from("/tmp/test_path").as_path()),
                path.as_path()
            )
            .unwrap(),
            PathBuf::from("/tmp/test_path/file.pdf")
        );

        assert_eq!(
            get_output_pdf_path(Some(PathBuf::from("/output/dir").as_path()), path.as_path())
                .unwrap(),
            PathBuf::from("/output/dir/file.pdf")
        );
    }

    #[test]
    fn test_path_build_real_dir_pdf() {
        let path = PathBuf::from("/tmp/file.json");
        let new_dest_path = temp_dir();

        // Real directory: PDF should be created inside it
        assert_eq!(
            get_output_pdf_path(None, path.as_path()).unwrap(),
            PathBuf::from("/tmp/file.pdf")
        );
        assert_eq!(
            get_output_pdf_path(Some(new_dest_path.as_path()), path.as_path()).unwrap(),
            new_dest_path.join("file.pdf")
        );
    }

    #[test]
    fn test_path_build_file_as_dest_dir_fails() {
        let path = PathBuf::from("/tmp/file.json");
        let fake_file = NamedTempFile::new().unwrap();

        // Passing a file as dest_dir should return an error
        let result = get_output_pdf_path(Some(fake_file.path()), path.as_path());
        assert!(result.is_err());
    }
}
