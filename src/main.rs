use chrono::Utc;
use std::io::{self, Read};

use pdf_annotations_converter::{
    parse_goodreader::parse_goodreader_annotations, render_md::render_md,
};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let parsed_items = parse_goodreader_annotations(&buffer);

    let now = Utc::now();
    let date_str = now.format("%c").to_string();

    let md = render_md(parsed_items, date_str);
    println!("{}", md);
    Ok(())
}
