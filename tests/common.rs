// General constants for test directory names
pub mod names {
    pub const EXPECTED_PDFS_DIR_NAME: &str = "expected_pdfs";
    pub const ERRONEOUS_PDFS_DIR_NAME: &str = "err_pdfs";
}

// Paths to test artifacts and executables
pub mod paths {
    pub const PATH_TO_EXE: &str = env!("CARGO_BIN_EXE_vex2pdf");
    pub const DEFAULT_WORKING_DIR: &str = env!("CARGO_MANIFEST_DIR");

    pub const REFERENCE_PDFS_DIR: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/expected_pdfs"
    );

    pub const ERRONEOUS_PDFS_DIR: &str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test_artifacts/err_pdfs");

    pub const SOURCE_BOMS_PARENT_DIR: &str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test_artifacts/bom_src");

    pub const SOURCE_BOMS_BASE_ARTIFACTS_DIR: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/bom_src/run_test"
    );

    pub const SOURCE_BOMS_NOVULNS_ARTIFACTS_DIR: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/bom_src/run_test_novulns"
    );

    pub const SOURCE_BOMS_XML_ARTIFACTS_DIR: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/bom_src/run_test_xml"
    );

    // Individual test file paths - JSON
    pub const SIMPLE_BOM_PATH: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/bom_src/run_test/bom_vex_simple_one_vuln.json"
    );

    pub const BOM_VDR_MINIMAL_WITH_VULNS: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/bom_src/run_test/bom_vdr_minimal_with_vulns.json"
    );

    pub const BOM_VDR_WITH_GHSA_ENTRIES: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/bom_src/run_test/bom_vdr_with_ghsa_entries.json"
    );

    pub const BOM_VDR_WITH_MANY_VULNS: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/bom_src/run_test/bom_vdr_with_many_vulns.json"
    );

    pub const BOM_VDR_WITH_NO_VULNS: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/bom_src/run_test/bom_vdr_with_no_vulns.json"
    );

    pub const BOM_VDR_WITH_LINKS_AS_VERSIONS: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/bom_src/run_test/bom_vdr_with_links_as_versions.json"
    );

    pub const BOM_VEX_WITH_LINKS_AS_VERSIONS: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/bom_src/run_test/bom_vex_with_links_as_versions.json"
    );

    // No vulns test
    pub const BOM_NOVULNS: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/bom_src/run_test/bom_vdr_with_no_vulns.json"
    );

    // XML test files
    pub const BOM_VDR_SIMPLE_XML: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/bom_src/run_test_xml/bom_xml_vdr_simple.xml"
    );

    pub const BOM_VEX_SIMPLE_XML: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/bom_src/run_test_xml/bom_xml_vex_simple.xml"
    );

    pub const SAMPLE_VEX_XML: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/bom_src/run_test_xml/sample_xml_vex.xml"
    );
}

// Utility functions for test assertions, we allow unused because tests expand and as such these functions maybe used temporarily for debugging purposes

#[allow(unused)]
pub mod utils {
    use std::borrow::Cow;
    use std::fs;
    use std::path::Path;
    use std::string::String;

    /// Check if byte output contains the expected text
    pub fn contains_text(bytes: &[u8], text: &str) -> bool {
        String::from_utf8_lossy(bytes).contains(text)
    }

    /// Convert bytes to string (lossy conversion for non-UTF8)
    pub fn bytes_to_str(bytes: &[u8]) -> Cow<str> {
        String::from_utf8_lossy(bytes)
    }

    /// Assert that output contains expected text with helpful error message
    pub fn assert_output_contains(bytes: &[u8], text: &str) {
        let output = String::from_utf8_lossy(bytes);
        assert!(
            output.contains(text),
            "\n\nAssertion failed: output does not contain expected text\n\
             Expected to find: \"{}\"\n\
             Actual output:\n{}\n",
            text,
            output
        );
    }

