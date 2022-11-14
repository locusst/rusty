use super::{Page, Site};
use maud::{html, Markup};

pub struct Template {
    pub page: Page,
}

// two template types:
// 1. index
// 2. page

impl Template {
    pub fn new(page: Page) -> Self {
        Self { page }
    }

    pub fn head(title: String) -> Markup {
        html! {
            head {
                title {(title)}
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
            }
        }
    }

    pub fn index(&self, site: &Site) -> Markup {
        html! {
        (Self::head(site.title.clone()))
        body {
            h1 { (site.title) }
            ul {
                @for page in &site.pages {
                    li {
                        a href=(format!("./{}.html", page.title)) { (page.title) }
                    }
                }
            }
        }
        }
    }

    pub fn page(&self) -> Markup {
        html! {
        (Self::head(self.page.title.clone()))
        body {
            h1 { (self.page.title) }
            h2 { (self.page.author) }
            h3 { (self.page.date) }
            (maud::PreEscaped(self.page.content.clone()))
        }
        }
    }
}
