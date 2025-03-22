//! The Bible module includes the data structure around a Bible, including books, chapters and verses. 
//! It also handles the parsing process which determins the validity of Bible references.
//!
//! # General structure
//! There exists several *types* of Bible references:
//! - A Reference to a Bible book [BibleBookReference] is defined by the concerned book only (as it is the highest layer).
//! - A Reference to a Bible chapter [BibleChapterReference] is defined by the concerned book *and* the chapter of the book.
//! - A Reference to a Bible verse [BibleVerseReference] is defined by the book, the chapter and the verse.
//!
//! [BibleChapterReference]s and [BibleVerseReference]s could be invalid if the chapter and verse don't exist in the Bible book. To prevent the creation of *invalid* references, the structs must be created via the `new` functions which return an [Result<BibleChapterReference, BibleReferenceValidationError>] or a [Result<BibleVerseReference, BibleReferenceValidationError>]. If the validation fails (the reference does not exist in the Bible), the [BibleReferenceValidationError::problem] field contains detailed information about the failure.

/// Includes data types for vectors of Bible references
pub mod lists;

/// Includes helper functions for the validation of Bible references.
pub mod validate;

/// Includes errors which might occur during validation, creation or manipulation of Bible references
pub mod errors;

use serde::{Serialize, Deserialize};
use validate::*;

use self::errors::BibleReferenceValidationError;

/// This struct represents a valid Bible reference which consists of a book.
#[derive(PartialEq, PartialOrd, Serialize, Deserialize, Debug, Clone)]
pub struct BibleBookReference {
    book: BibleBook
}

impl BibleBookReference {
    pub fn new(book: BibleBook) -> Self {
        BibleBookReference { book }
    }
    
    pub fn book(&self) -> BibleBook { self.book }
}

/// This struct represents a Bible reference which is valid (can be found in a real Bible), consisting of a book and a chapter.
#[derive(PartialEq, PartialOrd, Serialize, Deserialize, Debug, Clone)]
pub struct BibleChapterReference {
    book: BibleBook,
    chapter: BibleChapter,
}

impl BibleChapterReference {
    
    /// Takes a given [BibleBook] and [BibleChapter] and returns an `Result<BibleVerseReference>`. If `BibleBook`, `Chapter` and `Verse` are an existing Bible reference (which can be found in the Bible), `Ok([BibleVerseReference])` will be returned. In any other case, a [BibleReferenceValidationError] will be returned.
    pub fn new(book: BibleBook, chapter: BibleChapter) -> Result<Self, BibleReferenceValidationError> {
        match validate_book_chapter(&book, &chapter) {
            Ok(_) => Ok(
                BibleChapterReference {
                    book,
                    chapter
                }
            ),
            Err(err) => Err(err)
        }
    }
    
    /// Returns the book of the BibleChapterReference
    pub fn book(&self) -> BibleBook {
        self.book
    }
    
    /// Returns the chapter of the BibleChapterReference    
    pub fn chapter(&self) -> BibleChapter {
        self.chapter
    }
}

/// This struct contains a Bible reference which is valid (can be found in a real Bible), consisting of a book, a chapter and a verse.
///
/// Please note the following: There are some differences concerning the number of verses of certain chapters depending on some Bible versions, e.g. in English Bible translations, Psalms may have one verse more as in most German translations–because the introduction words at the beginning of some Psalms are counted as a separate verse, while other translations might render them as the preface (or a verse 0). In this crate, we are always assuming the **maximum amount** of verses, so that all translations and versions can be used.
/// In the new testament, the Textus Receptus is used as template for determining the numbers of chapters and verses.
/// Some books (like the book of Jude) may only have one Chapter. Normally, in human languages people would only quote the verse and leave the chapter out (e.g. Jude 13)–however, this will be parsed as Jude 1:13 technically.
#[derive(PartialEq, PartialOrd, Serialize, Deserialize, Debug, Clone)]
pub struct BibleVerseReference {
    book: BibleBook,
    chapter: BibleChapter,
    verse: BibleVerse,
}

impl BibleVerseReference {
    /// Takes a given BibleBook, Chapter and Verse and returns an `Result<BibleVerseReference>`. If `BibleBook`, `Chapter` and `Verse` are an existing Bible reference (which can be found in the Bible), `Ok(BibleVerseReference)` will be returned. In any other case, a [BibleReferenceValidationError] will be returned.
    pub fn new(book: BibleBook, chapter: BibleChapter, verse: BibleVerse) -> Result<Self, errors::BibleReferenceValidationError> {
        match validate_book_chapter_verse(&book, &chapter, &verse) {
            Ok(_) => Ok(
                BibleVerseReference {
                    book,
                    chapter,
                    verse,
                }
            ),
            Err(error) => Err(error)
        }
    }
    
