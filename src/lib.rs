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
//!
//! ## Architecture
//!
//! The library is organized into modules:
//! - `pdf`: PDF generation functionality

// Re-export cyclonedx-bom models for use by consumers of this library
pub use cyclonedx_bom as model;

pub mod pdf {
    pub mod generator;
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
    fn test_vex_file_io() {
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
}
