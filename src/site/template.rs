use super::{ Page, Site };
use maud::{ html, Markup, PreEscaped };

pub struct Template {
    pub site: Site,
    pub page: Page,
}

impl Template {
    pub fn new(site: Site, page: Page) -> Self {
        Self { site, page }
    }

    pub fn head(title: String) -> Markup {
        html! {
            head {
                title {(title)}
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="stylesheet" href="style.css";
            }
        }
    }

    pub fn index(&self) -> Markup {
        html! {
        (Self::head(self.site.title.clone()))
        body {
            div class="c"{
                h1 { a href="/" {(self.site.title.clone())} span style="margin-left:0.9rem; font-variant: small-caps; font-size:1.5rem" {(self.site.description.clone())} }
                p { "powered by rusty!" }
                hr {}
                h3 { "Pages" }
                ul {
                    @for page in &self.site.pages {
                        li {
                            a href=(format!("/pages/{}.html", page.title.clone())) {(page.title.clone())}
                        }
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
            div class="c"{
                h1 { a href="/" {(self.site.title.clone())} span style="margin-left:0.9rem; font-variant: small-caps; font-size:1.5rem" {(self.site.description.clone())} }
                p { "powered by rusty!" }
                hr {}
                h2 { (self.page.title) }
                p { (self.page.author) }
                p { (self.page.date) }
                (PreEscaped(&self.page.content))
            }
        }
        }
    }
}