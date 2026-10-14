# CycloneDX (VEX/VDR/(S)BoM) to PDF Converter

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![Documentation](https://docs.rs/vex2pdf/badge.svg)](https://docs.rs/vex2pdf)
[![Crates.io](https://img.shields.io/crates/v/vex2pdf.svg)](https://crates.io/crates/vex2pdf)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache%20License%202.0-blue)](LICENSE-APACHE)

A command-line tool that converts CycloneDX (VEX/VDR/(S)BoM) documents in JSON or XML into
human-readable PDF reports. Point it at a file or a directory and it renders vulnerabilities,
analysis states, components and document metadata into one PDF per input document.

CycloneDX **1.5** is fully supported. **1.6** documents are accepted in a compatibility mode as long as
they only use 1.5 fields — see [Version 1.6 Compatibility Mode](#version-16-compatibility-mode).

## Quick Start

```bash
cargo install vex2pdf          # or grab a binary from the releases page

vex2pdf                        # convert every CycloneDX file in the current directory
vex2pdf my-bom.json            # convert one file
vex2pdf ./boms/ -d ./reports/  # convert a directory into another directory
```

Each input produces a PDF of the same name next to it, or in `--output-dir` if given. No configuration
file, no fonts to install, no network access.

## Why vex2pdf?

- **Self-contained**: single static binary with embedded Liberation Sans fonts — no runtime
  dependencies, no system libraries, works fully offline and air-gapped
- **Small and fast**: ~10 MB binary, large BOMs render in milliseconds
- **Concurrent**: thread-safe batch processing with a configurable job limit
- **Safe**: 100% safe Rust (`unsafe` is forbidden)
- **Cross-platform**: the same code builds and runs on Linux, Windows and macOS
- **Usable as a library** as well as a CLI, with the CLI and concurrency behind feature flags

Built for automated security reporting in CI/CD pipelines and containerised workflows.

<!-- TOC -->
* [CycloneDX (VEX/VDR/(S)BoM) to PDF Converter](#cyclonedx-vexvdrsbom-to-pdf-converter)
  * [Quick Start](#quick-start)
  * [Why vex2pdf?](#why-vex2pdf)
  * [Supported Document Types](#supported-document-types)
    * [VEX (Vulnerability Exploitability eXchange)](#vex-vulnerability-exploitability-exchange)
    * [VDR (Vulnerability Disclosure Report)](#vdr-vulnerability-disclosure-report)
    * [BOM/SBOM (Bill of Materials)](#bomsbom-bill-of-materials)
  * [What the Report Contains](#what-the-report-contains)
    * [Vulnerability Analysis](#vulnerability-analysis)
    * [Nested Components (Assemblies)](#nested-components-assemblies)
  * [Fonts Handling](#fonts-handling)
    * [Font Licensing](#font-licensing)
  * [Installation](#installation)
    * [As a Command-Line Tool](#as-a-command-line-tool)
    * [As a Library](#as-a-library)
    * [From Source](#from-source)
  * [Usage](#usage)
    * [Command-Line Arguments](#command-line-arguments)
  * [Configuration](#configuration)
    * [Environment Variables](#environment-variables)
  * [Logging](#logging)
  * [Example Run](#example-run)
  * [Example Output](#example-output)
  * [CycloneDX Document Format](#cyclonedx-document-format)
    * [Version 1.6 Compatibility Mode](#version-16-compatibility-mode)
  * [Security Considerations](#security-considerations)
    * [Known dependency advisory](#known-dependency-advisory)
  * [Documentation](#documentation)
  * [License](#license)
    * [Contribution](#contribution)
    * [Third-Party Dependencies](#third-party-dependencies)
  * [Acknowledgments](#acknowledgments)
<!-- TOC -->

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

## What the Report Contains

Whatever the source document provides is rendered; sections with no data are omitted:

- Document metadata, timestamps, and the tools that generated the document
- Vulnerability details with severity ratings and their sources
- Component-vulnerability correlation through the `affected` property (VDR)
- Exploitability assessments and risk context (VEX)
- Component inventories and dependencies (BOM/SBOM), including nested assemblies

### Vulnerability Analysis

CycloneDX analysis data is colour-coded for quick risk assessment.

**Analysis states:**

| State | Colour | Meaning |
|---|---|---|
| Exploitable | Red | Directly or indirectly exploitable in the current environment |
| In Triage | Orange | Under active investigation |
| Resolved | Green | Successfully remediated |
| Resolved With Pedigree | Dark green | Remediated with verifiable commit history and audit trail |
| Not Affected | Forest green | Confirmed not vulnerable to this issue |
| False Positive | Steel blue | Incorrectly identified as vulnerable |

**Response actions:**

| Response | Colour | Meaning |
|---|---|---|
| Update | Blue | A software update fixes the vulnerability |
| Rollback | Blue | Reverting to an earlier version is recommended |
| Workaround Available | Orange | Temporary mitigation or configuration change exists |
| Can Not Fix | Red | Technical limitations prevent a fix |
| Will Not Fix | Red | Will not be addressed (risk accepted or deprioritised) |

When present, the analysis block follows each vulnerability's description and carries its **state**,
**response** actions (as a bracketed list), the **justification** for "Not Affected" determinations
(e.g. "Code Not Reachable", "Protected By Compiler"), the free-text **details** of the assessment, and
the `firstIssued` / `lastUpdated` **timestamps**.

### Nested Components (Assemblies)

CycloneDX lets a component contain components (`component.components`), a hierarchical representation of
a component assembly. Since bom-refs are document-wide, a vulnerability may target a component at any
depth. vex2pdf walks the whole tree and shows where a nested component sits, outermost ancestor first:

```
Affected Document Components : [ openssl: 3.6.3 (via platform-x: 0.55.2) ]
Affected Document Components : [ zlib: 1.3.1 (via platform-x: 0.55.2 > sdk-y: 8.0) ]
```

The components section lists nested components right after their parent, carrying the same marker:

```
Name: platform-x
Version: 0.55.2
Name: openssl (via platform-x: 0.55.2)
Version: 3.6.3
```

An ancestor without a version is shown as its name alone (`libgamma: 1.0 (via platform-z)`). Documents
without nesting are unaffected.

## Fonts Handling

This tool uses Liberation Sans fonts to render PDFs. The fonts are embedded directly in the binary, so
**no extra font configuration is required** and the binary works standalone and is fully portable.
(Since v0.6.1 — earlier versions needed external fonts.)

### Font Licensing

The embedded Liberation Sans fonts are licensed under the SIL Open Font License (OFL). Use the
`--license` flag to display full license details:

```bash
vex2pdf --license
```

The font license file is also available at
[Liberation fonts License file](external/fonts/liberation-fonts/LICENSE) in this repository.

## Installation

Requires Rust and Cargo (latest stable) to build or install from source.

### As a Command-Line Tool

```bash
cargo install vex2pdf
```

The `vex2pdf` binary lands in your Cargo bin directory. Pre-built Linux and Windows binaries are also
attached to each [release](https://gitlab.com/jurassicLizard/vex2pdf/-/releases). macOS has no
pre-built binary — build it from source, or
[open an issue](https://gitlab.com/jurassicLizard/vex2pdf/-/issues) if you need one.

### As a Library

```bash
# Default: CLI + concurrent processing
cargo add vex2pdf

# Library with concurrent processing (no CLI dependencies)
cargo add vex2pdf --no-default-features --features concurrency

# Library with sequential processing (minimal dependencies)
cargo add vex2pdf --no-default-features

# CLI with sequential processing
cargo add vex2pdf --no-default-features --features cli
```

| Feature | Description | Default |
|---------|-------------|---------|
| `cli` | Command-line interface and argument parsing (requires `clap`, `env_logger`) | ✓ Yes |
| `concurrency` | Concurrent file processing using a threadpool (requires `jlizard-simple-threadpool`) | ✓ Yes |

```rust
use vex2pdf::lib_utils::config::Config;
use vex2pdf::run;

fn main() {
  let config = Config::default()
          .working_path("./input")
          .output_dir("./output")
          .max_jobs(Some(4));  // Only available with the 'concurrency' feature

  run(config).expect("Failed to process files");
}
```

Full API documentation lives at [docs.rs/vex2pdf](https://docs.rs/vex2pdf).

### From Source

Clone the repository and run `cargo build --release`. The binary is at `target/release/vex2pdf`.

## Usage

vex2pdf is built for batch processing. Run without arguments to convert every CycloneDX document in the
current directory:

```shell
./vex2pdf
```

It then:
1. Scans the current directory for JSON and XML files
2. Attempts to parse each one as a CycloneDX (VEX/VDR/(S)BoM) document
3. Writes a PDF report named after the input file
4. Reports progress and results on the console

Directory scanning is **non-recursive** — only the first level is read.

### Command-Line Arguments

Every argument can also be set through an environment variable, listed under
[Configuration](#configuration). Run `vex2pdf --help` for the authoritative list.

```
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
  -L, --license
          Dumps the open-source software license text to the terminal and exits
  -h, --help
          Print help
  -V, --version
          Print version
```

**Examples:**

```bash
vex2pdf my-bom.json                              # one file
vex2pdf /path/to/bom/files/                      # a directory
vex2pdf my-bom.json -d /path/to/output/          # choose the output directory
vex2pdf -c false                                 # hide the components list
vex2pdf -t "Security Vulnerability Assessment"   # custom report title
vex2pdf --max-jobs 1 my-bom.json                 # single-threaded
vex2pdf my-bom.json -d ./reports/ -t "Q4 Security Report"
```

## Configuration

No configuration files are required. Everything is set through CLI arguments or the equivalent
environment variables.

### Environment Variables

| Variable | Purpose | Default | Example |
|---|---|---|---|
| `VEX2PDF_WORKING_PATH` | File or directory to process | Current directory | `VEX2PDF_WORKING_PATH=./boms` |
| `VEX2PDF_OUTPUT_DIR` | Where PDFs are written | Alongside the input | `VEX2PDF_OUTPUT_DIR=./reports` |
| `VEX2PDF_NOVULNS_MSG` | Show the "No Vulnerabilities reported" message | `true` | `VEX2PDF_NOVULNS_MSG=false` |
| `VEX2PDF_REPORT_TITLE` | Override the report title | Default title | `VEX2PDF_REPORT_TITLE="My Custom VEX Report"` |
| `VEX2PDF_PDF_META_NAME` | Override the PDF metadata title | Default metadata title | `VEX2PDF_PDF_META_NAME="VEX Report - Company XYZ"` |
| `VEX2PDF_PURE_BOM_NOVULNS` | Treat the file as a component list only, ignoring vulnerabilities | `false` | `VEX2PDF_PURE_BOM_NOVULNS=true` |
| `VEX2PDF_SHOW_COMPONENTS` | Show the component list after the vulnerability list | `true` | `VEX2PDF_SHOW_COMPONENTS=false` |
| `VEX2PDF_MAX_JOBS` | Maximum concurrent processing jobs | Unset (max parallelism) | `VEX2PDF_MAX_JOBS=4` |

Two of these need a little more explanation:

**`VEX2PDF_NOVULNS_MSG`** — when `true` or unset, a "Vulnerabilities" section is rendered carrying a
"No Vulnerabilities reported" message. When `false`, the section is omitted from the PDF entirely.

**`VEX2PDF_MAX_JOBS`** — `0` or unset uses all available CPU cores; `1` processes sequentially in the
main thread, which is useful for debugging, constrained systems and a reproducible processing order;
`2`–`255` uses that many concurrent jobs.

> **Windows users**: set environment variables with
> `set VEX2PDF_ENV_VARIABLE=false && vex2pdf` (Command Prompt) or
> `$env:VEX2PDF_ENV_VARIABLE="false"; vex2pdf` (PowerShell).

## Logging

Logs carry timestamps, severity levels and module paths. Four levels are available: **ERROR** and
**WARN** go to stderr, **INFO** (the default) and **DEBUG** go to stdout. That split makes normal
Unix redirection work:

```bash
vex2pdf 2> errors.log                  # capture only problems
vex2pdf > output.log 2> errors.log     # separate the two streams
```

The level is controlled with `RUST_LOG`:

```bash
vex2pdf                 # info (default, no configuration needed)
RUST_LOG=debug vex2pdf  # verbose, including worker thread activity
RUST_LOG=warn vex2pdf   # warnings and errors only
RUST_LOG=error vex2pdf  # errors only
RUST_LOG=off vex2pdf    # silent
```

**Note:** debug logs are compiled out of release builds entirely, for performance and binary size.

## Example Run

```
$ ./vex2pdf
[INFO] vex2pdf 4.1.1 - CycloneDX (VEX) to PDF Converter
[INFO] Copyright (c) 2025 Salem B. - MIT Or Apache 2.0 License
[INFO] Active font path: <embedded liberationSans fonts> -- use --license to show Font license details
[INFO] Scanning for BoM/Vex Files in ./documents
[INFO] Found 2 JSON files
[INFO] Found 5 XML files
[INFO] Processing ./documents/example1.json
[INFO] Successfully generated PDF: ./documents/example1.pdf
[INFO] Processing ./documents/example3.xml
[WARN] NOTE: Downgrading CycloneDX BOM from spec version 1.6 to 1.5
[WARN] Reason: Current implementation does not yet fully support spec version 1.6
[WARN] Warning: This compatibility mode only works for BOMs that don't utilize 1.6-specific fields
[INFO] Successfully generated PDF: ./documents/example3.pdf
[INFO] Processed 7 files
```

If any input fails to process, the run exits with a non-zero status (since v4.0.0), so CI pipelines
surface the failure instead of treating it as success.

## Example Output

Sample generated PDFs in this repository:

- **[VDR with Vulnerability Analysis](tests/test_artifacts/expected_pdfs/bom_vdr_with_analysis.pdf)** - Colour-coded analysis states and response actions
- **[VDR with Multiple Vulnerabilities](tests/test_artifacts/expected_pdfs/bom_vdr_with_many_vulns.pdf)** - Multiple CVEs and affected components
- **[Simple VEX Report](tests/test_artifacts/expected_pdfs/bom_vex_simple_one_vuln.pdf)** - Basic VEX document with a single vulnerability
- **[XML-based VDR](tests/test_artifacts/expected_pdfs/bom_xml_vdr_simple.pdf)** - XML input processing
- **[VDR with GHSA Entries](tests/test_artifacts/expected_pdfs/bom_vdr_with_ghsa_entries.pdf)** - GitHub Security Advisory identifiers
- **[VDR with Nested Components](tests/test_artifacts/expected_pdfs/bom_vdr_with_nested_components.pdf)** - Component assembly with the `(via ...)` ancestry marker

All PDFs embed their fonts and need nothing extra to view.

## CycloneDX Document Format

CycloneDX 1.5 is fully supported. See:
- [CycloneDX VEX Specification](https://cyclonedx.org/capabilities/vex/)
- [CycloneDX VDR Specification](https://cyclonedx.org/capabilities/vdr/)
- [CycloneDX Schema](https://cyclonedx.org/docs/1.5/json/)

### Version 1.6 Compatibility Mode

When the tool meets a document with `specVersion: "1.6"` it notifies you about the downgrade, rewrites
the spec version to `"1.5"` in memory, and parses it with the 1.5 parser.

This works for documents that do not use 1.6-specific fields. Documents that do may fail during
processing, no validation of 1.6-specific features is performed, and this remains a stopgap until the
underlying `cyclonedx-bom` library supports 1.6 fully.

## Security Considerations
- The application reads and processes files from the current directory
- No network connections are established
- Input validation is performed on all JSON files

### Known dependency advisory

`cargo audit` reports **[RUSTSEC-2026-0187](https://rustsec.org/advisories/RUSTSEC-2026-0187)** against
`lopdf 0.26.0` (stack overflow via deeply nested PDF objects, severity 7.5). `lopdf` is reached only
through the `genpdf → printpdf` chain, which pins it, so it cannot be upgraded without replacing the PDF
rendering stack.

**The vulnerable code path is PDF _parsing_, and vex2pdf never parses a PDF** — it only writes them, and
neither `genpdf` nor `printpdf` invokes lopdf's document parser. The advisory is therefore not believed
to be reachable from any vex2pdf functionality. It is nonetheless disclosed here because it will appear
in the audit output of any project that depends on vex2pdf.

Five further "unmaintained" advisories (`encoding`, `lzw`, `rusttype`, `stb_truetype`, `stdweb`) come from
the same chain. See the [Changelog](CHANGELOG.md) for the per-release security notes.

To report a vulnerability in vex2pdf itself, follow [SECURITY.md](SECURITY.md).

## Documentation

- **API reference**: [docs.rs/vex2pdf](https://docs.rs/vex2pdf), or `cargo doc --open` locally
- **Testing, architecture, traits system, CI**: [Developer Notes](docs/DEVELOPER_NOTES.md)
- **Upgrading from 1.x to 2.0**: [Migration Guide](docs/MIGRATION.md)
- **Release history**: [CHANGELOG.md](CHANGELOG.md)

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
