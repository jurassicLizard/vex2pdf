# Developer Notes

Personal notes on testing, architecture, and future development for vex2pdf.

## Testing

### Test Structure

The project has comprehensive test coverage across multiple levels:

- **Integration Tests**: Located in `tests/integration_tests.rs`
  - Test full PDF generation pipeline from JSON/XML inputs
  - Compare generated PDFs against expected outputs (timestamps stripped)
  - Cover various scenarios: VEX, VDR, analysis states, XML/JSON formats, edge cases

- **Threading Integration Tests**: Located in `tests/threading_integration_tests.rs`
  - Test concurrency modes via CLI (user perspective)
  - Verify `--max-jobs` argument and `VEX2PDF_MAX_JOBS` env var
  - Test single-threaded mode (`--max-jobs 1`), multi-threaded (2, 4 jobs), and default parallelism
  - Check log output messages about concurrency mode

- **Unit Tests**: Embedded across multiple modules:
  - `src/pdf/generator.rs` - utility functions, analysis formatting
  - `src/lib_utils/run_utils.rs`
  - `src/lib_utils/cli_args.rs`
  - `src/lib_utils/env_vars.rs`
  - `src/lib_utils/concurrency/threadpool.rs` - threadpool creation, execution, shutdown
  - `src/lib_utils/concurrency/worker.rs` - worker creation, job execution
  - `src/files_proc/processor.rs` - processor creation and state management
  - `src/files_proc/model/file_ident.rs`
  - `src/files_proc/model/input_file_type.rs`
  - `src/files_proc/model/files_pending_proc.rs`
  - `src/lib.rs`

### Test Artifacts

Test files are organized under `tests/test_artifacts/`:

```
tests/test_artifacts/
├── bom_src/
│   ├── run_test/           # JSON test inputs
│   ├── run_test_xml/        # XML test inputs
│   └── run_test_novulns/    # BOM-only (no vulns) inputs
├── expected_pdfs/           # Reference PDFs for comparison
└── err_pdfs/               # Failed test outputs for debugging
```

When integration tests fail, the generated PDFs are copied to `err_pdfs/` along with stripped content dumps (`*_generated_raw_dump.txt` and `*_expected_raw_dump.txt`) for easy diffing.

### Checksum-Based Testing

The integration tests validate PDF generation accuracy using BLAKE3 checksums of normalized content. This approach:

- **Normalizes PDFs** by stripping dynamic elements (timestamps, document IDs, UUIDs)
- **Calculates checksums** on the normalized content for consistent validation
- **Stores checksums** in `tests/test_artifacts/expected_pdfs_chksums.txt` (included in crates.io package)
- **Excludes reference PDFs** (~42MB) from the published package to keep download size reasonable

This is configured in `Cargo.toml`:

```toml
[package]
# ...
exclude = [
    "tests/test_artifacts/expected_pdfs/",  # ~42MB of reference PDFs
    ".gitlab-ci.yml",
]
```

**Benefits of checksum-based testing:**

1. **Works everywhere**: Tests run identically from git repository or crates.io package
2. **Small package size**: Checksums file is <1KB vs 42MB of reference PDFs
3. **Consistent validation**: Ignores dynamic content (timestamps, IDs) that changes on every build
4. **Debug and release**: No behavioral differences between build modes

To regenerate checksums after updating reference PDFs:

```bash
cargo run --example generate_checksums
```

### Code Coverage

Using `cargo-llvm-cov` for coverage analysis:

```bash
# Install llvm-tools-preview component
rustup component add llvm-tools-preview

# Install coverage tool
cargo install cargo-llvm-cov

# Generate coverage report (terminal output)
cargo llvm-cov

# Generate HTML report
cargo llvm-cov --html
# Opens in browser: target/llvm-cov/html/index.html

# Generate JSON report for CI
cargo llvm-cov --json --output-path coverage.json
```

**Note**: Debug logs are stripped at compile time in release builds (via `release_max_level_info` feature), so coverage may vary between debug/release.

## Traits System

### Current Implementation

The project uses a trait-based architecture for file processing, defined in `src/files_proc/traits.rs`:

#### `FileSearchProvider`

Responsible for discovering BOM files in the filesystem.

```rust
pub trait FileSearchProvider {
    type OkType;
    type ErrType;
    fn find_files(self) -> Result<Self::OkType, Self::ErrType>;
}
```

**Implementation**: `DefaultFilesProcessor` (in `src/files_proc/processor.rs`)
- Scans working directory for JSON/XML files
- Filters based on user configuration (ignore patterns)
- Returns `ProcessorReady` with list of files to process

#### `SingleFileProcProvider`

Processes individual BOM files.

```rust
pub trait SingleFileProcProvider<P: AsRef<Path> + Eq + Hash>: Send + 'static {
    fn process_single_file(
        &self,
        file: BomFileIdentifier<P>,
        config: Arc<Config>,
    ) -> Result<(), Vex2PdfError>;
}
```

