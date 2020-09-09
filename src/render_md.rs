use crate::core::ParsedItem;
use chrono::Utc;

pub fn render_md(items: Vec<ParsedItem>) -> String {
    let mut file = "";
    let content = items.iter().fold("".to_string(), |acc, x| match x {
        ParsedItem::File(x) => {
            file = x;
            format!("{}# {}\n", acc, file)
        }
        ParsedItem::Page(page_num) => format!("{}\n_Page {:04}_\n\n", acc, page_num),
        ParsedItem::Highlight(highlighted) => format!("{}## {}\n", acc, highlighted),
        ParsedItem::Underline(underlined) => format!("{}- {}\n", acc, underlined),
    });

    let now = Utc::now();
    let date_str = now.format("%c").to_string();

    let meta = format!(
        r#"---
file:    {} 
date:    {} 
created: [pdf-annotations-converter](https://github.com/thlorenz/pdf-annotations-converter)
---"#,
        file, date_str
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
        let s = render_md(items);
        print!("{}", s);
        let md = r#"---
file:    Practical_Common_Lisp.pdf
date:    Wed Sep  9 04:55:45 2020
created: [pdf-annotations-converter](https://github.com/thlorenz/pdf-annotations-converter)
---

# Practical_Common_Lisp.pdf

_Page 0045_

## Practical: A Simple Database

_Page 0046_

- property list, or plist
list
"#;
        assert_eq!(s, md)
    }
}
