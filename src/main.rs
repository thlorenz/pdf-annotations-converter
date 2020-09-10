use chrono::Utc;
use clap::{App, Arg};
use std::io::{self, Read};

use pdf_annotations_converter::{
    core::ParseConfig, parse_goodreader::parse_goodreader_annotations, render_md::render_md,
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
        .usage("cat annotations.txt > pdf-annotations-converter [OPTIONS] > notes.md")
        .arg(
            Arg::with_name("page offset")
                .short("p")
                .long("page-offset")
                .value_name("Page Offset")
                .default_value("0")
                .help("Offset of first numbered page (see --numbers flag)"),
        )
        .arg(
            Arg::with_name("page numbers")
                .short("n")
                .long("--numbers")
                .help("Flag specifying if page numbers should be included"),
        )
        .get_matches();

    let page_offset = matches.value_of("page offset").unwrap();
    let page_offset = page_offset
        .parse::<u32>()
        .expect(&format!("Invalid page offset {}", page_offset));

    let page_numbers = matches.is_present("page numbers");
    let parse_config = ParseConfig {
        page_offset,
        page_numbers,
        ..Default::default()
    };

    // Note: may support reading from file in the future
    let buffer = buffer_from_stdin()?;
    let parsed_items = parse_goodreader_annotations(&buffer, &parse_config);
    let md = render_md(parsed_items, date_str);

    println!("{}", md);
    Ok(())
}
