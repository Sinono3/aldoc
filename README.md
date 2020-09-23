***(all of the following information may be subject to change)***

# Aldoc

*aldoc* is a markup language, which takes heavy inspiration from Markdown. Its 
goal is to provide the simple syntax that Markdown (Pandoc's version, 
specifically) has, but without the quirks that it brings with it, such as:

- Different versions and editions of Markdown which vary mildly in 
syntax, thus making it unreliable for posting on multiple platforms 
(GitHub Markdown, original HTML Markdown, Pandoc Markdown, etc.)
- Markdown was not intended for use outside of small documents, such as
small notes or READMEs, which led to decisions that impacted the
ergonomics in the syntax (pandoc filters) and ended up in the creation of the different
variants.

The *aldoc* compiler actually only plays a small part in the compilation:

1. The aldoc source is parsed into a Rust abstraction.
2. The abstraction is compiled to LaTeX.
3. The LaTeX code is compiled to PDF via LatexMk.

## Syntax 

The syntax of aldoc is still *WIP*. I have not decided yet which syntax is the
most ergonomic in the end. But still, the one used for testing is the 
following:

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
- Enumerated lists can be written:
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
	The symbol after the number (terminator), can be any of `.`, `-`, or `)`.
- Bold text is written with asterisks around it.
	```
	Normal text is written *until the asterisks come around*.
	```

## Features

- [X] Normal paragraphs
- [X] Allow LaTeX in the source
- [X] Bold text
- [X] Unnumbered lists
- [X] Enumerated lists
	- [X] Numbered
	- [ ] Alphabetic
	- [ ] Roman
- [ ] Embeds
	- [ ] Images
	- [ ] Vector images
	- [ ] Tables
- [ ] Line separators
- [ ] LaTeX template support (for defaulting styles or packages)

## Usage

To actually compile the document, you only need to provide it with the input
file path (.ald) and the output pdf path, like this:

```shell
$ aldoc doc.ald out.pdf
```

You may even omit the output file, in which case, aldoc will output a pdf
with the same name as the document.

```shell
$ aldoc doc.ald # outputs pdf as "doc.pdf"
```

## Thanks

Thanks to Markdown, and the *Nom* parser library for providing the necessary 
tools, as it was fundamental to making the Aldoc parser more extensible. Links 
for reference:

- [https://github/Geal/nom](Nom)
- [https://github.com/adamchalmers/fountain-rs](*Fountain* markup language parser using Nom)
- [https://github.com/HGHimself/prose/blob/master/src/markdown.rs](Markdown parser using Nom)

*(ironically this README is written in Markdown)*
