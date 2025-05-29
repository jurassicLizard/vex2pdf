# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# [Unreleased]

### Removed
- Removed License.md file and integrated it in the README.md due to dual licensing
  (LICENSE-MIT and LICENSE-APACHE files now take that role)

### Changed
- Updated Readme.md with dual-licensing details
### Fixed
- Fix formatting of Apache 2.0 license text
- Fix minor typo and missing date in Changelog.md

# [0.7.0] - 2025-05-28

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
- fixed minor documentation issue that is causing some tests to fail

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

[0.6.0]: https://github.com/jurassicLizard/vex2pdf/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/jurassicLizard/vex2pdf/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/jurassicLizard/vex2pdf/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/jurassicLizard/vex2pdf/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/jurassicLizard/vex2pdf/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/jurassicLizard/vex2pdf/releases/tag/v0.1.0
