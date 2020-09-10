use regex::Regex;

use crate::core::ParsedItem;

enum ParsedToken {
    Highlight,
    Underline,
}

pub fn parse_goodreader_annotations(annotations: &str, page_offset: u32) -> Vec<ParsedItem> {
    let file_rx = Regex::new(r"^File: (.+)").unwrap();
    let page_rx = Regex::new(r"^--- Page (\d+) ---").unwrap();
    let highlight_rx = Regex::new(r"^Highlight( \([^)]+\))?:").unwrap();
    let underline_rx = Regex::new(r"^Underline( \([^)]+\))?:").unwrap();

    let lines = annotations.lines().filter(|&x| x.len() > 0);

    let mut token: Option<ParsedToken> = None;
    lines
        .filter_map(|line| {
            // Handling tokens we encountered on previous line
            let item = match token {
                Some(ParsedToken::Highlight) => {
                    token = None;
                    Some(ParsedItem::Highlight(line.to_string()))
                }
                Some(ParsedToken::Underline) => {
                    token = None;
                    Some(ParsedItem::Underline(line.to_string()))
                }
                None => None,
            };
            if item.is_some() {
                return item;
            }

            // File: File_Name.pdf
            if let Some(_) = file_rx.find(line) {
                let c = file_rx.captures(line).unwrap();
                let s: String = c.get(1).map(|x| x.as_str().to_string()).unwrap();
                return Some(ParsedItem::File(s));
            }

            // --- Page NNN ---
            if let Some(_) = page_rx.find(line) {
                let c = page_rx.captures(line).unwrap();
                let n: u32 = c
                    .get(1)
                    .map(|x| {
                        let s = x.as_str().to_string();
                        s.parse::<u32>().unwrap()
                    })
                    .unwrap();
                return Some(ParsedItem::Page(n + page_offset));
            }

            // Highlight:
            // Highlighted Text
            if let Some(_) = highlight_rx.find(line) {
                token = Some(ParsedToken::Highlight);
                return None;
            }
            // Underline:
            // Underlined Text
            if let Some(_) = underline_rx.find(line) {
                token = Some(ParsedToken::Underline);
                return None;
            }
            None
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_file_indicator() {
        let annotations = r#"
File: Hello_World.pdf
"#;
        let items = parse_goodreader_annotations(annotations, 0);
        assert_eq!(ParsedItem::File("Hello_World.pdf".to_string()), items[0]);
    }

    #[test]
    fn parsing_page_indicator() {
        let annotations = r#"
--- Page 45 ---
--- Page 22 ---
-- Page 45 ---
"#;
        let items = parse_goodreader_annotations(annotations, 0);
        assert_eq!(items, vec![ParsedItem::Page(45), ParsedItem::Page(22)])
    }

    #[test]
    fn parsing_highlight_indicator() {
        let annotations = r#"
Highlight:
Practical: A Simple Database
"#;
        let items = parse_goodreader_annotations(annotations, 0);
        assert_eq!(
            items,
            vec![ParsedItem::Highlight(
                "Practical: A Simple Database".to_string()
            )]
        )
    }

    #[test]
    fn parsing_highlight_color_indicator() {
        let annotations = r#"
Highlight (blue):
Practical: A Simple Database
"#;
        let items = parse_goodreader_annotations(annotations, 0);
        assert_eq!(
            items,
            vec![ParsedItem::Highlight(
                "Practical: A Simple Database".to_string()
            )],
        )
    }

    #[test]
    fn parsing_underline_indicator() {
        let annotations = r#"
Underline:
`(equal (getf cd ,field) ,value)
"#;
        let items = parse_goodreader_annotations(annotations, 0);
        assert_eq!(
            items,
            vec![ParsedItem::Underline(
                "`(equal (getf cd ,field) ,value)".to_string()
            )],
        )
    }

    #[test]
    fn parsing_underline_color() {
        let annotations = r#"
Underline: (color #6F77FF):
`(equal (getf cd ,field) ,value)
"#;
        let items = parse_goodreader_annotations(annotations, 0);
        assert_eq!(
            items,
            vec![ParsedItem::Underline(
                "`(equal (getf cd ,field) ,value)".to_string()
            )],
        );
    }

    #[test]
    fn parsing_section() {
        let annotations = r#"
File: Practical_Common_Lisp.pdf

Annotation summary:

--- Page 45 ---

Highlight:
Practical: A Simple Database


--- Page 46 ---

Underline:
property list, or plist

Underline:
(list :a 1 :b 2 :c 3)

Underline:
GETF, which takes a plist and a symbol and returns the value in the plist

Underline:
(getf (list :a 1 :b 2 :c 3) :a)


--- Page 47 ---

Underline:
global variable, *db*, which you can define with the DEFVAR macro
"#;
        let items = parse_goodreader_annotations(annotations, 0);
        assert_eq!(
            items,
            vec![
                ParsedItem::File("Practical_Common_Lisp.pdf".to_string()),
                ParsedItem::Page(45),
                ParsedItem::Highlight("Practical: A Simple Database".to_string()),
                ParsedItem::Page(46),
                ParsedItem::Underline("property list, or plist".to_string()),
                ParsedItem::Underline("(list :a 1 :b 2 :c 3)".to_string()),
                ParsedItem::Underline(
                    "GETF, which takes a plist and a symbol and returns the value in the plist"
                        .to_string()
                ),
                ParsedItem::Underline("(getf (list :a 1 :b 2 :c 3) :a)".to_string()),
                ParsedItem::Page(47),
                ParsedItem::Underline(
                    "global variable, *db*, which you can define with the DEFVAR macro".to_string()
                )
            ]
        );
    }
}
