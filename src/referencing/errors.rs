//! This module contains errors which occur while parsing Bible references into human languages.

use std::fmt::{Display, Formatter};
use std::error::Error;

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


#[derive(Debug)]
pub struct ReferenceIsEmptyError;

impl Display for ReferenceIsEmptyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The provided Bible reference has been empty.")
    }
}
impl Error for ReferenceIsEmptyError {}

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
