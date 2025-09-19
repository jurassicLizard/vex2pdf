use crate::files_proc::model::file_ident::BomFileIdentifier;
use crate::files_proc::model::files_pending_proc::FilesPendingProc;
use crate::lib_utils::errors::Vex2PdfError;
use cyclonedx_bom::prelude::Bom;
use std::error::Error;
use std::hash::Hash;
use std::io;
use std::path::{Path, PathBuf};

pub trait FileSearchProvider {
    fn find_files(&self) -> Result<FilesPendingProc<PathBuf>, Vex2PdfError>;
}

pub trait SingleFileProcProvider<P: AsRef<Path> + Eq + Hash>: Send + 'static {
    fn process_single_file(&self, file: BomFileIdentifier<P>) -> Result<Bom, Vex2PdfError>;
}
