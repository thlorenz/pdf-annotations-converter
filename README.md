# pdf-annotations-converter [![](https://github.com/thlorenz/pdf-annotations-converter/workflows/Rust/badge.svg?branch=master)](https://github.com/thlorenz/pdf-annotations-converter/actions)

Converts annotations found in PDF files to different formats.

```
USAGE:
    cat annotations.txt | pdf-annotations-converter [OPTIONS] > notes.md

FLAGS:
    -h, --help       Prints help information
    -n, --numbers    Flag specifying if page numbers should be included
    -V, --version    Prints version information

OPTIONS:
    -a, --add-offset <add offset>    Added to annotated page numbers to obtain physical page
    -s, --sub-offset <sub offset>    Subtracted from annotated physical page to obtain page number


Physical pages don't usually match page numbers. Therefore you can optionally provide an offset to
derive one from the other.

First determine the page offset which is the physical page on which page number `1` appears.

Then provide this offset via `--add-offset` if the annotations include the actual page number or via
`--sub-offset` if the annotations include the physical page.
```

## Installation

```sh
cargo install pdf-annotations-converter
```

## Supported Annotations

At this point annotations generated with [GoodReader](https://goodreader.com/) are supported.
These annotations are obtained by using the [export via
email](https://goodreader.com/user-manual/export-annotations) feature.

No email account needs to be connected to do this as you can just pretend to send them via
email and copy them to the clipboard to them transfer them via a different mechanism, i.e. via
Notes.

## Supported Renderers

At this point rendering to markdown is supported.

## Documentation

- [crates.io documentation](https://crates.io/crates/pdf-annotations-converter)

## License

MIT
