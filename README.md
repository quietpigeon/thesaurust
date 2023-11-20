# thesaurust
A simple dictionary application built within the terminal, written in Rust. 
![Demo](docs/demo.gif)
## How it works
The data is fetched from the API provided by https://dictionaryapi.dev/. Since words can contain more than one meanings, the user can toggle between different meanings based on the part of speech of the definition.
## Installation
Clone this repository:
```
$ git clone https://github.com/QuietPigeon2001/thesaurust
```
Navigate to the repository and run the following commands:
```
$ cargo install --path .
$ thesaurust
```
## Usage
* `/`: Insert the word you would like to look up.
* `<ENTER>`: Search.
* `j`, `k`: Select the part of speech and press `<ENTER>`.
* `l`, `h`: Toggle between multiple definitions.
* `q`: Exit the app.
## Roadmap
- [x] Show an example with the definition (if available)
- [x] Toggle between multiple definitions
- [ ] Show synonyms and antonyms
- [ ] Toggle between definitions with the same part of speech
