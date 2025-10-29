use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

mod common;

use common::paths;
use common::utils;

/// Helper function to run the vex2pdf command and verify success
fn run_vex2pdf(input_path: &str, output_dir: &Path) -> std::process::Output {
    let output = Command::new(paths::PATH_TO_EXE)
        .arg("-d")
        .arg(output_dir)
        .arg(input_path)
        .output()
        .expect("Failed to execute command");

    // Print stderr for debugging if not empty
    if !output.stderr.is_empty() {
        eprintln!("stderr: {}", utils::bytes_to_str(&output.stderr));
    }

    // Assert command succeeded
    assert!(
        output.status.success(),
        "Command failed with status: {}",
        output.status
    );

    // Assert success message
    utils::assert_output_contains(&output.stdout, "Successfully generated PDF:");

    output
}

/// Helper function to get the expected PDF filename from input path
fn get_expected_pdf_name(input_path: &str) -> String {
    let path = Path::new(input_path);
    let stem = path.file_stem().unwrap().to_str().unwrap();
    format!("{}.pdf", stem)
}

// ============================================================================
// JSON BOM Tests
// ============================================================================

#[test]
fn test_simple_bom_with_one_vuln() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    run_vex2pdf(paths::SIMPLE_BOM_PATH, temp_dir.path());

    let pdf_name = get_expected_pdf_name(paths::SIMPLE_BOM_PATH);
    let generated_pdf = temp_dir.path().join(&pdf_name);
    let expected_pdf = PathBuf::from(paths::REFERENCE_PDFS_DIR).join(&pdf_name);

    utils::assert_pdf_created(&generated_pdf);
    utils::assert_pdf_content_similar(&generated_pdf, &expected_pdf);
}

#[test]
fn test_vdr_minimal_with_vulns() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    run_vex2pdf(paths::BOM_VDR_MINIMAL_WITH_VULNS, temp_dir.path());

    let pdf_name = get_expected_pdf_name(paths::BOM_VDR_MINIMAL_WITH_VULNS);
    let generated_pdf = temp_dir.path().join(&pdf_name);
    let expected_pdf = PathBuf::from(paths::REFERENCE_PDFS_DIR).join(&pdf_name);

    utils::assert_pdf_created(&generated_pdf);
    utils::assert_pdf_content_similar(&generated_pdf, &expected_pdf);
}

#[test]
fn test_vdr_with_ghsa_entries() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    run_vex2pdf(paths::BOM_VDR_WITH_GHSA_ENTRIES, temp_dir.path());

    let pdf_name = get_expected_pdf_name(paths::BOM_VDR_WITH_GHSA_ENTRIES);
    let generated_pdf = temp_dir.path().join(&pdf_name);
    let expected_pdf = PathBuf::from(paths::REFERENCE_PDFS_DIR).join(&pdf_name);

    utils::assert_pdf_created(&generated_pdf);
    utils::assert_pdf_content_similar(&generated_pdf, &expected_pdf);
}

#[test]
fn test_vdr_with_many_vulns() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    run_vex2pdf(paths::BOM_VDR_WITH_MANY_VULNS, temp_dir.path());

    let pdf_name = get_expected_pdf_name(paths::BOM_VDR_WITH_MANY_VULNS);
    let generated_pdf = temp_dir.path().join(&pdf_name);
    let expected_pdf = PathBuf::from(paths::REFERENCE_PDFS_DIR).join(&pdf_name);

    utils::assert_pdf_created(&generated_pdf);
    utils::assert_pdf_content_similar(&generated_pdf, &expected_pdf);
}

#[test]
fn test_vdr_with_no_vulns() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    run_vex2pdf(paths::BOM_VDR_WITH_NO_VULNS, temp_dir.path());

    let pdf_name = get_expected_pdf_name(paths::BOM_VDR_WITH_NO_VULNS);
    let generated_pdf = temp_dir.path().join(&pdf_name);
    let expected_pdf = PathBuf::from(paths::REFERENCE_PDFS_DIR).join(&pdf_name);

    utils::assert_pdf_created(&generated_pdf);
    utils::assert_pdf_content_similar(&generated_pdf, &expected_pdf);
}

