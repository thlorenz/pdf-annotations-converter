/// Item that the parser emits.
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedItem {
    /// The file which was annotated.
    File(String),
    /// A page number indicator.
    Page(u32),
    /// A higlighted section.
    Highlight(String),
    /// An underlined section.
    Underline(String),
}

/// The config provided to the parser.
pub struct ParseConfig {
    /// The offset of the first numbered page.
    pub page_offset: u32,
    /// If `true` page number indicators will be emitted.
    pub page_numbers: bool,
}

impl Default for ParseConfig {
    fn default() -> Self {
        Self {
            page_offset: 0,
            page_numbers: false,
        }
    }
}
