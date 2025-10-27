# CycloneDX (VEX/VDR/(S)BoM) to PDF Converter

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![Documentation](https://docs.rs/vex2pdf/badge.svg)](https://docs.rs/vex2pdf)
[![Crates.io](https://img.shields.io/crates/v/vex2pdf.svg)](https://crates.io/crates/vex2pdf)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache%20License%202.0-blue)](LICENSE-APACHE)

A command-line tool to convert CycloneDX (VEX/VDR/(S)BoM) Documents in JSON or XML format to PDF reports.

<!-- TOC -->
* [CycloneDX (VEX/VDR/(S)BoM) to PDF Converter](#cyclonedx-vexvdrsbom-to-pdf-converter)
  * [Overview](#overview)
  * [Supported Document Types](#supported-document-types)
    * [VEX (Vulnerability Exploitability eXchange)](#vex-vulnerability-exploitability-exchange)
    * [VDR (Vulnerability Disclosure Report)](#vdr-vulnerability-disclosure-report)
    * [BOM/SBOM (Bill of Materials)](#bomsbom-bill-of-materials)
  * [Vulnerability Analysis Display](#vulnerability-analysis-display)
    * [Analysis States](#analysis-states)
    * [Response Actions](#response-actions)
    * [Analysis Information](#analysis-information)
  * [Fonts Handling](#fonts-handling)
    * [Font Licensing](#font-licensing)
  * [Features](#features)
  * [Installation](#installation)
    * [Prerequisites](#prerequisites)
    * [Via Cargo](#via-cargo)
    * [From Source](#from-source)
    * [Linux and Windows Users](#linux-and-windows-users)
    * [Mac Users](#mac-users)
  * [Usage](#usage)
  * [Example](#example)
  * [Configuration](#configuration)
    * [Environment Variables](#environment-variables)
      * [VEX2PDF_NOVULNS_MSG](#vex2pdf_novulns_msg)
      * [VEX2PDF_SHOW_OSS_LICENSES](#vex2pdf_show_oss_licenses)
      * [VEX2PDF_VERSION_INFO](#vex2pdf_version_info)
      * [VEX2PDF_REPORT_TITLE](#vex2pdf_report_title)
      * [VEX2PDF_PDF_META_NAME](#vex2pdf_pdf_meta_name)
      * [VEX2PDF_PURE_BOM_NOVULNS](#vex2pdf_pure_bom_novulns)
      * [VEX2PDF_SHOW_COMPONENTS](#vex2pdf_show_components)
  * [Logging](#logging)
    * [Log Levels](#log-levels)
    * [Controlling Log Output](#controlling-log-output)
  * [Documentation](#documentation)
  * [CycloneDX Document Format](#cyclonedx-document-format)
    * [Version 1.6 Compatibility Mode](#version-16-compatibility-mode)
  * [Security Considerations](#security-considerations)
  * [Changelog](#changelog)
  * [License](#license)
    * [Contribution](#contribution)
    * [Third-Party Dependencies](#third-party-dependencies)
  * [Acknowledgments](#acknowledgments)
<!-- TOC -->

## Overview

VEX2PDF is a Rust application that scans the current directory for CycloneDX (VEX/VDR/(S)BoM files (JSON and XML) and converts them to human-readable PDF reports. It fully supports the CycloneDX schema version 1.5 and provides compatibility for version 1.6 documents that only use 1.5 fields. Documents using 1.6-specific fields may not process correctly. The tool handles various elements of the CycloneDX document format including vulnerabilities, components, metadata, and more.

## Supported Document Types

### VEX (Vulnerability Exploitability eXchange)
- **Purpose**: Communicates exploitability of vulnerabilities in specific contexts
- **Focus**: Real-world risk assessment and exploitability analysis

### VDR (Vulnerability Disclosure Report)
- **Purpose**: Provides comprehensive vulnerability assessments for components
- **Focus**: Known and unknown vulnerabilities with detailed analysis
- **Features**: Includes the "affected" property for component-vulnerability correlation

### BOM/SBOM (Bill of Materials)
- **Purpose**: Inventories software components and dependencies
- **Focus**: Component listing and supply chain transparency

## Vulnerability Analysis Display

VEX2PDF renders CycloneDX vulnerability analysis information with visual enhancements to improve readability and quick risk assessment:

### Analysis States

The tool displays vulnerability analysis states with color-coded formatting for immediate visual recognition:

- **Exploitable** (Red) - Vulnerability is directly or indirectly exploitable in the current environment
- **In Triage** (Orange) - Vulnerability is under active investigation by the security team
- **Resolved** (Green) - Vulnerability has been successfully remediated
- **Resolved With Pedigree** (Dark Green) - Remediated with verifiable commit history and audit trail
- **Not Affected** (Forest Green) - Component is confirmed not vulnerable to this issue
- **False Positive** (Steel Blue) - Incorrectly identified as vulnerable; not a real security issue

### Response Actions

Response actions indicate what remediation steps are available or planned:

- **Update** (Blue) - Software update available to fix the vulnerability
- **Rollback** (Blue) - Rollback to a previous non-vulnerable version is the recommended action
- **Workaround Available** (Orange) - Temporary mitigation or configuration change exists
- **Can Not Fix** (Red) - Technical limitations prevent fixing this vulnerability
- **Will Not Fix** (Red) - Vulnerability will not be addressed (risk accepted or deprioritized)

### Analysis Information

When present in the CycloneDX document, the analysis section appears after each vulnerability's description and includes:

1. **State** - Current vulnerability assessment status (color-coded)
2. **Response** - Available or planned remediation actions (displayed as a bracketed list)
3. **Justification** - Explanation for "Not Affected" determinations (e.g., "Code Not Reachable", "Protected By Compiler")
4. **Details** - Comprehensive analysis text explaining the security team's assessment
5. **Timestamps** - First issued and last updated dates for tracking analysis history

Analysis sections only appear when the CycloneDX document includes analysis data. If no analysis information exists for a vulnerability, the section is omitted entirely.

## Fonts Handling

This tool uses Liberation Sans fonts to render PDFs. The fonts are embedded directly in the binary, so **no extra font configuration is required** and the binary works standalone and is fully portable.

### Font Licensing

The embedded Liberation Sans fonts are licensed under the SIL Open Font License (OFL).
Set the environment variable `VEX2PDF_SHOW_OSS_LICENSES=true` to display full license details at runtime. 
Check [VEX2PDF_SHOW_OSS_LICENSES](#VEX2PDF_SHOW_OSS_LICENSES) for more Information.

The font license file is also available at [Liberation fonts License file](external/fonts/liberation-fonts/LICENSE) in the current repository.

## Features
- Automatically scans directories for JSON and XML files with VEX/VDR/(S)BoM data
- Converts (VEX/VDR/(S)BoM) documents to structured PDF reports
- Supports both JSON and XML CycloneDX formats
- Preserves all key (VEX/VDR/(S)BoM) information including:
  - Document metadata and timestamps
  - Vulnerability details with severity ratings and sources
  - _**New since v0.8.0**_  : Component-vulnerability correlation via "affected" property (VDR)
  - Exploitability assessments and risk context (VEX)
  - **New**: Vulnerability analysis with color-coded states and responses
    - Analysis states: Exploitable (red), Resolved (green), In Triage (orange), False Positive (blue), Not Affected (green), Resolved With Pedigree (dark green)
    - Response actions: Update/Rollback (blue), Workaround Available (orange), Can Not Fix/Will Not Fix (red)
    - Justification explanations for not-affected vulnerabilities
    - Detailed analysis text and timestamps (firstIssued, lastUpdated)
  - Component inventories and dependencies (BOM/SBOM)
  - Tools used to generate the CycloneDX (VEX/VDR/(S)BoM)) document
- Cross-platform support

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

### Linux and Windows Users
Users can either:
1. Install via Cargo as described above
2. Build from the source using for the respective platform and operating-system. Please check the [From Source Section](#from-source)
3. Use a pre-built binary under the [Release Binaries section](https://gitlab.com/jurassicLizard/vex2pdf/-/releases) 

### Mac Users
Currently, No Mac Binaries are provided however Mac Users can build and install with cargo. Please check the [From Source Section](#from-source) 

> :note: If Mac release binaries are needed please [create an issue](https://gitlab.com/jurassicLizard/vex2pdf/-/issues)



## Usage

VEX2PDF is designed for batch processing and can be run without arguments to automatically process all CycloneDX files in the current directory. As of **v0.9.0**, the tool also supports command-line arguments for more precise control over input files, output directories, and report customization.

Run the application in a directory containing CycloneDX (VEX/VDR/(S)BoM files (JSON or XML):

```shell
./vex2pdf
```
The tool will:
1. Scan the current directory for JSON and XML files
2. Attempt to parse each file as a CycloneDX (VEX/VDR/(S)BoM) document
3. Generate a PDF report with the same name as the original file (with .pdf extension)
4. Display progress and results in the console

### Command-Line Arguments

VEX2PDF supports command-line arguments for convenient configuration. All arguments can also be set via environment variables as shown in the [Configuration](#configuration) section.

To see all available options, run:

```bash
vex2pdf --help
```

**Available options:**

```
A tool to convert CycloneDX(VEX) JSON or XML documents to PDF reports

Usage: vex2pdf [OPTIONS] [FILE_OR_FOLDER_TO_PROCESS]

Arguments:
  [FILE_OR_FOLDER_TO_PROCESS]  File to process (JSON or XML) or Folder containing said file types.
                               Please note that this tool is designed for batch processing. So If
                               this is not set the tool scans the current directory for all parseable
                               files and converts them. if a folder is set the tool scans just the
                               first level of the directory (non-recursive) [env: VEX2PDF_WORKING_PATH=]

Options:
  -m, --show-novulns-msg <SHOW_NOVULNS_MSG>
          [env: VEX2PDF_NOVULNS_MSG=] [possible values: true, false]
  -t, --report-title <REPORT_TITLE>
          Overrides the default title of the report [env: VEX2PDF_REPORT_TITLE=]
  -n, --pdf-meta-name <PDF_META_TITLE>
          Overrides the default PDF meta name [env: VEX2PDF_PDF_META_NAME=]
  -b, --bom-novulns <PURE_BOM_NOVULNS>
          Treats the file as a pure bill of materials and shows only the components
          without the vulnerabilities [env: VEX2PDF_PURE_BOM_NOVULNS=] [possible values: true, false]
  -c, --show-components <SHOW_COMPONENTS>
          Controls whether the component list is shown [env: VEX2PDF_SHOW_COMPONENTS=]
          [possible values: true, false]
  -d, --output-dir <OUTPUT_DIR>
          Sets the directory where the parser should output the files [env: VEX2PDF_OUTPUT_DIR=]
  -j, --max-jobs <MAX_JOBS>
          Sets the maximum number of jobs for concurrent generation tasks, when not set or set to `0`
          this defaults to using the maximum available parallelism on the system which is given by
          [`std::thread::available_parallelism`] [env: VEX2PDF_MAX_JOBS=]
  -h, --help
          Print help
  -V, --version
          Print version
```

**Examples:**

```bash
# Process a specific file
vex2pdf my-bom.json

# Process a specific directory
vex2pdf /path/to/bom/files/

# Specify output directory
vex2pdf my-bom.json -d /path/to/output/

# Hide components list
vex2pdf -c false

# Custom report title
vex2pdf -t "Security Vulnerability Assessment"

# Process with single-threaded mode (no concurrency)
vex2pdf --max-jobs 1 my-bom.json

# Process with 4 concurrent jobs
vex2pdf --max-jobs 4 /path/to/bom/files/

# Combine multiple options
vex2pdf my-bom.json -d ./reports/ -t "Q4 Security Report"
```


## Example
```
$ ./vex2pdf
vex2pdf v0.9.0 - CycloneDX (VEX) to PDF Converter
Copyright (c) 2025 Salem B. - MIT Or Apache 2.0 License

[2025-10-24T18:00:00Z INFO] Active font path: <embedded liberationSans fonts> -- the env variable VEX2PDF_SHOW_OSS_LICENSES=true shows Font license details
[2025-10-24T18:00:00Z INFO]
[2025-10-24T18:00:00Z INFO] Using default report title
[2025-10-24T18:00:00Z INFO] Using default pdf metadata title
[2025-10-24T18:00:00Z INFO]
[2025-10-24T18:00:00Z INFO] Scanning for BoM/Vex Files in ./documents
[2025-10-24T18:00:00Z INFO] Found 2 JSON files
[2025-10-24T18:00:00Z INFO] Found 5 XML files
[2025-10-24T18:00:00Z INFO] Processing ./documents/example1.json
[2025-10-24T18:00:00Z INFO] Generating PDF:  ./documents/example1.json
[2025-10-24T18:00:00Z INFO] Successfully generated PDF: ./documents/example1.pdf
[2025-10-24T18:00:00Z INFO] Processing ./documents/example2.json
[2025-10-24T18:00:00Z INFO] Generating PDF:  ./documents/example2.json
[2025-10-24T18:00:00Z INFO] Successfully generated PDF: ./documents/example2.pdf
[2025-10-24T18:00:00Z INFO] Processing ./documents/example3.xml
[2025-10-24T18:00:01Z WARN]
[2025-10-24T18:00:01Z WARN] NOTE: Downgrading CycloneDX BOM from spec version 1.6 to 1.5
[2025-10-24T18:00:01Z WARN] Reason: Current implementation does not yet fully support spec version 1.6
[2025-10-24T18:00:01Z WARN] Warning: This compatibility mode only works for BOMs that don't utilize 1.6-specific fields
[2025-10-24T18:00:01Z WARN]          Processing will fail if 1.6-specific fields are encountered
[2025-10-24T18:00:01Z WARN]
[2025-10-24T18:00:01Z INFO] Generating PDF:  ./documents/example3.xml
[2025-10-24T18:00:01Z INFO] Successfully generated PDF: ./documents/example3.pdf
[2025-10-24T18:00:01Z INFO] Processed 7 files
```
## Configuration

No configuration files are required. However the application has some customization options available via Environment variables.


### Environment Variables


> **Windows Users**: To set environment variables on Windows, use:
> - **Command Prompt**: `set VEX2PDF_ENV_VARIABLE=false && vex2pdf`
> - **PowerShell**: `$env:VEX2PDF_ENV_VARIABLE="false"; vex2pdf`

The following environment variables can be used to customize behavior:


| Variable                  | Purpose                                                                       | Default                               |
|---------------------------|-------------------------------------------------------------------------------|---------------------------------------|
| VEX2PDF_NOVULNS_MSG       | Controls the "No Vulnerabilities reported" message display                    | true                                  |
| VEX2PDF_SHOW_OSS_LICENSES | Shows all relevant licenses and exits                                         | off                                   |
| VEX2PDF_VERSION_INFO      | Shows version information before executing normally                           | off                                   |
| VEX2PDF_REPORT_TITLE      | Overrides the default report title                                            | Not set (uses default title)          |
| VEX2PDF_PDF_META_NAME     | Overrides the PDF metadata title                                              | Not set (uses default metadata title) |
| VEX2PDF_PURE_BOM_NOVULNS  | Whether to treat the file as a component list instead of a vulnerability list | false                                 |
| VEX2PDF_SHOW_COMPONENTS   | Whether to additionally show the component list after the vulnerability list  | true                                  |
| VEX2PDF_MAX_JOBS          | Controls the maximum number of concurrent processing jobs                     | Not set (uses max parallelism)        |                             

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

#### VEX2PDF_PURE_BOM_NOVULNS

Whether to treat the file as a pure CycloneDX Bill of Materials only listing the components and ignoring the vulnerability list 

Example : `VEX2PDF_PURE_BOM_NOVULNS=true vex2pdf`

#### VEX2PDF_SHOW_COMPONENTS

Whether to show the complete list of components after the vulnerabilities section. The default behaviour is `true` but this can be overridden

Example: `VEX2PDF_SHOW_COMPONENTS=false vex2pdf`

#### VEX2PDF_MAX_JOBS

Controls the maximum number of concurrent jobs for processing multiple BOM files:
- When not set or set to `0` (default): Uses all available CPU cores for maximum parallelism
- When set to `1`: Runs in single-threaded mode (sequential processing in main thread)
- When set to `2-255`: Uses the specified number of concurrent jobs

**Single-threaded mode** is useful for:
- Debugging and troubleshooting
- Systems with limited resources
- Reproducible processing order

**Multi-threaded mode** (default) provides:
- Faster processing of multiple files
- Better resource utilization on multi-core systems

Example (single-threaded): `VEX2PDF_MAX_JOBS=1 vex2pdf`

Example (4 concurrent jobs): `VEX2PDF_MAX_JOBS=4 vex2pdf`

Example (default parallelism): `VEX2PDF_MAX_JOBS=0 vex2pdf` or simply `vex2pdf`

## Logging

VEX2PDF uses structured logging to provide clear visibility into its operation. Logs include timestamps, severity levels, and module paths for easy troubleshooting.

### Log Levels

The tool supports four log levels:

- **ERROR** - Critical errors that prevent operation (output to stderr)
- **WARN** - Warnings about potential issues or compatibility concerns (output to stderr)
- **INFO** - Normal operational messages showing progress and results (output to stdout) - **Default level**
- **DEBUG** - Detailed internal information including worker thread activity (output to stdout)

### Controlling Log Output

By default, the tool shows **INFO** level logs without any configuration. You can control the log level using the `RUST_LOG` environment variable:

```bash
# Default behavior (info level) - no configuration needed
vex2pdf

# Show all logs including debug information (verbose mode)
RUST_LOG=debug vex2pdf

# Show only warnings and errors (quiet mode)
RUST_LOG=warn vex2pdf

# Show only errors (minimal output)
RUST_LOG=error vex2pdf

# Disable all logging
RUST_LOG=off vex2pdf
```

**Note:** Debug logs are completely removed from release builds at compile time for optimal performance and smaller binary size.

**Output Routing:**
- Informational logs (INFO, DEBUG) → stdout - normal operational output
- Problem logs (WARN, ERROR) → stderr - errors and warnings for script handling

This separation allows for proper Unix-style piping and redirection:
```bash
# Capture only errors to a file
vex2pdf 2> errors.log

# Separate normal output and errors
vex2pdf > output.log 2> errors.log
```

## Documentation

For full API documentation, please visit:
- [vex2pdf on docs.rs](https://docs.rs/vex2pdf)

To generate documentation locally:
```bash
cargo doc --open
```

### Developer Notes

For information about testing, code coverage, architecture, and the traits system, see [Developer Notes](docs/DEVELOPER_NOTES.md).


## CycloneDX Document Format
This tool fully supports CycloneDX schema version 1.5 and provides compatibility for version 1.6 documents that only use 1.5 fields. Documents using 1.6-specific fields may not process correctly. For more information about the CycloneDX format, see:
- [CycloneDX VEX Specification](https://cyclonedx.org/capabilities/vex/)
- [CycloneDX VDR Specification](https://cyclonedx.org/capabilities/vdr/)
- [CycloneDX Schema](https://cyclonedx.org/docs/1.5/json/)

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

## Changelog

Changes to the software between version increments are documented under [Changelog.md](CHANGELOG.md).

## License

This project is licensed under either of:

* [MIT License](LICENSE-MIT)
* [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you shall be dual-licensed as above, without any
additional terms or conditions.

### Third-Party Dependencies

This project uses third-party dependencies that may be distributed under different licenses.
Please refer to the license information provided with each dependency for details.

## Acknowledgments
- [CycloneDX](https://cyclonedx.org/) for CycloneDX document specification
- [cyclonedx-bom](https://crates.io/crates/cyclonedx-bom) for CycloneDX parsing
- [genpdf](https://crates.io/crates/genpdf) for PDF generation
- [serde_json](https://crates.io/crates/serde_json) for JSON processing
- [Liberation Fonts](https://github.com/liberationfonts/liberation-fonts) for the PDF rendering fonts