#[test]
fn test_vdr_with_links_as_versions() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    run_vex2pdf(paths::BOM_VDR_WITH_LINKS_AS_VERSIONS, temp_dir.path());

    let pdf_name = get_expected_pdf_name(paths::BOM_VDR_WITH_LINKS_AS_VERSIONS);
    let generated_pdf = temp_dir.path().join(&pdf_name);
    let expected_pdf = PathBuf::from(paths::REFERENCE_PDFS_DIR).join(&pdf_name);

    utils::assert_pdf_created(&generated_pdf);
    utils::assert_pdf_content_similar(&generated_pdf, &expected_pdf);
}

#[test]
fn test_vex_with_links_as_versions() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    run_vex2pdf(paths::BOM_VEX_WITH_LINKS_AS_VERSIONS, temp_dir.path());

    let pdf_name = get_expected_pdf_name(paths::BOM_VEX_WITH_LINKS_AS_VERSIONS);
    let generated_pdf = temp_dir.path().join(&pdf_name);
    let expected_pdf = PathBuf::from(paths::REFERENCE_PDFS_DIR).join(&pdf_name);

    utils::assert_pdf_created(&generated_pdf);
    utils::assert_pdf_content_similar(&generated_pdf, &expected_pdf);
}

#[test]
fn test_novulns_directory() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    run_vex2pdf(paths::BOM_NOVULNS, temp_dir.path());

    let pdf_name = get_expected_pdf_name(paths::BOM_NOVULNS);
    let generated_pdf = temp_dir.path().join(&pdf_name);
    let expected_pdf = PathBuf::from(paths::REFERENCE_PDFS_DIR).join(&pdf_name);

    utils::assert_pdf_created(&generated_pdf);
    utils::assert_pdf_content_similar(&generated_pdf, &expected_pdf);
}

// ============================================================================
// XML BOM Tests
// ============================================================================

#[test]
fn test_vdr_simple_xml() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    run_vex2pdf(paths::BOM_VDR_SIMPLE_XML, temp_dir.path());

    let pdf_name = get_expected_pdf_name(paths::BOM_VDR_SIMPLE_XML);
    let generated_pdf = temp_dir.path().join(&pdf_name);
    let expected_pdf = PathBuf::from(paths::REFERENCE_PDFS_DIR).join(&pdf_name);

    utils::assert_pdf_created(&generated_pdf);
    utils::assert_pdf_content_similar(&generated_pdf, &expected_pdf);
}

#[test]
fn test_vex_simple_xml() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    run_vex2pdf(paths::BOM_VEX_SIMPLE_XML, temp_dir.path());

    let pdf_name = get_expected_pdf_name(paths::BOM_VEX_SIMPLE_XML);
    let generated_pdf = temp_dir.path().join(&pdf_name);
    let expected_pdf = PathBuf::from(paths::REFERENCE_PDFS_DIR).join(&pdf_name);

    utils::assert_pdf_created(&generated_pdf);
    utils::assert_pdf_content_similar(&generated_pdf, &expected_pdf);
}

#[test]
fn test_sample_vex_xml() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    run_vex2pdf(paths::SAMPLE_VEX_XML, temp_dir.path());

    let pdf_name = get_expected_pdf_name(paths::SAMPLE_VEX_XML);
    let generated_pdf = temp_dir.path().join(&pdf_name);
    let expected_pdf = PathBuf::from(paths::REFERENCE_PDFS_DIR).join(&pdf_name);

    utils::assert_pdf_created(&generated_pdf);
    utils::assert_pdf_content_similar(&generated_pdf, &expected_pdf);
}

// ============================================================================
// Batch Processing Tests
// ============================================================================

