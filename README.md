# CycloneDX (VEX) to PDF Converter

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![Documentation](https://docs.rs/vex2pdf/badge.svg)](https://docs.rs/vex2pdf)
[![Crates.io](https://img.shields.io/crates/v/vex2pdf.svg)](https://crates.io/crates/vex2pdf)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE.md)
[![CI](https://github.com/jurassicLizard/vex2pdf/actions/workflows/rust.yml/badge.svg)](https://github.com/jurassicLizard/vex2pdf/actions/workflows/rust.yml)
[![GitHub Release](https://img.shields.io/github/v/release/jurassicLizard/vex2pdf)](https://github.com/jurassicLizard/vex2pdf/releases/latest)

A command-line tool to convert CycloneDX VEX (Vulnerability Exploitability eXchange) documents in JSON or XML format to PDF reports.

<!-- TOC -->
* [CycloneDX (VEX) to PDF Converter](#cyclonedx-vex-to-pdf-converter)
  * [Overview](#overview)
  * [Fonts Handling](#fonts-handling)
    * [Font Licensing](#font-licensing)
  * [Features](#features)
  * [Installation](#installation)
    * [Prerequisites](#prerequisites)
    * [Via Cargo](#via-cargo)
    * [From Source](#from-source)
    * [Windows Users](#windows-users)
  * [Usage](#usage)
  * [Example](#example)
  * [Configuration](#configuration)
    * [Environment Variables](#environment-variables)
      * [VEX2PDF_NOVULNS_MSG](#vex2pdf_novulns_msg)
      * [VEX2PDF_SHOW_OSS_LICENSES](#vex2pdf_show_oss_licenses)
      * [VEX2PDF_VERSION_INFO](#vex2pdf_version_info)
      * [VEX2PDF_REPORT_TITLE](#vex2pdf_report_title)
      * [VEX2PDF_PDF_META_NAME](#vex2pdf_pdf_meta_name)
  * [Documentation](#documentation)
  * [CycloneDX VEX Format](#cyclonedx-vex-format)
    * [Version 1.6 Compatibility Mode](#version-16-compatibility-mode)
  * [Security Considerations](#security-considerations)
* [Changelog](#changelog)
  * [License](#license)
  * [Acknowledgments](#acknowledgments)
<!-- TOC -->

## Overview

VEX2PDF is a Rust application that scans the current directory for CycloneDX VEX files (JSON and XML) and converts them to human-readable PDF reports. It fully supports the CycloneDX VEX schema version 1.5 and provides compatibility for version 1.6 documents that only use 1.5 fields. Documents using 1.6-specific fields may not process correctly. The tool handles various elements of the VEX documentation format including vulnerabilities, components, metadata, and more.

## Fonts Handling

This tool uses Liberation Sans fonts to render PDFs. The fonts are embedded directly in the binary, so **no extra font configuration is required** and the binary works standalone and is fully portable.

### Font Licensing

The embedded Liberation Sans fonts are licensed under the SIL Open Font License (OFL).
Set the environment variable `VEX2PDF_SHOW_OSS_LICENSES=true` to display full license details at runtime. 
Check [VEX2PDF_SHOW_OSS_LICENSES](#VEX2PDF_SHOW_OSS_LICENSES) for more Information.

The font license file is also available at [Liberation fonts License file](external/fonts/liberation-fonts/LICENSE) in the current repository.

## Features
- Automatically scans directories for JSON and XML files with VEX data
- Converts VEX documents to structured PDF reports
- Supports both JSON and XML CycloneDX formats
- Preserves all key VEX information including:
  - Document metadata and timestamps
  - Vulnerability details with severity ratings
  - Component information
  - Tools used to generate the VEX document
- Cross-platform support (Linux, Windows)

## Installation
### Prerequisites
- Rust and Cargo (latest stable version)

### Via Cargo
The easiest way to install VEX2PDF is directly from crates.io:

```bash
cargo install vex2pdf
```

After installation, the `vex2pdf` binary will be available in your Cargo bin directory.

> Notice: As of v0.6.1 no extra font configuration is needed. Fonts have been embedded in the software binary. Check [Fonts handling and license](#fonts-handling) for further information


### From Source
Clone the repository, then build the application with `cargo build --release`. The binary will be available at target/release/vex2pdf.

### Windows Users
Windows users can either:
1. Install via Cargo as described above
2. Build using Rust for Windows from source
3. Use a pre-built binary (GitHub Releases Section)


## Usage

Run the application in a directory containing CycloneDX VEX files (JSON or XML):

```shell 
./vex2pdf
```
The tool will:
1. Scan the current directory for JSON and XML files
2. Attempt to parse each file as a CycloneDX VEX document
3. Generate a PDF report with the same name as the original file (with .pdf extension)
4. Display progress and results in the console


## Example
``` 
$ ./vex2pdf
vex2pdf v0.6.1 - CycloneDX (VEX) to PDF Converter
Copyright (c) 2025 jurassicLizard - MIT License

Active font path: <embedded liberationSans fonts> -- the env variable VEX2PDF_SHOW_OSS_LICENSES=true shows Font license details

Scanning for JSON files in: ./documents
Found 2 JSON files
Processing: ./documents/example1.json
Generating PDF: ./documents/example1.pdf
Successfully generated PDF: ./documents/example1.pdf
Processing: ./documents/example2.json
Generating PDF: ./documents/example2.pdf
Successfully generated PDF: ./documents/example2.pdf

Scanning for XML files in: ./documents
Found 5 XML files
Processing: ./documents/example1.xml
Generating PDF: ./documents/example1.pdf
Successfully generated PDF: ./documents/example1.pdf
Processing: ./documents/example2.xml
Generating PDF: ./documents/example2.pdf
Successfully generated PDF: ./documents/example2.pdf
Processing: ./documents/example3.xml

NOTE: Downgrading CycloneDX BOM from spec version 1.6 to 1.5
Reason: Current implementation does not yet fully support spec version 1.6
Warning: This compatibility mode only works for BOMs that don't utilize 1.6-specific fields
         Processing will fail if 1.6-specific fields are encountered

Generating PDF: ./documents/example3.pdf
Successfully generated PDF: ./documents/example3.pdf
```
## Configuration

No configuration files are required. However the application has some customization options available via Environment variables.


### Environment Variables

The following environment variables can be used to customize behavior:

| Variable                            | Purpose                                                    | Default                                                   |
|-------------------------------------|------------------------------------------------------------|-----------------------------------------------------------|
| VEX2PDF_NOVULNS_MSG                 | Controls the "No Vulnerabilities reported" message display | true                                                      |
| VEX2PDF_SHOW_OSS_LICENSES           | Shows all relevant licenses and exits                      | off                                                       |
| VEX2PDF_VERSION_INFO                | Shows version information before executing normally        | off                                                       |
| VEX2PDF_REPORT_TITLE                | Overrides the default report title                         | Not set (uses default title)                              |
| VEX2PDF_PDF_META_NAME               | Overrides the PDF metadata title                           | Not set (uses default metadata title)                     |

#### VEX2PDF_NOVULNS_MSG

This variable controls how the Vulnerabilities section appears when no vulnerabilities exist:
- When set to "true" or not set (default): A "Vulnerabilities" section will be shown with a "No Vulnerabilities reported" message
- When set to "false": The Vulnerabilities section will be completely omitted from the PDF

Example : `VEX2PDF_NOVULNS_MSG=false vex2pdf`

#### VEX2PDF_SHOW_OSS_LICENSES

Shows all relevant OSS licenses:
- When set to "true" or "on": Show license texts and exit
  - MIT License for the current software
  - SIL License for the liberation-fonts
- When set to "false" or "off" or when it is unset: Run the software normally

Example : `VEX2PDF_SHOW_OSS_LICENSES=true vex2pdf`

#### VEX2PDF_VERSION_INFO

Shows version information prior to running software normally

#### VEX2PDF_REPORT_TITLE

Overrides the default report title with custom text

Example : `VEX2PDF_REPORT_TITLE="My Custom VEX Report" vex2pdf`

#### VEX2PDF_PDF_META_NAME

Overrides the PDF metadata title with custom text

Example 1 : `VEX2PDF_PDF_META_NAME="VEX Report - Company XYZ" vex2pdf`
Example 2 : `VEX2PDF_PDF_META_NAME="VEX Report - Company XYZ" VEX2PDF_REPORT_TITLE="My Custom VEX Report" vex2pdf`


## Documentation


For full API documentation, please visit:
- [vex2pdf on docs.rs](https://docs.rs/vex2pdf)

To generate documentation locally:
```bash
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

# Changelog

Changes to the software between version increments are documented under [Changelog.md](CHANGELOG.md).

## License
This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
The Liberation Fonts used by this application are licensed under the SIL Open Font License and must be obtained separately.


## Acknowledgments
- [CycloneDX](https://cyclonedx.org/) for the VEX specification
- [cyclonedx-bom](https://crates.io/crates/cyclonedx-bom) for CycloneDX parsing
- [genpdf](https://crates.io/crates/genpdf) for PDF generation
- [serde_json](https://crates.io/crates/serde_json) for JSON processing
- [Liberation Fonts](https://github.com/liberationfonts/liberation-fonts) for the PDF rendering fonts
