/// Environment variable names used in the application
pub enum EnvVarNames {
    /// Standard HOME environment variable
    Home,
    /// Controls whether to display "No Vulnerabilities reported" message
    /// When set to "false", the Vulnerabilities section will be omitted completely
    /// if no vulnerabilities exist.
    /// When set to "true" or not set (default), the "No Vulnerabilities reported"
    /// message will be displayed when no vulnerabilities are present.
    NoVulnsMsg,
    /// USER CANNOT YET OVERRIDE THIS OPTION
    ProcessJson,
    /// USER CANNOT YET OVERRIDE THIS OPTION
    ProcessXml,
    /// Shows OSS License Information if set to true
    ShowOssLicenses,
    /// Shows Software version and copyright Information if set to true
    VersionInfo,
    /// Controls the title shown in the PDF when it is generated
    ReportTitle,
    /// Controls the metadata name which is usually displayed in window headers of readers
    PdfName,
    /// When set, treats the document as a pure CycloneDX BoM without the vulnerabilities section
    /// Which would entail only listing the components and their versions
    PureBomNoVulns,
}

impl EnvVarNames {
    pub fn as_str(&self) -> &'static str {
        match self {
            EnvVarNames::Home => "HOME",
            EnvVarNames::NoVulnsMsg => "VEX2PDF_NOVULNS_MSG",
            EnvVarNames::ProcessJson => "VEX2PDF_JSON",
            EnvVarNames::ProcessXml => "VEX2PDF_XML",
            EnvVarNames::ShowOssLicenses => "VEX2PDF_SHOW_OSS_LICENSES",
            EnvVarNames::VersionInfo => "VEX2PDF_VERSION_INFO",
            EnvVarNames::ReportTitle => "VEX2PDF_REPORT_TITLE",
            EnvVarNames::PdfName => "VEX2PDF_PDF_META_NAME",
            EnvVarNames::PureBomNoVulns => "VEX2PDF_PURE_BOM_NOVULNS",
        }
    }
    /// this is useful for environment variables which should be on by default
    pub fn is_on_or_unset(&self) -> bool {
        match std::env::var(self.as_str()) {
            Ok(value) => self.is_value_on(&value),
            Err(_) => true, // Variable isn't set, default to ON
        }
    }

    pub fn is_on(&self) -> bool {
        match std::env::var(self.as_str()) {
            Ok(value) => self.is_value_on(&value),
            Err(_) => false, // Variable isn't set, so we are off
        }
    }

    /// Prints information about currently used pdf titles
    pub fn print_report_titles_info() {
        println!();
        match EnvVarNames::ReportTitle.get_value() {
            Some(title) => {
                println!("Overriding report title to {title}");
            }
            None => {
                println!("Using default report title");
                println!(
                    "to override this set the {} environment variable to the desired title",
                    EnvVarNames::ReportTitle.as_str()
                );
            }
        };
        println!();
        match EnvVarNames::PdfName.get_value() {
            Some(title) => {
                println!("Overriding pdf metadata title to {title}");
            }
            None => {
                println!("Using default pdf metadata title");
                println!(
                    "to override this set the {} environment variable to the desired title",
                    EnvVarNames::PdfName.as_str()
                );
            }
        };
        println!();
    }

    // Helper method to determine if a value represents "on"
    fn is_value_on(&self, value: &str) -> bool {
        !(value.eq_ignore_ascii_case("false")
            || value.eq_ignore_ascii_case("off")
            || value.eq_ignore_ascii_case("no")
            || value.eq_ignore_ascii_case("0"))
    }

    /// Helper method to get the value of the variable
    pub fn get_value(&self) -> Option<String> {
        match std::env::var(self.as_str()) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EnvVarNames;

    // tests for private functions that cannot be tested in lib
    #[test]
    fn test_is_value_on_private() {
        // Test is_value_on directly
        {
            let var = EnvVarNames::ProcessJson; // must be different than the tests under lib.rs to not cause race conditions

            // True values
            for value in &[
                "true",
                "True",
                "TRUE",
                "yes",
                "YES",
                "1",
                "on",
                "ON",
                "anything_else",
            ] {
                assert_eq!(
                    var.is_value_on(value),
                    true,
                    "is_value_on() failed for value: {}",
                    value
                );
            }

            // False values
            for value in &["false", "False", "FALSE", "no", "NO", "0", "off", "OFF"] {
                assert_eq!(
                    var.is_value_on(value),
                    false,
                    "is_value_on() failed for value: {}",
                    value
                );
            }
        }
    }
}
