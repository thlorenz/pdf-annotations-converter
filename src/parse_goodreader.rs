use regex::Regex;

use crate::core::{ParseConfig, ParsedItem};

enum ParsedToken {
    Highlight,
    Underline,
}

/// Parses the provided GoodReader annotations emitting a [`ParsedItem`] for each identified
/// annotation and page number indicators if so desired.
/// Use the [`ParseConfig`] to configure if page numbers should be included, etc.
pub fn parse_goodreader_annotations(annotations: &str, config: &ParseConfig) -> Vec<ParsedItem> {
    let file_rx = Regex::new(r"^File: (.+)").unwrap();
    let page_roman_rx = Regex::new(r"^--- Page ([mdclxvi]+) ---").unwrap();
    let page_number_rx = Regex::new(r"^--- Page (\d+) ---").unwrap();
    let highlight_rx = Regex::new(r"^Highlight( \([^)]+\))?:").unwrap();
    let underline_rx = Regex::new(r"(?i)^(Squiggly )?Underline( \([^)]+\))?:").unwrap();

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
            if config.page_numbers {
                if let Some(_) = page_number_rx.find(line) {
                    let c = page_number_rx.captures(line).unwrap();
                    let n: u32 = c
                        .get(1)
                        .map(|x| {
                            let s = x.as_str().to_string();
                            s.parse::<u32>().unwrap()
                        })
                        .unwrap();
                    return match config.page_offset {
                        // physical page is annotated, thus page offset is negative if set
                        offset if offset < 0 => { 
                            assert!(n as i32 > offset, format!("negative page offset {} seems to small, encountered physical page {}", offset, n));
                            Some(ParsedItem::PageNumber((n as i32 + offset) as u32, n))
                        }
                        // page number is annotated, page offset is positive if set
                        offset if offset > 0 => Some(ParsedItem::PageNumber(n, n + offset as u32)),
                        // page offset not set, i.e. is 0
                        _ =>Some(ParsedItem::PageNumber(n, n)), 
                    };
                }
                if let Some(_) = page_roman_rx.find(line) {
                    let c = page_roman_rx.captures(line).unwrap();
                    let s: String = c
                        .get(1)
                        .map(|x| {
                            x.as_str().to_string()
                        })
                        .unwrap();
                    return Some(ParsedItem::PageRoman(s))
                }
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

    static WITH_PAGE_NUMBERS: ParseConfig = ParseConfig {
        page_offset: 0,
        page_numbers: true,
    };
    static WITHOUT_PAGE_NUMBERS: ParseConfig = ParseConfig {
        page_offset: 0,
        page_numbers: false,
    };

    #[test]
    fn parsing_file_indicator() {
        let annotations = r#"
File: Hello_World.pdf
"#;
        let items = parse_goodreader_annotations(annotations, &WITHOUT_PAGE_NUMBERS);
        assert_eq!(ParsedItem::File("Hello_World.pdf".to_string()), items[0]);
    }

    #[test]
    fn parsing_page_indicator() {
        let annotations = r#"
--- Page 45 ---
--- Page 22 ---
-- Page 45 ---
"#;
        let items = parse_goodreader_annotations(annotations, &WITH_PAGE_NUMBERS);
        assert_eq!(
            items,
            vec![
                ParsedItem::PageNumber(45, 45),
                ParsedItem::PageNumber(22, 22)
            ]
        )
    }

    #[test]
    fn parsing_page_indicator_with_positive_page_offset() {
        let annotations = r#"
--- Page 45 ---
--- Page 22 ---
"#;
        let items = parse_goodreader_annotations(
            annotations,
            &ParseConfig {
                page_offset: 3,
                page_numbers: true,
            },
        );
        assert_eq!(
            items,
            vec![
                ParsedItem::PageNumber(45, 48),
                ParsedItem::PageNumber(22, 25)
            ]
        )
    }

    #[test]
    fn parsing_page_roman_indicator() {
        let annotations = r#"
--- Page xxxviii ---
--- Page xxxix ---
"#;
        let items = parse_goodreader_annotations(annotations, &WITH_PAGE_NUMBERS);
        assert_eq!(
            items,
            vec![
                ParsedItem::PageRoman("xxxviii".to_string()),
                ParsedItem::PageRoman("xxxix".to_string())
            ]
        )
    }

    #[test]
    fn parsing_page_indicator_with_negative_page_offset() {
        let annotations = r#"
--- Page 45 ---
--- Page 22 ---
"#;
        let items = parse_goodreader_annotations(
            annotations,
            &ParseConfig {
                page_offset: -3,
                page_numbers: true,
            },
        );
        assert_eq!(
            items,
            vec![
                ParsedItem::PageNumber(42, 45),
                ParsedItem::PageNumber(19, 22)
            ]
        )
    }

    #[test]
    fn parsing_highlight_indicator() {
        let annotations = r#"
Highlight:
Practical: A Simple Database
"#;
        let items = parse_goodreader_annotations(annotations, &WITHOUT_PAGE_NUMBERS);
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
        let items = parse_goodreader_annotations(annotations, &WITHOUT_PAGE_NUMBERS);
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
        let items = parse_goodreader_annotations(annotations, &WITHOUT_PAGE_NUMBERS);
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
        let items = parse_goodreader_annotations(annotations, &WITHOUT_PAGE_NUMBERS);
        assert_eq!(
            items,
            vec![ParsedItem::Underline(
                "`(equal (getf cd ,field) ,value)".to_string()
            )],
        );
    }

    static SECTION: &str = "
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
";
    #[test]
    fn parsing_section_with_zero_based_page_numbers() {
        let items = parse_goodreader_annotations(SECTION, &WITH_PAGE_NUMBERS);
        assert_eq!(
            items,
            vec![
                ParsedItem::File("Practical_Common_Lisp.pdf".to_string()),
                ParsedItem::PageNumber(45, 45),
                ParsedItem::Highlight("Practical: A Simple Database".to_string()),
                ParsedItem::PageNumber(46, 46),
                ParsedItem::Underline("property list, or plist".to_string()),
                ParsedItem::Underline("(list :a 1 :b 2 :c 3)".to_string()),
                ParsedItem::Underline(
                    "GETF, which takes a plist and a symbol and returns the value in the plist"
                        .to_string()
                ),
                ParsedItem::Underline("(getf (list :a 1 :b 2 :c 3) :a)".to_string()),
                ParsedItem::PageNumber(47, 47),
                ParsedItem::Underline(
                    "global variable, *db*, which you can define with the DEFVAR macro".to_string()
                )
            ]
        );
    }

    #[test]
    fn parsing_section_with_10_based_page_numbers() {
        let items = parse_goodreader_annotations(
            SECTION,
            &ParseConfig {
                page_offset: 10,
                page_numbers: true,
            },
        );
        assert_eq!(
            items,
            vec![
                ParsedItem::File("Practical_Common_Lisp.pdf".to_string()),
                ParsedItem::PageNumber(45, 55),
                ParsedItem::Highlight("Practical: A Simple Database".to_string()),
                ParsedItem::PageNumber(46, 56),
                ParsedItem::Underline("property list, or plist".to_string()),
                ParsedItem::Underline("(list :a 1 :b 2 :c 3)".to_string()),
                ParsedItem::Underline(
                    "GETF, which takes a plist and a symbol and returns the value in the plist"
                        .to_string()
                ),
                ParsedItem::Underline("(getf (list :a 1 :b 2 :c 3) :a)".to_string()),
                ParsedItem::PageNumber(47, 57),
                ParsedItem::Underline(
                    "global variable, *db*, which you can define with the DEFVAR macro".to_string()
                )
            ]
        );
    }

    #[test]
    fn parsing_section_without_page_numbers() {
        let items = parse_goodreader_annotations(SECTION, &WITHOUT_PAGE_NUMBERS);
        assert_eq!(
            items,
            vec![
                ParsedItem::File("Practical_Common_Lisp.pdf".to_string()),
                ParsedItem::Highlight("Practical: A Simple Database".to_string()),
                ParsedItem::Underline("property list, or plist".to_string()),
                ParsedItem::Underline("(list :a 1 :b 2 :c 3)".to_string()),
                ParsedItem::Underline(
                    "GETF, which takes a plist and a symbol and returns the value in the plist"
                        .to_string()
                ),
                ParsedItem::Underline("(getf (list :a 1 :b 2 :c 3) :a)".to_string()),
                ParsedItem::Underline(
                    "global variable, *db*, which you can define with the DEFVAR macro".to_string()
                )
            ]
        );
    }
}
