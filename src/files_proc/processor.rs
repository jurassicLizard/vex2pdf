use crate::files_proc::model::file_ident::BomFileIdentifier;
use crate::files_proc::model::files_pending_proc::FilesPendingProc;
use crate::files_proc::model::input_file_type::InputFileType;
use crate::files_proc::traits::{
    FileSearchProvider, MultipleFilesProcProvider, SingleFileProcProvider,
};
use crate::lib_utils::config::Config;
use crate::lib_utils::errors::Vex2PdfError;
use crate::pdf::generator::PdfGenerator;
use crate::utils::{get_output_pdf_path, parse_vex_json, parse_vex_xml};
#[cfg(feature = "concurrency")]
use jlizard_simple_threadpool::threadpool::ThreadPool;
use log::{error, info, warn};
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// The default processor implementation for this crate
#[derive(Default)]
pub(crate) struct DefaultFilesProcessor {
    config: Config,
}

impl DefaultFilesProcessor {
    pub(crate) fn new(config: Config) -> Self {
        Self { config }
    }
}

impl FileSearchProvider for DefaultFilesProcessor {
    type OkType = ProcessorReady<PathBuf>;
    type ErrType = Vex2PdfError;
    fn find_files(self) -> Result<Self::OkType, Self::ErrType> {
        // process map ignored pattern map
        let ignored_patterns_map = self.config.file_types_to_process.as_ref();

        // get the working path can be a file or folder
        let working_path = &self.config.working_path;

        // build ignored_patterns
        let ignored_patterns: HashSet<&InputFileType> =
            if let Some(ignore_map) = ignored_patterns_map {
                ignore_map
                    .iter()
                    .filter_map(|(k, v)| {
                        if !(*v) {
                            info!(
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
            info!(
                "Scanning for BoM/Vex Files in {}",
                &self.config.working_path.display()
            );

            for entry in fs::read_dir(&self.config.working_path)? {
                let path = entry?.path();

                if path.is_file() {
                    if let Err(e) = ret.add_sup_file_ignore(path.to_path_buf(), &ignored_patterns) {
                        match e {
                            Vex2PdfError::IgnoredByUser => info!("{e}"),
                            _ => error!("{e}"),
                        };
                    }
                }
            }
        }

        // inform over search results
        if ret.get_files_ref().is_empty() {
            info!("No parseable files in selected path");
        } else {
            info!(
                "Found {} JSON files",
                ret.get_file_count_by_type(InputFileType::JSON)
            );
            info!(
                "Found {} XML files",
                ret.get_file_count_by_type(InputFileType::XML)
            );
        }

        Ok(ProcessorReady {
            config: Arc::new(self.config),
            files: ret,
        })
    }
}

#[derive(Default)]
pub(crate) struct DefaultSingleFileProcessor;

impl<P: AsRef<Path> + Eq + Hash + Send + 'static> SingleFileProcProvider<P>
    for DefaultSingleFileProcessor
{
    fn process_single_file(
        &self,
        file: BomFileIdentifier<P>,
        config: Arc<Config>,
    ) -> Result<(), Vex2PdfError> {
        info!("Processing {}", file.get_path().as_ref().display());

        // Get BoM Object
        let bom =
            match file.get_type() {
                InputFileType::XML => parse_vex_xml(file.get_path())
                    .map_err(|e| Vex2PdfError::Parse(e.to_string()))?,
                InputFileType::JSON => parse_vex_json(file.get_path())
                    .map_err(|e| Vex2PdfError::Parse(e.to_string()))?,
                InputFileType::UNSUPPORTED => return Err(Vex2PdfError::UnsupportedFileType),
            };

        // Generate output PDF path with same base name
        let generator = PdfGenerator::new(Arc::clone(&config));

        info!("Generating PDF:  {}", file.get_path().as_ref().display());

        // FIXME consider if output path is ever handled here
        // Generate the PDF
        let output_path =
            get_output_pdf_path(Some(config.output_dir.as_path()), file.get_path().as_ref())?;
        match generator.generate_pdf(&bom, &output_path) {
            Ok(_) => info!("Successfully generated PDF: {}", output_path.display()),
            Err(e) => {
                warn!(
                    "Failed to generate PDF for {}: {}",
                    output_path.display(),
                    e
                )
            }
        }

        Ok(())
    }
}
pub(crate) struct ProcessorReady<P: AsRef<Path> + Eq + Hash> {
    config: Arc<Config>,
    pub(super) files: FilesPendingProc<P>,
}

impl<P: AsRef<Path> + Eq + Hash + Send + 'static> MultipleFilesProcProvider<P>
    for ProcessorReady<P>
{
    type OkType = ();
    type ErrType = Vex2PdfError;

    fn process(self) -> Result<Self::OkType, Self::ErrType> {
        #[cfg(feature = "concurrency")]
        let pool = if let Some(num_jobs) = self.config.max_jobs {
            ThreadPool::new(num_jobs)
        } else {
            ThreadPool::default()
        };

        #[cfg(feature = "concurrency")]
        info!("{pool}");

        let config = self.config;
        let file_count = self.files.get_file_count();

        for file in self.files {
            let single_file_proc = DefaultSingleFileProcessor;
            let config_clone = Arc::clone(&config);
            #[cfg(feature = "concurrency")]
            {
                pool.execute(move || {
                    if let Err(e) = single_file_proc.process_single_file(file, config_clone) {
                        error!("{e}");
                    }
                })
                    .expect(
                        "Failed to send job to pool. Consider disabling Multithreading if issues persist",
                    ); // we do not want to immediatly return an error if one of the jobs failed hence why we did not propagate
            }

            #[cfg(not(feature = "concurrency"))]
            {
                single_file_proc
                    .process_single_file(file, config_clone)
                    .unwrap_or_else(|e| error!("{e}"));
            }
        }

        #[cfg(feature = "concurrency")]
        drop(pool); // dropping here to show information message after worker status messages
                    // pool drops gracefully and cleans up here blocking until all jobs are finished

        info!("Processed {file_count} files");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_default_files_processor_new() {
        let config = Config::default();
        let processor = DefaultFilesProcessor::new(config);

        // Verify processor created with config (working_path should be set)
        assert!(processor.config.working_path.exists());
    }

    #[test]
    fn test_processor_ready_holds_state() {
        let config = Arc::new(Config::default());
        let files: FilesPendingProc<PathBuf> = FilesPendingProc::new();

        let processor_ready = ProcessorReady {
            config: Arc::clone(&config),
            files,
        };

        // Verify state is accessible
        assert_eq!(processor_ready.files.get_file_count(), 0);
        assert!(processor_ready.config.working_path.exists());
    }

    #[test]
    fn test_single_file_processor_creation() {
        let _processor = DefaultSingleFileProcessor::default();

        // Verify it can be created and is Send
        fn assert_send<T: Send>() {}
        assert_send::<DefaultSingleFileProcessor>();
    }
}
