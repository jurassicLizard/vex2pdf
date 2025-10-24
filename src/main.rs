//! # vex2pdf
//!
//! A command-line tool that converts CycloneDX VEX (JSON and XML) documents to PDF reports.
//!
//! ## CycloneDX Compatibility
//!
//! This tool fully supports CycloneDX schema version 1.5 and provides compatibility
//! for version 1.6 documents that only use 1.5 fields. Documents using 1.6-specific
//! fields may not process correctly.
//!
//! ## Usage
//!
//! Run the tool in a directory containing VEX JSON files:
//!
//! ```
//! vex2pdf
//! ```
//!
//! The tool will scan for JSON or XML files (or both depending on the configuration), process any valid VEX documents,
//! and generate corresponding PDF reports with the same filename but with a .pdf extension.
//!
//! ## Font Handling
//!
//! This tool has Liberation Sans fonts embedded in the binary to render PDFs correctly.
//! No extra configuration is required
//! See the README for more details.

use log::error;
use std::io::Write;
use std::process;
use vex2pdf::lib_utils::config::Config;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| {
            // Write errors and warnings to stderr, info and debug to stdout
            match record.level() {
                log::Level::Error | log::Level::Warn => {
                    writeln!(
                        &mut std::io::stderr(),
                        "[{} {}] {}",
                        buf.timestamp(),
                        record.level(),
                        record.args()
                    )
                }
                _ => {
                    writeln!(
                        buf,
                        "[{} {}] {}",
                        buf.timestamp(),
                        record.level(),
                        record.args()
                    )
                }
            }
        })
        .target(env_logger::Target::Stdout)
        .init();

    let config = Config::build().unwrap_or_else(|err| {
        error!("Problem setting up working environment:");
        error!("{}", { err });
        process::exit(1);
    });

    if let Err(e) = vex2pdf::run(config) {
        error!("Application error: {e}");
        process::exit(1);
    }
}
