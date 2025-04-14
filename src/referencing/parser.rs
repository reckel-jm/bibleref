//! This module contains functions for parsing real language bible references into the crate's internal structures.

use std::{collections::btree_map::Range, error::Error};

use crate::{bible::{BibleBook, BibleBookReference, BibleChapterReference, BibleRange, BibleReference, BibleReferenceRepresentation, BibleVerseReference}, referencing::{errors::{BibleBookNotFoundError, BibleRangeParsingError, ReferenceIsEmptyError}, language::get_language_by_code}};

use super::{errors::LanguageHasNoChapterVersDelimiterError, language::{BookReferenceType, ReferenceLanguage, REFERENCE_LANGUAGES}};

/// A struct representing a search result for a Bible reference.
pub struct BibleReferenceSearchResult {
    /// The valid Bible reference.
    bible_reference: BibleReference,
    
    /// The language code of the reference (e.g. 'de', 'en' etc).
    language_code: String,
    
    /// The type of the reference (long or short).
    reference_type: BookReferenceType
}

impl BibleReferenceSearchResult {
    /// Creates a new BibleReferenceSearchResult.
    /// 
    /// # Arguments
    /// - `bible_reference`: The valid Bible reference.
    /// - `language_code`: The language code of the reference (e.g. 'de', 'en' etc).
    /// - `reference_type`: The type of the reference (long or short).
    /// # Returns
    /// - A new BibleReferenceSearchResult.
    pub fn new(bible_reference: BibleReference, language_code: String, reference_type: BookReferenceType) -> Self {
        Self {
            bible_reference,
            language_code,
            reference_type
        }
    }

    /// Gets the valid Bible reference.
    /// # Returns
    /// - The valid Bible reference.
    pub fn bible_reference(&self) -> &BibleReference {
        &self.bible_reference
    }

    /// Gets the language code in which the search query has been issued (e.g. 'de', 'en' etc).
    /// # Returns
    /// - The language code of the reference as a [String].
    pub fn language_code(&self) -> &String {
        &self.language_code
    }

    /// Gets the type of the search query reference (long or short).
    /// # Returns
    /// - The type of the reference as a [BookReferenceType].
    pub fn reference_type(&self) -> &BookReferenceType {
        &self.reference_type
    }
}


/// A struct representing a search result for a Bible reference.
pub struct BibleReferenceRepresentationSearchResult {
    /// The valid Bible reference.
    bible_reference_representation: BibleReferenceRepresentation,
    
    /// The language code of the reference (e.g. 'de', 'en' etc).
    language_code: String,
    
    /// The type of the reference (long or short).
    reference_type: BookReferenceType
}

impl BibleReferenceRepresentationSearchResult {
    
    /// Creates a new BibleReferenceRepresentationSearchResult.
    /// 
    /// # Arguments
    /// - `bible_reference_representation`: The valid Bible reference.
    /// - `language_code`: The language code of the reference (e.g. 'de', 'en' etc).
    /// - `reference_type`: The type of the reference (long or short).
    /// # Returns
    /// - A new BibleReferenceSearchResult.
    pub fn new(bible_reference_representation: BibleReferenceRepresentation, 
        language_code: String, 
        reference_type: BookReferenceType) -> Self {
        Self {
            bible_reference_representation,
            language_code,
            reference_type
        }
    }

    /// Gets the valid Bible reference representation.
    /// # Returns
    /// - The valid Bible reference representation.
    pub fn bible_reference(&self) -> &BibleReferenceRepresentation {
        &self.bible_reference_representation
    }

    /// Gets the language code in which the search query has been issued (e.g. 'de', 'en' etc).
    /// # Returns
    /// - The language code of the reference as a [String].
    pub fn language_code(&self) -> &String {
        &self.language_code
    }

    /// Gets the type of the search query reference (long or short).
    /// # Returns
    /// - The type of the reference as a [BookReferenceType].
    pub fn reference_type(&self) -> &BookReferenceType {
        &self.reference_type
    }

}

