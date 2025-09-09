use super::super::pdf::font_config::FontsDir;
use super::env_vars::EnvVarNames;
use super::input_file_type::InputFileType;
use crate::lib_utils::run_utils::print_copyright;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

pub struct Config {
    pub working_dir: PathBuf,
    pub show_novulns_msg: bool, //FIXME still unused
    pub file_types_to_process: HashMap<InputFileType, bool>, //FIXME still unused
    pub show_oss_licenses: bool,
    pub pure_bom_novulns: bool,
    pub report_title: Option<String>,
    pub pdf_meta_name: Option<String>,
}

impl Config {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let working_dir = std::env::current_dir()?;
        let show_novulns_msg = EnvVarNames::NoVulnsMsg.is_on_or_unset();
        let mut process_json = EnvVarNames::ProcessJson.is_on_or_unset();
        let process_xml = EnvVarNames::ProcessXml.is_on_or_unset();
        let show_oss_licenses = EnvVarNames::ShowOssLicenses.is_on();
        let show_pure_bom_novulns = EnvVarNames::PureBomNoVulns.is_on();

        // print version info if requested
        if EnvVarNames::VersionInfo.is_on() {
            print_copyright();
        }

        if !show_oss_licenses {
            // print init information only if show oss licenses is off
            FontsDir::print_fonts_info();
            // print default titles details only if show oss licenses is off
            EnvVarNames::print_report_titles_info();
        }

        // validate
        if !(process_json || process_xml) {
            println!("**** WARNING: we cannot have both json and xml deactivated. defaulting to json processing");
            process_json = true;
        }

        // init result map
        let mut file_types_to_process: HashMap<InputFileType, bool> = HashMap::new();
        file_types_to_process.insert(InputFileType::JSON, process_json);
        file_types_to_process.insert(InputFileType::XML, process_xml);

        let config = Config {
            working_dir,
            show_novulns_msg,
            file_types_to_process,
            show_oss_licenses,
            pure_bom_novulns: show_pure_bom_novulns,
            report_title: EnvVarNames::ReportTitle.get_value(),
            pdf_meta_name: EnvVarNames::PdfName.get_value(),
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
    /// // All processing options are enabled by default
    /// assert!(config.pure_bom_novulns);
    /// assert!(config.show_novulns_msg);
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
        let working_dir = std::env::current_dir().expect("Failed to get current directory");

        Self {
            working_dir,
            show_novulns_msg: true,
            file_types_to_process,
            show_oss_licenses: true,
            pure_bom_novulns: true,
            report_title: Some(Self::get_default_report_title().to_string()),
            pdf_meta_name: Some(Self::get_default_pdf_meta_name().to_string()),
        }
    }
}
