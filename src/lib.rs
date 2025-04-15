//! # A Rust Crate for managing Bible references, books, chapters and verses
//!
//! Bibleref is a lightweight Rust crate which supports the management of Bible references including parsing, validity checks and output. It is designed to simplify the usage of God's infallible and Holy Word for computing purposes with the aim to simplify the spreading of the good news.
//! May it be used for the glory of God!
//!
//! # Features
//! - Provides internal structures for Bible references, books, chapters and verses
//! - Parses Bible references in and from various languages
//! - Translates Bible references into various languages
//! - Validates Bible references
//! - Gets the number of chapters and verses of a Bible book
//!
//! # Examples
//! ## Does Genesis 4:5 exist?
//! ```
//! // Genesis 4:5 exists and is a valid Bible reference (of type BibleVerse)
//! assert!(bibleref::parse("Genesis 4:5").is_ok());
//! ```
//!
//! ## How about 出埃及记2:3 (Exodus 2:3 in Chinese)?
//!  ```
//! // 出埃及记2:3 exists and is a valid Bible reference (of type BibleVerse)
//! assert!(bibleref::parse("出埃及记2:3").is_ok());
//! ```
//!
//! ## Translate John 3:16 into German
//! ```
//! // The German translation of John 3:16 is "Johannes 3,16"
//! let german_reference: String = bibleref::translate("John 3:16", "de").unwrap();
//! assert_eq!(german_reference, "Johannes 3,16");
//! ```
//!
//! ## Get the number of chapters in the book of Revelation
//! ```
//! # use bibleref::bible::validate::get_number_of_chapters;
//! # use bibleref::bible::BibleBook;
//! // The book of Revelation has 22 chapters
//! assert_eq!(get_number_of_chapters(&BibleBook::Revelation), 22);
//! ```

pub mod bible;

pub mod referencing;

pub mod errors;

use bible::BibleReferenceRepresentation;
use referencing::{language::get_reference_representation_in_language, parser::parse_reference};
use std::error::Error;

/// Parses a given bible reference with all supported languages and returns an [`Result<BibleReference, Box<dyn Error>>`] depending on whether the parsing was successful.
/// # Params
/// - `bible_reference`: the given bible reference as a string
/// # Returns
/// A [`Result<BibleReferenceRepresentation>`] with the following possible outcomes:
/// - [`Some<BibleReferenceRepresentation>`] if the BibleReference could be successfully parsed and is valid
/// - [`Box<dyn Error>`] if parsing failed or the Bible reference is not valid.
///
/// # Example
/// ```
/// # use bibleref::parse;
/// # use bibleref::referencing::errors::ReferenceIsEmptyError;
/// // Exodus 3 exists and is a valid Bible reference (of type BibleChapter)
/// assert!(parse("Exodus 3").is_ok());
/// // The same applies with John 3,16
/// assert!(parse("John 3,16").is_ok());
/// // Revelation 24 does not exist and (as the book only has 21 chapters) and therefore not a valid Bible reference
/// assert!(parse("Revelation 24").is_err());
/// // You can display the error message as a string (in English)
/// assert!(parse("Revelation 24").err().unwrap().to_string().contains("The chapter does not exist"));
/// // An empty string is not a valid Bible reference
/// assert!(parse("").err().unwrap().downcast_ref::<ReferenceIsEmptyError>().is_some());
/// ```
pub fn parse(bible_reference: &str) -> Result<BibleReferenceRepresentation, Box<dyn Error>> {
    match parse_reference(bible_reference) {
        Ok(bible_reference_representation_search_result) => {
            Ok(bible_reference_representation_search_result
                .bible_reference()
                .clone())
        }
        Err(boxed_error) => Err(boxed_error),
    }
}

/// Translates a Bible reference in an other language
///
/// # Params
/// - `bible_reference`: The Bible reference in any supported language
/// - `target_lang_code`: The language code of the target language (such as `de`, `en`, `zh_sim`)
///
/// # Returns
/// A [`Result<String, Box<dyn Error>>`] with the following possible outcomes:
/// - If the translation was successful, a String with the translated Bible reference will be returned.
/// - If an error occurred, a [`Box<dyn Error>`] with the specific error will be returned.
///
/// # Example
/// ```
/// // The German translation of Genesis 1:1 is "1. Mose 1,1"
/// let german_reference: String = bibleref::translate("Genesis 1:1", "de").unwrap();
/// assert_eq!(german_reference, "1. Mose 1,1");
/// // The Chinese translation of John 3:16 is "约翰福音3：16"
/// let chinese_reference: String = bibleref::translate("John 3:16", "zh_sim").unwrap();
/// assert_eq!(chinese_reference, "约翰福音3：16");
/// // The translation of a non-existing Bible reference will throw an error
/// assert!(bibleref::translate("Exodus 72", "de").is_err());
/// // You can also translate chapters of the Bible, the number of spaces will be ignored
/// let german_chapter: String = bibleref::translate("Matthew   19", "de").unwrap();
/// assert_eq!(german_chapter, "Matthäus 19");
/// ```
pub fn translate(bible_reference: &str, target_lang_code: &str) -> Result<String, Box<dyn Error>> {
    match parse_reference(bible_reference) {
        Ok(bible_reference_representation_search_result) => {
            match get_reference_representation_in_language(
                bible_reference_representation_search_result.bible_reference(),
                target_lang_code,
                *bible_reference_representation_search_result
                    .reference_type(),
                true,
            ) {
                Ok(translated_reference) => Ok(translated_reference),
                Err(err) => Err(Box::new(err)),
            }
        }
        Err(boxed_error) => Err(boxed_error),
    }
}
