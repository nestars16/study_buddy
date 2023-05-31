use markdown::{to_html_with_options,Options,CompileOptions};
use std::path::Path;
use tokio::fs::{read_dir, read_to_string};
use tokio::io::Result;

use markdown::{to_html, ParseOptions};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Entry {
    name: String,
    file_content: String,
}

impl Entry {
    async fn new(path_to_read: &Path) -> Result<Self> {
        let name = match path_to_read.file_name() {
            Some(valid_str) => match valid_str.to_os_string().into_string() {
                Ok(string) => string,
                Err(_) => "Invalid Filename".to_owned(),
            },
            None => "Invalid Filename".to_owned(),
        };

        let file_content = read_to_string(path_to_read).await?;

        Ok(Entry { name, file_content })
    }
}

pub async fn get_markdown_files(path_to_read: &str) -> Result<Vec<Entry>> {
    let mut fs_entries = read_dir(path_to_read).await?;

    let mut entries = Vec::new();

    while let Ok(Some(entry)) = fs_entries.next_entry().await {
        entries.push(Entry::new(&entry.path()).await?);
    }

    Ok(entries)
}

pub fn parse_markdown_files(md_files: Vec<Entry>) -> Vec<Entry> {
    md_files
        .into_iter()
        .map(|entry| Entry {
            file_content: to_html(&entry.file_content),
            ..entry
        })
        .collect()
}

pub fn parse_markdown_file(mut md_file: String) -> String {

    let last_modification_index = md_file.rfind("<p>");

    if let Some(index) = last_modification_index {
       md_file.replace_range(index..=index+2,"\n") 
    }

    let md_file = md_file.trim_end_matches("<br></p>");

    to_html_with_options(&md_file, &Options {
    compile: CompileOptions {
      allow_dangerous_html: true,
      allow_dangerous_protocol: true,
      ..CompileOptions::default()
    },
    ..Options::default()
}).unwrap()

    

}
