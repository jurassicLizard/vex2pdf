//! Configuration management for vex2pdf.
//!
//! This module provides the [`Config`] struct which holds all configuration options
//! for the PDF generation process. Configuration can be created using the builder pattern,
//! defaults, or manual construction.
//!
//! # Creating Configuration
//!
//! ## Builder Pattern (Recommended for Library Use)
//!
//! Use the builder pattern for flexible configuration:
//!
//! ```rust,no_run
//! use vex2pdf::lib_utils::config::Config;
//! use vex2pdf::run;
//!
//! let config = Config::default()
//!     .working_path("./input")
//!     .output_dir("./output")
//!     .max_jobs(Some(4))
//!     .report_title("Q4 2024 Security Report");
//!
//! let res = run(config).expect("something bad happened");
//! ```
//!
//! ## Using Defaults
//!
//! Start with sensible defaults:
//!
//! ```rust
//! use vex2pdf::lib_utils::config::Config;
//!
//! let config = Config::default();
//! // All fields use default values
//! ```
//!
//! ## Manual Construction
//!
//! Create the struct directly for full control:
//!
//! ```rust
//! use vex2pdf::lib_utils::config::Config;
//! use std::path::PathBuf;
//!
//! let config = Config {
//!     working_path: PathBuf::from("./input"),
//!     output_dir: PathBuf::from("./output"),
//!     show_novulns_msg: true,
//!     file_types_to_process: None,
//!     pure_bom_novulns: false,
//!     show_components: true,
//!     report_title: Some("Custom Report".to_string()),
//!     pdf_meta_name: Some("My PDF".to_string()),
//!     max_jobs: Some(4),
//! };
//! ```
//!
//! ## For CLI Applications
//!
//! CLI applications should use `Config::build_from_env_cli()` which parses
//! command-line arguments and environment variables. See the CLI documentation
//! for usage details.
//!
//! # Configuration Options
//!
//! The [`Config`] struct supports:
//!
//! - **Input/Output paths**: Working directory and output directory
//! - **Processing options**: File types to process (JSON/XML), concurrent job limit
//! - **Display options**: Show/hide vulnerability messages, components, licenses
//! - **Customization**: Custom report titles and PDF metadata
//!
//! # Builder Methods
//!
//! All builder methods consume `self` and return `Self`, enabling method chaining:
//!
//! - [`working_path()`](Config::working_path) - Set input directory
//! - [`output_dir()`](Config::output_dir) - Set output directory
//! - [`show_novulns_msg()`](Config::show_novulns_msg) - Show/hide "no vulnerabilities" message
//! - [`file_types_to_process()`](Config::file_types_to_process) - Control JSON/XML processing
//! - [`pure_bom_novulns()`](Config::pure_bom_novulns) - Treat as pure BOM
//! - [`show_components()`](Config::show_components) - Show/hide components list
//! - [`report_title()`](Config::report_title) - Custom report title
//! - [`pdf_meta_name()`](Config::pdf_meta_name) - Custom PDF metadata
//! - [`max_jobs()`](Config::max_jobs) - Set concurrent job limit
//!
//! # Examples
//!
//! ## Full Builder Chain
//!
//! ```rust
//! use vex2pdf::lib_utils::config::Config;
//!
//! let config = Config::default()
//!     .working_path("./input")
//!     .output_dir("./output")
//!     .show_novulns_msg(false)
//!     .pure_bom_novulns(false)
//!     .show_components(true)
//!     .report_title("Security Assessment")
//!     .pdf_meta_name("Company VEX Report")
//!     .max_jobs(Some(4));
//! ```
//!
//! ## Partial Configuration
//!
//! ```rust
//! use vex2pdf::lib_utils::config::Config;
//!
//! // Only override what you need, keep other defaults
//! let config = Config::default()
//!     .output_dir("./reports")
//!     .max_jobs(Some(2));
//! ```
//!
//! For complete CLI documentation:
//! - [README - Configuration](https://gitlab.com/jurassicLizard/vex2pdf/-/blob/master/README.md#configuration)
//! - [README - Environment Variables](https://gitlab.com/jurassicLizard/vex2pdf/-/blob/master/README.md#environment-variables)

use crate::files_proc::model::input_file_type::InputFileType;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[cfg(feature = "cli")]
use super::super::pdf::font_config::FontsDir;
#[cfg(feature = "cli")]
use super::env_vars::EnvVarNames;
#[cfg(feature = "cli")]
use super::run_utils::get_version_info;
#[cfg(feature = "cli")]
use crate::lib_utils::cli_args::CliArgs;
#[cfg(feature = "cli")]
use crate::lib_utils::errors::Vex2PdfError;
#[cfg(feature = "cli")]
use clap::Parser;
#[cfg(feature = "cli")]
use log::{info, warn};