/// Gets a (internal) Bible reference and the language code of a given human readable reference.
/// Returns an error if parsing fails.
/// 
/// # Arguments
/// - `reference`: A human readable Bible reference.
/// # Returns
/// - A result with either a [BibleReferenceSearchResult] or a [`Box<dyn Error>`] with an appropriate error message.
/// # Example
/// ```
/// use bibleref::referencing::parser::{BibleReferenceSearchResult, get_reference_and_language};
/// use bibleref::referencing::language::BookReferenceType;
/// use bibleref::bible::{BibleBook, BibleReference, BibleVerseReference};
/// 
/// let bible_reference_search_result: BibleReferenceSearchResult = get_reference_and_language("1. Mose 1,3".to_string()).unwrap();
/// assert_eq!(*bible_reference_search_result.bible_reference(), BibleReference::BibleVerse(BibleVerseReference::new(BibleBook::Genesis, 1, 3).unwrap()));
/// assert_eq!(bible_reference_search_result.language_code(), "de");
/// assert_eq!(*bible_reference_search_result.reference_type(), BookReferenceType::Long);
/// ```
pub fn get_reference_and_language(reference: String) -> Result<BibleReferenceSearchResult, Box<dyn Error>> {
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
                (0, 0) => return Ok(BibleReferenceSearchResult::new(
                    BibleReference::BibleBook(BibleBookReference::new(bible_book)),
                    language,
                    book_reference_type
                )),
                (0.., 0) => {
                    let chapter: u8 = reference_chapter_str.parse().unwrap();
                    match BibleChapterReference::new(bible_book, chapter) {
                        Ok(chapter_reference) => return Ok(BibleReferenceSearchResult::new(
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
                        Ok(verse_reference) => return Ok(BibleReferenceSearchResult::new(
                            BibleReference::BibleVerse(verse_reference),
                            language.clone(),
                            book_reference_type
                        )),
                        Err(err) => return Err(Box::new(err))
                    }
                },
            }
        }
    }

}