#[test]
fn test_batch_run_test_directory() {
    // Test batch processing of all files in run_test directory
    let temp_input_dir = TempDir::new().expect("Failed to create temp input dir");
    let temp_output_dir = TempDir::new().expect("Failed to create temp output dir");

    // Copy all test files from run_test directory
    let src_dir = Path::new(paths::SOURCE_BOMS_BASE_ARTIFACTS_DIR);
    let files_copied =
        utils::copy_directory_files(src_dir, temp_input_dir.path()).expect("Failed to copy files");

    assert!(
        files_copied > 0,
        "No files were copied from run_test directory"
    );

    // Count expected processable files
    let expected_count = utils::count_processable_files(temp_input_dir.path());

    // Run vex2pdf on the directory
    let output = Command::new(paths::PATH_TO_EXE)
        .arg("-d")
        .arg(temp_output_dir.path())
        .arg(temp_input_dir.path())
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Batch processing failed: {}",
        utils::bytes_to_str(&output.stderr)
    );

    // Verify correct number of PDFs created
    utils::assert_pdf_count(temp_output_dir.path(), expected_count);

    // Verify each PDF matches expected output
    for entry in std::fs::read_dir(temp_output_dir.path()).unwrap() {
        let entry = entry.unwrap();
        let generated_pdf = entry.path();
        if generated_pdf.extension().and_then(|e| e.to_str()) == Some("pdf") {
            let pdf_name = generated_pdf.file_name().unwrap();
            let expected_pdf = PathBuf::from(paths::REFERENCE_PDFS_DIR).join(pdf_name);

            if expected_pdf.exists() {
                utils::assert_pdf_content_similar(&generated_pdf, &expected_pdf);
            }
        }
    }
}

#[test]
fn test_batch_run_test_xml_directory() {
    // Test batch processing of all files in run_test_xml directory
    let temp_input_dir = TempDir::new().expect("Failed to create temp input dir");
    let temp_output_dir = TempDir::new().expect("Failed to create temp output dir");

    // Copy all test files from run_test_xml directory
    let src_dir = Path::new(paths::SOURCE_BOMS_XML_ARTIFACTS_DIR);
    let files_copied =
        utils::copy_directory_files(src_dir, temp_input_dir.path()).expect("Failed to copy files");

    assert!(
        files_copied > 0,
        "No files were copied from run_test_xml directory"
    );

    // Count expected processable files
    let expected_count = utils::count_processable_files(temp_input_dir.path());

    // Run vex2pdf on the directory
    let output = Command::new(paths::PATH_TO_EXE)
        .arg("-d")
        .arg(temp_output_dir.path())
        .arg(temp_input_dir.path())
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Batch processing failed: {}",
        utils::bytes_to_str(&output.stderr)
    );

    // Verify correct number of PDFs created
    utils::assert_pdf_count(temp_output_dir.path(), expected_count);

    // Verify each PDF matches expected output
    for entry in std::fs::read_dir(temp_output_dir.path()).unwrap() {
        let entry = entry.unwrap();
        let generated_pdf = entry.path();
        if generated_pdf.extension().and_then(|e| e.to_str()) == Some("pdf") {
            let pdf_name = generated_pdf.file_name().unwrap();
            let expected_pdf = PathBuf::from(paths::REFERENCE_PDFS_DIR).join(pdf_name);

            if expected_pdf.exists() {
                utils::assert_pdf_content_similar(&generated_pdf, &expected_pdf);
            }
        }
    }
}

#[test]
fn test_batch_no_args_current_directory() {
    // Test batch processing with no arguments (scans current directory)
    let temp_work_dir = TempDir::new().expect("Failed to create temp work dir");

    // Copy a few test files to the working directory
    std::fs::copy(
        paths::SIMPLE_BOM_PATH,
        temp_work_dir.path().join("test1.json"),
    )
    .expect("Failed to copy test file");

    std::fs::copy(
        paths::BOM_VDR_WITH_NO_VULNS,
        temp_work_dir.path().join("test2.json"),
    )
    .expect("Failed to copy test file");

    std::fs::copy(
        paths::BOM_VDR_SIMPLE_XML,
        temp_work_dir.path().join("test3.xml"),
    )
    .expect("Failed to copy test file");

    let expected_count = 3;

    // Run vex2pdf with NO arguments from that directory
    let output = Command::new(paths::PATH_TO_EXE)
        .current_dir(temp_work_dir.path())
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Batch processing in current dir failed: {}",
        utils::bytes_to_str(&output.stderr)
    );

    // Verify PDFs were created in the same directory
    utils::assert_pdf_count(temp_work_dir.path(), expected_count);

    // Verify specific PDFs exist
    utils::assert_pdf_created(&temp_work_dir.path().join("test1.pdf"));
    utils::assert_pdf_created(&temp_work_dir.path().join("test2.pdf"));
    utils::assert_pdf_created(&temp_work_dir.path().join("test3.pdf"));
}

