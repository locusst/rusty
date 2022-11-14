---
title: Static Site Generator in Rust
date: 2022-11-14
author: Locusst
---

## About
I've been working on a static site generator in Rust for a while now. It's not ready for production yet, but I'm getting there. I'm using it to generate this site, and I'm pretty happy with it so far.

## Technology
Rusty uses the following technologies:
- [maud](https://maud.lambda.xyz/) for templating
- [comrak](https://github.com/kivikakk/comrak) for markdown parsing
- [clap](https://github.com/clap-rs/clap) for command line parsing

## Flow
First, the program takes a source directory and output directory as command line arguments. It then reads all the files in the source directory, and parses them as to obtain front matter info (title, author, date). It then renders the markdown into HTML using comrak, and inserts that into a template along with the front matter using maud. Finally, it writes the rendered HTML to the output directory.

## Future
I'm planning on adding the following features:
- [✅] Support for multiple templates
- [❌] Support for a development server
- [❌] Support for custom CSS
- [❌] Support for custom JS