# A Rust Crate for managing Bible references, chapters and verses

Bibleref is a leightweight Rust crate which supports the management of Bible references including parsing, validity checks and output. It is designed to simplify the usage of God's word.
May it be used for the glory of God!

*If you like the crate, please consider giving it a star on Github!*

## Features

- Provides internal structures for Bible reference representations (single and ranging) consisting of books, chapters and/or verses
- Parses Bible references from real world languages
- Translates internal Bible references into real world languages
- Translates Bible references from one language to another
- Validates Bible references
- Gets the number of chapters and verses of a Bible book
- Upcast/downcast Bible references to/from different types
- Iterating over Bible references (e.g all books of the Bible, all chapters of a book, all verses of a chapter)

## Documentation

The crate is in itself [documented](https://docs.rs/bibleref).

## Examples

### Translating Bible references into other languages

The translation will only take place if the language is supported and the bible reference exists (can be found in the Bible).
```rust
let german_reference: String = bibleref::translate("Genesis 1:1", "de").unwrap();
assert_eq!(german_reference, "1. Mose 1,1");
let chinese_reference: String = bibleref::translate("John 3:16-18", "zh_sim").unwrap();
assert_eq!(chinese_reference, "约翰福音3：16-18");
```

### Check whether a Bible reference really exists
```rust
assert!(bibleref::parse("Exodus 3").is_ok()); // Exodus 3 exists in the Bible
assert!(bibleref::parse("Revelation 24").is_err()); // Revelation 24 doesn't exist as the book only has 22 chapters
```

## Supported Languages

The following languages have built-in support at the moment. You can add additional languages manually:

| Language | Language Code |
| -------- | ------------- |
| Chinese (Simplified) | zh_sim |
| Chinese (Traditional) | zh_trad |
| Czech | cs |
| Dutch | nl |
| English | en |
| French | fr |
| German | de |
| Hungarian | hu |
| Indonesian | id |
| Italian | it |
| Japanese | ja |
| Korean | ko |
| Polish | pl |
| Russian | ru |
| Spanish | es |
| Ukrainian | uk |
| Vietnamese | vi |