#[test]
fn test_batch_non_recursive_scanning() {
    // Test that directory scanning is non-recursive (only first level)
    let temp_input_dir = TempDir::new().expect("Failed to create temp input dir");
    let temp_output_dir = TempDir::new().expect("Failed to create temp output dir");

    // Create a subdirectory with a file
    let subdir = temp_input_dir.path().join("subdir");
    std::fs::create_dir(&subdir).expect("Failed to create subdir");

    std::fs::copy(paths::SIMPLE_BOM_PATH, subdir.join("nested.json"))
        .expect("Failed to copy to subdir");

    // Put files in the main directory
    std::fs::copy(
        paths::BOM_VDR_WITH_NO_VULNS,
        temp_input_dir.path().join("top_level1.json"),
    )
    .expect("Failed to copy to main dir");

    std::fs::copy(
        paths::BOM_VDR_SIMPLE_XML,
        temp_input_dir.path().join("top_level2.xml"),
    )
    .expect("Failed to copy to main dir");

    // Run vex2pdf on the directory
    let output = Command::new(paths::PATH_TO_EXE)
        .arg("-d")
        .arg(temp_output_dir.path())
        .arg(temp_input_dir.path())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    // Only top-level files should be processed (2 files)
    utils::assert_pdf_count(temp_output_dir.path(), 2);

    // Verify top-level PDFs created
    utils::assert_pdf_created(&temp_output_dir.path().join("top_level1.pdf"));
    utils::assert_pdf_created(&temp_output_dir.path().join("top_level2.pdf"));

    // Nested file should NOT be processed
    assert!(
        !temp_output_dir.path().join("nested.pdf").exists(),
        "Should not process files in subdirectories"
    );
}

#[test]
fn test_batch_empty_directory() {
    // Test that running on empty directory handles gracefully
    let temp_empty_dir = TempDir::new().expect("Failed to create temp empty dir");
    let temp_output_dir = TempDir::new().expect("Failed to create temp output dir");

    let output = Command::new(paths::PATH_TO_EXE)
        .arg("-d")
        .arg(temp_output_dir.path())
        .arg(temp_empty_dir.path())
        .output()
        .expect("Failed to execute command");

    // Should succeed but process 0 files
    assert!(
        output.status.success(),
        "Empty directory processing should succeed"
    );

    // Verify no PDFs created
    utils::assert_pdf_count(temp_output_dir.path(), 0);

    let stdout = utils::bytes_to_str(&output.stdout);
    assert!(
        stdout.contains("Found 0 JSON files") || stdout.contains("Processed 0 files"),
        "Should report 0 files processed"
    );
}

// ============================================================================
// CLI Argument Tests
// ============================================================================

#[test]
fn test_output_directory_argument() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    let output = Command::new(paths::PATH_TO_EXE)
        .arg("-d")
        .arg(temp_dir.path())
        .arg(paths::SIMPLE_BOM_PATH)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let pdf_name = get_expected_pdf_name(paths::SIMPLE_BOM_PATH);
    let generated_pdf = temp_dir.path().join(&pdf_name);

    utils::assert_pdf_created(&generated_pdf);
}

#[test]
fn test_invalid_output_directory_fails() {
    let output = Command::new(paths::PATH_TO_EXE)
        .arg("-d")
        .arg("/nonexistent/directory/that/does/not/exist")
        .arg(paths::SIMPLE_BOM_PATH)
        .output()
        .expect("Failed to execute command");

    // Command should fail
    assert!(!output.status.success());

    // Should have error message in stderr
    assert!(!output.stderr.is_empty());
}

