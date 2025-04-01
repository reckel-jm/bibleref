//! This module contains several error types which might occur during the process of creating and manipulating Bible references.

use std::fmt::{Display, Formatter};
use std::error::Error;

/// The [BibleReferenceValidationError] will be thrown in case of an error during a validation of a BibleReference, which means that the Bible reference does not exist because the chapter or verse of the reference are not in the Bible. 
/// The field `problem` contains more information about the problem which caused the error.
#[derive(PartialEq, Debug)]
pub struct BibleReferenceValidationError {
    /// The actual problem which caused the [BibleReferenceValidationError]
    pub problem: BibleReferenceProblem
}
impl Display for BibleReferenceValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "The Bible reference is invalid: {}",
            match self.problem {
                BibleReferenceProblem::ChapterDoesNotExist => "The chapter does not exist",
                BibleReferenceProblem::VerseDoesNotExist => "The verse does not exist"
            }
        )
    }
}
impl Error for BibleReferenceValidationError {}

/// Provides more details about the problem during the Bible reference validation which lead to a [BibleReferenceValidationError].
#[derive(PartialEq, Debug)]
pub enum BibleReferenceProblem {
    /// Indicates that the chapter of the Bible reference does not exist in the book
    ChapterDoesNotExist,
    /// Indicates that book and chapter do, but the verse does not exist.
    VerseDoesNotExist,
}

/// The [BibleRangeReferenceValidationError] will be thrown in case of an error during a validation of a BibleRangeReference, which can have multible causes. 
/// The field `problem` contains more information about the problem which caused the error.
#[derive(PartialEq, Debug)]
pub struct BibleRangeReferenceValidationError {
    /// The problem which occurs while parsing the reference
    pub problem: BibleRangeReferenceValidationProblem,
}

impl BibleRangeReferenceValidationError {
    pub fn new(problem: BibleRangeReferenceValidationProblem) -> Self {
        return BibleRangeReferenceValidationError {
            problem
        }
    }
}

/// Represents a problem which occurs while creating a BibleRangeReference
#[derive(PartialEq, Debug)]
pub enum BibleRangeReferenceValidationProblem {
    /// Indicates that the provided start book is after the end book (e.g. Jonah after Genesis)
    BookStartAfterEnd,
    /// Indicator of a problem with the start value
    ChapterStart(BibleReferenceProblem),
    /// Indicator of a problem with the end value
    ChapterEndProblem(BibleReferenceProblem),
    /// Indicats that the provided chapter is after the end value (which is not allowed)
    ChapterStartAfterEnd,
    /// Indicator of a problem with the start value of the verse
    VerseStart(BibleReferenceProblem),
    /// Indicator of a problem with the end value of a verse
    VerseEndProblem(BibleReferenceProblem),
    /// Indicats that the provided verse is after the end value (which is not allowed)
    VerseStartAfterEnd,
}