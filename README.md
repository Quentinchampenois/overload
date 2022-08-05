# Overload generator

Overload is a file generator for [decidim-app](https://github.com/OpenSourcePolitics/decidim-app) applications entirely written in Rust.

## Description

This script aims to generate a clean __OVERLOADS.md__ file which references all non-official customizations made by Open Source Politics. It helps to follow which files are overwritten.

For each not registered files, script will find the hash commit which introduced this file with the commit title.

For each override, a new line is added to the overloads registry like following : 

```markdown
* <FILENAME>
`<COMMIT HASH> - <COMMIT TITLE>`
```

__Note:__ Script deletes the overload registry and rewrite it, you can't edit manually the file since changes will be removed.

Overloads files must be present in Git history otherwise they won't be added in overloads generator

## Getting started

Create a `.overloadignore` files with the exhaustive list of files which must be ignored by program.

1. Run program 
```bash
cargo run
```

Or use the binary : 

1. Compile program
```bash
cargo build --release
```

2. Run the binary
```bash
target/release/overload
```
