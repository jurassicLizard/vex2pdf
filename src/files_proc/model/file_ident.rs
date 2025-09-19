use crate::files_proc::model::input_file_type::InputFileType;
use crate::lib_utils::errors::Vex2PdfError;
use std::hash::Hash;
use std::io;
use std::path::Path;

/// A filetype that encapsulates a (path,type) tuple
#[derive(Eq, Hash, PartialEq)]
pub struct BomFileIdentifier<P: AsRef<Path>>(P, InputFileType)
where
    P: Eq,
    P: std::hash::Hash;

impl<P: AsRef<Path> + Eq + Hash> BomFileIdentifier<P> {
    /// constructs a new identifier and checks for file existence
    ///  # Caveat
    ///  Any passed folder or unsupported extension will be parsed but set to UNSUPPORTED
    pub fn build(path: P) -> Result<Self, Vex2PdfError> {
        if !path.as_ref().exists() {
            return Err(Vex2PdfError::Io(io::Error::new(
                io::ErrorKind::NotFound,
                "File does not exist",
            )));
        }

        let file_type = InputFileType::with_extension(path.as_ref().extension());

        Ok(Self(path, file_type))
    }

    /// get a reference to the path
    #[inline]
    pub fn get_path(&self) -> &P {
        &self.0
    }

    #[inline]
    pub fn get_type(&self) -> &InputFileType {
        &self.1
    }

    /// whether this file extension is supported
    #[inline]
    pub fn is_supported_type(&self) -> bool {
        self.1 != InputFileType::UNSUPPORTED
    }
}

/// some mocking methods
#[cfg(test)]
impl<P: AsRef<Path> + Eq + Hash> BomFileIdentifier<P> {
    fn mock_new(path: P) -> Self {
        let file_type = InputFileType::with_extension(path.as_ref().extension());

        Self(path, file_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_ident() {
        let file_ident_json = BomFileIdentifier::mock_new("/test/path/to/file.json");
        let file_ident_xml = BomFileIdentifier::mock_new("/path/to/file.xml");
        let file_ident_folder = BomFileIdentifier::mock_new("/path/to/folder");
        let file_ident_unsupported = BomFileIdentifier::mock_new("/unsupported/file/type.txt");

        assert!(*file_ident_json.get_type() == InputFileType::JSON);
        assert!(*file_ident_xml.get_type() == InputFileType::XML);
        assert!(*file_ident_folder.get_type() == InputFileType::UNSUPPORTED);
        assert!(*file_ident_unsupported.get_type() == InputFileType::UNSUPPORTED);
    }

    #[test]
    fn test_nonexisting_files() {
        let does_not_exist = BomFileIdentifier::build("/does/not/exist");
        match does_not_exist {
            Err(e) => match e {
                Vex2PdfError::Io(_) => (),
                _ => panic!("Must return an Io error"),
            },
            _ => panic!("must return an error"),
        }
    }

    #[test]
    fn test_supported_file() {
        let supported_file_json = BomFileIdentifier::mock_new("/path/to/file.json");
        let supported_file_xml = BomFileIdentifier::mock_new("/path/to/file.xml");
        let unsupported_file = BomFileIdentifier::mock_new("/path/to/unsupported.txt");
        let unsupported_folder = BomFileIdentifier::mock_new("/path/to/folder/");

        assert!(supported_file_json.is_supported_type());
        assert!(supported_file_xml.is_supported_type());
        assert!(!unsupported_file.is_supported_type());
        assert!(!unsupported_folder.is_supported_type());
    }
}
