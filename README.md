# A Rust Crate for managing Bible references, chapters and verses

Bibleref is a leightweight Rust crate which supports the management of Bible references including parsing, validity checks and output. It is designed to simplify the usage of God's infallible and Holy Word for computing purposes with the aim to simplify the spreading of the good news.
May it be used for the glory of God!

## Features

The following features are already implemented:

- Structures for referencing Bible verses
- Validation parsing (checks whether verse really exists)
- Export reference to human languages
- Parse reference from human languages

## Documentation

The crate is in itself [documented](https://docs.rs/bibleref).

## Examples

### Translating Bible references into other languages

The translation will only take place if the language is supported and the bible reference exists (can be found in the Bible).
```rust
use bibleref::translate_bible_reference;

let german_reference: String = translate_bible_reference("Genesis 1:1".to_string(), "de".to_string()).unwrap();
assert_eq!(german_reference, "1. Mose 1,1".to_string());
let chinese_reference: String = translate_bible_reference("John 3:16".to_string(), "zh_sim".to_string()).unwrap();
assert_eq!(chinese_reference, "约翰福音3：16".to_string());
```

### Check whether a Bible reference really exists
```rust
use bibleref::parse_bible_reference;
assert!(parse_bible_reference("Exodus 3".to_string()).is_ok()); // Exodus 3 exists in the Bible
assert!(parse_bible_reference("Revelation 24".to_string()).is_err()); // Revelation 24 doesn't exist as the book only has 22 chapters
```

## Supported Languages

The following languages have built-in support at the moment. You can add additional languages manually:

| Language | Language Code |
| -------- | ------------- |
| English  | en            |
| German   | de            |
| Chinese (Simplified) | zh_sim |
| Chinese (Traditional) | zh_trad |
| Spanish | es |
| French  | fr |
| Ukrainian | uk |
| Russian | ru |
