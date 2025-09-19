use crate::files_proc::model::file_ident::BomFileIdentifier;
use crate::files_proc::model::files_pending_proc::FilesPendingProc;
use crate::files_proc::model::input_file_type::InputFileType;
use crate::files_proc::traits::{FileSearchProvider, SingleFileProcProvider};
use crate::lib_utils::config::Config;
use crate::lib_utils::errors::Vex2PdfError;
use crate::utils::{parse_vex_json, parse_vex_xml};
use cyclonedx_bom::prelude::Bom;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// The default processor implementation for this crate
pub(crate) struct DefaultFilesProcessor {
    config: Arc<Config>,
}

impl DefaultFilesProcessor {
    pub(crate) fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }
}

impl<P: AsRef<Path> + Eq + Hash> SingleFileProcProvider<P> for DefaultFilesProcessor {
    fn process_single_file(&self, file: BomFileIdentifier<P>) -> Result<Bom, Vex2PdfError> {
        println!("Processing {}", file.get_path().as_ref().display());

        match file.get_type() {
            InputFileType::XML => {
                parse_vex_xml(file.get_path()).map_err(|e| Vex2PdfError::Parse(e.to_string()))
            }
            InputFileType::JSON => {
                parse_vex_json(file.get_path()).map_err(|e| Vex2PdfError::Parse(e.to_string()))
            }
            InputFileType::UNSUPPORTED => Err(Vex2PdfError::UnsupportedFileType),
        }
    }
}

impl FileSearchProvider for DefaultFilesProcessor {
    fn find_files(&self) -> Result<FilesPendingProc<PathBuf>, Vex2PdfError> {
        // process map ignored pattern map
        let ignored_patterns_map = (&self.config).file_types_to_process.as_ref();

        // get the working path can be a file or folder
        let working_path = &self.config.working_path;

        // build ignored_patterns
        let ignored_patterns: HashSet<&InputFileType> =
            if let Some(ignore_map) = ignored_patterns_map {
                ignore_map
                    .iter()
                    .filter_map(|(k, v)| {
                        if *v == false {
                            println!(
                                "Skipping {} files : deactivated by user",
                                k.as_str_uppercase()
                            );
                            Some(k)
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                HashSet::new()
            };

        let mut ret = FilesPendingProc::new();

        if working_path.is_file() {
            ret.add_sup_file_ignore(working_path.to_path_buf(), &ignored_patterns)?;
        } else {
            // is a folder
            println!(
                "Scanning for BoM/Vex Files in {}",
                &self.config.working_path.display()
            );

            for entry in fs::read_dir(&self.config.working_path)? {
                let path = entry?.path();

                if path.is_file() {
                    if let Err(e) = ret.add_sup_file_ignore(path.to_path_buf(), &ignored_patterns) {
                        match e {
                            Vex2PdfError::IgnoredByUser => println!("{e}"),
                            _ => eprintln!("{e}"),
                        };
                    }
                }
            }
        }

        // inform over search results
        if ret.get_files().is_empty() {
            println!("No parseable files in selected path");
        } else {
            println!(
                "Found {} JSON files",
                ret.get_file_count_by_type(InputFileType::JSON)
            );
            println!(
                "Fond {} XML files",
                ret.get_file_count_by_type(InputFileType::XML)
            );
        }

        Ok(ret)
    }
}
