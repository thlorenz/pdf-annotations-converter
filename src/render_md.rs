use crate::core::ParsedItem;

pub fn render_md(items: Vec<ParsedItem>, date_time: String) -> String {
    let mut file = "";
    let mut previous_item: ParsedItem = ParsedItem::Page(0);

    let content = items.iter().fold("".to_string(), |acc, x| {
        let result = match x {
            ParsedItem::File(f) => {
                file = f;
                format!("{}# {}\n", acc, f)
            }
            ParsedItem::Page(page_num) => {
                let leading_new_lines = match &previous_item {
                    ParsedItem::File(_) | ParsedItem::Highlight(_) => "\n".to_string(),
                    _ => "\n\n".to_string(),
                };
                // github ignores the float style which (align right isa ll we can do)
                // however if viewed in other viewers it has the desired effect
                format!(
                    "{}{}<p align='right' style=\"float: right;\"><i>Page {}</i></p>\n",
                    acc, leading_new_lines, page_num
                )
            }
            ParsedItem::Highlight(highlighted) => {
                let leading_new_lines = match &previous_item {
                    ParsedItem::Page(_) => "\n",
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
            ParsedItem::Page(45),
            ParsedItem::Highlight("Practical: A Simple Database".to_string()),
            ParsedItem::Page(46),
            ParsedItem::Underline("property list, or plist".to_string()),
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

<p align='right' style="float: right;"><i>Page 46</i></p>

- property list, or plist"#;
        assert_eq!(s, md)
    }
}