pub fn parse_range(range_reference: String) -> Result<BibleReferenceRepresentationSearchResult, Box<dyn Error>> {
    if range_reference.is_empty() { return Err(Box::new(ReferenceIsEmptyError)) }
    
    // We remove all spaces in the string as we don't need them
    let binding = range_reference.replace(" ", "");
    let reference = binding.trim();

    // Traverse the string and try to get a reference out of it
    let mut current_part: String = "".to_string();
    let mut first_search_result_option: Option<BibleReferenceSearchResult> = None;
    let mut first_part_end_index: usize = 0;


    for (i, c) in reference.chars().enumerate() {
    
        match get_reference_and_language(current_part.clone()) {
            Ok(reference) => {
                first_search_result_option = Some(reference);
            },
            Err(_) => {
                current_part.push(c);
                if first_search_result_option.is_some() {
                    // We have found the reference greedily, so we can stop parsing
                    first_part_end_index = i;
                    break;
                }
            }
        }
    }

    if first_search_result_option.is_some() {
        let first_search_result = first_search_result_option.unwrap();
        let language = get_language_by_code(first_search_result.language_code()).unwrap();
        let second_part = reference.chars().skip(first_part_end_index).collect::<String>();

        if second_part.is_empty() {
            // The second part is empty
            return Err(Box::new(BibleRangeParsingError::NoSecondPartProvided));
        }

        // Check whether the language delimiter is at the beginning of the second part
        if !second_part.starts_with(&language.range_delimiter) {
            // The second part is empty
            return Err(Box::new(BibleRangeParsingError::DelimiterNotFound));
        }

        // We remove the delimiter from the second part
        let second_part = second_part.chars().skip(language.range_delimiter.len()).collect::<String>();

        match get_reference_and_language(second_part.clone()) {
            
            // The second reference is expanded
            Ok(reference) => {
                let second_found_reference = reference.bible_reference().clone();
                match BibleRange::new(
                    first_search_result.bible_reference().clone(),
                    second_found_reference
                ) {
                    Ok(range) => {
                        return Ok(BibleReferenceRepresentationSearchResult::new(
                            BibleReferenceRepresentation::Range(range),
                            first_search_result.language_code().clone(),
                            first_search_result.reference_type().clone()
                        ));
                    },
                    Err(err) => return Err(Box::new(err))
                }
            },
            Err(_) => {
                
                // Check if the first character of the second part is a number
                if second_part.clone().chars().next().unwrap().is_numeric() {
                    
                    // Split the second part by the chapter verse delimiter and check if both parts are a number
                    // TODO: Till now only the first delimiter is used
                    let parts: Vec<&str> = match language.chapter_vers_delimiters.first() {
                        Some(delimiter) => second_part.split(delimiter.as_str()).collect(),
                        
                        // We have to handle the case where the language has no delimiter (which should not be the case)
                        None => return Err(
                            Box::new(
                                LanguageHasNoChapterVersDelimiterError {
                                    language_code: language.language_code.clone()
                                }
                            )
                        ),
                    };
                    if parts.len() == 2 {
                        let first_part = parts[0];
                        let second_part = parts[1];

                        if first_part.chars().all(|c| c.is_numeric()) && second_part.chars().all(|c| c.is_numeric()) {
                            let first_part: u8 = first_part.parse().unwrap();
                            let second_part: u8 = second_part.parse().unwrap();

                            // Create the end reference
                            let second_reference = BibleReference::BibleVerse(BibleVerseReference::new(
                                match first_search_result.bible_reference() {
                                    BibleReference::BibleVerse(verse) => verse.book(),
                                    BibleReference::BibleChapter(chapter) => chapter.book(),
                                    BibleReference::BibleBook(book) => book.book()
                                },
                                first_part,
                                second_part
                            ).unwrap());

                            match BibleRange::new(
                                first_search_result.bible_reference().clone(),
                                second_reference
                            ) {
                                Ok(range) => {
                                    return Ok(BibleReferenceRepresentationSearchResult::new(
                                        BibleReferenceRepresentation::Range(range),
                                        first_search_result.language_code().clone(),
                                        first_search_result.reference_type().clone()
                                    ));
                                },
                                Err(err) => return Err(Box::new(err))
                            }
                        } else {
                            // The second part contains errors
                            return Err(Box::new(BibleRangeParsingError::InvalidSecondPart));
                        }
                    } else if parts.len() == 1 {
                        // A Chapter is given
                        let first_part = parts[0];
                        if first_part.chars().all(|c| c.is_numeric()) {
                            let first_part: u8 = first_part.parse().unwrap();
                            match BibleChapterReference::new(
                                match first_search_result.bible_reference() {
                                    BibleReference::BibleVerse(verse) => verse.book(),
                                    BibleReference::BibleChapter(chapter) => chapter.book(),
                                    BibleReference::BibleBook(book) => book.book()
                                },
                                first_part
                            )
                            {
                                Ok(chapter) => {
                                    return Ok(BibleReferenceRepresentationSearchResult::new(
                                        BibleReferenceRepresentation::Range(BibleRange::new(
                                            first_search_result.bible_reference().clone(),
                                            BibleReference::BibleChapter(chapter)
                                        ).unwrap()),
                                        first_search_result.language_code().clone(),
                                        first_search_result.reference_type().clone()
                                    ));
                                },
                                Err(err) => return Err(Box::new(err))
                            }
                        } else {
                            // The second part contains errors
                            return Err(Box::new(BibleRangeParsingError::InvalidFirstPart));
                        } 
                    }
                } else {
                    return Err(Box::new(BibleRangeParsingError::InvalidSecondPart));
                }
            }
        }       
    }

    Err(Box::new(BibleRangeParsingError::InvalidFirstPart))
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
        assert_eq!(*reference.bible_reference(), BibleReference::BibleVerse(BibleVerseReference::new(BibleBook::Genesis, 1, 3).unwrap()));
        assert_eq!(reference.language_code(), "de");
        assert_eq!(*reference.reference_type(), BookReferenceType::Long);

        let reference = get_reference_and_language("Joh 3".to_string()).unwrap();

        assert_eq!(*reference.bible_reference(), BibleReference::BibleChapter(BibleChapterReference::new(BibleBook::John, 3).unwrap()));
        assert_eq!(reference.language_code(), "de");
        assert_eq!(*reference.reference_type(), BookReferenceType::Short);
    }
}
