use crate::files_proc::model::file_ident::BomFileIdentifier;
use crate::files_proc::model::input_file_type::InputFileType;
use crate::lib_utils::errors::Vex2PdfError;
use std::collections::HashSet;
use std::hash::Hash;
use std::path::Path;

//FIXME add documentation
#[derive(Default)]
pub struct FilesPendingProc<P: AsRef<Path> + Eq + Hash> {
    files: HashSet<BomFileIdentifier<P>>,
}

impl<P: AsRef<Path> + Eq + Hash> FilesPendingProc<P> {
    #[inline]
    pub fn new() -> Self {
        Self {
            files: HashSet::new(),
        }
    }

    #[inline]
    pub fn with_capacity(size: usize) -> Self {
        Self {
            files: HashSet::with_capacity(size),
        }
    }

    #[inline]
    pub fn get_files_ref(&self) -> &HashSet<BomFileIdentifier<P>> {
        &self.files
    }

    /// adds a file only if it is of a supported format, skips unsupported
    /// # returns
    /// - Ok(())
    /// - [Vex2PdfError::UnsupportedFileType] if the file is unsupported
    ///
    /// # Related
    /// check [Self::add_sup_file_ignore] if you want the same functionality but with an ignore pattern
    pub fn add_supported_file(&mut self, path: P) -> Result<(), Vex2PdfError> {
        let file_ident = BomFileIdentifier::build(path)?;

        if file_ident.is_supported_type() {
            self.files.insert(file_ident);

            Ok(())
        } else {
            Err(Vex2PdfError::UnsupportedFileType)
        }
    }

    /// similar to [Self::add_supported_file] but ignored file patterns provided over the Hashset
    /// # returns
    /// - Ok(()) if all is well
    /// - [Vex2PdfError::IgnoredByUser] if the file extension is set to be ignored by the user
    pub fn add_sup_file_ignore(
        &mut self,
        path: P,
        ignored_types: &HashSet<&InputFileType>,
    ) -> Result<(), Vex2PdfError> {
        let file_ident = BomFileIdentifier::build(path)?;

        if file_ident.is_supported_type() {
            if ignored_types.contains(file_ident.get_type()) {
                Err(Vex2PdfError::IgnoredByUser)
            } else {
                self.files.insert(file_ident);

                Ok(())
            }
        } else {
            Err(Vex2PdfError::UnsupportedFileType)
        }
    }

    /// get count of files of the given type
    pub fn get_file_count_by_type(&self, file_type: InputFileType) -> usize {
        self.get_files_ref()
            .iter()
            .filter(|&k| *k.get_type() == file_type)
            .count()
    }

    pub fn get_file_count(&self) -> usize {
        self.get_files_ref().iter().count()
    }
}

impl<P: AsRef<Path> + Eq + Hash> IntoIterator for FilesPendingProc<P> {
    type Item = BomFileIdentifier<P>;
    type IntoIter = std::collections::hash_set::IntoIter<BomFileIdentifier<P>>;

