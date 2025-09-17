use super::config::Config;
use super::input_file_type::InputFileType;
use super::run_utils;
use crate::pdf::generator::PdfGenerator;
use cyclonedx_bom::errors::{BomError, JsonReadError, XmlReadError};
use cyclonedx_bom::prelude::Bom;
use std::error::Error;
use std::{fs, io};
use std::path::{Path, PathBuf};

/// Finds files of a given type in the configured working directory.
///
/// Checks configuration to see if processing this file type is enabled,
/// then scans the working directory for matching files.
/// Returns None if processing is disabled for this file type.
pub(crate) fn find_files(
    config: &Config,
    file_type: InputFileType,
) -> Result<Option<Vec<PathBuf>>, Box<dyn Error>> {

    // validate that our target scan path is a folder
    if config.working_path.is_file() { return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput,"Find files called on a regular file. This is an invalid use of this function. Expected a path validation from the caller.")));}

    if let Some(init_process) = config.file_types_to_process.get(&file_type) {
        if !init_process {
            println!(
                "Skipping {} files : deactivated by user",
                file_type.as_str_uppercase()
            );
            return Ok(None);
        }
    }
    println!(
        "Scanning for {} files in: {}",
        file_type.as_str_uppercase(),
        config.working_path.display()
    );

    let mut files: Vec<PathBuf> = Vec::new();

    // at this point we have already validated that working_path is a directory we can proceed
    for entry in fs::read_dir(&config.working_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension.to_string_lossy().to_lowercase() == file_type.as_str_lowercase() {
                    files.push(path);
                }
            }
        }
    }

    // inform over search results
    if files.is_empty() {
        println!(
            "No {} files found in the current directory.",
            file_type.as_str_uppercase()
        );
    } else {
        println!(
            "Found {} {} files",
            files.len(),
            file_type.as_str_uppercase()
        );
    }

    Ok(Some(files))
}

/// Processes a list of files found by find_files() and generates PDFs.
///
/// Iterates through each file in the provided list, attempts to parse it
/// according to the specified input file type, and generates a PDF if successful.
/// Does nothing if the files parameter is None.
//
/// # Parameters
///
/// `pdf_generator`: A reference to the PdfGenerator
/// `files`: An Optional list of files or None. Use Cases for None include the use-case when
///  file processing is disabled for a certain type and we pass None to avoid parsing all together
/// `input_file_type`: the file type to process
///
pub(crate) fn parse_files(
    pdf_generator: &PdfGenerator,
    files: &Option<Vec<PathBuf>>,
    input_file_type: InputFileType,
) {
    if let Some(files) = &files {
        // Process each JSON file
        for file_path in files {
            println!("Processing: {}", file_path.display());

            // Try to parse the JSON file as a CycloneDX Bom
            let parse_res = if input_file_type == InputFileType::JSON {
                run_utils::parse_vex_json(file_path)
            } else {
                run_utils::parse_vex_xml(file_path)
            };

            match parse_res {
                Ok(vex) => {
                    // Generate output PDF path with same base name
                    let output_path = run_utils::get_output_pdf_path(file_path);

                    println!("Generating PDF: {}", output_path.display());

                    // Generate the PDF
                    match pdf_generator.generate_pdf(&vex, &output_path) {
                        Ok(_) => println!("Successfully generated PDF: {}", output_path.display()),
                        Err(e) => {
                            println!("Failed to generate PDF for {}: {}", file_path.display(), e)
                        }
                    }
                }
                Err(e) => println!("Failed to parse {}: {}", file_path.display(), e),
            }
        }
    }
}

/// Parses an XML file into a CycloneDX Bom object.
///
/// Reads the file content and attempts to parse it as a CycloneDX 1.5 XML document.
/// Includes special handling for CycloneDX 1.6 documents by attempting to downgrade
/// them to version 1.5 by modifying the namespace.
///
/// Note: The downgrade from 1.6 to 1.5 is a compatibility feature and may not work
/// if the document uses 1.6-specific fields.
pub(crate) fn parse_vex_xml(path: &Path) -> Result<Bom, Box<dyn Error>> {
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
pub(crate) fn parse_vex_json(path: &Path) -> Result<Bom, Box<dyn Error>> {
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
    println!();
    println!("NOTE: Downgrading CycloneDX BOM from spec version 1.6 to 1.5");
    println!("Reason: Current implementation does not yet fully support spec version 1.6");
    println!("Warning: This compatibility mode only works for BOMs that don't utilize 1.6-specific fields");
    println!("         Processing will fail if 1.6-specific fields are encountered");
    println!();
}

/// Constructs an output PDF path based on the input file path.
///
/// Creates a new path with the same base name as the input file but with a .pdf extension.
/// Used internally to determine where to save generated PDF files.
pub fn get_output_pdf_path(file_path: &Path) -> PathBuf {
    if let Some(file_stem) = file_path.file_stem() {
        file_path.with_file_name(format!("{}.pdf", file_stem.to_string_lossy()))
    } else {
        file_path.with_extension("pdf")
    }
}

/// Prints the application version, copyright, and license information.
pub fn print_copyright() {
    println!(
        "vex2pdf v{} - CycloneDX (VEX) to PDF Converter",
        env!("CARGO_PKG_VERSION")
    );
    println!("Copyright (c) 2025 Salem B. - MIT Or Apache 2.0 License");
    println!();
}
