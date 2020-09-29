***(some of the following information may be subject to change)***
# Aldoc

[![Crates.io](https://img.shields.io/crates/v/aldoc.svg)](https://crates.io/crates/aldoc)
[![Docs](https://docs.rs/aldoc/badge.svg)](https://docs.rs/aldoc)

*aldoc* is a markup language with the goal of providing the beauty and control 
of LaTeX documents with Markdown's pretty syntax, in other words, letting you 
write your documents without hardcoding LaTeX.

It does so while also trying to solve some quirks that Markdown had since its 
creation: its rules were never clearly established, and as more features were 
needed for document formatting, variants began appearing, each with their own 
differences.

The different versions and editions of Markdown vary mildly in syntax, thus 
making it unreliable for posting on multiple platforms (GitHub Markdown, 
original HTML Markdown, Pandoc Markdown, etc.)

## Status

This project is still in its infancy (pre-alpha), and major design decisions 
haven't been taken yet. The goals spoken of haven't been reached yet, and 
features are lacking, this shouldn't be used on its current state.

## Syntax 

The syntax of aldoc is still *WIP*: what syntax will be the most beneficious 
has not yet been decided, but still, the one used for testing temporarily is 
the following:

- Paragraphs are spaced with a blank line between them.
	```
	Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum dolor 
	quam, sagittis quis porta id, mattis eget ligula. Morbi id eros ut mi 
	lobortis laoreet non vel magna. Sed sollicitudin dapibus metus ut ultrices. 

	Sed ornare dolor risus, sit amet pretium tellus tempus et. Vivamus aliquet,
	felis venenatis finibus lacinia, nisi velit laoreet odio, vitae 
	pellentesque turpis eros nec est. Maecenas vestibulum scelerisque cursus.
	```
- Unnumbered lists can be written with the `-` or the `+` character.
	```
	- Alement
	- Belement
	- Celement
	```
- Aldoc's design allow you to use any combination of enumerator (`1`, `a`, 
`III`) and symbols (`.`, `)`, `-`), in different orders.
	- With numbers:
		```
		1. Alement
		2. Belement
		3. Celement
		```
	- With letters (uppercase or lowercase):
		```
		(a) Alement
		(b) Belement
		(c) Celement
		```
	- With roman numbers (uppercase or lowercase):
		```
		I- Alement
		II- Belement
		III- Celement
		```

## Tool

As a tool, library and Cargo package, it provides an abstraction for the 
language and also a way to compile the documents to PDF. To do that the 
following processes takes place:

1. The aldoc source is parsed into a Rust abstraction.
2. The abstraction is compiled to LaTeX.
3. The LaTeX code is compiled to PDF via Tectonic.

### Usage

To actually compile the document, you only need to provide it with the input
file path (.ald) and the output pdf path, like this:

```shell
$ aldoc doc.ald compile out.pdf
```

You may even omit the output file, in which case, aldoc will output a pdf
with the same basename as the document.

```shell
$ aldoc doc.ald compile # outputs pdf as "doc.pdf"
```

### Features

- [X] Normal paragraphs
- [X] Allow LaTeX in the source
- [X] Unnumbered lists
- [X] Enumerated lists
	- [X] Numbered
	- [X] Alphabetic
	- [X] Roman
- [X] UTF-8 support
- [X] Cross-platform line endings
- [X] Control list tokens completely (make the selected token symbol appear in 
the final document)
- [ ] Bold, italics and emphasize text
- [ ] Checkboxes
- [ ] Embeds
	- [ ] Images
	- [ ] Vector images
	- [ ] Tables
- [ ] Line separators
- [ ] LaTeX template support (for defaulting styles or packages)

## Thanks

Thanks to Markdown, and the *Nom* parser library for providing the necessary 
tools, as it was fundamental to making the Aldoc parser more extensible. Links 
for reference:

- [Nom](https://github.com/Geal/nom)
- [*Fountain* markup language parser using Nom](https://github.com/adamchalmers/fountain-rs)
- [Markdown parser using Nom](https://github.com/HGHimself/prose/blob/master/src/markdown.rs)

*(ironically this README is written in Markdown)*
