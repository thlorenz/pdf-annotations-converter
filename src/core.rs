#[derive(Debug, Clone, PartialEq)]
pub enum ParsedItem {
    File(String),
    Page(u32),
    Highlight(String),
    Underline(String),
}