pub struct Config {
    pub working_path: PathBuf,
    pub output_dir: PathBuf,
    pub show_novulns_msg: bool,
    pub file_types_to_process: Option<HashMap<InputFileType, bool>>,
    pub pure_bom_novulns: bool,
    pub show_components: bool,
    pub report_title: Option<String>,
    pub pdf_meta_name: Option<String>,
    pub max_jobs: Option<u8>,
}

impl Config {
    /// This is the legacy build function. has been replaced with [`Self::build_with_env_cli`] and is functially identical
    ///
    #[cfg(feature = "cli")]
    /// Builds a `Config` instance by parsing CLI arguments and environment variables.
    ///
    /// This is the recommended way to create configuration for CLI applications.
    /// It parses command-line arguments using [`clap`], validates paths, and reads
    /// environment variables to populate configuration options.
    ///
    /// # Priority Order
    ///
    /// Configuration values are resolved with this precedence (highest to lowest):
    /// 1. **CLI arguments** (e.g., `--max-jobs 4`)
    /// 2. **Environment variables** (e.g., `VEX2PDF_MAX_JOBS=4`)
    /// 3. **Default values**
    ///
    /// # Returns
    ///
    /// Returns `Ok(Config)` with fully populated configuration, or an error if:
    /// - Path validation fails (e.g., output directory doesn't exist or lacks write permissions)
    /// - Current directory cannot be determined
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vex2pdf::lib_utils::config::Config;
    ///
    /// // Parse configuration from CLI args and environment
    /// let config = Config::build_with_env_cli().expect("Failed to build config");
    ///
    /// // Config now contains values from CLI/env vars with proper precedence
    /// println!("Working directory: {:?}", config.working_path);
    /// ```
    ///
    /// # See Also
    ///
    /// For complete documentation of all CLI arguments and environment variables:
    /// - [README - CLI Arguments](https://gitlab.com/jurassicLizard/vex2pdf/-/blob/master/README.md#command-line-arguments)
    /// - [README - Environment Variables](https://gitlab.com/jurassicLizard/vex2pdf/-/blob/master/README.md#environment-variables)
    pub fn build_with_env_cli() -> Result<Self, Vex2PdfError> {
        // handle cli arguments
        let args = CliArgs::parse();

        // interrupt execution to show license if user has chosen to
        if args.license {
            return Err(Vex2PdfError::VoluntaryLicenseDisplayInterruption);
        }

        // validate potential permissions issues
        args.validate()?;

        // print version info
        info!("{}", get_version_info());
        info!("");

        let working_path = args.input.unwrap_or(std::env::current_dir()?);
        let output_dir = args.output_dir.unwrap_or(std::env::current_dir()?);
        let show_novulns_msg = args
            .show_novulns_msg
            .unwrap_or(EnvVarNames::NoVulnsMsg.is_on_or_unset());
        let mut process_json = EnvVarNames::ProcessJson.is_on_or_unset();
        let process_xml = EnvVarNames::ProcessXml.is_on_or_unset();
        let show_pure_bom_novulns = args
            .pure_bom_novulns
            .unwrap_or(EnvVarNames::PureBomNoVulns.is_on());
        let show_comps = args
            .show_components
            .unwrap_or(EnvVarNames::ShowComponentList.is_on_or_unset());
        let report_title_override = args
            .report_title
            .map(Some)
            .unwrap_or(EnvVarNames::ReportTitle.get_value());
        let pdf_meta_name_override = args
            .meta_name
            .map(Some)
            .unwrap_or(EnvVarNames::PdfName.get_value());
        // set number of jobs
        #[cfg(feature = "concurrency")]
        let max_jobs = args.max_jobs;
        #[cfg(not(feature = "concurrency"))]
        let max_jobs = None;

        // print init information
        FontsDir::print_fonts_info();
        // print default titles details
        EnvVarNames::print_report_titles_info();

        // validate
        if !(process_json || process_xml) {
            warn!("**** WARNING: we cannot have both json and xml deactivated. defaulting to json processing");
            process_json = true;
        }

        // init result map
        let mut file_types_to_process: HashMap<InputFileType, bool> = HashMap::new();
        file_types_to_process.insert(InputFileType::JSON, process_json);
        file_types_to_process.insert(InputFileType::XML, process_xml);

        let config = Config {
            working_path,
            output_dir,
            show_novulns_msg,
            file_types_to_process: Some(file_types_to_process),
            pure_bom_novulns: show_pure_bom_novulns,
            show_components: show_comps,
            report_title: report_title_override,
            pdf_meta_name: pdf_meta_name_override,
            max_jobs,
        };

        Ok(config)
    }

