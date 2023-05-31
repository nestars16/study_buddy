mod parsing;
use parsing::Entry;
use std::io::Result;

pub async fn get_parsed_markdown_in_folder(path: &str) -> Result<Vec<Entry>> {
    let file_contents = parsing::get_markdown_files(path).await?;

    Ok(parsing::parse_markdown_files(file_contents))
}

pub fn parse_single_file(file_contents: String) -> String{
    
    parsing::parse_markdown_file(file_contents)

}

#[cfg(test)]
mod tests {}