    /// Returns the book of the BibleVerseReference
    pub fn book(&self) -> BibleBook {
        self.book
    }
    
    /// Returns the chapter of the BibleVerseReference    
    pub fn chapter(&self) -> BibleChapter {
        self.chapter
    }
    
    /// Returns the verse of the BibleVerseReference    
    pub fn verse(&self) -> BibleVerse {
        self.verse
    }
    
}

/// This enum represents *any* Bible reference (a book, a chapter or a verse)
#[derive(PartialEq, PartialOrd, Deserialize, Debug, Clone)]
pub enum BibleReference {
    BibleBook(BibleBookReference),
    BibleChapter(BibleChapterReference),
    BibleVerse(BibleVerseReference)
}

/// The struct BibleBook contains all books of the Bible in their correct order. As it derives from `PartialOrd` and `PartialEq`, you can make comparisons like `<` or `>` to determine whether a book is before or after an other.
#[derive(PartialEq, PartialOrd, Eq, Serialize, Deserialize, Debug, Copy, Clone, Hash)]
pub enum BibleBook {
    Genesis,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,
    Joshua,
    Judges,
    Ruth,
    ISamuel,
    IISamuel,
    IKings,
    IIKings,
    IChronicles,
    IIChronicles,
    Ezra,
    Nehemiah,
    Esther,
    Job,
    Psalm,
    Proverbs,
    Ecclesiastes,
    SongofSolomon,
    Isaiah,
    Jeremiah,
    Lamentations,
    Ezekiel,
    Daniel,
    Hosea,
    Joel,
    Amos,
    Obadiah,
    Jonah,
    Micah,
    Nahum,
    Habakkuk,
    Zephaniah,
    Haggai,
    Zechariah,
    Malachi,
    Matthew,
    Mark,
    Luke,
    John,
    Acts,
    Romans,
    ICorinthians,
    IICorinthians,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    IThessalonians,
    IIThessalonians,
    ITimothy,
    IITimothy,
    Titus,
    Philemon,
    Hebrews,
    James,
    IPeter,
    IIPeter,
    IJohn,
    IIJohn,
    IIIJohn,
    Jude,
    Revelation    
}

impl BibleBook {
    /// This function determines whether the current Bible book is part of the Old Testament.
    /// # Parameters
    /// - No parameter
    /// # Returns
    /// `true` if the book is part of the Old Testament, `false` if it is part of the New Testament.
    /// # Note
    /// This function is per definition the inverse of `is_new_testament`.
    pub fn is_old_testament(&self) -> bool {
        self < &BibleBook::Matthew
    }
    
    /// This function determines whether the current Bible book is part of the New Testament.
    /// # Parameters
    /// - No parameter
    /// # Returns
    /// `true` if the book is part of the New Testament, `false` if it is part of the Old Testament.
    /// # Note
    /// This function is per definition the inverse of `is_old_testament`.
    pub fn is_new_testament(&self) -> bool {
        self >= &BibleBook::Matthew
    }
}

/// An unsigned positive number which represents the chapter of a Bible reference
pub type BibleChapter = u8;

/// An unsigned positive number which represents the verse of a Bible reference
pub type BibleVerse = u8;


#[cfg(test)]
mod tests {
    use super::*;    
    
    #[test]
    fn test_book_ot_nt() {
        assert!(BibleBook::Malachi.is_old_testament());
        assert!(BibleBook::Matthew.is_new_testament());
        assert!(BibleBook::Genesis.is_old_testament());
        assert!(BibleBook::Revelation.is_new_testament());
    }
    
    #[test]
    fn test_bibleversereference_creation() {
        let bibleref = BibleVerseReference::new(
            BibleBook::Matthew,
            11,
            28
        );
        assert!(bibleref.is_ok());
        
        let bibleref = BibleVerseReference::new(
            BibleBook::Revelation,
            23,
            8
        );
        assert!(bibleref.is_err());
    }
    
    #[test]
    fn test_biblechapterreference_creation() {
        let bibleref = BibleChapterReference::new(BibleBook::Genesis, 1);
        assert!(bibleref.is_ok());
        
        let bibleref = BibleChapterReference::new(BibleBook::Ruth, 0);
        assert!(bibleref.is_err());
    }
}
