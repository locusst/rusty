pub mod render;
pub mod template;
use std::io::Write;

pub struct Page {
    pub title: String,
    pub author: String,
    pub date: String,
    pub content: String,
}

impl Page {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            author: String::new(),
            date: String::new(),
            content: String::new(),
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            author: self.author.clone(),
            date: self.date.clone(),
            content: self.content.clone(),
        }
    }
}

pub struct Site {
    pub title: String,
    pub description: String,
    pub index: Page,
    pub pages: Vec<Page>,
    pub renderer: render::Renderer,
}

impl Site {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            index: Page::new(),
            pages: Vec::new(),
            renderer: render::Renderer::new(),
        }
    }

    pub fn render(&mut self, source: &str) {
        let start = std::time::Instant::now();
        if std::fs::metadata(source).is_err() {
            panic!("Source directory does not exist");
        }
        let paths = std::fs::read_dir(source).unwrap();
        for path in paths {
            let path = path.unwrap().path().to_str().unwrap().to_string();
            if path.ends_with(".md") {
                let content = std::fs::read_to_string(path).unwrap();
                let page = render::parse_string(&content);
                self.pages.push(page);
            }
        }
        for page in &mut self.pages {
            let start = std::time::Instant::now();
            page.content = self.renderer.render(&page.content);
            let content_template = template::Template::new(page.clone());
            let content = content_template.page().into_string();
            page.content = content;
            println!(
                "\"{}\" rendered in {}ms",
                page.title.clone(),
                start.elapsed().as_millis()
            );
        }

        let index_template = template::Template::new(Page::new());
        let index = index_template.index(&self);
        self.index.content = index.into_string();
        println!("Rendered in {}ms", start.elapsed().as_millis());
    }

    pub fn write(&self, output: &str) {
        let start = std::time::Instant::now();
        if std::fs::metadata(output).is_err() {
            std::fs::create_dir_all(output).unwrap();
        }
        for page in &self.pages {
            let start = std::time::Instant::now();
            let mut file = std::fs::File::create(format!("{}/{}.html", output, &page.title)).unwrap();
            file.write_all(page.content.as_bytes()).unwrap();
            println!("Wrote {} in {}ms", &page.title, start.elapsed().as_millis());
        }
        let mut file = std::fs::File::create(format!("{}/index.html", output)).unwrap();
        file.write_all(self.index.content.as_bytes()).unwrap();
        println!("Wrote in {}ms", start.elapsed().as_millis());
    }
}
