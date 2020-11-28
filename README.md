# KOOMPI Wiki

This reposotory contains all documentations of KOOMPI Open Source.

## Developing

1. Installing Rust

```
sudo pacman -S rustup
rustup default stable
```

2. Get the project

```
git clone https://github.com/koompi/wiki.koompi.org
cd wiki.koompi.org
cargo run
```

## USAGE

### URL level
The structure of documents can be categorized as below. Please notice that the maximum depths of routes is only 5 levels. Each level can have any numbers of files, but at least every directory must have 1 index.md.

**NOTES:** No spaces allow

```url
https://website.com/language/chapter_name/unit_name/section_name/section_file.md
```

### Content structure
```bash
content/
|__ language
    |___ chapter
        |___ lesson
        |   |___sesions
        |___ lesson
            |___sesions
```

**Example**

```bash
content/
├── en
|   |___ index.md
│   |___ Applications
|       |___index.md
│       ├── Browser
│       │   └── index.md
│       |__ Code Editors
│           └── index.md
└── kh
    |--- index.md
    |___ Applications
        |___index.md
        ├── Browser
        │   └── index.md
        |___ Code Editors
            └── index.md
```

### Menu navigation

You need to create menu file for each language you support. Inside each file you need to create as many levels as possible. 
**Menu directory structure**

```bash
menu
├── en.md
└── kh.md
```

**Example**

```bash
**Table of Content**

## Introduction

- [Computer Hardware](/en/Introduction/Computer_Hardware.md)
- [Computer Software](/en/Introduction/Computer_Software.md)

```

### Static files

Static files are all files that are not MD but you need to making publicly accessable such as images, pdf, etc...
All static files my place under public directory.

## DEPLOYMENT

```bash
cargo run --release
```