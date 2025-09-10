//! PDF generation functionality for CycloneDX (VEX) reports.
//!
//! This module handles the conversion from CycloneDX (VEX) data structures to
//! formatted PDF documents using the genpdf library.
//!
//! The generator supports various VEX elements, including vulnerabilities,
//! components, and document metadata.
//!

use crate::pdf::font_config::FontsDir;
use cyclonedx_bom::models::tool::Tools;
use cyclonedx_bom::prelude::Bom;
use genpdf::elements::Paragraph;
use genpdf::style::{Color, Style, StyledString};
use genpdf::{Alignment, Document, Element};
use std::collections::HashMap;
use std::io;
use std::path::Path;

type ComponentTuple<'b> = (&'b str, &'b str);
pub struct PdfGenerator<'a> {
    title_style: Style,
    header_style: Style,
    normal_style: Style,
    indent_style: Style,
    comp_name_style: Style,
    version_style: Style,
    cve_id_style: Style,
    /// This is the title of the report; which is the first heading
    /// in the first page if no value is given a default title is used.
    report_title: Option<&'a str>,
    /// Sets the PDF document's metadata title that appears in PDF reader applications.
    /// This is distinct from the filename on disk (which typically matches the original file with a .pdf extension).
    /// If not specified, a default title will be used.
    pdf_meta_name: Option<&'a str>,
    /// Controls whether a `No Vulnerabilities` message is shown if there are no vulnerabilities
    show_novulns_msg: bool,
    /// Controls whether to treat the BoM as a CycloneDX or a CycloneDX-VEX
    /// when set to `true` only a list of components is shown ignoring
    /// the vulnerabilities section completely
    pure_bom_novulns: bool,
    /// Shows the component list even with vulnerabilities. The difference to [`pure_bom_novulns`] is that show_components
    /// dumps the component list after the Vulnerabilities whereas [`pure_bom_novulns`] controls
    /// showing either the vulnerabilities OR the components but not both
    show_components: bool,
}

impl Default for PdfGenerator<'_> {
    /// Creates a new PDF generator with default report and PDF titles.
    ///
    /// This implementation of the `Default` trait provides a convenient way to create a
    /// `PdfGenerator` with standard titles suitable for vulnerability reports.
    ///
    /// # Returns
    ///
    /// A new `PdfGenerator` instance with:
    /// - Report title: "Vulnerability Report Document"
    /// - PDF title: "VEX Vulnerability Report"
    /// - Default styling for all text elements
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vex2pdf::pdf::generator::PdfGenerator;
    /// use std::default::Default;
    ///
    /// // Create a generator with default settings
    /// let generator = PdfGenerator::default();
    ///
    /// // Alternatively
    /// let generator: PdfGenerator = Default::default();
    /// ```
    fn default() -> Self {
        Self::new(
            Some(Self::get_default_report_title()),
            Some(Self::get_default_pdf_meta_name()),
            true,
            false,
            true,
        )
    }
}

