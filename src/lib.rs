//! # A Rust Crate for managing Bible references, books, chapters and verses
//!
//! Bibleref is a lightweight Rust crate which supports the management of Bible references including parsing, validity checks and output. It is designed to simplify the usage of God's infallible and Holy Word for computing purposes with the aim to simplify the spreading of the good news.
//! May it be used for the glory of God!

use bible::BibleReference;

pub mod bible;

pub mod referencing;

use referencing::{language::get_reference_in_language, parser::get_reference_and_language};

/// Parses a given bible reference with all supported languages and returns an [Option<BibleReference>] depending on whether the parsing was succesful.
/// # Params
/// - `bible_reference`: the given bible reference as a string
/// # Returns
/// An [Option<BibleReference>]
/// - [Some<BibleReference>] if the BibleReference could be succesfuly parsed and is valid
/// - [None] if parsing failed or the Bible reference is not valid.
/// 
/// # Example
/// ```
/// # use bibleref::parse_bible_reference;
/// assert!(parse_bible_reference("Exodus 3".to_string()).is_some());
/// assert!(parse_bible_reference("Revelation 24".to_string()).is_none());
/// ```
pub fn parse_bible_reference(bible_reference: String) -> Option<BibleReference> {
    match get_reference_and_language(bible_reference) {
        Some((bible_reference, _, _)) => Some(bible_reference),
        None => None
    }
}

/// Translates a Bible reference in an other language
/// 
/// # Params
/// - `bible_reference`: The Bible reference in any supported language
/// - `target_lang_code`: The language code of the target language (such as `de`, `en`, `zh_sim`)
/// 
/// # Returns
/// An [Option<String>]
/// - If the translation was succesful, a String with the translated Bible reference will be returned.
/// - If an error occured, None will be returned.
/// 
/// # Example
/// ```
/// # use bibleref::translate_bible_reference;
/// let german_reference: String = translate_bible_reference("Genesis 1:1".to_string(), "de".to_string()).unwrap();
/// assert_eq!(german_reference, "1. Mose 1,1".to_string());
/// let chinese_reference: String = translate_bible_reference("John 3:16".to_string(), "zh_sim".to_string()).unwrap();
/// assert_eq!(chinese_reference, "约翰福音3：16".to_string());
/// ```
pub fn translate_bible_reference(bible_reference: String, target_lang_code: String) -> Option<String> {
    match get_reference_and_language(bible_reference) {
        Some((bible_reference, _, book_reference_type)) => {
            match get_reference_in_language(&bible_reference, &target_lang_code, book_reference_type) {
                Some(translated_reference) => Some(translated_reference),
                None => None,
            }
        }
        None => None
    }
}