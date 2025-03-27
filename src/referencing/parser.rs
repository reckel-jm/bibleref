//! This module contains functions for parsing real language bible references into the crate's internal structures.

use std::error::Error;

use crate::{bible::{BibleBook, BibleBookReference, BibleChapterReference, BibleReference, BibleVerseReference}, referencing::errors::{BibleBookNotFoundError, ReferenceIsEmptyError}};

use super::language::{BookReferenceType, ReferenceLanguage, REFERENCE_LANGUAGES};

/// Gets a (internal) Bible reference and the language code of a given human readable reference.
/// Returns an error if parsing fails.
/// 
/// # Arguments
/// - `reference`: A human readable Bible reference.
/// # Returns
/// - A result with either a tuple containing the internal Bible reference and the language code of the reference or a [Box<dyn Error>] with an appropriate error message.
/// # Example
/// ```
/// use bibleref::referencing::parser::get_reference_and_language;
/// use bibleref::referencing::language::BookReferenceType;
/// use bibleref::bible::{BibleBook, BibleReference, BibleVerseReference};
/// 
/// let (reference, language, reference_type) = get_reference_and_language("1. Mose 1,3".to_string()).unwrap();
/// assert_eq!(reference, BibleReference::BibleVerse(BibleVerseReference::new(BibleBook::Genesis, 1, 3).unwrap()));
/// assert_eq!(language, "de");
/// assert_eq!(reference_type, BookReferenceType::Long);
/// ```
pub fn get_reference_and_language(reference: String) -> Result<(BibleReference, String, BookReferenceType), Box<dyn Error>> {
    if reference.is_empty() { return Err(Box::new(ReferenceIsEmptyError)) }
    
    enum ParserFlag {
        BookPart,
        ChapterPart,
        VersePart
    }
    
    // We remove all spaces in the string as we don't need them
    let binding = reference.replace(" ", "");
    let reference = binding.trim();

    let mut reference_book_str: String = "".to_string();
    let mut reference_chapter_str: String = "".to_string();
    let mut reference_verse_str: String = "".to_string();

    let mut parser_flag: ParserFlag = ParserFlag::BookPart;

    for (i, c) in reference.chars().enumerate() {
        match parser_flag {
            ParserFlag::BookPart => {
                if i == 0 {
                    reference_book_str.push(c);
                } else {
                    if c.is_numeric() {
                        reference_chapter_str.push(c);
                        parser_flag = ParserFlag::ChapterPart;
                    } else {
                        reference_book_str.push(c);
                    }
                }
            },
            ParserFlag::ChapterPart => {
                if c.is_numeric() {
                    reference_chapter_str.push(c);
                } else {
                    parser_flag = ParserFlag::VersePart
                }
            },
            ParserFlag::VersePart => {
                if c.is_numeric() {
                    reference_verse_str.push(c);
                }
            }
        }
    }

    let book_finding = find_book_in_any_language(&reference_book_str);

    match book_finding {
        None => return Err(
            Box::new(
                BibleBookNotFoundError {
                    provided_bible_book_string: reference_book_str.clone()
                }
            )
        ),
        Some((bible_book, language, book_reference_type)) => {
            match (reference_chapter_str.len(), reference_verse_str.len()) {
                (0, 0) => return Ok((
                    BibleReference::BibleBook(BibleBookReference::new(bible_book)),
                    language,
                    book_reference_type
                )),
                (0.., 0) => {
                    let chapter: u8 = reference_chapter_str.parse().unwrap();
                    match BibleChapterReference::new(bible_book, chapter) {
                        Ok(chapter_reference) => return Ok((
                            BibleReference::BibleChapter(chapter_reference),
                            language.clone(),
                            book_reference_type
                        )),
                        Err(err)  => return Err(Box::new(err))
                    }
                },
                (0.., 0..) => {
                    let chapter: u8 = reference_chapter_str.parse().unwrap();
                    let verse: u8 = reference_verse_str.parse().unwrap();

                    match BibleVerseReference::new(bible_book, chapter, verse) {
                        Ok(verse_reference) => return Ok((
                            BibleReference::BibleVerse(verse_reference),
                            language.clone(),
                            book_reference_type
                        )),
                        Err(err) => return return Err(Box::new(err))
                    }
                },
            }
        }
    }

}

fn find_book_in_any_language(book_name: &str) -> Option<(BibleBook, String, BookReferenceType)> {
    let languages = &*REFERENCE_LANGUAGES.read().unwrap();

    for language in languages {
        let result = find_book_in_certain_language(book_name, language);
        if result.is_some() {
            return result;
        }
    }

    None
}

fn find_book_in_certain_language(book_name: &str, language: &ReferenceLanguage) -> Option<(BibleBook, String, BookReferenceType)> {

    for book in language.long_names.keys() {
        let language_long_space_removed: Vec<String> = language.long_names[book].iter().map(|str| str.replace(" ", "")).collect();
        if language_long_space_removed.contains(&book_name.to_string()) {
            return Some((*book, language.language_code.clone(), BookReferenceType::Long))
        }
        let language_short_space_removed: Vec<String> = language.short_names[book].iter().map(|str| str.replace(" ", "")).collect();
        if language_short_space_removed.contains(&book_name.to_string()) {
            return Some((*book, language.language_code.clone(), BookReferenceType::Short))
        }
    }

    None
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_reference_finding() {
        let reference = get_reference_and_language("1. Mose 1,3".to_string()).unwrap();
        let bible_reference = reference.0;
        let lang = reference.1;
        let ref_type = reference.2;

        dbg!(bible_reference);
        dbg!(lang);
        dbg!(ref_type);

        let reference = get_reference_and_language("Joh 3".to_string()).unwrap();
        let bible_reference = reference.0;
        let lang = reference.1;
        let ref_type = reference.2;

        dbg!(bible_reference);
        dbg!(lang);
        dbg!(ref_type);
    }
}