impl<'a, 'b> PdfGenerator<'a> {
    /// Creates a new PDF generator with custom report and PDF titles.
    ///
    /// # Arguments
    ///
    /// * `report_title` - The title displayed as the main heading on the first page of the report
    /// * `pdf_title` - The title displayed in the PDF reader window/tab when the document is opened
    /// * `show_novulns_msg` - Whether the `No Vulnerabilities reported` message is shown when no vulnerabilities are available
    /// * `show_components` - Whether the `components section` is shown
    ///
    /// # Returns
    ///
    /// A new `PdfGenerator` instance with default styles and the specified titles
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vex2pdf::pdf::generator::PdfGenerator;
    ///
    /// // Create a generator with custom titles showing a No Vulnerabilities message if
    /// // the vulnerabilities array is empty and hiding the components section
    /// let generator = PdfGenerator::new(Some("Security Analysis Results"), Some("Product Security Report"),true,false,true);
    /// ```
    pub fn new(
        report_title: Option<&'a str>,
        pdf_meta_name: Option<&'a str>,
        show_novulns_msg: bool,
        pure_bom_novulns: bool,
        show_components: bool,
    ) -> Self {
        // Initialize with default styles
        let title_style = Style::new()
            .with_font_size(18)
            .with_color(Color::Rgb(0, 0, 80));

        let header_style = Style::new()
            .with_font_size(14)
            .with_color(Color::Rgb(0, 0, 80));

        let normal_style = Style::new().with_font_size(11);

        let indent_style = Style::new()
            .with_font_size(10)
            .with_color(Color::Rgb(40, 40, 40));

        let comp_name_style = Style::new()
            .with_font_size(10)
            .with_color(Color::Rgb(0, 51, 102))
            .italic();

        let version_style = Style::new()
            .with_font_size(10)
            .with_color(Color::Rgb(128, 128, 128));

        let cve_id_style = Style::new()
            .with_font_size(11)
            .with_color(Color::Rgb(139, 0, 0))
            .bold();

        Self {
            title_style,
            header_style,
            normal_style,
            indent_style,
            comp_name_style,
            version_style,
            cve_id_style,
            report_title,
            pdf_meta_name,
            show_novulns_msg,
            show_components,
            pure_bom_novulns,
        }
    }

    /// Gets the default title for the pdf metadata
    fn get_default_pdf_meta_name() -> &'static str {
        "Vulnerability Report"
    }

    /// Gets the default title for pure BoM reports with no vulnerabilities' section
    fn get_default_pdf_meta_name_bom() -> &'static str {
        "Bill of Materials"
    }

    /// Gets the default title of the report which shows on the first page
    fn get_default_report_title() -> &'static str {
        "Vulnerability Report Document"
    }

    /// Gets the default title of the report which shows on the first page for pure BoM reports
    /// with no vulnerabilies' section
    fn get_default_report_title_bom() -> &'static str {
        "Bill of Materials Document"
    }

    /// Generates a PDF report from a CycloneDX VEX document.
    ///
    /// # Arguments
    ///
    /// * `vex` - The CycloneDX VEX document to convert
    /// * `output_path` - Path where the PDF report will be saved
    ///
    /// # Returns
    ///
    /// Result indicating success or an error with details
    pub fn generate_pdf<P: AsRef<Path>>(
        &self,
        vex: &'b Bom,
        output_path: P,
    ) -> Result<(), io::Error> {
        // Extract component list if available this will later be used to extract affected components

        let mut comp_ref_map = HashMap::<&'b str, ComponentTuple>::new();

        if let Some(components) = &vex.components {
            // preallocate our primary container
            comp_ref_map = HashMap::with_capacity(components.0.len());

            for component in &components.0 {
                // extract component name and version
                let component_version = if let Some(version) = &component.version {
                    version
                } else {
                    "undefined"
                };
                let component_detail: ComponentTuple = (&component.name, component_version);

                if let Some(ref_id) = &component.bom_ref {
                    comp_ref_map.insert(ref_id, component_detail);
                }
            }
        }
        let mut doc = Document::new(FontsDir::build().font_family);
        // Set up the document with default fonts

        let document_title = self.get_report_title();
        let pdf_title = self.get_doc_meta_name();

        doc.set_title(pdf_title);
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        let header_title = document_title.to_string();
        decorator.set_header(move |page| {
            let mut layout = genpdf::elements::LinearLayout::vertical();
            if page > 1 {
                layout.push(Paragraph::new(&header_title).aligned(Alignment::Left));

                layout.push(Paragraph::new(format!("Page {page}")).aligned(Alignment::Center));
                layout.push(genpdf::elements::Break::new(2));
            }
            layout.styled(
                Style::new()
                    .with_font_size(10)
                    .with_color(Color::Rgb(0, 0, 80)),
            )
        });

        doc.set_page_decorator(decorator);

        // Add title and basic information
        doc.push(Paragraph::default().styled_string(document_title, self.title_style));
        doc.push(genpdf::elements::Break::new(1.0));

        // Add metadata if available
        if let Some(metadata) = &vex.metadata {
            doc.push(Paragraph::default().styled_string("Document Information", self.header_style));
            doc.push(genpdf::elements::Break::new(1));

            // Add timestamp if available
            if let Some(timestamp) = &metadata.timestamp {
                doc.push(
                    Paragraph::default()
                        .styled_string(format!("Date: {timestamp}"), self.normal_style.italic()),
                );
            }

            doc.push(genpdf::elements::Break::new(1));

            // Add tools information if available
            if let Some(tools) = &metadata.tools {
                doc.push(Paragraph::default().styled_string("Tools:", self.normal_style.bold()));

                let mut ul_tools = genpdf::elements::UnorderedList::new();

                match tools {
                    Tools::List(tools_list) => {
                        for tool in tools_list {
                            if let Some(tool_name) = &tool.name {
                                let mut meta_tool_para = Paragraph::default()
                                    .styled_string(tool_name.to_string(), self.indent_style);

                                if let Some(tool_ver) = &tool.version {
                                    let tool_ver_str = StyledString::new(
                                        format!(" ({tool_ver})"),
                                        self.version_style,
                                    );
                                    meta_tool_para.push(tool_ver_str);
                                }
                                ul_tools.push(meta_tool_para);
                            }
                        }
                    }
                    Tools::Object {
                        services: services_obj,
                        components: components_obj,
                    } => {
                        // Handle components used as tools
                        if let Some(components) = &components_obj {
                            for component in &components.0 {
                                let component_name = &component.name;
                                let display_name = if let Some(version) = &component.version {
                                    format!("{component_name} (v{version})")
                                } else {
                                    component_name.clone().to_string()
                                };

                                ul_tools.push(
                                    Paragraph::default()
                                        .styled_string(&display_name, self.indent_style),
                                );
                            }
                        }

                        // Handle services used as tools
                        if let Some(services) = &services_obj {
                            for service in &services.0 {
                                let service_name = &service.name;
                                let display_name = if let Some(version) = &service.version {
                                    format!("{service_name} (v{version})")
                                } else {
                                    service_name.clone().to_string()
                                };

                                ul_tools.push(
                                    Paragraph::default()
                                        .styled_string(&display_name, self.indent_style),
                                );
                            }
                        }
                    }
                }

                doc.push(ul_tools);
                doc.push(genpdf::elements::Break::new(1));
            }

            if let Some(component) = &metadata.component {
                let mut comp_para = Paragraph::default()
                    .styled_string("Component name : ", self.normal_style.bold())
                    .styled_string(component.name.to_string(), self.indent_style);

                if let Some(comp_vers) = &component.version {
                    let ver_str = StyledString::new(format!(" ({comp_vers})"), self.version_style);
                    comp_para.push(ver_str);
                }

                doc.push(comp_para);
            }

            doc.push(genpdf::elements::Break::new(1.0));
        }

        // Add basic BOM information
        doc.push(
            Paragraph::default()
                .styled_string("BOM Format: ", self.normal_style.bold())
                .styled_string("CycloneDX", self.normal_style),
        );
        doc.push(
            Paragraph::default()
                .styled_string("Specification Version: ", self.normal_style.bold())
                .styled_string(format!("{}", vex.spec_version), self.normal_style),
        );

        doc.push(
            Paragraph::default()
                .styled_string("Version: ", self.normal_style.bold())
                .styled_string(format!("{}", vex.version), self.normal_style),
        );

        if let Some(serial) = &vex.serial_number {
            doc.push(
                Paragraph::default()
                    .styled_string("Serial Number: ", self.normal_style.bold())
                    .styled_string(format!("{serial}"), self.normal_style),
            );
        }

        doc.push(genpdf::elements::Break::new(2.0));

        // Add a Vulnerabilities section or a components list or both depending on user options

        if !self.pure_bom_novulns {
            doc = self.render_vulns(doc, vex, &comp_ref_map);
        }

        if self.pure_bom_novulns || self.show_components {
            doc = self.render_components(doc, vex);
        }

        // Render the document
        doc.render_to_file(output_path)
            .expect("failed to write file");

        Ok(())
    }

    /// report title helper
    fn get_report_title(&self) -> &str {
        if let Some(title) = self.report_title {
            title
        } else if self.pure_bom_novulns {
            Self::get_default_report_title_bom()
        } else {
            Self::get_default_report_title()
        }
    }

    /// Document meta name helper
    fn get_doc_meta_name(&self) -> &str {
        if let Some(meta_name) = self.pdf_meta_name {
            meta_name
        } else if self.pure_bom_novulns {
            Self::get_default_pdf_meta_name_bom()
        } else {
            Self::get_default_pdf_meta_name()
        }
    }
    /// Internal helper function specific for the Vulnerabilities section
    fn render_vulns(
        &self,
        mut doc: Document,
        vex: &Bom,
        comp_ref_map: &HashMap<&'b str, ComponentTuple>,
    ) -> Document {
        // First determine if vulnerabilities exist
        let mut vulns_available = false;
        if let Some(vulnerabilities) = &vex.vulnerabilities {
            vulns_available = !vulnerabilities.0.is_empty();
        }

        // Decide if we should show the vulnerabilities section at all
        let show_vulns_section = vulns_available || self.show_novulns_msg;

        if show_vulns_section {
            doc.push(Paragraph::default().styled_string("Vulnerabilities", self.header_style));
            doc.push(genpdf::elements::Break::new(1.0));
        }

        if let Some(vulnerabilities) = &vex.vulnerabilities {
            let mut ordered_list = genpdf::elements::OrderedList::new();

            // Add each vulnerability
            for vuln in &vulnerabilities.0 {
                let mut vuln_layout = genpdf::elements::LinearLayout::vertical();

                let id_paragraph = if let Some(vuln_id) = &vuln.id {
                    Paragraph::default()
                        .styled_string("ID: ", self.normal_style)
                        .styled_string(format!("{vuln_id}"), self.cve_id_style)
                } else {
                    Paragraph::default().styled_string("ID: N/A", self.normal_style)
                };

                vuln_layout.push(id_paragraph);

                let desc_paragraph = if let Some(desc) = &vuln.description {
                    Paragraph::default()
                        .styled_string("Description: ", self.indent_style.bold())
                        .styled_string(desc, self.indent_style)
                } else {
                    Paragraph::default()
                        .styled_string("Description: ", self.indent_style.bold())
                        .styled_string("N/A", self.indent_style)
                };

                vuln_layout.push(desc_paragraph);
                vuln_layout.push(genpdf::elements::Break::new(0.5));

                let mut ratings_list = genpdf::elements::UnorderedList::new();

                if let Some(ratings) = &vuln.vulnerability_ratings {
                    for rating in &ratings.0 {
                        let rating_method = if let Some(method) = &rating.score_method {
                            method.to_string()
                        } else {
                            "N/A".to_string()
                        };

                        let source_str: Option<String> =
                            rating.vulnerability_source.as_ref().and_then(|source| {
                                source
                                    .name
                                    .as_ref()
                                    .map(|source_name| source_name.to_string())
                            });

                        if let Some(severity) = &rating.severity {
                            // add Severity ratings and sources

                            let mut severity_par = Paragraph::default()
                                .styled_string("Severity: ", self.indent_style.bold())
                                .styled_string(
                                    format!("{severity} ({rating_method}"),
                                    self.indent_style,
                                );

                            if let Some(source_name) = source_str {
                                severity_par = severity_par
                                    .styled_string(" — Source: ", self.indent_style)
                                    .styled_string(source_name, self.indent_style);
                            }

                            severity_par = severity_par.styled_string(")", self.indent_style);
                            ratings_list.push(severity_par);
                        }
                    }
                }

                // add our ratings list to the vuln layout
                vuln_layout.push(ratings_list);

                // add affected components to vulnerability
                if !comp_ref_map.is_empty() {
                    // get list of affected references
                    if let Some(affected_comps) = &vuln.vulnerability_targets {
                        let mut affected_comps_detailed: Vec<&ComponentTuple> =
                            Vec::with_capacity(affected_comps.0.len());

                        for comp in &affected_comps.0 {
                            if let Some(map_val) = comp_ref_map.get(comp.bom_ref.as_str()) {
                                affected_comps_detailed.push(map_val);
                            }
                        }

                        // Create our affected components paragraph
                        let mut affected_comp_para = Paragraph::default().styled_string(
                            "Affected Document Components : [ ",
                            self.indent_style.bold(),
                        );

                        for (i, affected_comp) in affected_comps_detailed.iter().enumerate() {
                            if i > 0 {
                                affected_comp_para.push(", ");
                            }

                            affected_comp_para.push_styled(affected_comp.0, self.comp_name_style);
                            affected_comp_para
                                .push_styled(format!(": {}", affected_comp.1), self.version_style);
                        }
                        affected_comp_para.push_styled(" ]", self.indent_style.bold());

                        // add our affected components to the vulnerability layout
                        vuln_layout.push(genpdf::elements::Break::new(0.5));
                        vuln_layout.push(affected_comp_para);
                    }
                }
                vuln_layout.push(genpdf::elements::Break::new(1));
                ordered_list.push(vuln_layout);
            }

            // list_layout.push(ordered_list);
            doc.push(ordered_list);
            doc.push(genpdf::elements::Break::new(0.5));
        }

        //Add message if vulns are not available
        if !vulns_available && self.show_novulns_msg {
            let vulns_style = Style::new()
                .bold()
                .with_font_size(16)
                .with_color(Color::Rgb(0, 100, 0));

            doc.push(
                Paragraph::new("No Vulnerabilities reported")
                    .aligned(Alignment::Center)
                    .padded(genpdf::Margins::vh(10, 0))
                    .framed()
                    .styled(vulns_style),
            );
            doc.push(genpdf::elements::Break::new(1.0));
        }

        doc
    }

    fn render_components(&self, mut doc: Document, vex: &Bom) -> Document {
        if let Some(components) = &vex.components {
            doc.push(Paragraph::default().styled_string("Components", self.header_style));
            doc.push(genpdf::elements::Break::new(0.5));

            for component in &components.0 {
                doc.push(
                    Paragraph::default()
                        .styled_string("Name: ", self.indent_style)
                        .styled_string(format!("{}", component.name), self.comp_name_style),
                );

                if let Some(version) = &component.version {
                    doc.push(
                        Paragraph::default()
                            .styled_string("Version: ", self.indent_style)
                            .styled_string(format!("{version}"), self.version_style),
                    );
                }

                doc.push(genpdf::elements::Break::new(0.5));
            }
        }

        doc
    }
}
