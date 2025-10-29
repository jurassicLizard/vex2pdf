//! # vex2pdf - CycloneDX to PDF Converter
//!
//! A Rust library for converting CycloneDX VEX/VDR/SBOM documents (JSON and XML formats)
//! to professional PDF reports with embedded fonts, color-coded vulnerability analysis,
//! and concurrent processing support.
//!
//! ## CycloneDX Compatibility
//!
//! This library fully supports CycloneDX schema version 1.5 and provides compatibility
//! for version 1.6 documents that only use 1.5 fields. Documents using 1.6-specific
//! fields may not process correctly.
//!
//! ## Quick Start
//!
//! ```rust
//! use vex2pdf::lib_utils::config::Config;
//! use vex2pdf::run;
//!
//! let config = Config::default()
//!     .working_path("./input")
//!     .output_dir("./output")
//!     .max_jobs(Some(4));
//!
//! run(config).expect("Failed to process files");
//! ```
//!
//! ## Configuration
//!
//! Configuration is managed through the [`Config`](lib_utils::config::Config) struct
//! using the builder pattern for flexibility.
//!
//! ### Builder Pattern (Recommended)
//!
//! Use method chaining to configure exactly what you need:
//!
//! ```rust
//! use vex2pdf::lib_utils::config::Config;
//!
//! let config = Config::default()
//!     .working_path("./input")
//!     .output_dir("./output")
//!     .max_jobs(Some(4))
//!     .report_title("Q4 2024 Security Report")
//!     .show_components(true);
//! ```
//!
//! ### Available Builder Methods
//!
//! - [`working_path()`](lib_utils::config::Config::working_path) - Set input directory
//! - [`output_dir()`](lib_utils::config::Config::output_dir) - Set output directory
//! - [`max_jobs()`](lib_utils::config::Config::max_jobs) - Control concurrent processing
//! - [`report_title()`](lib_utils::config::Config::report_title) - Custom report title
//! - [`pdf_meta_name()`](lib_utils::config::Config::pdf_meta_name) - Custom PDF metadata
//! - [`show_novulns_msg()`](lib_utils::config::Config::show_novulns_msg) - Show/hide "no vulnerabilities" message
//! - [`show_components()`](lib_utils::config::Config::show_components) - Show/hide components list
//! - [`pure_bom_novulns()`](lib_utils::config::Config::pure_bom_novulns) - Treat as pure BOM
//! - And more...
//!
//! See [`Config`](lib_utils::config::Config) documentation for the complete list.
//!
//! ### For CLI Applications
//!
//! CLI applications should use `Config::build_from_env_cli()` to parse
//! command-line arguments and environment variables.
//!
//! For detailed CLI documentation:
//! - [Configuration Guide](https://gitlab.com/jurassicLizard/vex2pdf/-/blob/master/README.md#configuration)
//! - [Environment Variables Reference](https://gitlab.com/jurassicLizard/vex2pdf/-/blob/master/README.md#environment-variables)
//!
//! ## Features
//!
//! - **Multi-format support**: JSON and XML CycloneDX documents
//! - **Document types**: VEX, VDR, and SBOM/BOM
//! - **Vulnerability analysis rendering**: Color-coded states (Exploitable, Resolved, In Triage, etc.) and response actions
//! - **Concurrent processing**: Custom threadpool with configurable job limits (single-threaded to max parallelism)
//! - **Embedded fonts**: Liberation Sans fonts built-in, no external dependencies
//! - **Structured logging**: Info/debug to stdout, warnings/errors to stderr
//! - **Memory safe**: Unsafe code forbidden at compile-time
//! - **CLI and library**: Use as standalone tool or integrate into your application
//!
//! ## Documentation
//!
//! - **[README](https://gitlab.com/jurassicLizard/vex2pdf/-/blob/master/README.md)** - Installation instructions, CLI usage, environment variables, and configuration
//! - **[Developer Notes](https://gitlab.com/jurassicLizard/vex2pdf/-/blob/master/docs/DEVELOPER_NOTES.md)** - Testing, code coverage, architecture details, and trait system
//! - **[API Documentation](https://docs.rs/vex2pdf)** - Full API reference on docs.rs
//! - **[Changelog](https://gitlab.com/jurassicLizard/vex2pdf/-/blob/master/CHANGELOG.md)** - Version history and release notes
//!
//! ## Library Architecture
//!
//! The library is organized into modules:
//! - `pdf`: PDF generation functionality
//!   - `font_config`: Embedded font management
//!   - `generator`: PDF document generation with analysis rendering
//! - `lib_utils`: Configuration, CLI arguments, environment variables, and concurrency
//!   - `concurrency`: Custom threadpool and worker implementation
//! - `files_proc`: File discovery, processing pipeline, and trait system
//!   - `processor`: Main processing logic with trait abstractions
//!   - `model`: File identification and processing state
//!