    /// Gets the default title for the pdf metadata
    pub fn get_default_pdf_meta_name() -> &'static str {
        "VEX Vulnerability Report"
    }

    /// Gets the default title of the report which shows on the first page
    pub fn get_default_report_title() -> &'static str {
        "Vulnerability Report Document"
    }

    // Builder pattern methods

    /// Sets the working directory path where input files will be searched.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vex2pdf::lib_utils::config::Config;
    ///
    /// let config = Config::default()
    ///     .working_path("./input");
    /// ```
    pub fn working_path(mut self, path: impl AsRef<Path>) -> Self {
        self.working_path = path.as_ref().to_path_buf();
        self
    }

    /// Sets the output directory path where generated PDFs will be saved.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vex2pdf::lib_utils::config::Config;
    ///
    /// let config = Config::default()
    ///     .output_dir("./output");
    /// ```
    pub fn output_dir(mut self, path: impl AsRef<Path>) -> Self {
        self.output_dir = path.as_ref().to_path_buf();
        self
    }

    /// Controls whether to show "No Vulnerabilities reported" message when no vulnerabilities exist.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vex2pdf::lib_utils::config::Config;
    ///
    /// let config = Config::default()
    ///     .show_novulns_msg(false);  // Hide the message
    /// ```
    pub fn show_novulns_msg(mut self, show: bool) -> Self {
        self.show_novulns_msg = show;
        self
    }

    /// Sets which file types (JSON/XML) should be processed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vex2pdf::lib_utils::config::Config;
    /// use vex2pdf::files_proc::model::input_file_type::InputFileType;
    /// use std::collections::HashMap;
    ///
    /// let mut types = HashMap::new();
    /// types.insert(InputFileType::JSON, true);
    /// types.insert(InputFileType::XML, false);
    ///
    /// let config = Config::default()
    ///     .file_types_to_process(Some(types));  // Only process JSON
    /// ```
    pub fn file_types_to_process(mut self, types: Option<HashMap<InputFileType, bool>>) -> Self {
        self.file_types_to_process = types;
        self
    }

    /// Sets whether to treat files as pure BOM (components only, no vulnerabilities).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vex2pdf::lib_utils::config::Config;
    ///
    /// let config = Config::default()
    ///     .pure_bom_novulns(true);  // Show only components
    /// ```
    pub fn pure_bom_novulns(mut self, pure_bom: bool) -> Self {
        self.pure_bom_novulns = pure_bom;
        self
    }

    /// Controls whether to show the components list in the PDF report.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vex2pdf::lib_utils::config::Config;
    ///
    /// let config = Config::default()
    ///     .show_components(false);  // Hide components list
    /// ```
    pub fn show_components(mut self, show: bool) -> Self {
        self.show_components = show;
        self
    }

    /// Sets a custom report title for the PDF.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vex2pdf::lib_utils::config::Config;
    ///
    /// let config = Config::default()
    ///     .report_title("Q4 2024 Security Assessment");
    /// ```
    pub fn report_title(mut self, title: impl Into<String>) -> Self {
        self.report_title = Some(title.into());
        self
    }

    /// Sets custom PDF metadata name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vex2pdf::lib_utils::config::Config;
    ///
    /// let config = Config::default()
    ///     .pdf_meta_name("Company VEX Report");
    /// ```
    pub fn pdf_meta_name(mut self, name: impl Into<String>) -> Self {
        self.pdf_meta_name = Some(name.into());
        self
    }

    /// Sets the maximum number of concurrent jobs for processing multiple files.
    ///
    /// - `Some(1)` - Single-threaded mode (sequential processing)
    /// - `Some(n)` - Use n concurrent jobs (2-255)
    /// - `None` or `Some(0)` - Use maximum available parallelism
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vex2pdf::lib_utils::config::Config;
    ///
    /// let config = Config::default()
    ///     .max_jobs(Some(4));  // Use 4 concurrent jobs
    /// ```
    pub fn max_jobs(mut self, jobs: Option<u8>) -> Self {
        self.max_jobs = jobs;
        self
    }
}

