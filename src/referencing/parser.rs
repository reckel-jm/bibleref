//! This module contains functions for parsing real language bible references into the crate's internal structures.

use std::error::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{
    errors::LanguageHasNoChapterVersDelimiterError,
    language::{BookReferenceType, REFERENCE_LANGUAGES, ReferenceLanguage},
};
use crate::{
    bible::{
        BibleBook, BibleBookReference, BibleChapterReference, BibleRange, BibleReference,
        BibleReferenceRepresentation, BibleVerseReference,
    },
    referencing::{
        errors::{BibleBookNotFoundError, BibleRangeParsingError, ReferenceIsEmptyError},
        language::get_language_by_code,
    },
};

/// A struct representing a search result for a Bible reference.
#[derive(Debug, Clone)]
pub struct BibleReferenceSearchResult {
    /// The valid Bible reference.
    bible_reference: BibleReference,

    /// The language code of the reference (e.g. 'de', 'en' etc).
    language_code: String,

    /// The type of the reference (long or short).
    reference_type: BookReferenceType,
}

/// A struct representing a search result for a Bible reference representation.
#[derive(Debug, Clone)]
pub struct BibleReferenceRepresentationSearchResult {
    /// The valid Bible reference representation.
    bible_reference_representation: BibleReferenceRepresentation,

    /// The language code of the reference (e.g. 'de', 'en' etc).
    language_code: String,

    /// The type of the reference (long or short).
    reference_type: BookReferenceType,
}

impl BibleReferenceRepresentationSearchResult {
    /// Creates a new BibleReferenceRepresentationSearchResult.
    ///
    /// # Arguments
    /// - `bible_reference_representation`: The valid Bible reference representation.
    /// - `language_code`: The language code of the reference (e.g. 'de', 'en' etc).
    /// - `reference_type`: The type of the reference (long or short).
    /// # Returns
    /// - A new BibleReferenceRepresentationSearchResult.
    pub fn new(
        bible_reference_representation: BibleReferenceRepresentation,
        language_code: String,
        reference_type: BookReferenceType,
    ) -> Self {
        Self {
            bible_reference_representation,
            language_code,
            reference_type,
        }
    }

