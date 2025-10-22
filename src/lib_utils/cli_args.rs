//! This section is dedicated to all logic related to argument handling for the cli
//! All Arguments are optional, and the software is designed to work with default values
//! Whether it is environment variables or cli arguments.
//!

use super::env_vars::EnvVarNames;
use clap::Parser;
use std::path::PathBuf;
use std::{fs, io};
#[derive(Parser)]
#[command(version,about,long_about = None)]
pub struct CliArgs {
    /// File to process (JSON or XML) or Folder containing said file types. Please note that
    /// this tool is designed for batch processing. So If this is not set the tool scans the current directory for all parseable files and converts them.
    /// if a folder is set the tool scans just the first level of the directory (non-recursive).
    #[arg(value_name = "FILE_OR_FOLDER_TO_PROCESS", env= EnvVarNames::WorkingPath.as_str())]
    pub input: Option<PathBuf>,

    #[arg(short='m', long="show-novulns-msg", env= EnvVarNames::NoVulnsMsg.as_str())]
    pub show_novulns_msg: Option<bool>,

    /// Overrides the default title of the report
    #[arg(short='t', long="report-title", value_name = "REPORT_TITLE", env= EnvVarNames::ReportTitle.as_str())]
    pub report_title: Option<String>,

    /// Overrides the default PDF meta name
    #[arg(short='n', long="pdf-meta-name", value_name = "PDF_META_TITLE", env=EnvVarNames::PdfName.as_str())]
    pub meta_name: Option<String>,

    /// Treats the file as a pure bill of materials and shows only the components without the vulnerabilities
    #[arg(short='b', long="bom-novulns", env=EnvVarNames::PureBomNoVulns.as_str())]
    pub pure_bom_novulns: Option<bool>,

    /// Controls whether the component list is shown
    #[arg(short='c', long, env=EnvVarNames::ShowComponentList.as_str())]
    pub show_components: Option<bool>,

    /// Sets the directory where the parser should output the files
    #[arg(short='d', long="output-dir", env=EnvVarNames::OutputDir.as_str())]
    pub output_dir: Option<PathBuf>,
}

impl CliArgs {
    /// validates paths that may be passed by the user and verifies write permission
    pub fn validate(&self) -> Result<(), io::Error> {
        if let Some(path) = self.output_dir.as_ref() {
            if !path.is_dir() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Expected a directory got {}", path.display()),
                ));
            } else {
                // test if we have permissions to write

                let tmp_file = path.join("vex2pdf_perm_test_file");
                let res_io = fs::File::create(&tmp_file);

                if let Err(_) = res_io {
                    return Err(io::Error::new(
                        io::ErrorKind::PermissionDenied,
                        "Could not create a test file. possible permissions issue",
                    ));
                } else if let Ok(_) = res_io {
                    if let Err(_) = fs::remove_file(tmp_file) {
                        return Err(io::Error::new(
                            io::ErrorKind::PermissionDenied,
                            "unable to delete permissions test file",
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_validate_no_output_dir() {
        let args = CliArgs {
            input: None,
            show_novulns_msg: None,
            report_title: None,
            meta_name: None,
            pure_bom_novulns: None,
            show_components: None,
            output_dir: None,
        };
        assert!(args.validate().is_ok());
    }

    #[test]
    fn test_validate_valid_directory() {
        let temp_dir = TempDir::new().unwrap();
        let args = CliArgs {
            input: None,
            show_novulns_msg: None,
            report_title: None,
            meta_name: None,
            pure_bom_novulns: None,
            show_components: None,
            output_dir: Some(temp_dir.path().to_path_buf()),
        };
        assert!(args.validate().is_ok());
    }

    #[test]
    fn test_validate_path_is_file_not_dir() {
        let temp_dir = TempDir::new().unwrap();
        let file = temp_dir.path().join("test.json");
        fs::write(&file, r#"{"test": "data"}"#).unwrap();

        let args = CliArgs {
            input: None,
            show_novulns_msg: None,
            report_title: None,
            meta_name: None,
            pure_bom_novulns: None,
            show_components: None,
            output_dir: Some(file),
        };
        let err = args.validate().unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn test_validate_nonexistent_directory() {
        let args = CliArgs {
            input: None,
            show_novulns_msg: None,
            report_title: None,
            meta_name: None,
            pure_bom_novulns: None,
            show_components: None,
            output_dir: Some(PathBuf::from("/nonexistent/path/that/does/not/exist")),
        };
        let err = args.validate().unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn test_validate_readonly_directory() {
        let temp_dir = TempDir::new().unwrap();
        let readonly_dir = temp_dir.path().join("readonly");
        fs::create_dir(&readonly_dir).unwrap();

        // Set directory to read-only
        let mut perms = fs::metadata(&readonly_dir).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(&readonly_dir, perms).unwrap();

        let args = CliArgs {
            input: None,
            show_novulns_msg: None,
            report_title: None,
            meta_name: None,
            pure_bom_novulns: None,
            show_components: None,
            output_dir: Some(readonly_dir.clone()),
        };

        let err = args.validate().unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::PermissionDenied);

        // Cleanup: restore write permissions so TempDir can delete it
        let mut perms = fs::metadata(&readonly_dir).unwrap().permissions();
        perms.set_readonly(false);
        fs::set_permissions(&readonly_dir, perms).unwrap();
    }

    #[test]
    fn test_validate_can_create_and_delete_test_file() {
        let temp_dir = TempDir::new().unwrap();
        let args = CliArgs {
            input: None,
            show_novulns_msg: None,
            report_title: None,
            meta_name: None,
            pure_bom_novulns: None,
            show_components: None,
            output_dir: Some(temp_dir.path().to_path_buf()),
        };

        // This validates write + delete permissions
        assert!(args.validate().is_ok());

        // Verify no test file was left behind
        assert!(!temp_dir.path().join("vex2pdf_perm_test_file").exists());
    }
}
