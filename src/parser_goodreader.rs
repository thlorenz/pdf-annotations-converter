use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    File(String),
    Page(u32),
    Highlight(String),
    Underline(String),
}

enum Token {
    Highlight,
    Underline,
}

pub fn parse_goodreader_annotations(annotations: &str) -> Vec<Item> {
    let file_rx = Regex::new(r"^File: (.+)").unwrap();
    let page_rx = Regex::new(r"^--- Page (\d+) ---").unwrap();
    let highlight_rx = Regex::new(r"^Highlight( \([^)]+\))?:").unwrap();
    let underline_rx = Regex::new(r"^Underline( \([^)]+\))?:").unwrap();

    let lines = annotations.lines().filter(|&x| x.len() > 0);

    let mut token: Option<Token> = None;
    let items: Vec<Item> = lines
        .filter_map(|line| {
            // Handling tokens we encountered on previous line
            let item = match token {
                Some(Token::Highlight) => {
                    token = None;
                    Some(Item::Highlight(line.to_string()))
                }
                Some(Token::Underline) => {
                    token = None;
                    Some(Item::Underline(line.to_string()))
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
                return Some(Item::File(s));
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
                return Some(Item::Page(n));
            }

            // Highlight:
            // Highlighted Text
            if let Some(_) = highlight_rx.find(line) {
                token = Some(Token::Highlight);
                return None;
            }
            // Underline:
            // Underlined Text
            if let Some(_) = underline_rx.find(line) {
                token = Some(Token::Underline);
                return None;
            }
            None
        })
        .collect();
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_file_indicator() {
        let annotations = r#"
File: Hello_World.pdf
"#;
        let items = parse_goodreader_annotations(annotations);
        assert_eq!(Item::File("Hello_World.pdf".to_string()), items[0]);
    }

    #[test]
    fn parsing_page_indicator() {
        let annotations = r#"
--- Page 45 ---
--- Page 22 ---
-- Page 45 ---
"#;
        let items = parse_goodreader_annotations(annotations);
        assert_eq!(items, vec![Item::Page(45), Item::Page(22)])
    }

    #[test]
    fn parsing_highlight_indicator() {
        let annotations = r#"
Highlight:
Practical: A Simple Database
"#;
        let items = parse_goodreader_annotations(annotations);
        assert_eq!(
            items,
            vec![Item::Highlight("Practical: A Simple Database".to_string())]
        )
    }

    #[test]
    fn parsing_highlight_color_indicator() {
        let annotations = r#"
Highlight (blue):
Practical: A Simple Database
"#;
        let items = parse_goodreader_annotations(annotations);
        assert_eq!(
            items,
            vec![Item::Highlight("Practical: A Simple Database".to_string())],
        )
    }

    #[test]
    fn parsing_underline_indicator() {
        let annotations = r#"
Underline:
`(equal (getf cd ,field) ,value)
"#;
        let items = parse_goodreader_annotations(annotations);
        assert_eq!(
            items,
            vec![Item::Underline(
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
        let items = parse_goodreader_annotations(annotations);
        assert_eq!(
            items,
            vec![Item::Underline(
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
        let items = parse_goodreader_annotations(annotations);
        assert_eq!(
            items,
            vec![
                Item::File("Practical_Common_Lisp.pdf".to_string()),
                Item::Page(45),
                Item::Highlight("Practical: A Simple Database".to_string()),
                Item::Page(46),
                Item::Underline("property list, or plist".to_string()),
                Item::Underline("(list :a 1 :b 2 :c 3)".to_string()),
                Item::Underline(
                    "GETF, which takes a plist and a symbol and returns the value in the plist"
                        .to_string()
                ),
                Item::Underline("(getf (list :a 1 :b 2 :c 3) :a)".to_string()),
                Item::Page(47),
                Item::Underline(
                    "global variable, *db*, which you can define with the DEFVAR macro".to_string()
                )
            ]
        );
    }
}
