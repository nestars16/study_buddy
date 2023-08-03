use markdown::{to_html_with_options, Options};

pub fn parse_markdown(md_file: &str) -> String {
    to_html_with_options(md_file, &Options::gfm()).expect("GFM is a safe variant")
}
