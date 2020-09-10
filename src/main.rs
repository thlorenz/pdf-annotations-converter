use chrono::Utc;
use clap::{App, Arg};
use std::io::{self, Read};

use pdf_annotations_converter::{
    parse_goodreader::parse_goodreader_annotations, render_md::render_md,
};

fn buffer_from_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn get_date_str() -> String {
    let now = Utc::now();
    now.format("%c").to_string()
}

fn main() -> io::Result<()> {
    let date_str = get_date_str();

    let matches = App::new("pdf-annotations-converter")
        .version("0.1.0")
        .about("Converts annotations found in PDF files to different formats")
        .usage("cat annotations.txt > pdf-annotations-converter [OPTIONS]")
        .arg(
            Arg::with_name("page offset")
                .short("p")
                .long("page-offset")
                .value_name("Page Offset")
                .default_value("0"),
        )
        .get_matches();

    let page_offset = matches.value_of("page offset").unwrap();
    let page_offset = page_offset
        .parse::<u32>()
        .expect(&format!("Invalid page offset {}", page_offset));

    // Note: may support reading from file in the future
    let buffer = buffer_from_stdin()?;
    let parsed_items = parse_goodreader_annotations(&buffer, page_offset);
    let md = render_md(parsed_items, date_str);

    println!("{}", md);
    Ok(())
}