impl Default for Config {
    /// Creates a `Config` instance with default values for all configuration options.
    ///
    /// This implementation provides sensible defaults that match the application's
    /// standard behavior when no environment variables are set. This does not process
    /// any environment variables, if you need to process environment variables use `Config::build()`
    /// instead.
    ///
    /// # Default Values
    ///
    /// - **working_dir**: Current working directory
    /// - **show_novulns_msg**: `true` - Display "No Vulnerabilities" message when applicable
    /// - **file_types_to_process**: Both JSON and XML processing enabled (`true`)
    /// - **show_oss_licenses**: `true` - Display open source license information
    /// - **show_components**: `true` - Include component information in reports
    /// - **report_title**: Default report title from `get_default_report_title()`
    /// - **pdf_meta_name**: Default PDF metadata name from `get_default_pdf_meta_name()`
    ///
    /// # Behavior
    ///
    /// These defaults represent the "out-of-the-box" configuration that provides
    /// the most comprehensive reporting. Users can override these values through
    /// environment variables or by using `Config::build()` which respects
    /// environment variable settings.
    ///
    /// # Panics
    ///
    /// Panics if the current working directory cannot be determined.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vex2pdf::lib_utils::config::Config;
    /// use std::default::Default;
    ///
    /// // Create config with all default values
    /// let config = Config::default();
    ///
    /// // All processing options are set to defaults according to the most common, perceived, use
    /// // This can be overridden using the respective environment variables check
    /// assert_eq!(config.pure_bom_novulns,false);
    /// assert_eq!(config.show_novulns_msg,true);
    /// assert_eq!(config.show_components,true);
    /// ```
    ///
    /// # See Also
    ///
    /// - `Config::build()` for environment-variable-aware configuration
    /// - README.md for detailed environment variable documentation
    fn default() -> Self {
        let mut file_types_to_process: HashMap<InputFileType, bool> = HashMap::new();
        file_types_to_process.insert(InputFileType::JSON, true);
        file_types_to_process.insert(InputFileType::XML, true);
        let working_path = std::env::current_dir().expect("Failed to get current directory");
        let output_dir = working_path.clone();
        Self {
            working_path,
            output_dir,
            show_novulns_msg: true,
            file_types_to_process: Some(file_types_to_process),
            pure_bom_novulns: false,
            show_components: true,
            report_title: Some(Self::get_default_report_title().to_string()),
            pdf_meta_name: Some(Self::get_default_pdf_meta_name().to_string()),
            max_jobs: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_working_path() {
        let config = Config::default().working_path("/tmp/test");
        assert_eq!(config.working_path, PathBuf::from("/tmp/test"));
    }

    #[test]
    fn test_builder_output_dir() {
        let config = Config::default().output_dir("/tmp/output");
        assert_eq!(config.output_dir, PathBuf::from("/tmp/output"));
    }

    #[test]
    fn test_builder_show_novulns_msg() {
        let config = Config::default().show_novulns_msg(false);
        assert_eq!(config.show_novulns_msg, false);
    }

    #[test]
    fn test_builder_pure_bom_novulns() {
        let config = Config::default().pure_bom_novulns(true);
        assert_eq!(config.pure_bom_novulns, true);
    }

    #[test]
    fn test_builder_show_components() {
        let config = Config::default().show_components(false);
        assert_eq!(config.show_components, false);
    }

    #[test]
    fn test_builder_report_title() {
        let config = Config::default().report_title("Custom Title");
        assert_eq!(config.report_title, Some("Custom Title".to_string()));
    }

    #[test]
    fn test_builder_pdf_meta_name() {
        let config = Config::default().pdf_meta_name("Custom Meta");
        assert_eq!(config.pdf_meta_name, Some("Custom Meta".to_string()));
    }

    #[test]
    fn test_builder_max_jobs() {
        let config = Config::default().max_jobs(Some(4));
        assert_eq!(config.max_jobs, Some(4));
    }

    #[test]
    fn test_builder_chaining() {
        let config = Config::default()
            .working_path("/tmp/input")
            .output_dir("/tmp/output")
            .max_jobs(Some(2))
            .report_title("Chained Config")
            .show_components(false);

        assert_eq!(config.working_path, PathBuf::from("/tmp/input"));
        assert_eq!(config.output_dir, PathBuf::from("/tmp/output"));
        assert_eq!(config.max_jobs, Some(2));
        assert_eq!(config.report_title, Some("Chained Config".to_string()));
        assert_eq!(config.show_components, false);
    }

    #[test]
    fn test_default_values() {
        let config = Config::default();
        assert_eq!(config.show_novulns_msg, true);
        assert_eq!(config.show_components, true);
        assert_eq!(config.pure_bom_novulns, false);
        assert!(config.working_path.exists());
    }

    #[test]
    fn test_get_default_titles() {
        assert_eq!(
            Config::get_default_report_title(),
            "Vulnerability Report Document"
        );
        assert_eq!(
            Config::get_default_pdf_meta_name(),
            "VEX Vulnerability Report"
        );
    }
}
