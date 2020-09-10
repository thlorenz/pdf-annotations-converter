#[derive(Debug, Clone, PartialEq)]
pub enum ParsedItem {
    File(String),
    Page(u32),
    Highlight(String),
    Underline(String),
}

#[derive(Clone, Debug)]
pub struct ParseConfig {
    pub page_offset: u32,
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