    /// Gets the valid Bible reference representation.
    /// # Returns
    /// - The valid [BibleReferenceRepresentation].
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

/// Parses a Bible reference string and returns a BibleReferenceRepresentationSearchResult.
/// This function tries to parse the input as a range reference first, and if that fails,
/// it tries to parse it as a single reference.
///
/// # Arguments
/// - `bible_reference`: A human readable Bible reference.
/// # Returns
/// - A result with either a [BibleReferenceRepresentationSearchResult] or a [`Box<dyn Error>`] with an appropriate error message.
/// # Example
/// ```
/// use bibleref::referencing::parser::{BibleReferenceRepresentationSearchResult, parse_reference};
/// use bibleref::referencing::language::BookReferenceType;
/// use bibleref::bible::{BibleBook, BibleReference, BibleVerseReference};
/// use bibleref::bible::BibleReferenceRepresentation;
///
/// // Parse a single reference
/// let result = parse_reference("1. Mose 1,3").unwrap();
/// assert_eq!(result.language_code(), "de");
///
/// // Parse a range reference
/// let result = parse_reference("1. Mose 1,3-5").unwrap();
/// assert_eq!(result.language_code(), "de");
/// ```
pub fn parse_reference(
    bible_reference: &str,
) -> Result<BibleReferenceRepresentationSearchResult, Box<dyn Error>> {
    // Try to parse as a range reference first
    match parse_range_reference(bible_reference.to_string()) {
        Ok(result) => Ok(result),
        Err(_) => {
            // If that fails, try to parse as a single reference
            match parse_single_reference(bible_reference.to_string()) {
                Ok(result) => {
                    // Convert BibleReferenceSearchResult to BibleReferenceRepresentationSearchResult
                    Ok(BibleReferenceRepresentationSearchResult::new(
                        BibleReferenceRepresentation::Single(result.bible_reference().clone()),
                        result.language_code().clone(),
                        *result.reference_type(),
                    ))
                }
                Err(err) => Err(err),
            }
        }
    }
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
    pub fn new(
        bible_reference: BibleReference,
        language_code: String,
        reference_type: BookReferenceType,
    ) -> Self {
        Self {
            bible_reference,
            language_code,
            reference_type,
        }
    }

    /// Gets the valid Bible reference.
    /// # Returns
    /// - The valid [BibleReference].
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

/// Gets a (internal) Bible reference and the language code of a given human readable reference.
/// Returns an error if parsing fails.
///
/// # Arguments
/// - `reference`: A human readable Bible reference.
/// # Returns
/// - A result with either a [BibleReferenceSearchResult] or a [`Box<dyn Error>`] with an appropriate error message.
/// # Example
/// ```
/// use bibleref::referencing::parser::{BibleReferenceSearchResult, parse_single_reference};
/// use bibleref::referencing::language::BookReferenceType;
/// use bibleref::bible::{BibleBook, BibleReference, BibleVerseReference};
///
/// let bible_reference_search_result: BibleReferenceSearchResult = parse_single_reference("1. Mose 1,3".to_string()).unwrap();
/// assert_eq!(*bible_reference_search_result.bible_reference(), BibleReference::BibleVerse(BibleVerseReference::new(BibleBook::Genesis, 1, 3).unwrap()));
/// assert_eq!(bible_reference_search_result.language_code(), "de");
/// assert_eq!(*bible_reference_search_result.reference_type(), BookReferenceType::Long);
/// ```
pub fn parse_single_reference(
    reference: String,
) -> Result<BibleReferenceSearchResult, Box<dyn Error>> {
    if reference.is_empty() {
        return Err(Box::new(ReferenceIsEmptyError));
    }

    enum ParserFlag {
        Book,
        Chapter,
        Verse,
    }

    // We remove all spaces in the string as we don't need them
    let binding = reference.replace(" ", "");
    let reference = binding.trim();

    let mut reference_book_str: String = "".to_string();
    let mut reference_chapter_str: String = "".to_string();
    let mut reference_verse_str: String = "".to_string();

    let mut parser_flag: ParserFlag = ParserFlag::Book;

    for (i, c) in reference.chars().enumerate() {
        match parser_flag {
            ParserFlag::Book => {
                if i == 0 {
                    reference_book_str.push(c);
                } else if c.is_numeric() {
                    reference_chapter_str.push(c);
                    parser_flag = ParserFlag::Chapter;
                } else {
                    reference_book_str.push(c);
                }
            }
            ParserFlag::Chapter => {
                if c.is_numeric() {
                    reference_chapter_str.push(c);
                } else {
                    parser_flag = ParserFlag::Verse
                }
            }
            ParserFlag::Verse => {
                if c.is_numeric() {
                    reference_verse_str.push(c);
                }
            }
        }
    }

    let book_finding = find_book_in_any_language(&reference_book_str);

    match book_finding {
        None => Err(Box::new(BibleBookNotFoundError {
            provided_bible_book_string: reference_book_str.clone(),
        })),
        Some((bible_book, language, book_reference_type)) => {
            match (reference_chapter_str.len(), reference_verse_str.len()) {
                (0, 0) => Ok(BibleReferenceSearchResult::new(
                    BibleReference::BibleBook(BibleBookReference::new(bible_book)),
                    language,
                    book_reference_type,
                )),
                (0.., 0) => {
                    let chapter: u8 = reference_chapter_str.parse().unwrap();
                    match BibleChapterReference::new(bible_book, chapter) {
                        Ok(chapter_reference) => Ok(BibleReferenceSearchResult::new(
                            BibleReference::BibleChapter(chapter_reference),
                            language.clone(),
                            book_reference_type,
                        )),
                        Err(err) => Err(Box::new(err)),
                    }
                }
                (0.., 0..) => {
                    let chapter: u8 = reference_chapter_str.parse().unwrap();
                    let verse: u8 = reference_verse_str.parse().unwrap();

                    match BibleVerseReference::new(bible_book, chapter, verse) {
                        Ok(verse_reference) => Ok(BibleReferenceSearchResult::new(
                            BibleReference::BibleVerse(verse_reference),
                            language.clone(),
                            book_reference_type,
                        )),
                        Err(err) => Err(Box::new(err)),
                    }
                }
            }
        }
    }
}

/// Parses a range of Bible references.
///
/// # Arguments
/// - `range_reference`: A human readable Bible range reference.
/// # Returns
/// - A result with either a [BibleReferenceRepresentationSearchResult] or a [`Box<dyn Error>`] with an appropriate error message.
/// # Example
/// ```
/// use bibleref::referencing::parser::{BibleReferenceRepresentationSearchResult, parse_range_reference};
/// use bibleref::referencing::language::BookReferenceType;
/// use bibleref::bible::{BibleBook, BibleReference, BibleVerseReference};
/// use bibleref::bible::BibleRange;
/// use bibleref::bible::BibleReferenceRepresentation;
/// let bible_reference_representation_search_result: BibleReferenceRepresentationSearchResult = parse_range_reference("1. Mose 1,3-5".to_string()).unwrap();
/// assert_eq!(bible_reference_representation_search_result.language_code(), "de");
/// assert_eq!(*bible_reference_representation_search_result.reference_type(), BookReferenceType::Long);
/// assert_eq!(bible_reference_representation_search_result.bible_reference(), &BibleReferenceRepresentation::Range(BibleRange::new(
///     BibleReference::BibleVerse(BibleVerseReference::new(BibleBook::Genesis, 1, 3).unwrap()),
///     BibleReference::BibleVerse(BibleVerseReference::new(BibleBook::Genesis, 1, 5).unwrap())
/// ).unwrap()));
/// ```
/// # Errors
/// - [`ReferenceIsEmptyError`]: The provided Bible reference has been empty.
/// - [`BibleRangeParsingError::NoSecondPartProvided`]: The second part of the range reference is empty.
/// - [`BibleRangeParsingError::DelimiterNotFound`]: The delimiter between the two parts of the range reference is not found.
/// - [`BibleRangeParsingError::InvalidFirstPart`]: The first part of the range reference is invalid.
/// - [`BibleRangeParsingError::InvalidSecondPart`]: The second part of the range reference is invalid.
/// - [`LanguageHasNoChapterVersDelimiterError`]: The language has no chapter/verse delimiter.
pub fn parse_range_reference(
    range_reference: String,
) -> Result<BibleReferenceRepresentationSearchResult, Box<dyn Error>> {
    if range_reference.is_empty() {
        return Err(Box::new(ReferenceIsEmptyError));
    }

    // We remove all spaces in the string as we don't need them
    let binding = range_reference.replace(" ", "");
    let reference = binding.trim();

    // Traverse the string and try to get a reference out of it
    let mut current_part: String = "".to_string();
    let mut first_search_result_option: Option<BibleReferenceSearchResult> = None;

    for c in reference.chars() {
        current_part.push(c);

        if let Ok(reference) = parse_single_reference(current_part.clone()) {
            // We have found a valid reference
            first_search_result_option = Some(reference);
            // Now get the language code and the range delimiter
            let language =
                get_language_by_code(first_search_result_option.clone().unwrap().language_code())
                    .unwrap();
            // Split the current part by the range delimiter
            let parts: Vec<&str> = range_reference
                .split(language.range_delimiter.as_str())
                .collect();
            if parts.len() < 2 {
                return Err(Box::new(BibleRangeParsingError::DelimiterNotFound));
            }
            match parse_single_reference(parts[0].to_string()) {
                Ok(reference) => {
                    // We have found the first part of the range
                    let first_found_reference = reference.bible_reference().clone();
                    let chapter_verse_delimiter = match language.chapter_vers_delimiters.first() {
                        Some(delimiter) => delimiter,
                        None => {
                            return Err(Box::new(LanguageHasNoChapterVersDelimiterError {
                                language_code: language.language_code.clone(),
                            }));
                        }
                    };
                    match parse_second_range_part(
                        &first_found_reference,
                        chapter_verse_delimiter,
                        parts[1].to_string(),
                    ) {
                        Ok(second_found_reference) => {
                            // We have found the second part of the range
                            let range =
                                BibleRange::new(first_found_reference, second_found_reference)
                                    .unwrap();
                            return Ok(BibleReferenceRepresentationSearchResult::new(
                                BibleReferenceRepresentation::Range(range),
                                reference.language_code().clone(),
                                *reference.reference_type(),
                            ));
                        }
                        Err(error) => {
                            // The second part is invalid
                            return Err(error);
                        }
                    }
                }
                Err(_) => {
                    // The first part is invalid
                    return Err(Box::new(BibleRangeParsingError::InvalidFirstPart));
                }
            }
        }
    }

    dbg!(&first_search_result_option);

    Err(Box::new(BibleRangeParsingError::InvalidFirstPart))
}

/// Parses the second part of a range reference.
/// The second part could be a complete reference or just a chapter or verse number (e.g. "1" or "1,3").
/// # Arguments
/// - `first_part`: The first part of the range reference.
/// - `chapter_vers_delimiter`: The delimiter between the chapter and verse.
/// - `part_string`: The second part of the range reference.
/// # Returns
/// - A result with either a [BibleReference] or a [`Box<dyn Error>`] with an appropriate error message.
/// # Errors
/// - [`BibleRangeParsingError::InvalidSecondPart`]: The second part of the range reference is invalid.
/// - [`BibleRangeParsingError::NoSecondPartProvided`]: The second part of the range reference is empty.
fn parse_second_range_part(
    first_part: &BibleReference,
    chapter_vers_delimiter: &str,
    part_string: String,
) -> Result<BibleReference, Box<dyn Error>> {
    match parse_single_reference(part_string.clone()) {
        Ok(reference_search_result) => {
            // We have found a valid reference
            Ok(reference_search_result.bible_reference)
        }
        Err(_) => {
            // Try to split the part string by the chapter/verse delimiter
            let parts: Vec<&str> = part_string.split(chapter_vers_delimiter).collect();
            match parts.len() {
                2 => {
                    // Check that both parts are numeric
                    if !parts[0].chars().all(|c| c.is_numeric())
                        || !parts[1].chars().all(|c| c.is_numeric())
                    {
                        return Err(Box::new(BibleRangeParsingError::InvalidSecondPart));
                    }
                    // We have found numbers which are valid
                    let chapter: u8 = parts[0].parse().unwrap();
                    let verse: u8 = parts[1].parse().unwrap();
                    match first_part {
                        BibleReference::BibleVerse(reference) => {
                            // We have a verse reference, so we can create a new verse reference
                            Ok(BibleReference::BibleVerse(
                                BibleVerseReference::new(reference.book(), chapter, verse).unwrap(),
                            ))
                        }
                        BibleReference::BibleChapter(reference) => {
                            // We have a chapter reference, so we can create a new verse reference
                            Ok(BibleReference::BibleVerse(
                                BibleVerseReference::new(reference.book(), chapter, verse).unwrap(),
                            ))
                        }
                        BibleReference::BibleBook(reference) => {
                            // We have a book reference, so we can create a new chapter reference
                            Ok(BibleReference::BibleVerse(
                                BibleVerseReference::new(reference.book(), chapter, verse).unwrap(),
                            ))
                        }
                    }
                }
                1 => {
                    // We have only one part, so we can create a new chapter reference
                    let number: u8 = match parts[0].parse() {
                        Ok(n) => n,
                        Err(_) => return Err(Box::new(BibleRangeParsingError::InvalidSecondPart)),
                    };
                    match first_part {
                        BibleReference::BibleVerse(reference) => {
                            // We have a verse reference, so we can create a new chapter reference
                            Ok(BibleReference::BibleVerse(
                                BibleVerseReference::new(
                                    reference.book(),
                                    reference.chapter(),
                                    number,
                                )
                                .unwrap(),
                            ))
                        }
                        BibleReference::BibleChapter(reference) => {
                            // We have a chapter reference, so we can create a new chapter reference
                            Ok(BibleReference::BibleChapter(
                                BibleChapterReference::new(reference.book(), number).unwrap(),
                            ))
                        }
                        BibleReference::BibleBook(_) => {
                            // We have a book reference, so we can create a new chapter reference
                            Err(Box::new(BibleRangeParsingError::InvalidSecondPart))
                        }
                    }
                }
                _ => {
                    // We have more than two parts, so this is unvalid
                    Err(Box::new(BibleRangeParsingError::InvalidSecondPart))
                }
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

/// Finds a book in any known language
///
/// # Parameters
///
/// * `book_name`: The name of the book to find. This is not case-sensitive.
/// * `language`: The language in which the book name is written.
///
/// # Returns
///
/// An `Option` containing a tuple with the found `BibleBook`, the language code, and the reference type.
/// If no book is found, `None` is returned.
fn find_book_in_certain_language(
    book_name: &str,
    language: &ReferenceLanguage,
) -> Option<(BibleBook, String, BookReferenceType)> {
    for book in language.long_names.keys() {
        let language_long_space_removed_lowercase: Vec<String> = language.long_names[book]
            .iter()
            .map(|str| str.replace(" ", "").to_lowercase())
            .collect();
        if language_long_space_removed_lowercase.contains(&book_name.to_lowercase()) {
            return Some((
                *book,
                language.language_code.clone(),
                BookReferenceType::Long,
            ));
        }
        let language_short_space_removed_lowercase: Vec<String> = language.short_names[book]
            .iter()
            .map(|str| str.replace(" ", "").to_lowercase())
            .collect();
        if language_short_space_removed_lowercase.contains(&book_name.to_lowercase()) {
            return Some((
                *book,
                language.language_code.clone(),
                BookReferenceType::Short,
            ));
        }
    }

    None
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_reference_finding() {
        let reference = parse_single_reference("1. Mose 1,3".to_string()).unwrap();
        assert_eq!(
            *reference.bible_reference(),
            BibleReference::BibleVerse(BibleVerseReference::new(BibleBook::Genesis, 1, 3).unwrap())
        );
        assert_eq!(reference.language_code(), "de");
        assert_eq!(*reference.reference_type(), BookReferenceType::Long);

        let reference = parse_single_reference("Joh 3".to_string()).unwrap();

        assert_eq!(
            *reference.bible_reference(),
            BibleReference::BibleChapter(BibleChapterReference::new(BibleBook::John, 3).unwrap())
        );
        assert_eq!(reference.language_code(), "de");
        assert_eq!(*reference.reference_type(), BookReferenceType::Short);
    }

    #[test]
    fn test_range_parsing() {
        let range_reference = parse_range_reference("Johannes 4-6".to_string()).unwrap();
        assert_eq!(range_reference.language_code(), "de");
        assert_eq!(*range_reference.reference_type(), BookReferenceType::Long);
        assert_eq!(
            range_reference.bible_reference(),
            &BibleReferenceRepresentation::Range(
                BibleRange::new(
                    BibleReference::BibleChapter(
                        BibleChapterReference::new(BibleBook::John, 4).unwrap()
                    ),
                    BibleReference::BibleChapter(
                        BibleChapterReference::new(BibleBook::John, 6).unwrap()
                    )
                )
                .unwrap()
            )
        );

        let range_reference = parse_range_reference("3. Mose - 4. Mose 3".to_string()).unwrap();
        assert_eq!(range_reference.language_code(), "de");
        assert_eq!(*range_reference.reference_type(), BookReferenceType::Long);
        assert_eq!(
            range_reference.bible_reference(),
            &BibleReferenceRepresentation::Range(
                BibleRange::new(
                    BibleReference::BibleChapter(
                        BibleChapterReference::new(BibleBook::Leviticus, 1).unwrap()
                    ),
                    BibleReference::BibleChapter(
                        BibleChapterReference::new(BibleBook::Numbers, 3).unwrap()
                    )
                )
                .unwrap()
            )
        );
    }
}
