mod parsing;
use std::io::Result;
use parsing::Entry;


pub async fn get_parsed_markdown_in_folder(path: &str) -> Result<Vec<Entry>> {

    let file_contents = parsing::get_markdown_files(path).await?;

    Ok(parsing::parse_markdown_files(file_contents))
}

#[cfg(test)]
mod tests{


}
