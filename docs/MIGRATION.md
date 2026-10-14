# Migration Guide: 1.x to 2.0

Version 2.0.0 introduces breaking changes to improve the library architecture and provide more flexibility. Most users won't need to make any changes.

## For CLI Users

**No changes required!** The CLI interface remains unchanged. All default features (CLI + concurrency) are enabled by default.

## For Library Users

### If you're using default features
**No changes required!** Your code will continue to work as-is.

### If you're using `--no-default-features`
Version 2.0 introduces a new `concurrency` feature that's enabled by default. If you're using `--no-default-features`, you now have two options:

**Option 1: Add concurrency back** (recommended for most use cases)
```bash
# Update your Cargo.toml or installation command
cargo add vex2pdf --no-default-features --features concurrency
```

**Option 2: Use sequential processing** (simpler, no threading)
```bash
# No changes needed - files will now process sequentially
cargo add vex2pdf --no-default-features
```

### If you're using the `PdfGenerator` API directly
The `PdfGenerator` struct no longer has a lifetime parameter:

**Before (v1.x):**
```rust
let generator: PdfGenerator<'_> = PdfGenerator::new(config);
```

**After (v2.0):**
```rust
let generator: PdfGenerator = PdfGenerator::new(config);
// Or simply let type inference handle it:
let generator = PdfGenerator::new(config);
```

### Environment Variable Changes
If you were using these deprecated environment variables, switch to CLI flags:

| Removed Variable | Replacement |
|------------------|-------------|
| `VEX2PDF_SHOW_OSS_LICENSES=true` | `vex2pdf --license` |
| `VEX2PDF_VERSION_INFO=true` | `vex2pdf --version` |

## What Changed Internally

- **Concurrency is now optional**: The `concurrency` feature flag allows you to choose between concurrent (faster) or sequential (simpler) processing
- **External threadpool**: Replaced internal implementation with `jlizard-simple-threadpool` for better maintainability
- **Cleaner API**: Removed deprecated fields from `PdfGenerator` struct (deprecated since v0.9.0)

For a complete list of changes, see the [CHANGELOG.md](../CHANGELOG.md#200---2025-12-29).

---

Released after 2.0: see the [Changelog](../CHANGELOG.md) for 3.0.0 (vulnerability `published` date and
`recommendation` fields) and 4.0.0, which is **breaking for library users**: `run()` and
`ProcessorReady::process()` now return `Err(Vex2PdfError::ProcessingFailures(n))` instead of silently
returning `Ok(())` when input files fail, and the CLI exits non-zero in that case.
