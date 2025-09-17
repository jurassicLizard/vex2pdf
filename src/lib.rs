//! # vex2pdf library
//!
//! Core functionality for converting CycloneDX VEX documents to PDF format.
//!
//! ## CycloneDX Compatibility
//!
//! This library fully supports CycloneDX schema version 1.5 and provides compatibility
//! for version 1.6 documents that only use 1.5 fields. Documents using 1.6-specific
//! fields may not process correctly.
//!
//! ## Features
//!
//! This library provides:
//! - PDF generation capabilities for CycloneDX VEX documents
//! - Support for various VEX elements including vulnerabilities, components, and metadata
//! - Flexible font configuration with environment variable support
//!
//! ## Vulnerabilities Section Behavior
//!
//! By default, the library will:
//! - Display a "Vulnerabilities" section with vulnerability details when vulnerabilities exist
//! - Display a "Vulnerabilities" section with a "No Vulnerabilities reported" message when no vulnerabilities exist
//! - The "No Vulnerabilities" message display can be controlled with the `VEX2PDF_NOVULNS_MSG` environment variable
//!   (set to "false" to hide the section entirely when no vulnerabilities exist)

//!
//! ## Font Configuration
//!
//! Liberation Sans Fonts are embedded and no extra configuration is needed for fonts
//!
//! ## Architecture
//!
//! The library is organized into modules:
//! - `pdf`: PDF generation functionality
//!   - `font_config`: Font configuration and discovery
//!   - `generator`: PDF document generation
//! - `lib_utils`: Utilities and data models used in this library and accompanying runnable
//!
//! For installation instructions, usage examples, and project overview,
//! see the [project README](https://github.com/jurassicLizard/vex2pdf/blob/master/README.md).
//!
// Re-export and simplify paths for consumers of this library
pub use crate::lib_utils::run_utils as utils;

pub mod pdf {
    pub mod font_config;
    pub mod generator;
}

pub mod lib_utils {
    pub mod config;
    pub mod env_vars;
    pub mod cli_args;
    pub mod input_file_type;
    pub mod run_utils;
}

use crate::lib_utils::run_utils::print_copyright;
use lib_utils::config::Config;
use lib_utils::input_file_type::InputFileType;
use lib_utils::run_utils::{find_files, parse_files};
use pdf::generator::PdfGenerator;
use std::error::Error;

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
/// if let Err(e) = vex2pdf::run(&config) {
///     eprintln!("Application error: {e}");
///     process::exit(1);
/// }
/// ```
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {

    if config.show_oss_licenses {
        // show OSS licenses and return
        show_full_licenses();

        // abort any processing
        return Ok(());
    }

    // Begin pdf generation
    let report_title = config.report_title.as_deref();
    let pdf_name = config.pdf_meta_name.as_deref();

    let pdf_generator = PdfGenerator::new(
        report_title,
        pdf_name,
        config.show_novulns_msg,
        config.pure_bom_novulns,
        config.show_components,
    );

    // Find json files
    let json_files = find_files(config, InputFileType::JSON)?;
    // Generate PDFs out of given json files
    parse_files(&pdf_generator, &json_files, InputFileType::JSON);

    // Find xml files and parse them
    let xml_files = find_files(config, InputFileType::XML)?;
    // Generate PDFs out of given xml files
    parse_files(&pdf_generator, &xml_files, InputFileType::XML);

    Ok(())
}

/// Helper to show OSS License information
fn show_full_licenses() {

    print_copyright();
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
