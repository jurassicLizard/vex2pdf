use crate::files_proc::model::file_ident::BomFileIdentifier;
use crate::files_proc::model::files_pending_proc::FilesPendingProc;
use crate::lib_utils::errors::Vex2PdfError;
use cyclonedx_bom::prelude::Bom;
use std::error::Error;
use std::hash::Hash;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::lib_utils::config::Config;

pub trait FileSearchProvider {
    type OkType;
    type ErrType;
    fn find_files(self) -> Result<Self::OkType, Self::ErrType>;
}

pub trait SingleFileProcProvider<P: AsRef<Path> + Eq + Hash>: Send + 'static {
    fn process_single_file(&self, file: BomFileIdentifier<P>, config: Arc<Config>) -> Result<(), Vex2PdfError>;
}

/// no need to restrict this to send as typically threads are created inside this function
/// TODO complete documentation
pub trait MultipleFilesProcProvider<P: AsRef<Path> + Eq + Hash> {

    type OkType;
    type ErrType;
    fn process(self) -> Result<Self::OkType,Self::ErrType>;
}