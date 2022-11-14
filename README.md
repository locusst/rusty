# rusty
## a static site generator in Rust

### About
This is a personal project of mine; Don't expect any support for your use case (sorry!)

### Usage
```bash
rusty [FLAGS] [OPTIONS] <source> <output>
```

### Flags
```sh
-h, --help       Prints help information
-V, --version    Prints version information
```

### Options
```sh
-d, --directory <directory>    The directory to use for posts and config
-o, --output <output>          The directory to output the generated site to
```

### Config
Save as config.toml in source directory.
```toml
[site]
# The title of the site
title = "Rusty"
# The author of the site
description = "A static site generator in Rust"
```

### Posts
```md
---

title: Static Site Generator in Rust
date: 2022-11-14
author: Locusst

---
*content*
```

