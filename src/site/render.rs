use super::Page;
use comrak::{markdown_to_html, ComrakOptions};
pub struct Renderer {
    options: ComrakOptions,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            options: ComrakOptions::default(),
        }
    }

    pub fn render(&self, content: &str) -> String {
        let html = markdown_to_html(content, &self.options);
        html
    }
}

struct FrontMatter {
    title: String,
    author: String,
    date: String,
}

impl FrontMatter {
    fn new() -> Self {
        Self {
            title: String::new(),
            author: String::new(),
            date: String::new(),
        }
    }
}

pub fn parse_string(markdown: &str) -> Page {
    let mut page = Page::new();
    let mut lines = markdown.lines();
    let front_matter = parse_front_matter(&mut lines);

    page.title = front_matter.title;
    page.author = front_matter.author;
    page.date = front_matter.date;
    page.content = lines.collect::<Vec<&str>>().join("\n");

    page
}

fn parse_front_matter(lines: &mut std::str::Lines) -> FrontMatter {
    let mut front_matter = FrontMatter::new();
    let mut in_front_matter = false;

    for line in lines {
        if line == "---" {
            if in_front_matter {
                break;
            } else {
                in_front_matter = true;
            }
        } else if in_front_matter {
            let mut parts = line.split(":");
            let key = parts.next().unwrap().trim();
            let value = parts.next().unwrap().trim();
            match key {
                "title" => {
                    front_matter.title = value.to_string();
                    println!("title: {}: {}", key, value);
                }
                "author" => {
                    front_matter.author = value.to_string();
                    println!("author: {}: {}", key, value);
                }
                "date" => {
                    front_matter.date = value.to_string();
                    println!("date: {}: {}", key, value);
                }
                _ => (),
            }
        }
    }

    front_matter
}
