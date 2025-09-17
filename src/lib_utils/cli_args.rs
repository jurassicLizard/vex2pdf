//! This section is dedicated to all logic related to argument handling for the cli
//! All  Arguments are optional and the software is designed to work with default values
//! Whether it is environment variables or cli arguments.
//!

use std::{fs, io};
use std::path::PathBuf;
use clap::Parser;
use super::env_vars::EnvVarNames;
#[derive(Parser)]
#[command(version,about,long_about = None)]
pub struct CliArgs {
    /// File to process (JSON or XML). Please note that this tool is designed for batch processing
    /// so If this is not set the tool scans the current directory for all parseable files and converts them
    #[arg(value_name = "FILE_TO_PROCESS")]
    pub file: Option<PathBuf>,

    /// Show version information and exit
    #[arg(short, long)]
    pub version: bool,

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
    #[arg(short, long)]
    pub show_components: Option<bool>,

    /// Sets the directory where the parser should output the files
    #[arg(short='d', long="output-dir", env=EnvVarNames::OutputDir.as_str())]
    pub output_dir: Option<PathBuf>

}

impl CliArgs {

    /// validates paths that may be passed by the user and verifies write permission
    pub fn validate(&self) -> Result<(), io::Error> {

        if let Some(path) = self.output_dir.as_ref() {
            if ! path.is_dir() {
                return Err(io::Error::new(io::ErrorKind::InvalidInput,"Expected a directory"));
            } else {
                // test if we have permissions to write

                let tmp_file= path.join("vex2pdf_perm_test_file");
                let res_io = fs::File::create(&tmp_file);

                if let Err(_) = res_io {
                    return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Could not create a test file. possible permissions issue"));
                }else if let Ok(_) =  res_io {
                    if let Err(_) = fs::remove_file(tmp_file) {
                        return Err(io::Error::new(io::ErrorKind::PermissionDenied, "unable to delete permissions test file"));
                    }
                }

            }
        }

        Ok(())
    }
}