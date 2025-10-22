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
