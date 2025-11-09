/// Utility to generate checksums for reference PDFs
///
/// This program calculates BLAKE3 checksums of normalized PDF content
/// (with timestamps and dynamic IDs stripped) for all reference PDFs.
/// The checksums are used by integration tests to validate generated PDFs
/// without needing to include the full 42MB of reference PDFs in the crate.
///
/// Run with: cargo run --example generate_checksums

use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let pdfs_dir = PathBuf::from(manifest_dir)
        .join("tests")
        .join("test_artifacts")
        .join("expected_pdfs");

    let checksums_file = PathBuf::from(manifest_dir)
        .join("tests")
        .join("test_artifacts")
        .join("expected_pdfs_chksums.txt");

    println!("Generating checksums for PDFs in: {:?}", pdfs_dir);
    println!("Output file: {:?}", checksums_file);
    println!();

    if !pdfs_dir.exists() {
        eprintln!("Error: Reference PDFs directory not found: {:?}", pdfs_dir);
        std::process::exit(1);
    }

    let mut entries = fs::read_dir(&pdfs_dir)
        .expect("Failed to read PDFs directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "pdf")
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    // Sort by filename for consistent output
    entries.sort_by_key(|e| e.file_name());

    let mut checksums = Vec::new();

    for entry in entries {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();

        let checksum = calculate_normalized_checksum(&path);

        println!("{}  {}", checksum, filename);
        checksums.push(format!("{}  {}", checksum, filename));
    }

    let output = checksums.join("\n") + "\n";
    fs::write(&checksums_file, output)
        .expect("Failed to write checksums file");

    println!();
    println!("✓ Generated {} checksums", checksums.len());
    println!("✓ Written to: {:?}", checksums_file);
}

/// Strip timestamp and dynamic ID-related lines from PDF content
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

/// Normalize PDF content by stripping dynamic elements
fn normalize_pdf_content(pdf_path: &Path) -> String {
    let content = fs::read(pdf_path).expect("Failed to read PDF file");
    let content_str = String::from_utf8_lossy(&content);
    strip_pdf_timestamps(&content_str)
}

/// Calculate BLAKE3 checksum of normalized PDF content
fn calculate_normalized_checksum(pdf_path: &Path) -> String {
    let normalized = normalize_pdf_content(pdf_path);
    let hash = blake3::hash(normalized.as_bytes());
    hash.to_hex().to_string()
}
