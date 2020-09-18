***(all of the following information may be subject to change)***

# Aldoc

*aldoc* is a markup language, which takes heavy inspiration from Markdown. Its 
goal is to provide the simple syntax that Markdown (Pandoc's version, 
especifically) has, but without the quirks that it brings with it, such as:

- Different versions and editions of Markdown which vary mildly in 
syntax, thus making it unreliable for posting on multiple platforms 
(GitHub Markdown, original HTML Markdown, Pandoc Markdown, etc.)
- Markdown was not intended for use outside of small documents, such as
small notes or READMEs, which led to decisions that impacted the
ergonomics in the syntax (pandoc filters) and ended up in the creation of the different
versions.

The *aldoc* compiler actually only plays a small part in the compilation:

1. The aldoc source is parsed and compiled to LaTeX.
2. The LaTeX code is compiled to PDF.

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
- Lists can only be written with the `-` character.
	```
	- Alement
	- Belement
	- Celement
	```
- Bold text is written with asterisks around it.
	```
	Normal text is written *until the asterisks come around*.
	```

## Features

- [X] Normal paragraphs
- [X] Allow LaTeX in the source
- [X] Bold text
- [X] Unnumbered lists
- [ ] Numbered lists
- [ ] Embeds
	- [ ] Images
	- [ ] Vector images
	- [ ] Tables
- [ ] Line separators
- [ ] LaTeX template support (for defaulting styles or packages)

*(ironically this README is written in Markdown)*
