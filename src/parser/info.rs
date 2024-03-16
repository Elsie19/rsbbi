use anyhow::anyhow;

use super::shape::Shape;

pub fn handle_info(index: &Shape, _book: &str) -> Result<String, anyhow::Error> {
    let mut info_vec: Vec<String> = vec![];
    for section in index {
        info_vec.push(format!(
            "**{}** ~ **{}**\nChapters: **{}**\nVerses: **{}**",
            section.title,
            section.section,
            section.length,
            section.chapters.iter().sum::<i64>(),
        ));
    }
    if info_vec.is_empty() {
        Err(anyhow!(
            "Could not loop over index in 'handle_info' in subcommand 'info'"
        ))
    } else {
        Ok(info_vec.join("\n").to_string())
    }
}
