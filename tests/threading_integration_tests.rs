use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

mod common;

use common::paths;
use common::utils;

/// Helper function to run vex2pdf with max_jobs and verify success
fn run_vex2pdf_with_jobs(
    input_path: &str,
    output_dir: &Path,
    max_jobs: Option<u8>,
) -> std::process::Output {
    let mut cmd = Command::new(paths::PATH_TO_EXE);
    cmd.arg("-d").arg(output_dir).arg(input_path);

    if let Some(jobs) = max_jobs {
        cmd.arg("--max-jobs").arg(jobs.to_string());
    }

    let output = cmd.output().expect("Failed to execute command");

    // Print output for debugging
    if !output.stdout.is_empty() {
        println!("stdout: {}", utils::bytes_to_str(&output.stdout));
    }
    if !output.stderr.is_empty() {
        eprintln!("stderr: {}", utils::bytes_to_str(&output.stderr));
    }

    // Assert command succeeded
    assert!(
        output.status.success(),
        "Command failed with status: {}",
        output.status
    );

    output
}

#[test]
fn test_single_threaded_mode() {
    let output_dir = TempDir::new().expect("Failed to create temp dir");

    let output = run_vex2pdf_with_jobs(paths::SIMPLE_BOM_PATH, output_dir.path(), Some(1));

    // Verify single-threaded message appears in output
    utils::assert_output_contains(
        &output.stdout,
        "Concurrency Disabled: running all jobs sequentially in main thread",
    );

    // Verify PDF was generated
    let pdf_count = utils::count_pdf_files(output_dir.path());
    assert_eq!(
        pdf_count, 1,
        "Should generate exactly 1 PDF in single-threaded mode"
    );
}

#[test]
fn test_multi_threaded_mode_2_jobs() {
    let output_dir = TempDir::new().expect("Failed to create temp dir");

    // Use directory with multiple files
    let output = run_vex2pdf_with_jobs(
        paths::SOURCE_BOMS_BASE_ARTIFACTS_DIR,
        output_dir.path(),
        Some(2),
    );

    // Verify multi-threaded message with 2 jobs
    utils::assert_output_contains(&output.stdout, "Concurrency Enabled: running with 2 jobs");

    // Verify PDFs were generated
    let pdf_count = utils::count_pdf_files(output_dir.path());
    assert!(
        pdf_count > 1,
        "Should generate multiple PDFs with 2 jobs, found {}",
        pdf_count
    );
}

#[test]
fn test_multi_threaded_mode_4_jobs() {
    let output_dir = TempDir::new().expect("Failed to create temp dir");

    // Use directory with multiple files
    let output = run_vex2pdf_with_jobs(
        paths::SOURCE_BOMS_BASE_ARTIFACTS_DIR,
        output_dir.path(),
        Some(4),
    );

    // Verify multi-threaded message with 4 jobs
    utils::assert_output_contains(&output.stdout, "Concurrency Enabled: running with 4 jobs");

    // Verify PDFs were generated
    let pdf_count = utils::count_pdf_files(output_dir.path());
    assert!(
        pdf_count > 1,
        "Should generate multiple PDFs with 4 jobs, found {}",
        pdf_count
    );
}

#[test]
fn test_default_parallelism() {
    let output_dir = TempDir::new().expect("Failed to create temp dir");

    // No max_jobs argument - should use default (all available cores)
    let output = run_vex2pdf_with_jobs(paths::SIMPLE_BOM_PATH, output_dir.path(), None);

    // Verify multi-threaded message (default should enable concurrency)
    utils::assert_output_contains(&output.stdout, "Concurrency Enabled: running with");

    // Verify PDF was generated
    let pdf_count = utils::count_pdf_files(output_dir.path());
    assert_eq!(
        pdf_count, 1,
        "Should generate exactly 1 PDF with default parallelism"
    );
}

#[test]
fn test_max_jobs_zero_uses_default() {
    let output_dir = TempDir::new().expect("Failed to create temp dir");

    // max_jobs = 0 should use default parallelism (all available cores)
    let output = run_vex2pdf_with_jobs(paths::SIMPLE_BOM_PATH, output_dir.path(), Some(0));

    // Verify multi-threaded message (0 should trigger default behavior)
    utils::assert_output_contains(&output.stdout, "Concurrency Enabled: running with");

    // Verify PDF was generated
    let pdf_count = utils::count_pdf_files(output_dir.path());
    assert_eq!(
        pdf_count, 1,
        "Should generate exactly 1 PDF with max_jobs=0"
    );
}

#[test]
fn test_env_var_single_threaded() {
    let output_dir = TempDir::new().expect("Failed to create temp dir");

    // Use VEX2PDF_MAX_JOBS=1 environment variable
    let output = Command::new(paths::PATH_TO_EXE)
        .env("VEX2PDF_MAX_JOBS", "1")
        .arg("-d")
        .arg(output_dir.path())
        .arg(paths::SIMPLE_BOM_PATH)
        .output()
        .expect("Failed to execute command");

    // Print output for debugging
    if !output.stdout.is_empty() {
        println!("stdout: {}", utils::bytes_to_str(&output.stdout));
    }
    if !output.stderr.is_empty() {
        eprintln!("stderr: {}", utils::bytes_to_str(&output.stderr));
    }

    // Assert command succeeded
    assert!(
        output.status.success(),
        "Command failed with status: {}",
        output.status
    );

    // Verify single-threaded message appears in output
    utils::assert_output_contains(
        &output.stdout,
        "Concurrency Disabled: running all jobs sequentially in main thread",
    );

    // Verify PDF was generated
    let pdf_count = utils::count_pdf_files(output_dir.path());
    assert_eq!(
        pdf_count, 1,
        "Should generate exactly 1 PDF with VEX2PDF_MAX_JOBS=1"
    );
}

#[test]
fn test_env_var_default_parallelism() {
    let output_dir = TempDir::new().expect("Failed to create temp dir");

    // Use VEX2PDF_MAX_JOBS=0 environment variable (should use default)
    let output = Command::new(paths::PATH_TO_EXE)
        .env("VEX2PDF_MAX_JOBS", "0")
        .arg("-d")
        .arg(output_dir.path())
        .arg(paths::SIMPLE_BOM_PATH)
        .output()
        .expect("Failed to execute command");

    // Print output for debugging
    if !output.stdout.is_empty() {
        println!("stdout: {}", utils::bytes_to_str(&output.stdout));
    }
    if !output.stderr.is_empty() {
        eprintln!("stderr: {}", utils::bytes_to_str(&output.stderr));
    }

    // Assert command succeeded
    assert!(
        output.status.success(),
        "Command failed with status: {}",
        output.status
    );

    // Verify multi-threaded message (0 should trigger default behavior)
    utils::assert_output_contains(&output.stdout, "Concurrency Enabled: running with");

    // Verify PDF was generated
    let pdf_count = utils::count_pdf_files(output_dir.path());
    assert_eq!(
        pdf_count, 1,
        "Should generate exactly 1 PDF with VEX2PDF_MAX_JOBS=0"
    );
}