#![forbid(unsafe_code)]
// Re-export and simplify paths for consumers of this library
pub use crate::lib_utils::run_utils as utils;
// re-export upstream cyclondx bom path
pub use cyclonedx_bom;

pub mod files_proc {
    pub mod model {
        pub mod file_ident;
        pub mod files_pending_proc;
        pub mod input_file_type;
    }
    pub mod processor;
    pub mod traits;
}
pub mod pdf {
    pub mod font_config;
    pub mod generator;
}

pub mod lib_utils {
    pub mod errors;

    pub mod cli_args;
    pub mod config;
    pub mod env_vars;
    pub mod run_utils;
    pub(crate) mod concurrency {
        pub(crate) mod common;
        pub(crate) mod threadpool;
        pub(crate) mod worker;
    }
}

use crate::files_proc::processor::DefaultFilesProcessor;
use crate::files_proc::traits::{FileSearchProvider, MultipleFilesProcProvider};
use crate::lib_utils::errors::Vex2PdfError;
use lib_utils::config::Config;

/// Processes CycloneDX VEX documents according to the provided configuration.
///
/// This function serves as the main entry point for the library's functionality.
/// It finds and processes both JSON and XML files as specified in the configuration,
/// converting them to PDF format with embedded fonts.
///
/// # Arguments
///
/// * `config` - Configuration settings that control processing behavior
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - Success (`Ok`) if processing completes without errors,
///   or an error (`Err`) if something goes wrong
///
/// # Behavior
///
/// When `show_oss_licenses` is enabled in the configuration, this function displays
/// license information and exits without processing any files.
///
/// Otherwise, it performs these operations in sequence:
/// 1. Finds JSON files according to the configuration
/// 2. Processes found JSON files to generate PDFs
/// 3. Finds XML files according to the configuration
/// 4. Processes found XML files to generate PDFs
///
/// # Fonts
///
/// Liberation Sans fonts are embedded in the generated PDFs, eliminating the need
/// for font installation on the system viewing the PDFs.
///
/// # Environment Variables
///
/// Various aspects of PDF generation can be controlled through environment variables:
/// - `VEX2PDF_NOVULNS_MSG`: Controls whether to show a message when no vulnerabilities exist
/// - `VEX2PDF_REPORT_TITLE`: Sets a custom title for the report
/// - `VEX2PDF_PDF_META_NAME`: Sets the PDF metadata name
/// - `VEX2PDF_VERSION_INFO`: Shows version information before executing normally
///
/// # Example
///
/// ```
/// use std::process;
/// use vex2pdf::lib_utils::config::Config;
/// use vex2pdf::run;
///
/// let config = Config::build().unwrap_or_else(|err| {
///     eprintln!("Problem setting up working environment:");
///     eprintln!("{}", { err });
///     process::exit(1);
/// });
///
/// if let Err(e) = vex2pdf::run(config) {
///     eprintln!("Application error: {e}");
///     process::exit(1);
/// }
/// ```
pub fn run(config: Config) -> Result<(), Vex2PdfError> {
    let _ = DefaultFilesProcessor::new(config).find_files()?.process();

    Ok(())
}

/// Helper to show OSS License information
pub fn show_full_licenses() {
    let main_license_text = r#"VEX2PDF is licensed under either MIT or Apache License, Version 2.0 at your option.
license text can be found under: https://gitlab.com/jurassicLizard/vex2pdf/-/blob/master/README.md#license"#;

    println!("{main_license_text}");
    println!();
    println!("-----------------------------------------------------------------------------\n");
    println!("This software makes use of Liberation Fonts licensed under SIL as follows : ");
    println!();
    let sil_license_text = include_bytes!("../external/fonts/liberation-fonts/LICENSE");

    println!("{}", String::from_utf8_lossy(sil_license_text));
}

