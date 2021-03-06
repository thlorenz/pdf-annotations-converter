use crate::core::ParsedItem;

/// Renders the provided [`ParsedItem`]s into a Markdown document.
/// The date_time [`String`] can be in any format and will be included in the metadata in order to
/// indicate when the document was created.
pub fn render_md(items: Vec<ParsedItem>, date_time: String) -> String {
    let mut file = "";
    let mut previous_item: ParsedItem = ParsedItem::PageNumber(0, 0);

    let content = items.iter().fold("".to_string(), |acc, x| {
        let result = match x {
            ParsedItem::File(f) => {
                file = f;
                format!("{}# {}\n", acc, f)
            }
            ParsedItem::PageNumber(page_num, physical_page) => {
                let leading_new_lines = match &previous_item {
                    ParsedItem::File(_) | ParsedItem::Highlight(_) => "\n".to_string(),
                    _ => "\n\n".to_string(),
                };
                let page_str = if page_num == physical_page {
                    format!("{}", page_num)
                } else {
                    format!("{} ({})", page_num, physical_page)
                };

                // github ignores the float style which (align right isa ll we can do)
                // however if viewed in other viewers it has the desired effect
                format!(
                    "{}{}<p align='right' style=\"float: right;\"><i>Page {}</i></p>\n",
                    acc, leading_new_lines, page_str,
                )
            }
            ParsedItem::PageRoman(s) => {
                let leading_new_lines = match &previous_item {
                    ParsedItem::File(_) | ParsedItem::Highlight(_) => "\n".to_string(),
                    _ => "\n\n".to_string(),
                };

                format!(
                    "{}{}<p align='right' style=\"float: right;\"><i>Page {}</i></p>\n",
                    acc, leading_new_lines, s,
                )
            }
            ParsedItem::Highlight(highlighted) => {
                let leading_new_lines = match &previous_item {
                    ParsedItem::PageNumber(_, _) | ParsedItem::PageRoman(_) => "\n",
                    _ => "\n\n",
                };
                format!("{}{}## {}\n", acc, leading_new_lines, highlighted)
            }
            ParsedItem::Underline(underlined) => format!("{}\n- {}", acc, underlined),
        };

        previous_item = x.clone();
        result
    });

    let meta = format!(
        r#"---
file:    {}
date:    {}
created with: https://github.com/thlorenz/pdf-annotations-converter
---"#,
        file, date_time
    );

    format!("{}\n\n{}", meta, content)
}

#[cfg(test)]
mod tests {
    #[test]
    fn rendering_items_md() {
        use super::*;
        let items = vec![
            ParsedItem::File("Practical_Common_Lisp.pdf".to_string()),
            ParsedItem::PageNumber(45, 45),
            ParsedItem::Highlight("Practical: A Simple Database".to_string()),
            ParsedItem::PageNumber(46, 50),
            ParsedItem::Underline("property list, or plist".to_string()),
            ParsedItem::PageRoman("xvii".to_string()),
        ];
        let s = render_md(items, "date_time".to_string());
        let md = r#"---
file:    Practical_Common_Lisp.pdf
date:    date_time
created with: https://github.com/thlorenz/pdf-annotations-converter
---

# Practical_Common_Lisp.pdf

<p align='right' style="float: right;"><i>Page 45</i></p>

## Practical: A Simple Database

<p align='right' style="float: right;"><i>Page 46 (50)</i></p>

- property list, or plist

<p align='right' style="float: right;"><i>Page xvii</i></p>
"#;
        assert_eq!(s, md)
    }
}
