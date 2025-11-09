// Paths to test artifacts and executables
#[allow(unused)]
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

    // Analysis test files - JSON
    pub const BOM_VDR_WITH_ANALYSIS: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/bom_src/run_test/bom_vdr_with_analysis.json"
    );

    // Analysis test files - XML
    pub const BOM_VDR_WITH_ANALYSIS_XML: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/bom_src/run_test_xml/bom_xml_vdr_with_analysis.xml"
    );

    // Expected PDFs for analysis tests
    pub const EXPECTED_BOM_VDR_WITH_ANALYSIS_PDF: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/expected_pdfs/bom_vdr_with_analysis.pdf"
    );

    pub const EXPECTED_BOM_VDR_WITH_ANALYSIS_XML_PDF: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_artifacts/expected_pdfs/bom_xml_vdr_with_analysis.pdf"
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
    pub fn bytes_to_str(bytes: &[u8]) -> Cow<'_, str> {
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
    ///
    /// # Release Mode Behavior
    /// In release mode, content comparison is skipped because PDF libraries optimize
    /// differently, resulting in binary differences that don't indicate test failures.
    /// Only PDF creation is verified in release mode.
    pub fn assert_pdf_content_similar(generated: &Path, expected: &Path) {
        // Skip content comparison in release mode due to PDF library optimization differences
        #[cfg(not(debug_assertions))]
        {
            eprintln!(
                "ℹ️  Release mode: Skipping PDF content comparison for {:?}",
                generated.file_name().unwrap_or_default()
            );
            return;
        }

        #[cfg(debug_assertions)]
        {
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
                    let gen_stripped_path =
                        err_dir.join(format!("{}_generated_raw_dump.txt", stem));
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

    /// Normalize PDF content by stripping dynamic elements (timestamps, IDs)
    /// This is the same normalization used by assert_pdf_content_similar
    pub fn normalize_pdf_content(pdf_path: &Path) -> String {
        let content = fs::read(pdf_path).expect("Failed to read PDF file");
        let content_str = String::from_utf8_lossy(&content);
        strip_pdf_timestamps(&content_str)
    }

    /// Calculate BLAKE3 checksum of normalized PDF content
    pub fn calculate_normalized_checksum(pdf_path: &Path) -> String {
        let normalized = normalize_pdf_content(pdf_path);
        let hash = blake3::hash(normalized.as_bytes());
        hash.to_hex().to_string()
    }

    /// Assert that a generated PDF's normalized checksum matches the expected checksum
    /// from the checksums file. This validates PDF content while ignoring dynamic elements.
    pub fn assert_pdf_checksum_matches(generated_pdf: &Path) {
        let checksums_path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/test_artifacts/expected_pdfs_chksums.txt"
        ));

        let checksums_content =
            fs::read_to_string(checksums_path).expect("Failed to read checksums file");

        let pdf_filename = generated_pdf
            .file_name()
            .expect("Failed to get PDF filename")
            .to_str()
            .expect("Invalid filename");

        // Find the expected checksum for this PDF
        let expected_checksum = checksums_content
            .lines()
            .find(|line| line.ends_with(pdf_filename))
            .map(|line| line.split_whitespace().next().unwrap())
            .unwrap_or_else(|| panic!("No checksum found for {} in checksums file", pdf_filename));

        // Calculate the checksum of the generated PDF
        let actual_checksum = calculate_normalized_checksum(generated_pdf);

        if actual_checksum != expected_checksum {
            // Copy failed PDF to err_pdfs for debugging
            let err_dir = Path::new(super::paths::ERRONEOUS_PDFS_DIR);
            if let Err(e) = fs::create_dir_all(err_dir) {
                eprintln!("Warning: Failed to create err_pdfs directory: {}", e);
            }

            if let Some(filename) = generated_pdf.file_name() {
                let err_pdf_path = err_dir.join(filename);
                if let Err(e) = fs::copy(generated_pdf, &err_pdf_path) {
                    eprintln!("Warning: Failed to copy failed PDF to err_pdfs: {}", e);
                } else {
                    eprintln!("\n⚠️  Failed PDF copied to: {:?}", err_pdf_path);
                }
            }

            panic!(
                "\n\nPDF checksum mismatch for: {}\n\
                 Expected: {}\n\
                 Actual:   {}\n\
                 \n\
                 The generated PDF differs from the expected content.\n\
                 Failed PDF copied to {:?} for inspection.\n",
                pdf_filename,
                expected_checksum,
                actual_checksum,
                super::paths::ERRONEOUS_PDFS_DIR
            );
        }
    }

    /// Helper function to count PDF files in a directory
    pub fn count_pdf_files(dir: &Path) -> usize {
        std::fs::read_dir(dir)
            .expect("Failed to read directory")
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let path = entry.path();
                path.is_file()
                    && path
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext == "pdf")
                        .unwrap_or(false)
            })
            .count()
    }

    /// Helper function to count processable files (.json and .xml) in a directory
    pub fn count_processable_files(dir: &Path) -> usize {
        std::fs::read_dir(dir)
            .expect("Failed to read directory")
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let path = entry.path();
                path.is_file()
                    && path
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext == "json" || ext == "xml")
                        .unwrap_or(false)
            })
            .count()
    }

    /// Helper function to assert the number of PDF files created
    pub fn assert_pdf_count(dir: &Path, expected: usize) {
        let actual = count_pdf_files(dir);
        assert_eq!(
            actual, expected,
            "Expected {} PDFs but found {} in directory: {:?}",
            expected, actual, dir
        );
    }

    /// Helper function to copy all files from source directory to destination directory
    pub fn copy_directory_files(src_dir: &Path, dest_dir: &Path) -> std::io::Result<usize> {
        let mut count = 0;
        for entry in std::fs::read_dir(src_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap();
                std::fs::copy(&path, dest_dir.join(file_name))?;
                count += 1;
            }
        }
        Ok(count)
    }
}