#[cfg(test)]
mod tests {
    use cyclonedx_bom::models::bom::Bom;
    use cyclonedx_bom::models::metadata::Metadata;
    use cyclonedx_bom::models::tool::{Tool, Tools};
    use cyclonedx_bom::models::vulnerability::{Vulnerabilities, Vulnerability};

    use cyclonedx_bom::models::vulnerability_rating::{
        Score, ScoreMethod, Severity, VulnerabilityRating, VulnerabilityRatings,
    };
    use cyclonedx_bom::prelude::{DateTime, NormalizedString, SpecVersion, UrnUuid};
    use std::fs;
    use std::io::BufReader;

    fn create_sample_vex() -> Bom {
        // Create a VEX document following CycloneDX structure

        Bom {
            spec_version: SpecVersion::V1_5,
            version: 1,
            serial_number: Some(
                UrnUuid::new("urn:uuid:3e671687-395b-41f5-a30f-a58921a69b79".to_string())
                    .expect("Unable to create urn:uuid"),
            ),
            metadata: Some(Metadata {
                timestamp: Some(DateTime::now().expect("failed to convert date")),
                tools: Some(Tools::List(vec![Tool {
                    name: Some(NormalizedString::new("my_tool")),
                    ..Tool::default()
                }])),
                ..Metadata::default()
            }),
            vulnerabilities: Some(Vulnerabilities(vec![
                Vulnerability {
                    bom_ref: None,
                    id: None,
                    vulnerability_source: None,
                    description: Some(
                        "Known vulnerability in library that allows unauthorized access"
                            .to_string(),
                    ),
                    detail: Some(
                        "Detailed explanation of the vulnerability and its potential impact."
                            .to_string(),
                    ),
                    recommendation: Some("Upgrade to version 1.2.4 or later".to_string()),
                    workaround: None,
                    proof_of_concept: None,
                    advisories: None,
                    created: None,
                    published: None,
                    updated: None,
                    rejected: None,
                    vulnerability_credits: None,
                    tools: None,
                    vulnerability_analysis: None,
                    vulnerability_targets: None,
                    vulnerability_ratings: Some(VulnerabilityRatings(vec![VulnerabilityRating {
                        score: Some(Score::from(8.1)),
                        severity: Some(Severity::High),
                        score_method: Some(ScoreMethod::CVSSv31),
                        vector: Some(NormalizedString::new(
                            "CVSS:3.1/AV:N/AC:L/PR:N/UI:R/S:U/C:H/I:H/A:H",
                        )),
                        vulnerability_source: None,
                        justification: None,
                    }])),

                    vulnerability_references: None,
                    cwes: None,
                    properties: None,
                },
                Vulnerability {
                    bom_ref: None,
                    id: None,
                    vulnerability_source: None,
                    description: Some("Component does not use the affected library".to_string()),
                    detail: Some(
                        "Detailed explanation of the vulnerability and its potential impact."
                            .to_string(),
                    ),
                    recommendation: Some("Upgrade to version 1.2.3 or later".to_string()),
                    workaround: None,
                    proof_of_concept: None,
                    advisories: None,
                    created: None,
                    published: None,
                    updated: None,
                    rejected: None,
                    vulnerability_credits: None,
                    tools: None,
                    vulnerability_analysis: None,
                    vulnerability_targets: None,
                    vulnerability_ratings: Some(VulnerabilityRatings(vec![VulnerabilityRating {
                        score: Some(Score::from(6.5)),
                        severity: Some(Severity::High),
                        score_method: Some(ScoreMethod::CVSSv31),
                        vector: Some(NormalizedString::new(
                            "CVSS:3.1/AV:N/AC:L/PR:L/UI:N/S:U/C:L/I:L/A:L",
                        )),
                        vulnerability_source: None,
                        justification: None,
                    }])),

                    vulnerability_references: None,
                    cwes: None,
                    properties: None,
                },
            ])),
            ..Bom::default()
        }
    }

