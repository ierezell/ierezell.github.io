use chrono::NaiveDate;
use leptos::logging::error;
use pulldown_cmark::{Options, Parser};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub description: String,
    pub date: NaiveDate,
    pub tags: Vec<String>,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadatas {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownFile {
    pub metadatas: Metadatas,
    pub html_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadFile {
    pub name: String,
    pub content: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubFile {
    name: String,
    path: String,
    sha: String,
    size: i32,
    url: String,
    html_url: String,
    git_url: String,
    download_url: String,
    #[serde(rename = "type")]
    kind: String,
}

pub async fn read_markdown_files() -> Result<Vec<ReadFile>, String> {
    let mut markdown_files: Vec<ReadFile> = Vec::new();

    let posts_url = "https://api.github.com/repos/ierezell/ierezell.github.io/contents/posts";

    let body = match reqwest::get(posts_url).await {
        Ok(body) => match body.json::<Vec<GitHubFile>>().await {
            Ok(body) => body,
            Err(error) => return Err(error.to_string()),
        },
        Err(error) => return Err(error.to_string()),
    };

    for github_markdown in body {
        if github_markdown.kind == "file" && github_markdown.name.ends_with(".md") {
            match reqwest::get(github_markdown.download_url).await {
                Ok(body) => match body.text().await {
                    Ok(text) => markdown_files.push(ReadFile {
                        name: github_markdown.name,
                        content: Some(text),
                        error: None,
                    }),
                    Err(error) => markdown_files.push(ReadFile {
                        name: github_markdown.name,
                        content: None,
                        error: Some(error.to_string()),
                    }),
                },
                Err(error) => markdown_files.push(ReadFile {
                    name: github_markdown.name,
                    content: None,
                    error: Some(error.to_string()),
                }),
            };
        };
    }

    Ok(markdown_files)
}

pub fn markdown_to_html(markdown: String) -> Result<Option<(Metadatas, String)>, String> {
    if let Some(index) = markdown.find("----------") {
        let (metadatas_str, content) = markdown.split_at(index);

        let mut title = String::new();
        let mut description = String::new();
        let mut tags = Vec::new();
        let mut date: NaiveDate =
            NaiveDate::from_ymd_opt(1970, 1, 1).expect("The default date is invalid");

        for line in metadatas_str.lines() {
            if line.starts_with("TITLE=") {
                title = line[6..].to_string();
            } else if line.starts_with("DESCRIPTION=") {
                description = line[12..].to_string();
            } else if line.starts_with("TAGS=") {
                tags = line[5..].split(',').map(|s| s.trim().to_string()).collect();
            } else if line.starts_with("DATE=") {
                date = match NaiveDate::parse_from_str(line[5..].trim(), "%Y-%m-%d") {
                    Ok(date) => date,
                    _ => return Err(format!("Failed to parse date {}", metadatas_str)),
                };
            }
        }

        let content = content[10..].trim_start();

        let mut options = Options::empty();
        options.insert(
            Options::ENABLE_STRIKETHROUGH
                | Options::ENABLE_SMART_PUNCTUATION
                | Options::ENABLE_TABLES
                | Options::ENABLE_TASKLISTS
                | Options::ENABLE_GFM
                | Options::ENABLE_HEADING_ATTRIBUTES,
        );
        let parser = Parser::new_ext(content, options);

        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);

        Ok(Some((
            Metadatas {
                title,
                date,
                description,
                tags,
            },
            html_output,
        )))
    } else {
        return Err("Failed extract metadatas".to_string());
    }
}

pub async fn load_markdown() -> Vec<Post> {
    let markdown_files = match read_markdown_files().await {
        Ok(files) => files,
        Err(err) => {
            error!("Error reading blog posts {}", err);
            return Vec::new();
        }
    };

    let parsed_posts: Vec<Post> = markdown_files
        .into_iter()
        .filter_map(|parsed_file| {
            return match parsed_file.content {
                Some(content) => match markdown_to_html(content) {
                    Ok(parsed_content) => match parsed_content {
                        Some((parsed_metadata, parsed_html)) => Some(Post {
                            title: parsed_metadata.title,
                            description: parsed_metadata.description,
                            tags: parsed_metadata.tags,
                            date: parsed_metadata.date,
                            content: parsed_html,
                        }),
                        _ => {
                            error!("Error blog post {} have no metadata", parsed_file.name);
                            return None;
                        }
                    },
                    Err(err) => {
                        error!("Error parsing blog post {}", err);
                        return None;
                    }
                },
                None => {
                    error!("Error blog post {} have no content", parsed_file.name);
                    return None;
                }
            };
        })
        .collect();

    return parsed_posts;
}