#[test]
fn test_nonexistent_input_file_fails() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    let output = Command::new(paths::PATH_TO_EXE)
        .arg("-d")
        .arg(temp_dir.path())
        .arg("/nonexistent/file.json")
        .output()
        .expect("Failed to execute command");

    // Command should fail
    assert!(!output.status.success());
}

#[test]
fn test_json_with_analysis_renders_correctly() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    let _ = run_vex2pdf(paths::BOM_VDR_WITH_ANALYSIS, temp_dir.path());

    // Verify PDF was created
    let pdf_name = get_expected_pdf_name(paths::BOM_VDR_WITH_ANALYSIS);
    let pdf_path = temp_dir.path().join(&pdf_name);
    assert!(
        pdf_path.exists(),
        "Expected PDF not found: {}",
        pdf_path.display()
    );

    // Compare with expected PDF
    utils::assert_pdf_content_similar(
        &pdf_path,
        Path::new(paths::EXPECTED_BOM_VDR_WITH_ANALYSIS_PDF),
    );
}

#[test]
fn test_xml_with_analysis_renders_correctly() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    let _ = run_vex2pdf(paths::BOM_VDR_WITH_ANALYSIS_XML, temp_dir.path());

    // Verify PDF was created
    let pdf_name = get_expected_pdf_name(paths::BOM_VDR_WITH_ANALYSIS_XML);
    let pdf_path = temp_dir.path().join(&pdf_name);
    assert!(
        pdf_path.exists(),
        "Expected PDF not found: {}",
        pdf_path.display()
    );

    // Compare with expected PDF
    utils::assert_pdf_content_similar(
        &pdf_path,
        Path::new(paths::EXPECTED_BOM_VDR_WITH_ANALYSIS_XML_PDF),
    );
}

#[test]
fn test_err_output() {
    // trigger error path
    let output = Command::new(paths::PATH_TO_EXE)
        .arg("-d")
        .arg("/path/to/unknown")
        .output()
        .expect("failed to run executable");

    // verify error output
    let stderr_str = String::from_utf8_lossy(&output.stderr);
    assert!(stderr_str.contains("Problem setting up working environment"));
}

#[test]
fn test_license_output() {
    // trigger license output

    let output = Command::new(paths::PATH_TO_EXE)
        .arg("--license")
        .output()
        .expect("failed to run executable");

    // Verify content
    let stderr_str = String::from_utf8_lossy(&output.stdout);
    assert!(stderr_str.contains("VEX2PDF is licensed under either MIT or Apache License, Version 2.0 at your option."));
    assert!(stderr_str.contains("license text can be found under: https://gitlab.com/jurassicLizard/vex2pdf/-/blob/master/README.md#license"));
    assert!(stderr_str.contains("SIL OPEN FONT LICENSE Version 1.1 - 26 February 2007"));
    assert!(stderr_str.contains("DEALINGS IN THE FONT SOFTWARE"));

}

#[test]
fn test_version_long_output() {
    // Test --version flag output
    let output = Command::new(paths::PATH_TO_EXE)
        .arg("--version")
        .output()
        .expect("failed to run executable");

    // Verify version output contains copyright info
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    assert!(stdout_str.contains("vex2pdf"));
    assert!(stdout_str.contains("CycloneDX (VEX) to PDF Converter"));
    assert!(stdout_str.contains("Copyright (c) 2025 Salem B. - MIT Or Apache 2.0 License"));
}

#[test]
fn test_version_info_on_startup() {
    // Test that version info appears in logs when software runs normally
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let output = Command::new(paths::PATH_TO_EXE)
        .current_dir(temp_dir.path())
        .output()
        .expect("failed to run executable");

    // Verify version info appears in stdout logs
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    assert!(stdout_str.contains("vex2pdf"));
    assert!(stdout_str.contains("CycloneDX (VEX) to PDF Converter"));
    assert!(stdout_str.contains("Copyright (c) 2025 Salem B. - MIT Or Apache 2.0 License"));
}