    #[test]
    fn test_vex_serialization() {
        let vex = create_sample_vex();

        // Test serialization
        let mut output = Vec::<u8>::new();
        vex.clone()
            .output_as_json_v1_5(&mut output)
            .expect("failed to read vex object");

        let json_str = String::from_utf8(output).expect("failed to serialize json object");

        println!("Serialized VEX: {}", json_str);

        let parsed_json =
            serde_json::from_str(&json_str).expect("serde failed to read json from string object");
        let deserialization_result = Bom::parse_json_value(parsed_json);

        // Test deserialization
        match deserialization_result {
            Ok(deserialized) => {
                println!("Deserialized CycloneDX: {:?}", deserialized);
                // Verify the round trip works
                assert_eq!(vex.serial_number, deserialized.serial_number);
                assert_eq!(vex.spec_version, deserialized.spec_version);
            }
            Err(err) => {
                panic!("Deserialization failed: {:?}", err);
            }
        }
    }

    #[test]
    fn test_vex_json_file_io() {
        use std::io::Write;

        let vex = create_sample_vex();
        let mut output = Vec::<u8>::new();
        vex.clone()
            .output_as_json_v1_5(&mut output)
            .expect("failed to read vex object");
        let json_str = String::from_utf8(output).expect("failed to serialize json object");

        // Create a temporary file
        let mut temp_file = std::env::temp_dir();
        temp_file.push("test_vex.json");

        // Write the VEX to the file
        let mut file = fs::File::create(&temp_file).expect("Failed to create temp file");
        file.write_all(json_str.as_bytes())
            .expect("Failed to write to temp file");

        // Read it back
        let content_reader =
            BufReader::new(fs::File::open(&temp_file).expect("failed to open file"));
        let loaded_vex: Bom = Bom::parse_from_json(content_reader).expect("Failed to parse JSON");

        // Clean up
        fs::remove_file(&temp_file).expect("Failed to remove temp file");

        // Verify
        assert_eq!(vex.serial_number, loaded_vex.serial_number);
    }

    #[test]
    fn test_vex_xml_file_io() {
        use std::io::Write;

        let vex = create_sample_vex();
        let mut output = Vec::<u8>::new();
        vex.clone()
            .output_as_xml_v1_5(&mut output)
            .expect("failed to read vex object");
        let xml_str = String::from_utf8(output).expect("failed to serialize json object");

        // Create a temporary file
        let mut temp_file = std::env::temp_dir();
        temp_file.push("test_vex.xml");

        // Write the VEX to the file
        let mut file = fs::File::create(&temp_file).expect("Failed to create temp file");
        file.write_all(xml_str.as_bytes())
            .expect("Failed to write to temp file");

        // Read it back
        let content_reader =
            BufReader::new(fs::File::open(&temp_file).expect("failed to open file"));
        let loaded_vex: Bom =
            Bom::parse_from_xml_v1_5(content_reader).expect("Failed to parse JSON");

        // Clean up
        fs::remove_file(&temp_file).expect("Failed to remove temp file");

        // Verify
        assert_eq!(vex.serial_number, loaded_vex.serial_number);
    }

    #[test]
    fn test_generate_sample_file() {
        let vex = create_sample_vex();
        let mut output = Vec::<u8>::new();
        vex.clone()
            .output_as_json_v1_5(&mut output)
            .expect("failed to read vex object");
        let json_str = String::from_utf8(output).expect("failed to serialize json object");

        // Create a sample file in the current directory
        fs::write("sample_vex.json", json_str).expect("Failed to write sample file");

        println!("Sample VEX file created at sample_vex.json");
    }

