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
        .usage("cat annotations.txt | pdf-annotations-converter [OPTIONS] > notes.md")
        .arg(
            Arg::with_name("add offset")
                .short("a")
                .long("add-offset")
                .value_name("add offset")
                .help("Added to annotated page numbers to obtain physical page"),
        )
        .arg(
            Arg::with_name("sub offset")
                .short("s")
                .long("sub-offset")
                .value_name("sub offset")
                .help("Subtracted from annotated physical page to obtain page number"),
        )
        .arg(
            Arg::with_name("page numbers")
                .short("n")
                .long("--numbers")
                .help("Flag specifying if page numbers should be included"),
        )
        .after_help(
            r#"
Physical pages don't usually match page numbers. Therefore you can optionally provide an offset to
derive one from the other.

First determine the page offset which is the physical page on which page number `1` appears.

Then provide this offset via `--add-offset` if the annotations include the actual page number or via
`--sub-offset` if the annotations include the physical page.
        "#,
        )
        .get_matches();

    // We upport two ways to supply a page offset, mainly because we cannot pass a negative value,
    // i.e. -10 as that is interpreted as a flag instead
    if matches.is_present("add offset") && matches.is_present("sub offset") {
        panic!(
            r#"Only 
    --add-offset (when physical page numbers are annotated) or
    --sub-offset (when actual page numbers are annotated) can be provided
    "#
        );
    }

    let page_offset: i32 = if matches.is_present("add offset") {
        let add_offset = matches.value_of("add offset").unwrap();
        add_offset
            .parse::<u16>()
            .expect(&format!("Invalid add offset {}", add_offset)) as i32
    } else if matches.is_present("sub offset") {
        let sub_offset = matches.value_of("sub offset").unwrap();
        -(sub_offset
            .parse::<u16>()
            .expect(&format!("Invalid sub offset {}", sub_offset)) as i32)
    } else {
        0
    };

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
