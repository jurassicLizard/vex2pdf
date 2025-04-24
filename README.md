# CycloneDX (VEX) to PDF Converter

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![Documentation](https://docs.rs/vex2pdf/badge.svg)](https://docs.rs/vex2pdf)
[![Crates.io](https://img.shields.io/crates/v/vex2pdf.svg)](https://crates.io/crates/vex2pdf)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE.md)
[![CI](https://github.com/jurassicLizard/vex2pdf/actions/workflows/rust.yml/badge.svg)](https://github.com/jurassicLizard/vex2pdf/actions/workflows/rust.yml)
[![GitHub Release](https://img.shields.io/github/v/release/jurassicLizard/vex2pdf)](https://github.com/jurassicLizard/vex2pdf/releases/latest)

A command-line tool to convert CycloneDX VEX (Vulnerability Exploitability eXchange) documents in JSON format to PDF reports.

## ⚠️ Font Requirement

This application requires Liberation Sans fonts to properly render PDF documents.

The Liberation Sans fonts are NOT included in this repository due to licensing considerations. To set up the required fonts:

1. Download the Liberation Sans TTF fonts from the [official repository](https://github.com/liberationfonts/liberation-fonts/releases)
2. Create a directory in the project root: `fonts/liberation-fonts`
3. Place the following TTF files in that directory:
  - LiberationSans-Regular.ttf
  - LiberationSans-Bold.ttf
  - LiberationSans-Italic.ttf
  - LiberationSans-BoldItalic.ttf


**Alternative options:**
- On Linux systems, if the fonts are installed system-wide (typically in `/usr/share/fonts/liberation-sans`), the application will attempt to use them.
- Set the font path through an environment variable (not implemented yet).


> Note: The Liberation Fonts are licensed under the SIL Open Font License, not MIT. Please respect their license terms when using them.
>

## Overview
VEX-to-PDF is a Rust application that scans the current directory for CycloneDX VEX JSON files and converts them to human-readable PDF reports. It fully supports the CycloneDX VEX schema version 1.5 and provides compatibility for version 1.6 documents that only use 1.5 fields. Documents using 1.6-specific fields may not process correctly. The tool handles various elements of the VEX documentation format including vulnerabilities, components, metadata, and more.
## Features
- Automatically scans directories for JSON files with VEX data
- Converts VEX documents to structured PDF reports
- Preserves all key VEX information including:
  - Document metadata and timestamps
  - Vulnerability details with severity ratings
  - Component information
  - Tools used to generate the VEX document

- Cross-platform support (Linux, Windows)

## Installation
### Prerequisites
- Rust and Cargo (latest stable version)
- Liberation Sans fonts (must be obtained separately as described above)

### From Source
Clone the repository, download and place Liberation fonts as described in the Font Requirement section, then build the application with . The binary will be available at target/release/vex2pdf. `cargo build --release`
### Windows Users
Windows users can either:
1. Build using Rust for Windows
2. Use a pre-built binary (when available)

## Usage
Run the application in a directory containing CycloneDX VEX JSON files:
``` 
./vex2pdf
```
The tool will:
1. Scan the current directory for JSON files
2. Attempt to parse each file as a CycloneDX VEX document
3. Generate a PDF report with the same name as the original file (with .pdf extension)
4. Display progress and results in the console

## Example
``` 
$ ./vex2pdf
Scanning for JSON files in: /home/user/vex-documents
Found 3 JSON files
Processing: sample_vex.json
Generating PDF: sample_vex.pdf
Successfully generated PDF: sample_vex.pdf
Processing: second-sample.json
Generating PDF: second-sample.pdf
Successfully generated PDF: UTF-8VEX.pdf
```
## Configuration
No configuration is currently required. The application will:
- Look for Liberation Sans fonts in `/usr/share/fonts/liberation-fonts`
- Fall back to the included directory `./fonts/liberation-fonts`

## Documentation

> **Note**: Rust documentation is a work in progress. Please refer to the code comments for details on specific functions and data structures.
>

To generate documentation:
``` 
cargo doc --open
```

## CycloneDX VEX Format
This tool fully supports CycloneDX VEX schema version 1.5 and provides compatibility for version 1.6 documents that only use 1.5 fields. Documents using 1.6-specific fields may not process correctly. For more information about the CycloneDX VEX format, see:
- [CycloneDX VEX Specification](https://cyclonedx.org/capabilities/vex/)
- [CycloneDX VEX Schema](https://cyclonedx.org/docs/1.5/json/)

### Version 1.6 Compatibility Mode

This tool implements a special compatibility mode for CycloneDX 1.6 documents:

- When the tool encounters a document with `specVersion: "1.6"`, it will:
  1. Display a notification about downgrading to 1.5
  2. Automatically modify the document's spec version to "1.5"
  3. Attempt to process it using the 1.5 schema parser

This compatibility approach works well for documents that don't use 1.6-specific fields but allows the tool to process newer documents without requiring users to manually modify them.

**Limitations:**
- Documents that use 1.6-specific fields or structures may fail during processing
- No validation is performed for 1.6-specific features
- This is a temporary solution until full 1.6 support is implemented in the underlying cyclonedx-bom library

When processing 1.6 documents, you'll see console messages indicating the compatibility mode is active.

## Security Considerations
- The application reads and processes files from the current directory
- No network connections are established
- Input validation is performed on all JSON files
- Font paths are validated before use

## License
This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
The Liberation Fonts used by this application are licensed under the SIL Open Font License and must be obtained separately.


## Acknowledgments
- [CycloneDX](https://cyclonedx.org/) for the VEX specification
- [cyclonedx-bom](https://crates.io/crates/cyclonedx-bom) for CycloneDX parsing
- [genpdf](https://crates.io/crates/genpdf) for PDF generation
- [serde_json](https://crates.io/crates/serde_json) for JSON processing
- [Liberation Fonts](https://github.com/liberationfonts/liberation-fonts) for the PDF rendering fonts