**Implementation**: `DefaultSingleFileProcessor`
- Parses JSON/XML to CycloneDX BOM
- Generates PDF using `PdfGenerator`
- Thread-safe (Send + 'static) for parallel processing

#### `MultipleFilesProcProvider`

Orchestrates batch processing of multiple files.

```rust
pub trait MultipleFilesProcProvider<P: AsRef<Path> + Eq + Hash> {
    type OkType;
    type ErrType;
    fn process(self) -> Result<Self::OkType, Self::ErrType>;
}
```

**Implementation**: `ProcessorReady`
- Spawns threadpool workers
- Distributes files across threads
- Handles errors gracefully (logs but continues processing)

### Future: PDF Renderer Traits (v1.0.0)

**Status**: Work in progress, planned for v1.0.0 release at the latest

Currently, PDF generation is tightly coupled to `genpdf` library in `src/pdf/generator.rs`. The goal is to abstract PDF rendering behind traits to enable:

- **Multiple backends**: Support for genpdf, typst, or custom renderers
- **Easier testing**: Mock PDF generation without file I/O
- **Migration flexibility**: Smooth transition from genpdf to typst (or other libraries)

**Planned trait structure** (subject to change):

```rust
// Future API (not yet implemented)
pub trait PdfRenderer {
    fn render_metadata(&mut self, bom: &Bom) -> Result<(), RenderError>;
    fn render_vulnerabilities(&mut self, vulns: &[Vulnerability]) -> Result<(), RenderError>;
    fn render_components(&mut self, components: &[Component]) -> Result<(), RenderError>;
    fn finalize(&mut self, output_path: &Path) -> Result<(), RenderError>;
}
```

This is mentioned in `TODO.md`:
> "version 0.10.0 (introduce trait system for pdf generation as a precursor to moving to typst from genpdf"

## Project Architecture

### Module Structure

```
src/
├── main.rs                  # Entry point, CLI setup, logging initialization
├── lib.rs                   # Library interface, exports public API
├── files_proc/              # File processing pipeline
│   ├── traits.rs            # Core traits (FileSearchProvider, etc.)
│   ├── processor.rs         # Default implementations
│   └── model/               # Data structures (FileIdentifier, FileType, etc.)
├── pdf/                     # PDF generation
│   ├── generator.rs         # PdfGenerator (tightly coupled to genpdf)
│   └── font_config.rs       # Embedded Liberation Sans fonts
└── lib_utils/               # Utilities and configuration
    ├── config.rs            # Configuration struct
    ├── cli_args.rs          # CLI argument parsing (clap)
    ├── env_vars.rs          # Environment variable handling
    ├── errors.rs            # Error types
    ├── run_utils.rs         # Runtime utilities (version 1.6 compat, etc.)
    └── concurrency/         # Custom threadpool
        ├── threadpool.rs
        ├── worker.rs
        └── common.rs
```

### Concurrency Model

VEX2PDF uses a custom threadpool implementation (`lib_utils/concurrency/threadpool.rs`) instead of libraries like rayon:

- **Why custom?**: Fine-grained control over worker lifecycle, logging, error handling
- **Thread count**: Defaults to `num_cpus::get()` (can be configured via Config)
- **Worker model**: MPSC channel with job queue
- **Graceful shutdown**: Pool waits for all jobs to complete on drop

### Processing Pipeline

```
CLI/Env Parsing (main.rs)
   ↓
Config Creation (lib_utils/config.rs)
   ↓
File Search (DefaultFilesProcessor::find_files)
   ↓
ProcessorReady (holds files + config)
   ↓
Batch Processing (ThreadPool spawns workers)
   ↓
Per-File Processing (DefaultSingleFileProcessor)
   ↓
Parse JSON/XML → CycloneDX BOM (utils::parse_vex_*)
   ↓
Generate PDF (PdfGenerator::generate_pdf)
   ↓
Write to output_dir
```

## Development Workflow

### CI Pipeline

GitLab CI configuration (`.gitlab-ci.yml`) has three stages:

1. **Test** (on MR and branches):
   - Format check, clippy linting, test suite

2. **Build** (only on tags `v*`):
   - Linux build: `release-optimized` profile
   - Windows cross-compile: `x86_64-pc-windows-gnu`
   - Artifacts saved for 1 hour

3. **Release** (only on tags `v*`):
   - Upload binaries to GitLab package registry
   - Create release with auto-generated description
   - Link binaries to release

### Build Profiles

```toml
[profile.release]
strip = true

[profile.release-optimized]
inherits = "release"
opt-level = 3
lto = "fat"
codegen-units = 1
```

Use `release-optimized` for production binaries (smaller size, better performance).

## Notes for Future Development

### Planned Features (see TODO.md)

- Document traits system more thoroughly
- Add tests for Processor and related types
- Introduce PDF renderer trait system (v0.10.0 or v1.0.0)
- Evaluate migration from genpdf to typst

### Known Quirks

1. **CycloneDX 1.6 compatibility**: Manual downgrade to 1.5 (see `lib_utils/run_utils.rs`)
   - Works for BOMs without 1.6-specific fields
   - Logs warnings when downgrading

2. **Embedded fonts**: Liberation Sans bundled in binary (no runtime dependency)
   - License displayed via `VEX2PDF_SHOW_OSS_LICENSES=true`

3. **Logging in tests**: Integration tests expect info logs on stdout
   - Custom formatter routes ERROR/WARN → stderr, INFO/DEBUG → stdout
   - Debug logs stripped in release builds
