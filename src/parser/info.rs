use anyhow::anyhow;

use super::{bible_verse::ReturnedBibleVerse, shape::Shape};

pub fn handle_info(
    index: &Shape,
    _book: &str,
    info: &ReturnedBibleVerse,
) -> Result<String, anyhow::Error> {
    let mut info_vec: Vec<String> = vec![];
    if info.section.is_some() {
        for section in index {
            info_vec.push(format!(
                "## **{}** ~ **{}**\n> Verses: **{}**",
                section.title,
                info.section.as_ref().unwrap(),
                section.chapters.get::<usize>(info.section.as_ref().unwrap().parse::<usize>().unwrap() - 1).unwrap(),
            ));
        }
    } else {
        for section in index {
            info_vec.push(format!(
                "## **{}** ~ **{}**\n> Chapters: **{}**\n> Verses: **{}**",
                section.title,
                section.section,
                section.length,
                section.chapters.iter().sum::<i64>(),
            ));
        }
    }
    if info_vec.is_empty() {
        Err(anyhow!(
            "Could not loop over index in 'handle_info' in subcommand 'info'"
        ))
    } else {
        Ok(info_vec.join("\n---\n").to_string())
    }
}