    /// Assert that a PDF file was created and is valid
    pub fn assert_pdf_created(pdf_path: &Path) {
        assert!(pdf_path.exists(), "PDF file not created at: {:?}", pdf_path);

        let metadata = fs::metadata(pdf_path).expect("Failed to read PDF metadata");
        assert!(metadata.len() > 0, "PDF file is empty");

        let contents = fs::read(pdf_path).expect("Failed to read PDF file");
        assert!(
            contents.starts_with(b"%PDF-"),
            "File is not a valid PDF (missing PDF header)"
        );
    }

    /// Strip timestamp and dynamic ID-related lines from PDF content for comparison
    fn strip_pdf_timestamps(content: &str) -> String {
        content
            .lines()
            .filter(|line| {
                // Filter out timestamp-related lines and dynamic IDs
                !line.contains("CreateDate")
                    && !line.contains("ModifyDate")
                    && !line.contains("MetadataDate")
                    && !line.contains("CreationDate")
                    && !line.contains("ModDate")
                    && !line.contains("InstanceID")  // XMP metadata UUID
                    && !line.contains("DocumentID")  // XMP document UUID
                    && !line.contains("/ID[") // PDF document IDs
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Compare PDF content ignoring timestamps
    /// On failure, copies the generated PDF to err_pdfs directory for debugging
    pub fn assert_pdf_content_similar(generated: &Path, expected: &Path) {
        let gen_content = fs::read(generated).expect("Failed to read generated PDF");
        let exp_content = fs::read(expected).expect("Failed to read expected PDF");

        let gen_content_str = String::from_utf8_lossy(&gen_content);
        let exp_content_str = String::from_utf8_lossy(&exp_content);

        let gen_stripped = strip_pdf_timestamps(&gen_content_str);
        let exp_stripped = strip_pdf_timestamps(&exp_content_str);

        if gen_stripped != exp_stripped {
            // Copy failed PDF and stripped content to err_pdfs for post-mortem analysis
            let err_dir = Path::new(super::paths::ERRONEOUS_PDFS_DIR);
            if let Err(e) = fs::create_dir_all(err_dir) {
                eprintln!("Warning: Failed to create err_pdfs directory: {}", e);
            }

            if let Some(filename) = generated.file_name() {
                let stem = generated.file_stem().unwrap().to_str().unwrap();

                // Copy the failed PDF
                let err_pdf_path = err_dir.join(filename);
                if let Err(e) = fs::copy(generated, &err_pdf_path) {
                    eprintln!("Warning: Failed to copy failed PDF to err_pdfs: {}", e);
                } else {
                    eprintln!("\n⚠️  Failed PDF copied to: {:?}", err_pdf_path);
                }

                // Write stripped generated content
                let gen_stripped_path = err_dir.join(format!("{}_generated_raw_dump.txt", stem));
                if let Err(e) = fs::write(&gen_stripped_path, &gen_stripped) {
                    eprintln!("Warning: Failed to write generated stripped content: {}", e);
                } else {
                    eprintln!("📄 Generated (stripped) saved to: {:?}", gen_stripped_path);
                }

                // Write stripped expected content
                let exp_stripped_path = err_dir.join(format!("{}_expected_raw_dump.txt", stem));
                if let Err(e) = fs::write(&exp_stripped_path, &exp_stripped) {
                    eprintln!("Warning: Failed to write expected stripped content: {}", e);
                } else {
                    eprintln!("📄 Expected (stripped) saved to: {:?}", exp_stripped_path);
                }
            }

            // Panic with minimal output (no binary gibberish)
            panic!(
                "\n\nPDF content differs (timestamps excluded)\n\
                 Generated: {:?}\n\
                 Expected: {:?}\n\
                 \n\
                 Debug files saved to {:?}:\n\
                 - *.pdf (failed PDF)\n\
                 - *_generated.txt (stripped generated content)\n\
                 - *_expected.txt (stripped expected content)\n\
                 \n\
                 Run: diff {:?}/*_generated.txt {:?}/*_expected.txt\n",
                generated,
                expected,
                super::paths::ERRONEOUS_PDFS_DIR,
                super::paths::ERRONEOUS_PDFS_DIR,
                super::paths::ERRONEOUS_PDFS_DIR
            );
        }
    }
}