    fn into_iter(self) -> Self::IntoIter {
        self.files.into_iter()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::files_proc::processor::DefaultFilesProcessor;
    use crate::files_proc::traits::FileSearchProvider;
    use std::fs;
    use std::io::Read;
    use std::path::PathBuf;

    macro_rules! create_multiple_files {
        ($x:expr) => {{
            let mut processor = FilesPendingProc::<PathBuf>::new();
            let mut files = Vec::with_capacity($x);

            for _ in 0..$x {
                files.push(TempFile::with_timestamp_prefix("test.json"));
            }

            for file in &files {
                let _ = processor.add_supported_file(file.path.clone());
            }

            (files, processor)
        }};
    }

    struct TempFile {
        path: PathBuf,
        file: fs::File,
    }

    impl TempFile {
        pub(super) fn with_timestamp_prefix(filename: &str) -> Self {
            let mut temp_file_path = std::env::temp_dir();
            let cur_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();

            let filename = format!("{cur_time}_{filename}");

            temp_file_path.push(filename);

            let file = fs::File::create(temp_file_path.clone()).unwrap();

            Self {
                path: PathBuf::from(temp_file_path),
                file,
            }
        }
    }

    impl Drop for TempFile {
        fn drop(&mut self) {
            fs::remove_file(self.path.as_path()).unwrap();
        }
    }

    #[test]
    fn test_new() {
        let processor: FilesPendingProc<PathBuf> = FilesPendingProc::new();
        assert!(processor.get_files_ref().is_empty());
    }

    #[test]
    fn test_with_capacity() {
        let processor: FilesPendingProc<PathBuf> = FilesPendingProc::with_capacity(10);
        assert!(processor.get_files_ref().is_empty());
        // Note: capacity testing is not directly observable in HashSet
    }

    #[test]
    fn test_add_supported_file_success() {
        let mut processor: FilesPendingProc<&Path> = FilesPendingProc::new();
        let temp_file = TempFile::with_timestamp_prefix("test.json");

        let result = processor.add_supported_file(temp_file.path.as_path());

        assert!(result.is_ok());
        assert_eq!(processor.get_files_ref().len(), 1);
    }

    #[test]
    fn test_add_supported_file_unsupported() {
        let mut processor: FilesPendingProc<&Path> = FilesPendingProc::new();
        let temp_unsupported = TempFile::with_timestamp_prefix("test.unsupported");

        let result = processor.add_supported_file(temp_unsupported.path.as_path());

        assert!(result.is_err());
        assert!(matches!(result, Err(Vex2PdfError::UnsupportedFileType)));
        assert!(processor.get_files_ref().is_empty());
    }

    #[test]
    fn test_add_supported_file_duplicate() {
        let mut processor: FilesPendingProc<&Path> = FilesPendingProc::new();

        let temp_file = TempFile::with_timestamp_prefix("test.json");
        let path1 = temp_file.path.as_path();
        let path2 = path1;

        let result1 = processor.add_supported_file(path1);
        let result2 = processor.add_supported_file(path2);

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        // HashSet should prevent duplicates
        assert_eq!(processor.get_files_ref().len(), 1);
    }

    #[test]
    fn test_add_sup_file_ignore_not_ignored() {
        let mut processor: FilesPendingProc<&Path> = FilesPendingProc::new();
        let temp_file = TempFile::with_timestamp_prefix("test.json");
        let ignored_types = HashSet::new(); // Empty ignore list

        let result = processor.add_sup_file_ignore(temp_file.path.as_path(), &ignored_types);

        assert!(result.is_ok());
        assert_eq!(processor.get_files_ref().len(), 1);
    }

    #[test]
    fn test_add_sup_file_ignore_is_ignored() {
        let mut processor: FilesPendingProc<&Path> = FilesPendingProc::new();
        let temp_file = TempFile::with_timestamp_prefix("test.json");
        let mut ignored_types = HashSet::new();
        ignored_types.insert(&InputFileType::JSON);

        let result = processor.add_sup_file_ignore(temp_file.path.as_path(), &ignored_types);

        assert!(result.is_err());
        assert!(matches!(result, Err(Vex2PdfError::IgnoredByUser)));
        assert!(processor.get_files_ref().is_empty());
    }

    #[test]
    fn test_add_sup_file_ignore_unsupported() {
        let mut processor: FilesPendingProc<&Path> = FilesPendingProc::new();

        let ignored_types = HashSet::new();
        let temp_file = TempFile::with_timestamp_prefix("temp_file");

        let result = processor.add_sup_file_ignore(temp_file.path.as_path(), &ignored_types);

        assert!(result.is_err());
        assert!(matches!(result, Err(Vex2PdfError::UnsupportedFileType)));
        assert!(processor.get_files_ref().is_empty());
    }

    #[test]
    fn test_multiple_supported_files() {
        let mut processor: FilesPendingProc<&Path> = FilesPendingProc::new();
        let temp_json = TempFile::with_timestamp_prefix("test.json");
        let temp_xml = TempFile::with_timestamp_prefix("test.xml");

        let result1 = processor.add_supported_file(temp_json.path.as_path());
        let result2 = processor.add_supported_file(temp_xml.path.as_path());

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert_eq!(processor.get_files_ref().len(), 2);
    }

    #[test]
    fn test_get_files_immutable_reference() {
        let mut processor: FilesPendingProc<&Path> = FilesPendingProc::new();
        let temp_json = TempFile::with_timestamp_prefix("test.json");
        let _ = processor.add_supported_file(temp_json.path.as_path());

        let files_ref = processor.get_files_ref();
        assert_eq!(files_ref.len(), 1);
        // Verify it's immutable reference - this shouldn't compile if uncommented:
        // files_ref.insert(BomFileIdentifier::build(PathBuf::from("test2.json")).unwrap());
    }

    #[test]
    fn test_file_count() {
        let (_files, processor) = create_multiple_files!(6);
        assert_eq!(processor.get_file_count(), 6);
    }
    #[test]
    fn test_file_count_by_type() {
        let mut processor = FilesPendingProc::<&Path>::new();
        let temp_file = TempFile::with_timestamp_prefix("test.json");
        let second_temp_file = TempFile::with_timestamp_prefix("test.json");
        let another_temp_file = TempFile::with_timestamp_prefix("test.xml");
        let unsupported_file = TempFile::with_timestamp_prefix("test.cfg");

        let _ = processor.add_supported_file(temp_file.path.as_path());
        let _ = processor.add_supported_file(second_temp_file.path.as_path());
        let _ = processor.add_supported_file(another_temp_file.path.as_path());
        let _ = processor.add_supported_file(unsupported_file.path.as_path());

        assert_eq!(processor.get_file_count_by_type(InputFileType::JSON), 2);
        assert_eq!(processor.get_file_count_by_type(InputFileType::XML), 1);
        assert_eq!(
            processor.get_file_count_by_type(InputFileType::UNSUPPORTED),
            0
        );
    }
    #[test]
    fn test_consuming_iter() {
        let count = 6;
        let (_, processor) = create_multiple_files!(count);

        // Test consuming iteration on FilesPendingProc
        let mut iter_count = 0usize;
        for _file in processor {
            iter_count += 1;
        }

        assert_eq!(count, iter_count);
    }
}
