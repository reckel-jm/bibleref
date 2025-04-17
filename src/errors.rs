//! The crate contains several error types that are used to represent errors that occur when working with Bible references.
//!
//! - The module [crate::bible::errors] contains errors that can occur when working with the Bible reference data types.
//! - The module [crate::referencing::errors] contains errors that can occur when parsing Bible references into or from human languages.
//!
//! The public functions of the crate return these errors as a [`Box<dyn Error>`] which can be downcasted to the specific error type.
//!
//! # Example
//! ```
//! # use bibleref::parse;
//! # use bibleref::referencing::errors::ReferenceIsEmptyError;
//! # use bibleref::bible::errors::{BibleReferenceValidationError, BibleReferenceProblem};
//! // Exodus 3 exists and is a valid Bible reference (of type BibleChapter), therefore no error is returned.
//! assert!(parse("Exodus 3").is_ok());
//! // The same applies to John 3:16.
//! assert!(parse("John 3:16").is_ok());
//!
//! // Revelation 24 does not exist and therefore is not a valid Bible reference, an error will be returned.
//! assert!(parse("Revelation 24").is_err());
//!
//! // You can display the error message as a string (in English)
//! assert!(parse("Revelation 24").err().unwrap().to_string().contains("The chapter does not exist"));
//!
//! // Or you downcast the error to the specific error type.
//! // In this case, the error is a [BibleReferenceValidationError] which contains more information about the problem in the field `problem` as a [BibleReferenceProblem].
//! assert_eq!(
//!     parse("Revelation 24")
//!     .err().unwrap().downcast_ref::<BibleReferenceValidationError>().unwrap()
//!     .problem,
//!     BibleReferenceProblem::ChapterDoesNotExist
//! );
//!
//! // An empty string is not a valid Bible reference, therefore a [ReferenceIsEmptyError] will be returned.
//! assert!(parse("").err().unwrap().downcast_ref::<ReferenceIsEmptyError>().is_some());
//! ```
