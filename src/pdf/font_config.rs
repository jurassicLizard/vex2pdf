use genpdf::fonts::{FontData, FontFamily};
use log::info;
use std::error::Error;

/// `FontsDir` manages font directory paths across different locations in the system.
///  Used to manage different external font path directories before.
///  As of v0.6.1 the logic is simplified to only load font files embedded in the library.
pub struct FontsDir {
    pub font_family: FontFamily<FontData>,
}

impl FontsDir {
    /// Loads a complete font family using embedded LiberationSans font data.
    ///
    /// This method creates a `FontFamily` containing all four standard LiberationSans font variants
    /// (regular, bold, italic, and bold-italic) that are embedded directly in the binary.
    /// This approach eliminates the need for external font files on the host system.
    ///
    /// This method is private since v0.7.0 and is automatically called by the `FontsDir::build()` builder
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `Ok(FontFamily<FontData>)` - A complete LiberationSans font family with all four variants
    /// - `Err(Box<dyn Error>)` - If any font variant fails to load or parse
    ///
    /// # Usage in PdfGenerator
    ///
    /// This method is intended to replace the traditional filesystem font loading
    /// It builds a `FontFamily<FontData>` object to be used by the PdfGenerator
    /// Internally this function uses the private method `load_embedded_font_family`
    ///
    /// ```rust
    /// use genpdf::Document;
    /// use genpdf::fonts::{FontData, FontFamily};
    /// use vex2pdf::pdf::font_config::FontsDir;
    ///
    /// let fonts_dir : FontsDir = FontsDir::build();  
    /// let mut doc = Document::new(fonts_dir.font_family);
    /// ```
    ///
    /// # Returns
    ///
    /// A `FontFamily<FontData>` object to be used by the PdfGenerator
    ///
    /// # Panics
    ///
    /// This builder panics if embedded files have failed to load. This only happens if the binary
    /// was built incorrectly without embedding the files and will never occur with official builds
    ///
    /// # Advantages
    ///
    /// - Works without access to the filesystem
    /// - Consistent font rendering across all platforms
    /// - No dependency on system-installed fonts
    /// - Eliminates the need for users to install fonts separately
    ///
    /// # License Information
    ///
    /// The embedded LiberationSans fonts are licensed under the SIL Open Font License.
    /// Set the environment variable `VEX2PDF_SHOW_OSS_LICENSES=true` to display full license details.
    pub fn build() -> Self {
        FontsDir {
            font_family : FontsDir::load_embedded_font_family().expect("Embedded fonts failed to load. \
             this is a fatal failure that is caused by a corrupt build. Please contact the DEV or submit and Issue \
             under https://github.com/jurassicLizard/vex2pdf/issues")
        }
    }
    /// Prints information about the active font source to standard output.
    ///
    ///
    /// The output will include either:
    /// - A message indicating embedded fonts are in use (with optional license information)
    ///
    /// # Examples
    ///
    /// ```
    /// use vex2pdf::pdf::font_config::FontsDir;
    ///
    /// FontsDir::print_fonts_info(); // Prints the active font source to stdout
    /// ```
    ///
    /// # Output Example
    ///
    /// ```text
    /// Active font path: <embedded liberationSans fonts> -- the env variable VEX2PDF_SHOW_OSS_LICENSES=true shows Font license details
    ///
    /// ```
    pub fn print_fonts_info() {
        info!("Active font path: <embedded liberationSans fonts> -- the env variable VEX2PDF_SHOW_OSS_LICENSES=true shows Font license details");
        info!("");
    }

    /// private builder method that loads the embedded font files
    fn load_embedded_font_family() -> Result<FontFamily<FontData>, Box<dyn Error>> {
        let regular_font_data =
            include_bytes!("../../external/fonts/liberation-fonts/LiberationSans-Regular.ttf");
        let bold_font_data =
            include_bytes!("../../external/fonts/liberation-fonts/LiberationSans-Bold.ttf");
        let italic_font_data =
            include_bytes!("../../external/fonts/liberation-fonts/LiberationSans-Italic.ttf");
        let bold_italic_font_data =
            include_bytes!("../../external/fonts/liberation-fonts/LiberationSans-BoldItalic.ttf");

        // Create FontData objects from the byte arrays
        let regular = FontData::new(regular_font_data.to_vec(), None)?;
        let bold = FontData::new(bold_font_data.to_vec(), None)?;
        let italic = FontData::new(italic_font_data.to_vec(), None)?;
        let bold_italic = FontData::new(bold_italic_font_data.to_vec(), None)?;

        // Create a FontFamily with FontData
        Ok(FontFamily {
            regular,
            bold,
            italic,
            bold_italic,
        })
    }
}
