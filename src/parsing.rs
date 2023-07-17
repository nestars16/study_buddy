use markdown::{Options, to_html_with_options};

pub fn parse_markdown_file(md_file: &str) -> String {
    to_html_with_options(&md_file, &Options::gfm() ).expect("GFM is a safe variant")
}
