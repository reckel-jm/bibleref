//! This module contains errors which occur while parsing Bible references into human languages.

use std::fmt::{Display, Formatter};
use std::error::Error;

/// This error is returned when no [ReferenceLanguage](crate::referencing::language::ReferenceLanguage) for a given language code exists in [REFERENCE_LANGUAGES](crate::referencing::language::REFERENCE_LANGUAGES).
#[derive(Debug)]
pub struct LanguageDoesNotExistError {
    pub language_code: String
}

impl Display for LanguageDoesNotExistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The language with language code '{}' is unknown.", self.language_code)
    }
}
impl Error for LanguageDoesNotExistError {}

/// This error is returned when a [ReferenceLanguage](crate::referencing::language::ReferenceLanguage) does not have a chapter/verse delimiter.
/// This must not be confused with the [ReferenceLanguage](crate::referencing::language::ReferenceLanguage) not existing and means that the language is known but wrongly configured.
#[derive(Debug)]
pub struct LanguageHasNoChapterVersDelimiterError {
    pub language_code: String
}
impl Display for LanguageHasNoChapterVersDelimiterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The language with language code '{}' has no chapter/verse delimiter.", self.language_code)
    }
}
impl Error for LanguageHasNoChapterVersDelimiterError {}

/// This error is returned when a given reference string is empty after it has been trimmed.
#[derive(Debug)]
pub struct ReferenceIsEmptyError;

impl Display for ReferenceIsEmptyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The provided Bible reference has been empty.")
    }
}
impl Error for ReferenceIsEmptyError {}

/// This error is returned if a given bible book string is not found in any of the [languages](crate::referencing::language::ReferenceLanguage) provided in [REFERENCE_LANGUAGES](crate::referencing::language::REFERENCE_LANGUAGES).
#[derive(Debug)]
pub struct BibleBookNotFoundError {
    pub provided_bible_book_string: String
}

impl std::fmt::Display for BibleBookNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The Bible book {} found in any provided language.", 
        self.provided_bible_book_string)
    }
}
impl Error for BibleBookNotFoundError {}

/// This error is returned when a [BibleRange](crate::bible::BibleRange) cannot be parsed from a given string.
#[derive(Debug)]
pub enum BibleRangeParsingError {
    /// The first part of the range is invalid and can not be parsed into a [BibleReference](crate::bible::BibleReference).
    InvalidFirstPart,

    /// The second part of the range is invalid and can not be parsed into a [BibleReference](crate::bible::BibleReference).
    InvalidSecondPart,

    /// The second part of the range is missing in the provided string.
    NoSecondPartProvided,

    /// The delimiter between the first and second part of the [BibleRange](crate::bible::BibleRange) cannot be found.
    DelimiterNotFound,
}
impl Display for BibleRangeParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BibleRangeParsingError::InvalidFirstPart => write!(f, "The first part of the Bible range is invalid."),
            BibleRangeParsingError::InvalidSecondPart => write!(f, "The second part of the Bible range is invalid."),
            BibleRangeParsingError::NoSecondPartProvided => write!(f, "The second part of the Bible range is missing."),
            BibleRangeParsingError::DelimiterNotFound => write!(f, "The delimiter between the first and second part of the Bible range is missing."),
        }
    }
}

impl std::error::Error for BibleRangeParsingError { }