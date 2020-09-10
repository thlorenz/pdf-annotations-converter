# pdf-annotations-converter [![](https://github.com/thlorenz/pdf-annotations-converter/workflows/Rust/badge.svg?branch=master)](https://github.com/thlorenz/pdf-annotations-converter/actions)

Converts annotations found in PDF files to different formats.

```sh
USAGE:
    cat annotations.txt > pdf-annotations-converter [OPTIONS] > notes.md

FLAGS:
    -h, --help       Prints help information
    -n, --numbers    Flag specifying if page numbers should be included
    -V, --version    Prints version information

OPTIONS:
    -p, --page-offset <Page Offset>    Offset of first numbered page (see --numbers flag) [default: 0]
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

## License

MIT
