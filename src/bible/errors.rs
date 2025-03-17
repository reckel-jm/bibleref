//! This module contains several error types which might occur during the process of creating and manipulating Bible references.

/// The [BibleReferenceValidationError] will be thrown in case of an error during a validation of a BibleReference, which means that the Bible reference does not exist because the chapter or verse of the reference are not in the Bible. 
/// The field `problem` contains more information about the problem which caused the error.
#[derive(PartialEq, Debug)]
pub struct BibleReferenceValidationError {
    /// The actual problem which caused the [BibleReferenceValidationError]
    pub problem: BibleReferenceProblem
}

/// Provides more details about the problem during the Bible reference validation which lead to a [BibleReferenceValidationError].
#[derive(PartialEq, Debug)]
pub enum BibleReferenceProblem {
    /// Indicates that the chapter of the Bible reference does not exist in the book
    ChapterDoesNotExist,
    /// Indicates that book and chapter do, but the verse does not exist.
    VerseDoesNotExist,
}