    #[test]
    fn test_novulns_msg_env_var_handling() {
        use crate::lib_utils::env_vars::EnvVarNames;
        use std::env;

        // Save original env var value
        let original = env::var(EnvVarNames::NoVulnsMsg.as_str()).ok();

        // Test setting and retrieving the env var
        env::remove_var(EnvVarNames::NoVulnsMsg.as_str());
        assert_eq!(
            env::var(EnvVarNames::NoVulnsMsg.as_str()).is_err(),
            true,
            "Env var should not exist initially"
        );

        env::set_var(EnvVarNames::NoVulnsMsg.as_str(), "false");
        assert_eq!(
            env::var(EnvVarNames::NoVulnsMsg.as_str()).unwrap(),
            "false",
            "Env var should be retrievable with correct value"
        );

        // Clean up
        if let Some(val) = original {
            env::set_var(EnvVarNames::NoVulnsMsg.as_str(), val);
        } else {
            env::remove_var(EnvVarNames::NoVulnsMsg.as_str());
        }
    }

    #[test]
    fn test_embedded_fonts_load_correctly() {
        use crate::pdf::font_config::FontsDir;

        FontsDir::build();
    }

    #[cfg(test)]
    mod tests {
        use crate::lib_utils::env_vars::EnvVarNames;
        use std::env;

        #[test]
        fn test_env_var_behavior() {
            // Use a different enum variant for each test section to avoid interference

            // Test is_on when var not set
            {
                let var = EnvVarNames::ProcessXml;
                env::remove_var(var.as_str());
                assert_eq!(
                    var.is_on(),
                    false,
                    "is_on() should return false when var not set"
                );
            }

            // Test is_on with true values
            {
                let var = EnvVarNames::ProcessXml;
                for value in &["true", "True", "TRUE", "yes", "YES", "1", "on", "ON"] {
                    env::set_var(var.as_str(), value);
                    assert_eq!(var.is_on(), true, "is_on() failed for value: {}", value);
                    env::remove_var(var.as_str()); // Clean up after each test
                }
            }

            // Test is_on with false values
            {
                let var = EnvVarNames::ProcessXml;
                for value in &["false", "False", "FALSE", "no", "NO", "0", "off", "OFF"] {
                    env::set_var(var.as_str(), value);
                    assert_eq!(var.is_on(), false, "is_on() failed for value: {}", value);
                    env::remove_var(var.as_str()); // Clean up after each test
                }
            }

            // Test is_on_or_unset when var not set
            {
                let var = EnvVarNames::ProcessXml;
                env::remove_var(var.as_str());
                assert_eq!(
                    var.is_on_or_unset(),
                    true,
                    "is_on_or_unset() should return true when var not set"
                );
            }

            // Test is_on_or_unset with true values
            {
                let var = EnvVarNames::ProcessXml;
                for value in &["true", "True", "TRUE", "yes", "YES", "1", "on", "ON"] {
                    env::set_var(var.as_str(), value);
                    assert_eq!(
                        var.is_on_or_unset(),
                        true,
                        "is_on_or_unset() failed for value: {}",
                        value
                    );
                    env::remove_var(var.as_str()); // Clean up after each test
                }
            }

            // Test is_on_or_unset with false values
            {
                let var = EnvVarNames::ProcessXml;
                for value in &["false", "False", "FALSE", "no", "NO", "0", "off", "OFF"] {
                    env::set_var(var.as_str(), value);
                    assert_eq!(
                        var.is_on_or_unset(),
                        false,
                        "is_on_or_unset() failed for value: {}",
                        value
                    );
                    env::remove_var(var.as_str()); // Clean up after each test
                }
            }
        }

        #[test]
        fn test_get_value() {
            use std::env;

            // Test with an environment variable that doesn't exist
            {
                let var = EnvVarNames::ReportTitle;
                env::remove_var(var.as_str());
                assert_eq!(
                    var.get_value(),
                    None,
                    "Should return None for non-existent env var"
                );
            }

            // Test with an environment variable that exists
            {
                let var = EnvVarNames::PdfName;
                let test_value = "Test PDF Name";
                env::set_var(var.as_str(), test_value);
                assert_eq!(
                    var.get_value(),
                    Some(test_value.to_string()),
                    "Should return the value of the env var"
                );
                env::remove_var(var.as_str()); // Clean up
            }

            // Test with an empty string value
            {
                let var = EnvVarNames::ReportTitle;
                env::set_var(var.as_str(), "");
                assert_eq!(
                    var.get_value(),
                    Some("".to_string()),
                    "Should return an empty string for empty env var"
                );
                env::remove_var(var.as_str()); // Clean up
            }
        }
    }
}
