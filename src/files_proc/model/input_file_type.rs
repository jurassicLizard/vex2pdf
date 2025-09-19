use std::ffi::OsStr;

/// Represents the supported input file types for VEX document processing.
///
/// This enum defines the file formats that can be processed by the vex2pdf application.
/// Currently, two formats are supported:
/// - XML: For VEX documents in XML format
/// - JSON: For VEX documents in JSON format
///
/// The enum implements methods to obtain string representations of the file type
/// for various use cases like file extension matching, logging, or error messages.
///
/// # Examples
///
/// ```rust
/// use vex2pdf::files_proc::model::input_file_type::InputFileType;
///
/// // Get lowercase representation for file extension matching
/// assert_eq!(InputFileType::XML.as_str_lowercase(), "xml");
/// assert_eq!(InputFileType::JSON.as_str_lowercase(), "json");
///
/// // Get uppercase representation for display or logging
/// assert_eq!(InputFileType::XML.as_str_uppercase(), "XML");
/// assert_eq!(InputFileType::JSON.as_str_uppercase(), "JSON");
/// ```
#[derive(Eq, Hash, PartialEq)]
pub enum InputFileType {
    /// Represents an XML format VEX document
    XML,
    /// Represents a JSON format VEX document
    JSON,
    /// Represents an unsupported file format
    UNSUPPORTED,
}

impl InputFileType {
    /// Returns a lowercase string representation of the file type.
    ///
    /// This is useful for file extension matching or generating paths
    /// where lowercase is preferred.
    ///
    /// # Returns
    ///
    /// A lowercase static string representation of the file type:
    /// - `"xml"` for `InputFileType::XML`
    /// - `"json"` for `InputFileType::JSON`
    ///
    /// # Examples
    ///
    /// ```
    /// use vex2pdf::files_proc::model::input_file_type::InputFileType;
    ///
    /// let file_extension = InputFileType::XML.as_str_lowercase();
    /// assert_eq!(file_extension, "xml");
    ///
    /// // Can be used for file filtering
    /// let path = "document.json";
    /// if path.ends_with(InputFileType::JSON.as_str_lowercase()) {
    ///     // Process JSON file
    /// }
    ///
    /// // This following would fail with const_panic during compilation
    /// // let is_unsupported = path.ends_with(InputFileType::UNSUPPORTED.as_str_lowercase());
    /// ```
    pub const fn as_str_lowercase(&self) -> &'static str {
        match self {
            InputFileType::XML => "xml",
            InputFileType::JSON => "json",
            InputFileType::UNSUPPORTED => panic!("Can not call this on an unsupported type"),
        }
    }

    /// Returns an uppercase string representation of the file type.
    ///
    /// This is useful for display purposes, logging, or error messages
    /// where uppercase is typically used for file type names.
    ///
    /// # Returns
    ///
    /// An uppercase static string representation of the file type:
    /// - `"XML"` for `InputFileType::XML`
    /// - `"JSON"` for `InputFileType::JSON`
    ///
    /// # Examples
    ///
    /// ```
    /// use vex2pdf::files_proc::model::input_file_type::InputFileType;
    ///
    /// let file_type = InputFileType::JSON;
    /// println!("Processing {} file", file_type.as_str_uppercase());
    /// // Outputs: "Processing JSON file"
    ///
    /// // Can be used in error messages
    /// let error_msg = format!("Failed to parse {} document", file_type.as_str_uppercase());
    /// assert_eq!(error_msg, "Failed to parse JSON document");
    ///
    /// // this will panic at compile time
    /// // let _ = InputFileType::UNSUPPORTED.as_str_uppercase();
    /// ```
    pub const fn as_str_uppercase(&self) -> &'static str {
        match self {
            InputFileType::XML => "XML",
            InputFileType::JSON => "JSON",
            InputFileType::UNSUPPORTED => panic!("not supposed to call this on unsupported types"),
        }
    }

    /// parses an extension Option<OsStr> and returns the corresponding object
    pub fn with_extension(ext: Option<&OsStr>) -> Self {
        let ext = if let Some(os_str) = ext {
            os_str
        } else {
            return Self::UNSUPPORTED;
        };
        match ext.to_string_lossy().to_ascii_lowercase().as_str() {
            "xml" => Self::XML,
            "json" => Self::JSON,
            _ => Self::UNSUPPORTED,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_conversions() {
        let x_type = InputFileType::XML;
        let j_type = InputFileType::JSON;

        assert_eq!(x_type.as_str_lowercase(), "xml");
        assert_eq!(j_type.as_str_lowercase(), "json");
        assert_eq!(x_type.as_str_uppercase(), "XML");
        assert_eq!(j_type.as_str_uppercase(), "JSON");
    }

    #[test]
    #[should_panic]
    const fn test_unsupported_conv_lowercase() {
        let _ = InputFileType::UNSUPPORTED.as_str_lowercase();
    }

    #[test]
    #[should_panic]
    const fn test_unsupported_conv_uppercase() {
        let _ = InputFileType::UNSUPPORTED.as_str_uppercase();
    }

    #[test]
    fn test_extension_handling() {
        let fake_xml = Path::new("/fictional/path/file.xml");
        let fake_json = Path::new("/fictional/path/to/location/file.json");
        let fake_other = Path::new("/random/path/file.db");

        assert!(InputFileType::with_extension(fake_xml.extension()) == InputFileType::XML);
        assert!(InputFileType::with_extension(fake_json.extension()) == InputFileType::JSON);
        assert!(
            InputFileType::with_extension(fake_other.extension()) == InputFileType::UNSUPPORTED
        );
        assert!(InputFileType::with_extension(None) == InputFileType::UNSUPPORTED);
    }
}
