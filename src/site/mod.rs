pub mod render;
pub mod template;
use std::io::Write;
use toml::Value;
#[derive(Clone)]
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

    pub fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            description: self.description.clone(),
            index: self.index.clone(),
            pages: self.pages.clone(),
            renderer: self.renderer.clone(),
        }
    }

    pub fn load_config(&mut self, source: &str) {
        // [site]
        // title = "Rusty"
        // description = "A static site generator written in Rust"

        // load config from self./config.toml
        let config = std::fs::read_to_string(format!("{}/config.toml", source)).unwrap();
        let config: Value = toml::from_str(&config).unwrap();
        self.title = config["site"]["title"].as_str().unwrap().to_string();
        self.description = config["site"]["description"].as_str().unwrap().to_string();
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
        let mut content_template = template::Template::new(self.clone(), Page::new());
        for page in &mut self.pages {
            let start = std::time::Instant::now();
            page.content = self.renderer.render(&page.content);
            content_template.page.clone_from(page);
            page.content = content_template.page().into_string();
            let content = content_template.page();
            page.content = content.into_string();
            println!(
                "Page \"{}\" rendered in {}ms",
                page.title.clone(),
                start.elapsed().as_millis()
            );
        }

        let index_template = template::Template::new(self.clone(), self.index.clone());
        let index = index_template.index();
        self.index.content = index.into_string();
        println!("Rendered in {}ms", start.elapsed().as_millis());
    }

    pub fn write(&self, output: &str) {
        let start = std::time::Instant::now();
        if std::fs::metadata(output).is_err() {
            // output/pages
            std::fs::create_dir_all(format!("{}/pages", output)).unwrap();
        }

        for page in &self.pages {
            let mut file = std::fs::File
                ::create(format!("{}/pages/{}.html", output, &page.title))
                .unwrap();
            file.write_all(page.content.as_bytes()).unwrap();
        }
        let mut file = std::fs::File::create(format!("{}/index.html", output)).unwrap();
        file.write_all(self.index.content.as_bytes()).unwrap();
        println!("Wrote in {}ms", start.elapsed().as_millis());
    }

    pub fn load_render_write(&mut self, source: &str, output: &str) {
        self.load_config(source);
        self.render(source);
        self.write(output);
    }
}