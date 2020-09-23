***(some of the following information may be subject to change)***
# Aldoc

*aldoc* is a markup language, which takes heavy inspiration from Markdown. Its
main goal is to provide the beauty and control of LaTeX documents with 
Markdown's pretty syntax.

Another one of its goals is to remove the quirks that Markdown brings with its 
simplistic design, such as:

- Different versions and editions of Markdown which mildly in syntax, thus 
making it unreliable for posting on multiple platforms (GitHub Markdown, 
original HTML Markdown, Pandoc Markdown, etc.)
- Markdown was not intended for use outside of small documents, such as
small notes or READMEs (this one), which led to decisions that impacted the
ergonomics in the syntax (pandoc filters) and ended up in the creation of the 
different variants.

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
- Enumerated lists can be written in many ways. Aldoc's design allow you to use
any combination of tokens.
	- With numbers:
		```
		1. Alement
		2. Belement
		3. Celement
		```
	- With letters (uppercase or lowercase):
		```
		a) Alement
		b) Belement
		c) Celement
		```
	- With roman numbers (uppercase or lowercase):
		```
		I- Alement
		II- Belement
		III- Celement
		```
- Bold text is written with asterisks around it.
	```
	Normal text is written *until the asterisks come around*.
	```
## Tool

As a tool, library and Cargo package, it provides an abstraction for the 
language and also a way to compile the documents to PDF. To do that the 
following processes takes place:

1. The aldoc source is parsed into a Rust abstraction.
2. The abstraction is compiled to LaTeX.
3. The LaTeX code is compiled to PDF via LatexMk (this step is planned to 
change)


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
- [X] Bold text
- [X] Unnumbered lists
- [X] Enumerated lists
	- [X] Numbered
	- [X] Alphabetic
	- [X] Roman
- [ ] UTF-8 support
- [ ] Windows line endings
- [ ] Control list tokens completely (make the selected token symbol appear in 
the final document)
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

- [Nom](https://github/Geal/nom)
- [*Fountain* markup language parser using Nom](https://github.com/adamchalmers/fountain-rs)
- [Markdown parser using Nom](https://github.com/HGHimself/prose/blob/master/src/markdown.rs)

*(ironically this README is written in Markdown)*
