/// Item that the parser emits.
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedItem {
    /// The file which was annotated.
    File(String),
    /// A page number indicator, first element is printed number, second is physical page.
    PageNumber(u32, u32),
    /// A roman page indicator
    PageRoman(String),
    /// A higlighted section.
    Highlight(String),
    /// An underlined section.
    Underline(String),
}

/// The config provided to the parser.
pub struct ParseConfig {
    /// The offset of the first numbered page.
    /// Set it to a negative value if physical page numbers are annotated to derive page number
    /// from it.
    /// Set it to positive if page numbers are annotated to derive physical page from it.
    /// Set it to [0] if page numbers match physical page.
    pub page_offset: i32,
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
