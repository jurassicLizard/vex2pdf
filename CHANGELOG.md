# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Added cli arguments without changing the default behaviour of the application which is to automatically scan the current directory upon execution and work in one depth
- Added cli arguments to env variable handling making cli arguments override environment variables
- Added support for single-file processing instead of automatic batch processing. The default is still to scan automatically and batch process all files in the working directory
- Added aggressive optimization profile for CI builds in Cargo.toml
- Added re-export paths for some crate paths to simplify things for consumers
- Added `VEX2PDF_OUTPUT_DIR` environment variable to override destination directory
- Added Processor and Renderer trait system for improved extensibility
- Added support for parallel processing of files
- Added handling for working with single files
- Added handling for a distinct working directory definition
- Added handling for passing an input path or file which is optional and the tool reverts to default behaviour
- Added integration tests
- Added a vulnerability analysis section to PDF reports displaying CycloneDX analysis data with color-coded states and responses
- Added color-coded analysis state indicators (Exploitable=red, Resolved=green, In Triage=orange, False Positive=blue, Not Affected=green, Resolved With Pedigree=dark green)
- Added color-coded response action indicators (Update/Rollback=blue, Workaround Available=orange, Can Not Fix/Will Not Fix=red)
- Added utility functions for analysis formatting (`get_style_analysis_state`, `get_style_analysis_response`, `get_styled_vector_as_paragraph`, `get_formatted_key_val_text`, `prettify_string_analysis`)
- Added unit tests for analysis utility functions in generator module

### Fixed
- Fixed GitLab CI not able to test due to missing rustup dependencies
- Fixed the rendering issue for the newline character

### Changed
- Changed Env_vars as_str() method to be const to allow some compile time operations
- Changed signature of `vex2pdf::pdf::generator::PdfGenerator::new(Option<'a str>, Option<'a str>, bool, bool, bool)` to `PdfGenerator::new(Arc<crate::lib_utils::config::Config>)`
- Changed signature of `vex2pdf::run(config: &Config)` to `crate::run(config::Config)`, i.e. run now owns the configuration struct
- Changed signature of `vex2pdf::utils::get_output_pdf_path` to return a `Result<T,E>`

### Deprecated
- `VEX2PDF_VERSION_INFO` is now replaced with a cli argument and has entered a deprecation phase (will be removed by the next minor release)
- `VEX2PDF_SHOW_OSS_LICENSES` is now replaced with a cli argument and has entered a deprecation phase (will be removed by the next minor release)


## [0.8.2] - 2025-09-10

### Added
- Added GitLab CI yml file for automatic binary releases
- Added VEX2PDF CycloneDX Bill of Materials title rendering when running in Pure BoM mode with `VEX2PDF_PURE_BOM_NOVULNS=true`
- Added version string rendering for Metadata Tools
- Added version string rendering for Metadata Component
- Added various updates and fixes to the README.md

## [0.8.1] - 2025-09-10

### Added
- Added `VEX2PDF_SHOW_COMPONENT` environment variable to restore possibility to show a flat components list as well as the vulnerabilities and associated affected components

### Fixed
- Updated [Readme](README.md) to make it more obvious that the tool also handles vdr as well as vex and bom
- Fixed wrong default value in [Readme](README.md) for the `VEX2PDF_PURE_BOM_NOVULNS` to `false`
- Fixed Regression where it is no longer possible to show components as well as vulnerabilities. It is now possible
to show both through the `VEX2PDF_SHOW_COMPONENTS` environment variable which is set by default to true

## [0.8.0] - 2025-09-09

### Added
- Added `VEX2PDF_PURE_BOM_NOVULNS` environment variable to control whether we show only the components (CycloneDX BoM) instead of the full vulnerability list (CycloneDX-VEX)
- Changed Behaviour of the Vulnerability section renderer to also show affected components

### Fixed
- Fixed Readme.md Section chapter hierarchy for the Changelog chapter

### Changed
- Updated Readme.md with `VEX2PDF_PURE_BOM_NOVULNS` environment variable information
- Migrated repository to new GitLab space
- Changed Readme.md with new notice on Binary releases
- Changed Vulnerability section handling to deal with the new component handling
- Updated styles of some sections of the PDF to enhance visibility

## [0.7.1] - 2025-06-11

### Added
- Added Source Detail information for the severity rating output

### Removed
- Removed License.md file and integrated it in the README.md due to dual licensing
  (LICENSE-MIT and LICENSE-APACHE files now take that role)

### Changed
- Updated Readme.md with dual-licensing details

### Fixed
- Fixed formatting of Apache 2.0 license text
- Fixed minor typo and missing date in Changelog.md
- Fixed `VEX2PDF_SHOW_OSS_LICENSES` env variable handlers to show updated license information


## [0.7.0] - 2025-05-28

### Added
- Added `VEX2PDF_REPORT_TITLE` environment variable to override the default report title
- Added `VEX2PDF_PDF_META_NAME` environment variable to override the PDF metadata title
- Added ability to customize report titles via environment variables

### Removed
- Removed old fonts handling completely which has been replaced with embedded fonts
- Removed deprecated functions from documentation

### Changed
- Upgraded license from MIT to `MIT OR Apache 2.0` at the user's discretion

## [0.6.2] - 2025-05-22

### Fixed
- Fixed minor documentation issue that is causing some tests to fail

## [0.6.1] - 2025-05-22

### Changed
- Updated [Readme](README.md)

### Added
- Made liberation-fonts embedded in the binary
- Added the VEX2PDF_SHOW_OSS_LICENSES environment variable for showing relevant OSS Licenses
- Added the VEX2PDF_VERSION_INFO environment variable in order to make version info optional

### Deprecated
- the VEX2PDF_FONTS_PATH environment variable is now deprecated. Starting from 0.7.0 we will only rely on embedded fonts to make
the software more portable and simplify the code
- Old fonts handling is now deprecated, and in the future only embedded fonts will be used

## [0.6.0] - 2025-05-20

### Changed
- Major internal code reorganization for better maintainability
- Added XML parsing capability
- Added centralized configuration in preperation for extending environment variables to provide further options
- No changes to the public API

## [0.5.0] - 2025-05-15

### Added
- `VEX2PDF_NOVULNS_MSG` environment variable to control visibility of 'No Vulnerabilities reported' message
- `VEX2PDF_FONTS_PATH` environment variable to override liberation-fonts directory

### Changed
- Standardized and centralized environment variable naming and handling
- Improved user control over output formatting for cleaner reports
- Refactored font handling code to its own module

### Documentation
- Documented all environment variables in configuration section

## [0.4.0] - 2025-04-24

### Changed
- Migrated to cyclonedx-bom official crate for model definitions
- Removed old cyclonedx model definitions

### Added
- Added support for 1.6 BOMs until upstream adds it
- Added verbose error output for font failures

## [0.3.0] - 2025-04-23

### Fixed
- Schema issue with the metadata object
- Tools field under metadata to be fully compliant with 1.5 or 1.6 specs of CycloneDX
- Various formatting issues

## [0.2.0] - 2025-04-23

### Fixed
- Advisory fields marked as optional to not fail when they do not exist (as per the CycloneDX spec)

## [0.1.0] - 2025-04-22

### Added
- Initial public release
- File exclusion functionality
- Documentation and license information

