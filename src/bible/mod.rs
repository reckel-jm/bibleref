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

use std::cmp::Ordering;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use lists::{BibleBookList, BibleChapterList, BibleReferenceList, BibleVerseList};

use validate::*;

use self::errors::BibleReferenceValidationError;

/// This struct represents a valid Bible reference which consists of a book.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BibleBookReference {
    book: BibleBook,
}

impl BibleBookReference {
    pub fn new(book: BibleBook) -> Self {
        BibleBookReference { book }
    }

    pub fn book(&self) -> BibleBook {
        self.book
    }
}

/// This struct represents a Bible reference which is valid (can be found in a real Bible), consisting of a book and a chapter.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BibleChapterReference {
    book: BibleBook,
    chapter: BibleChapter,
}

impl BibleChapterReference {
    /// Takes a given [BibleBook] and [BibleChapter] and returns an `Result<BibleVerseReference>`. If `BibleBook`, `Chapter` and `Verse` are an existing Bible reference (which can be found in the Bible), `Ok([BibleVerseReference])` will be returned. In any other case, a [BibleReferenceValidationError] will be returned.
    pub fn new(
        book: BibleBook,
        chapter: BibleChapter,
    ) -> Result<Self, BibleReferenceValidationError> {
        match validate_book_chapter(&book, &chapter) {
            Ok(_) => Ok(BibleChapterReference { book, chapter }),
            Err(err) => Err(err),
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BibleVerseReference {
    book: BibleBook,
    chapter: BibleChapter,
    verse: BibleVerse,
}

impl BibleVerseReference {
    /// Takes a given BibleBook, Chapter and Verse and returns an `Result<BibleVerseReference>`. If `BibleBook`, `Chapter` and `Verse` are an existing Bible reference (which can be found in the Bible), `Ok(BibleVerseReference)` will be returned. In any other case, a [BibleReferenceValidationError] will be returned.
    pub fn new(
        book: BibleBook,
        chapter: BibleChapter,
        verse: BibleVerse,
    ) -> Result<Self, errors::BibleReferenceValidationError> {
        match validate_book_chapter_verse(&book, &chapter, &verse) {
            Ok(_) => Ok(BibleVerseReference {
                book,
                chapter,
                verse,
            }),
            Err(error) => Err(error),
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

/// This enum represents all possible representations of one or multiple Bible references.
/// It can be a reference to a book, a chapter or a verse. It can also be a range of books, chapters or verses or to a list of books, chapters or verses.
#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BibleReferenceRepresentation {
    /// A single Bible reference
    Single(BibleReference),

    /// A range of Bible references
    Range(BibleRange),
}

impl BibleReferenceRepresentation {
    /// Returns true if the representation is a single reference
    pub fn is_single(&self) -> bool {
        matches!(self, BibleReferenceRepresentation::Single(_))
    }

    /// Returns true if the representation is a range reference
    pub fn is_range(&self) -> bool {
        matches!(self, BibleReferenceRepresentation::Range(_))
    }

    pub fn try_upcast(&self) -> BibleReferenceRepresentation {
        match self {
            BibleReferenceRepresentation::Single(_) => self.clone(),
            BibleReferenceRepresentation::Range(BibleRange::VerseRange(verse_range)) => {
                match verse_range.as_single_verse() {
                    Some(verse) => {
                        BibleReferenceRepresentation::Single(BibleReference::BibleVerse(verse))
                    }
                    None => match verse_range.as_chapter_range() {
                        Some(chapter) => {
                            BibleReferenceRepresentation::Range(BibleRange::ChapterRange(chapter))
                                .try_upcast()
                        }
                        None => self.clone(),
                    },
                }
            }
            BibleReferenceRepresentation::Range(BibleRange::ChapterRange(chapter_range)) => {
                match chapter_range.as_single_chapter() {
                    Some(chapter) => {
                        BibleReferenceRepresentation::Single(BibleReference::BibleChapter(chapter))
                    }
                    None => match chapter_range.as_book_range() {
                        Some(book_range) => {
                            BibleReferenceRepresentation::Range(BibleRange::BookRange(book_range))
                                .try_upcast()
                        }
                        None => self.clone(),
                    },
                }
            }
            BibleReferenceRepresentation::Range(BibleRange::BookRange(book_range)) => {
                match book_range.as_single_book() {
                    Some(book) => {
                        BibleReferenceRepresentation::Single(BibleReference::BibleBook(book))
                    }
                    None => self.clone(),
                }
            }
        }
    }
}

impl Ord for BibleReferenceRepresentation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (BibleReferenceRepresentation::Single(a), BibleReferenceRepresentation::Single(b)) => {
                a.cmp(b)
            }
            (BibleReferenceRepresentation::Range(a), BibleReferenceRepresentation::Range(b)) => {
                a.cmp(b)
            }

            (BibleReferenceRepresentation::Single(a), BibleReferenceRepresentation::Range(b)) => {
                match a {
                    BibleReference::BibleBook(a) => {
                        let a_ref = BibleReference::BibleBook(a.clone());
                        a_ref.cmp(&b.end())
                    }
                    BibleReference::BibleChapter(a) => {
                        let a_ref = BibleReference::BibleChapter(a.clone());
                        a_ref.cmp(&b.end())
                    }
                    BibleReference::BibleVerse(a) => {
                        let a_ref = BibleReference::BibleVerse(a.clone());
                        a_ref.cmp(&b.end())
                    }
                }
            }
            (BibleReferenceRepresentation::Range(a), BibleReferenceRepresentation::Single(b)) => {
                match b {
                    BibleReference::BibleBook(b) => {
                        let b_ref = BibleReference::BibleBook(b.clone());
                        a.end().cmp(&b_ref)
                    }
                    BibleReference::BibleChapter(b) => {
                        let b_ref = BibleReference::BibleChapter(b.clone());
                        a.end().cmp(&b_ref)
                    }
                    BibleReference::BibleVerse(b) => {
                        let b_ref = BibleReference::BibleVerse(b.clone());
                        a.end().cmp(&b_ref)
                    }
                }
            }
        }
    }
}

impl PartialOrd for BibleReferenceRepresentation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// This enum represents *any* single Bible reference (one book, one chapter or one verse)
#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BibleReference {
    BibleBook(BibleBookReference),
    BibleChapter(BibleChapterReference),
    BibleVerse(BibleVerseReference),
}

impl BibleReference {
    /// Returns the next Bible reference of the current reference. If the current reference is a book, the next reference will be the first chapter of the next book. If the current reference is a chapter, the next reference will be the first verse of the next chapter. If the current reference is a verse, the next reference will be the next verse in the same chapter.
    /// # Returns
    /// - An Option with the next Bible reference as a [BibleReference] if it exists, or None if it does not exist.
    pub fn next(&self) -> Option<BibleReference> {
        match self {
            BibleReference::BibleBook(book) => {
                let next_book = get_bible_book_by_number(book.book.number() + 1);
                next_book
                    .map(|next_book| BibleReference::BibleBook(BibleBookReference::new(next_book)))
            }
            BibleReference::BibleChapter(chapter) => {
                let next_chapter = chapter.chapter + 1;
                match BibleChapterReference::new(chapter.book(), next_chapter) {
                    Ok(next_chapter) => Some(BibleReference::BibleChapter(next_chapter)),
                    Err(_) => {
                        let next_book = get_bible_book_by_number(chapter.book().number() + 1);
                        next_book.map(|next_book| {
                            BibleReference::BibleBook(BibleBookReference::new(next_book))
                        })
                    }
                }
            }
            BibleReference::BibleVerse(verse) => {
                let next_verse = verse.verse + 1;
                match BibleVerseReference::new(verse.book(), verse.chapter(), next_verse) {
                    Ok(next_verse) => Some(BibleReference::BibleVerse(next_verse)),
                    Err(_) => {
                        let next_chapter = verse.chapter + 1;
                        match BibleChapterReference::new(verse.book(), next_chapter) {
                            Ok(next_chapter) => Some(BibleReference::BibleChapter(next_chapter)),
                            Err(_) => {
                                let next_book = get_bible_book_by_number(verse.book().number() + 1);
                                next_book.map(|next_book| {
                                    BibleReference::BibleBook(BibleBookReference::new(next_book))
                                })
                            }
                        }
                    }
                }
            }
        }
    }

    /// Returns the previous Bible reference of the current reference. If the current reference is a book, the previous reference will be the last chapter of the previous book. If the current reference is a chapter, the previous reference will be the last verse of the previous chapter. If the current reference is a verse, the previous reference will be the previous verse in the same chapter.
    /// # Returns
    /// - An Option with the previous Bible reference as a [BibleReference] if it exists, or None if it does not exist.
    pub fn previous(&self) -> Option<BibleReference> {
        match self {
            BibleReference::BibleBook(book) => {
                let previous_book = get_bible_book_by_number(book.book.number() - 1);
                previous_book.map(|previous_book| {
                    BibleReference::BibleBook(BibleBookReference::new(previous_book))
                })
            }
            BibleReference::BibleChapter(chapter) => {
                let previous_chapter = chapter.chapter - 1;
                match BibleChapterReference::new(chapter.book(), previous_chapter) {
                    Ok(previous_chapter) => Some(BibleReference::BibleChapter(previous_chapter)),
                    Err(_) => {
                        let previous_book = get_bible_book_by_number(chapter.book().number() - 1);
                        previous_book.map(|previous_book| {
                            BibleReference::BibleBook(BibleBookReference::new(previous_book))
                        })
                    }
                }
            }
            BibleReference::BibleVerse(verse) => {
                let previous_verse = verse.verse - 1;
                match BibleVerseReference::new(verse.book(), verse.chapter(), previous_verse) {
                    Ok(previous_verse) => Some(BibleReference::BibleVerse(previous_verse)),
                    Err(_) => {
                        let previous_chapter = verse.chapter - 1;
                        match BibleChapterReference::new(verse.book(), previous_chapter) {
                            Ok(previous_chapter) => {
                                Some(BibleReference::BibleChapter(previous_chapter))
                            }
                            Err(_) => {
                                let previous_book =
                                    get_bible_book_by_number(verse.book().number() - 1);
                                previous_book.map(|previous_book| {
                                    BibleReference::BibleBook(BibleBookReference::new(
                                        previous_book,
                                    ))
                                })
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Ord for BibleReference {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (BibleReference::BibleBook(a), BibleReference::BibleBook(b)) => a.cmp(b),
            (BibleReference::BibleChapter(a), BibleReference::BibleChapter(b)) => a.cmp(b),
            (BibleReference::BibleVerse(a), BibleReference::BibleVerse(b)) => a.cmp(b),
            (BibleReference::BibleBook(a), BibleReference::BibleChapter(b)) => {
                if a.book == b.book {
                    Ordering::Less
                } else {
                    a.book().number().cmp(&b.book.number())
                }
            }
            (BibleReference::BibleChapter(a), BibleReference::BibleBook(b)) => {
                if a.book == b.book {
                    Ordering::Greater
                } else {
                    a.book().number().cmp(&b.book.number())
                }
            }
            (BibleReference::BibleBook(a), BibleReference::BibleVerse(b)) => {
                if a.book == b.book {
                    Ordering::Less
                } else {
                    a.book().number().cmp(&b.book.number())
                }
            }
            (BibleReference::BibleVerse(a), BibleReference::BibleBook(b)) => {
                if a.book == b.book {
                    Ordering::Greater
                } else {
                    a.book().number().cmp(&b.book.number())
                }
            }
            (BibleReference::BibleChapter(a), BibleReference::BibleVerse(b)) => {
                if a.book == b.book && a.chapter == b.chapter {
                    Ordering::Less
                } else {
                    match a.book().number().cmp(&b.book.number()) {
                        Ordering::Equal => a.chapter.cmp(&b.chapter),
                        other => other,
                    }
                }
            }
            (BibleReference::BibleVerse(a), BibleReference::BibleChapter(b)) => {
                if a.book == b.book && a.chapter == b.chapter {
                    Ordering::Greater
                } else {
                    match a.book().number().cmp(&b.book.number()) {
                        Ordering::Equal => a.chapter.cmp(&b.chapter),
                        other => other,
                    }
                }
            }
        }
    }
}

impl PartialOrd for BibleReference {
    fn partial_cmp(&self, other: &BibleReference) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The struct BibleBook contains all books of the Bible in their correct order. As it derives from `PartialOrd` and `PartialEq`, you can make comparisons like `<` or `>` to determine whether a book is before or after an other.
#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    Revelation,
}

impl BibleBook {
    /// Gets a [Vec] with all books of the Bible
    pub fn all() -> Vec<Self> {
        vec![
            Self::Genesis,
            Self::Exodus,
            Self::Leviticus,
            Self::Numbers,
            Self::Deuteronomy,
            Self::Joshua,
            Self::Judges,
            Self::Ruth,
            Self::ISamuel,
            Self::IISamuel,
            Self::IKings,
            Self::IIKings,
            Self::IChronicles,
            Self::IIChronicles,
            Self::Ezra,
            Self::Nehemiah,
            Self::Esther,
            Self::Job,
            Self::Psalm,
            Self::Proverbs,
            Self::Ecclesiastes,
            Self::SongofSolomon,
            Self::Isaiah,
            Self::Jeremiah,
            Self::Lamentations,
            Self::Ezekiel,
            Self::Daniel,
            Self::Hosea,
            Self::Joel,
            Self::Amos,
            Self::Obadiah,
            Self::Jonah,
            Self::Micah,
            Self::Nahum,
            Self::Habakkuk,
            Self::Zephaniah,
            Self::Haggai,
            Self::Zechariah,
            Self::Malachi,
            Self::Matthew,
            Self::Mark,
            Self::Luke,
            Self::John,
            Self::Acts,
            Self::Romans,
            Self::ICorinthians,
            Self::IICorinthians,
            Self::Galatians,
            Self::Ephesians,
            Self::Philippians,
            Self::Colossians,
            Self::IThessalonians,
            Self::IIThessalonians,
            Self::ITimothy,
            Self::IITimothy,
            Self::Titus,
            Self::Philemon,
            Self::Hebrews,
            Self::James,
            Self::IPeter,
            Self::IIPeter,
            Self::IJohn,
            Self::IIJohn,
            Self::IIIJohn,
            Self::Jude,
            Self::Revelation,
        ]
    }

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

    /// This function returns the number of the book which it has in the Bible
    /// # Example
    /// ```
    /// use bibleref::bible::BibleBook;
    /// assert_eq!(BibleBook::Genesis.number(), 1);
    /// assert_eq!(BibleBook::Exodus.number(), 2);
    /// assert_eq!(BibleBook::Leviticus.number(), 3);
    /// assert_eq!(BibleBook::Numbers.number(), 4);
    /// assert_eq!(BibleBook::Deuteronomy.number(), 5);
    /// assert_eq!(BibleBook::Matthew.number(), 40);
    /// ```
    pub fn number(&self) -> u8 {
        match self {
            BibleBook::Genesis => 1,
            BibleBook::Exodus => 2,
            BibleBook::Leviticus => 3,
            BibleBook::Numbers => 4,
            BibleBook::Deuteronomy => 5,
            BibleBook::Joshua => 6,
            BibleBook::Judges => 7,
            BibleBook::Ruth => 8,
            BibleBook::ISamuel => 9,
            BibleBook::IISamuel => 10,
            BibleBook::IKings => 11,
            BibleBook::IIKings => 12,
            BibleBook::IChronicles => 13,
            BibleBook::IIChronicles => 14,
            BibleBook::Ezra => 15,
            BibleBook::Nehemiah => 16,
            BibleBook::Esther => 17,
            BibleBook::Job => 18,
            BibleBook::Psalm => 19,
            BibleBook::Proverbs => 20,
            BibleBook::Ecclesiastes => 21,
            BibleBook::SongofSolomon => 22,
            BibleBook::Isaiah => 23,
            BibleBook::Jeremiah => 24,
            BibleBook::Lamentations => 25,
            BibleBook::Ezekiel => 26,
            BibleBook::Daniel => 27,
            BibleBook::Hosea => 28,
            BibleBook::Joel => 29,
            BibleBook::Amos => 30,
            BibleBook::Obadiah => 31,
            BibleBook::Jonah => 32,
            BibleBook::Micah => 33,
            BibleBook::Nahum => 34,
            BibleBook::Habakkuk => 35,
            BibleBook::Zephaniah => 36,
            BibleBook::Haggai => 37,
            BibleBook::Zechariah => 38,
            BibleBook::Malachi => 39,
            BibleBook::Matthew => 40,
            BibleBook::Mark => 41,
            BibleBook::Luke => 42,
            BibleBook::John => 43,
            BibleBook::Acts => 44,
            BibleBook::Romans => 45,
            BibleBook::ICorinthians => 46,
            BibleBook::IICorinthians => 47,
            BibleBook::Galatians => 48,
            BibleBook::Ephesians => 49,
            BibleBook::Philippians => 50,
            BibleBook::Colossians => 51,
            BibleBook::IThessalonians => 52,
            BibleBook::IIThessalonians => 53,
            BibleBook::ITimothy => 54,
            BibleBook::IITimothy => 55,
            BibleBook::Titus => 56,
            BibleBook::Philemon => 57,
            BibleBook::Hebrews => 58,
            BibleBook::James => 59,
            BibleBook::IPeter => 60,
            BibleBook::IIPeter => 61,
            BibleBook::IJohn => 62,
            BibleBook::IIJohn => 63,
            BibleBook::IIIJohn => 64,
            BibleBook::Jude => 65,
            BibleBook::Revelation => 66,
        }
    }
}

/// This function returns a Bible book by its number. The number is the number of the book in the Bible (1-66).
/// # Parameters
/// - `number`: The number of the book in the Bible (1-66)
/// # Returns
/// - an `Option<BibleBook>` which contains the book if it exists, or `None` if it does not exist.
/// # Note
/// This function is the inverse of `BibleBook::number()`.
/// # Example
/// ```
/// use bibleref::bible::get_bible_book_by_number;
/// use bibleref::bible::BibleBook;
/// assert_eq!(get_bible_book_by_number(1), Some(BibleBook::Genesis));
/// assert_eq!(get_bible_book_by_number(2), Some(BibleBook::Exodus));
/// assert_eq!(get_bible_book_by_number(3), Some(BibleBook::Leviticus));
/// assert_eq!(get_bible_book_by_number(4), Some(BibleBook::Numbers));
/// assert_eq!(get_bible_book_by_number(5), Some(BibleBook::Deuteronomy));
/// assert_eq!(get_bible_book_by_number(6), Some(BibleBook::Joshua));
/// assert_eq!(get_bible_book_by_number(7), Some(BibleBook::Judges));
/// ```
pub fn get_bible_book_by_number(number: u8) -> Option<BibleBook> {
    match number {
        1 => Some(BibleBook::Genesis),
        2 => Some(BibleBook::Exodus),
        3 => Some(BibleBook::Leviticus),
        4 => Some(BibleBook::Numbers),
        5 => Some(BibleBook::Deuteronomy),
        6 => Some(BibleBook::Joshua),
        7 => Some(BibleBook::Judges),
        8 => Some(BibleBook::Ruth),
        9 => Some(BibleBook::ISamuel),
        10 => Some(BibleBook::IISamuel),
        11 => Some(BibleBook::IKings),
        12 => Some(BibleBook::IIKings),
        13 => Some(BibleBook::IChronicles),
        14 => Some(BibleBook::IIChronicles),
        15 => Some(BibleBook::Ezra),
        16 => Some(BibleBook::Nehemiah),
        17 => Some(BibleBook::Esther),
        18 => Some(BibleBook::Job),
        19 => Some(BibleBook::Psalm),
        20 => Some(BibleBook::Proverbs),
        21 => Some(BibleBook::Ecclesiastes),
        22 => Some(BibleBook::SongofSolomon),
        23 => Some(BibleBook::Isaiah),
        24 => Some(BibleBook::Jeremiah),
        25 => Some(BibleBook::Lamentations),
        26 => Some(BibleBook::Ezekiel),
        27 => Some(BibleBook::Daniel),
        28 => Some(BibleBook::Hosea),
        29 => Some(BibleBook::Joel),
        30 => Some(BibleBook::Amos),
        31 => Some(BibleBook::Obadiah),
        32 => Some(BibleBook::Jonah),
        33 => Some(BibleBook::Micah),
        34 => Some(BibleBook::Nahum),
        35 => Some(BibleBook::Habakkuk),
        36 => Some(BibleBook::Zephaniah),
        37 => Some(BibleBook::Haggai),
        38 => Some(BibleBook::Zechariah),
        39 => Some(BibleBook::Malachi),
        40 => Some(BibleBook::Matthew),
        41 => Some(BibleBook::Mark),
        42 => Some(BibleBook::Luke),
        43 => Some(BibleBook::John),
        44 => Some(BibleBook::Acts),
        45 => Some(BibleBook::Romans),
        46 => Some(BibleBook::ICorinthians),
        47 => Some(BibleBook::IICorinthians),
        48 => Some(BibleBook::Galatians),
        49 => Some(BibleBook::Ephesians),
        50 => Some(BibleBook::Philippians),
        51 => Some(BibleBook::Colossians),
        52 => Some(BibleBook::IThessalonians),
        53 => Some(BibleBook::IIThessalonians),
        54 => Some(BibleBook::ITimothy),
        55 => Some(BibleBook::IITimothy),
        56 => Some(BibleBook::Titus),
        57 => Some(BibleBook::Philemon),
        58 => Some(BibleBook::Hebrews),
        59 => Some(BibleBook::James),
        60 => Some(BibleBook::IPeter),
        61 => Some(BibleBook::IIPeter),
        62 => Some(BibleBook::IJohn),
        63 => Some(BibleBook::IIJohn),
        64 => Some(BibleBook::IIIJohn),
        65 => Some(BibleBook::Jude),
        66 => Some(BibleBook::Revelation),
        _ => None,
    }
}

/// An unsigned positive number which represents the chapter of a Bible reference
pub type BibleChapter = u8;

/// An unsigned positive number which represents the verse of a Bible reference
pub type BibleVerse = u8;

/// A Bible Book range is a range of Bible books, e.g. Genesis to Exodus. It is represented by two [BibleBook]s. The first book is the start of the range and the second book is the end of the range.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BibleBookRange {
    start: BibleBookReference,
    end: BibleBookReference,
}

impl BibleBookRange {
    /// Creates a new BibleBookRange with the given start and end books, if the start book is before the end book. If the start book is after the end book, an error will be returned.
    /// # Parameters
    /// - `start`: The start of the range
    /// - `end`: The end of the range
    pub fn new(
        start: BibleBookReference,
        end: BibleBookReference,
    ) -> Result<Self, errors::BibleReferenceValidationError> {
        if start > end {
            return Err(errors::BibleReferenceValidationError {
                problem: errors::BibleReferenceProblem::StartReferenceAfterEndReference,
            });
        }
        Ok(BibleBookRange { start, end })
    }

    /// Returns the start of the range
    pub fn start(&self) -> BibleBookReference {
        self.start.clone()
    }

    /// Returns the end of the range
    pub fn end(&self) -> BibleBookReference {
        self.end.clone()
    }

    /// Returns the range as a [BibleBookList]
    pub fn as_list(&self) -> BibleBookList {
        let mut books: BibleBookList = vec![];
        for i in self.start.book().number()..=self.end.book().number() {
            books.push(BibleBookReference::new(
                get_bible_book_by_number(i).unwrap(),
            ));
        }
        books
    }

    /// Downcasts the [BibleBookRange] to a [BibleChapterRange]
    /// The first chapter for the start book is assumed, and the last chapter for the end book is assumed.
    pub fn as_chapter_range(&self) -> BibleChapterRange {
        let start = BibleChapterReference::new(self.start.book(), 1).unwrap();
        let end =
            BibleChapterReference::new(self.end.book(), get_number_of_chapters(&self.end.book()))
                .unwrap();
        BibleChapterRange::new(start, end).unwrap()
    }

    /// Tries to convert the [BibleBookRange] to a [BibleBookReference] if it represents a single book.
    pub fn as_single_book(&self) -> Option<BibleBookReference> {
        if self.start.book() == self.end.book() {
            Some(self.start.clone())
        } else {
            None
        }
    }
}

/// A Bible Chapter range is a range of Bible chapters, e.g. Genesis 1 to Genesis 2. It is represented by two [BibleChapterReference]s. The first chapter is the start of the range and the second chapter is the end of the range.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BibleChapterRange {
    start: BibleChapterReference,
    end: BibleChapterReference,
}
impl BibleChapterRange {
    /// Creates a new BibleChapterRange with the given start and end chapters, if the start chapter is before the end chapter. If the start chapter is after the end chapter, an error will be returned.
    /// # Parameters
    /// - `start`: The start of the range
    /// - `end`: The end of the range
    pub fn new(
        start: BibleChapterReference,
        end: BibleChapterReference,
    ) -> Result<Self, errors::BibleReferenceValidationError> {
        if start > end {
            return Err(errors::BibleReferenceValidationError {
                problem: errors::BibleReferenceProblem::StartReferenceAfterEndReference,
            });
        }
        Ok(BibleChapterRange { start, end })
    }

    /// Returns the start of the range
    pub fn start(&self) -> BibleChapterReference {
        self.start.clone()
    }
    /// Returns the end of the range
    pub fn end(&self) -> BibleChapterReference {
        self.end.clone()
    }
    /// Returns the range as a [BibleChapterList]
    pub fn as_list(&self) -> BibleChapterList {
        let mut chapters: BibleChapterList = vec![];
        for i in self.start.chapter()..=self.end.chapter() {
            chapters.push(BibleChapterReference::new(self.start.book(), i).unwrap());
        }
        chapters
    }

    /// Downcasts the [BibleChapterRange] to a [BibleVerseRange]
    /// The first verse for the start chapter is assumed, and the last verse for the end chapter is assumed.
    pub fn as_verse_range(&self) -> BibleVerseRange {
        let start = BibleVerseReference::new(self.start.book(), self.start.chapter(), 1).unwrap();
        let end = BibleVerseReference::new(
            self.end.book(),
            self.end.chapter(),
            get_number_of_verses(&self.end.book(), &self.end.chapter()).unwrap(),
        )
        .unwrap();
        BibleVerseRange::new(start, end).unwrap()
    }

    /// Tries to convert the range into a [BibleBookRange] if the range spans multiple books completely.
    /// Returns None if the range does not span over multiple books.
    pub fn as_book_range(&self) -> Option<BibleBookRange> {
        if self.start.chapter() == 1 && self.end.chapter == get_number_of_chapters(&self.end.book) {
            let start = BibleBookReference::new(self.start.book());
            let end = BibleBookReference::new(self.end.book());
            Some(BibleBookRange::new(start, end).unwrap())
        } else {
            None
        }
    }

    /// Converts the range into a [BibleChapterReference] if the range spans a single chapter.
    /// Returns None if the range does not span over a single chapter.
    pub fn as_single_chapter(&self) -> Option<BibleChapterReference> {
        if self.start.book() == self.end.book() && self.start.chapter() == self.end.chapter() {
            Some(BibleChapterReference::new(self.start.book(), self.start.chapter()).unwrap())
        } else {
            None
        }
    }
}

/// A Bible Verse range is a range of Bible verses, e.g. Genesis 1:1 to Genesis 1:2. It is represented by two [BibleVerseReference]s. The first verse is the start of the range and the second verse is the end of the range.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BibleVerseRange {
    start: BibleVerseReference,
    end: BibleVerseReference,
}
impl BibleVerseRange {
    /// Creates a new [BibleVerseRange] with the given start and end verses, if the start verse is before the end verse. If the start verse is after the end verse, an error will be returned.
    /// # Parameters
    /// - `start`: The start of the range
    /// - `end`: The end of the range
    pub fn new(
        start: BibleVerseReference,
        end: BibleVerseReference,
    ) -> Result<Self, errors::BibleReferenceValidationError> {
        if start > end {
            return Err(errors::BibleReferenceValidationError {
                problem: errors::BibleReferenceProblem::StartReferenceAfterEndReference,
            });
        }
        Ok(BibleVerseRange { start, end })
    }

    /// Returns the start of the range
    pub fn start(&self) -> BibleVerseReference {
        self.start.clone()
    }
    /// Returns the end of the range
    pub fn end(&self) -> BibleVerseReference {
        self.end.clone()
    }

    /// Returns the range as a [BibleVerseList]
    /// # Note
    /// This function will return all verses in the range, including the start and end verse.
    pub fn as_list(&self) -> BibleVerseList {
        let mut verses: BibleVerseList = vec![];
        for i in self.start.verse()..=self.end.verse() {
            verses.push(
                BibleVerseReference::new(self.start.book(), self.start.chapter(), i).unwrap(),
            );
        }
        verses
    }

    /// Tries to convert the range into a [BibleChapterRange] if the range spans completely over multiple chapters.
    /// Returns None if the range does not span over multiple chapters.
    pub fn as_chapter_range(&self) -> Option<BibleChapterRange> {
        if self.start.verse() == 1
            && self.end.verse()
                == get_number_of_verses(&self.end.book(), &self.end.chapter()).unwrap()
        {
            let start =
                BibleChapterReference::new(self.start.book(), self.start.chapter()).unwrap();
            let end = BibleChapterReference::new(self.end.book(), self.end.chapter()).unwrap();
            Some(BibleChapterRange::new(start, end).unwrap())
        } else {
            None
        }
    }

    /// Tries to convert the range into a [BibleVerseReference] if the range spans over a single verse.
    /// Returns None if the range does not span over a single verse.
    pub fn as_single_verse(&self) -> Option<BibleVerseReference> {
        if self.start.book() == self.end.book()
            && self.start.chapter() == self.end.chapter()
            && self.start.verse() == self.end.verse()
        {
            Some(
                BibleVerseReference::new(
                    self.start.book(),
                    self.start.chapter(),
                    self.start.verse(),
                )
                .unwrap(),
            )
        } else {
            None
        }
    }
}

/// This enum represents a range of Bible references. It can be a range of books, chapters or verses.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BibleRange {
    /// A range of Bible books
    BookRange(BibleBookRange),

    /// A range of Bible chapters
    ChapterRange(BibleChapterRange),

    /// A range of Bible verses
    VerseRange(BibleVerseRange),
}
impl BibleRange {
    pub fn new(
        start: BibleReference,
        end: BibleReference,
    ) -> Result<Self, errors::BibleReferenceValidationError> {
        match (start, end) {
            (BibleReference::BibleBook(start), BibleReference::BibleBook(end)) => {
                match BibleBookRange::new(start, end) {
                    Ok(range) => Ok(BibleRange::BookRange(range)),
                    Err(err) => Err(err),
                }
            }
            (BibleReference::BibleChapter(start), BibleReference::BibleChapter(end)) => {
                match BibleChapterRange::new(start, end) {
                    Ok(range) => Ok(BibleRange::ChapterRange(range)),
                    Err(err) => Err(err),
                }
            }
            (BibleReference::BibleVerse(start), BibleReference::BibleVerse(end)) => {
                match BibleVerseRange::new(start, end) {
                    Ok(range) => Ok(BibleRange::VerseRange(range)),
                    Err(err) => Err(err),
                }
            }
            (BibleReference::BibleBook(start), BibleReference::BibleChapter(end)) => {
                match BibleChapterRange::new(
                    BibleChapterReference::new(start.book(), 1).unwrap(),
                    end,
                ) {
                    Ok(range) => Ok(BibleRange::ChapterRange(range)),
                    Err(err) => Err(err),
                }
            }
            (BibleReference::BibleChapter(start), BibleReference::BibleBook(end)) => {
                match BibleChapterRange::new(
                    start,
                    BibleChapterReference::new(end.book(), 1).unwrap(),
                ) {
                    Ok(range) => Ok(BibleRange::ChapterRange(range)),
                    Err(err) => Err(err),
                }
            }
            (BibleReference::BibleBook(start), BibleReference::BibleVerse(end)) => {
                match BibleVerseRange::new(
                    BibleVerseReference::new(start.book(), 1, 1).unwrap(),
                    end,
                ) {
                    Ok(range) => Ok(BibleRange::VerseRange(range)),
                    Err(err) => Err(err),
                }
            }
            (BibleReference::BibleVerse(start), BibleReference::BibleBook(end)) => {
                match BibleVerseRange::new(
                    start,
                    BibleVerseReference::new(end.book(), 1, 1).unwrap(),
                ) {
                    Ok(range) => Ok(BibleRange::VerseRange(range)),
                    Err(err) => Err(err),
                }
            }
            (BibleReference::BibleChapter(start), BibleReference::BibleVerse(end)) => {
                match BibleVerseRange::new(
                    BibleVerseReference::new(start.book(), start.chapter(), 1).unwrap(),
                    end,
                ) {
                    Ok(range) => Ok(BibleRange::VerseRange(range)),
                    Err(err) => Err(err),
                }
            }
            (BibleReference::BibleVerse(start), BibleReference::BibleChapter(end)) => {
                match BibleVerseRange::new(
                    start,
                    BibleVerseReference::new(end.book(), end.chapter(), 1).unwrap(),
                ) {
                    Ok(range) => Ok(BibleRange::VerseRange(range)),
                    Err(err) => Err(err),
                }
            }
        }
    }

    /// Returns the range as a list (vector) of Bible references ([BibleReference]). The list will contain all references in the range, including the start and end reference.
    pub fn as_list(&self) -> BibleReferenceList {
        match self {
            BibleRange::BookRange(range) => range
                .as_list()
                .iter()
                .map(|x| BibleReference::BibleBook(x.clone()))
                .collect(),
            BibleRange::ChapterRange(range) => range
                .as_list()
                .iter()
                .map(|x| BibleReference::BibleChapter(x.clone()))
                .collect(),
            BibleRange::VerseRange(range) => range
                .as_list()
                .iter()
                .map(|x| BibleReference::BibleVerse(x.clone()))
                .collect(),
        }
    }

    pub fn end(&self) -> BibleReference {
        match self {
            BibleRange::BookRange(range) => BibleReference::BibleBook(range.end()),
            BibleRange::ChapterRange(range) => BibleReference::BibleChapter(range.end()),
            BibleRange::VerseRange(range) => BibleReference::BibleVerse(range.end()),
        }
    }
    pub fn start(&self) -> BibleReference {
        match self {
            BibleRange::BookRange(range) => BibleReference::BibleBook(range.start()),
            BibleRange::ChapterRange(range) => BibleReference::BibleChapter(range.start()),
            BibleRange::VerseRange(range) => BibleReference::BibleVerse(range.start()),
        }
    }
}

/// This function takes a vector of [BibleReferenceRepresentation]s and aggregates them,
/// which means that it combines overlapping or adjacent ranges or references into one or multible range.
/// It wil also remove duplicates and sort the references.
/// # Parameters
/// - `bible_representations`: A vector of [BibleReferenceRepresentation]s
/// # Returns
/// - A vector of [BibleReferenceRepresentation]s which contains the aggregated references.
pub fn aggregate_bible_representations(
    bible_representations: Vec<BibleReferenceRepresentation>,
) -> Vec<BibleReferenceRepresentation> {
    let mut representations = bible_representations.clone();

    'outer: loop {
        if representations.len() < 2 {
            return representations;
        }

        representations.sort_unstable();
        representations.dedup();

        for i in 0..representations.len() - 1 {
            let j = i + 1;
            if representations.get(i).is_none() || representations.get(j).is_none() {
                continue;
            }

            if representations[i] == representations[j] {
                continue;
            }
            match (&representations[i], &representations[j]) {
                (
                    BibleReferenceRepresentation::Single(a),
                    BibleReferenceRepresentation::Single(b),
                ) => {
                    if a == b {
                        continue;
                    }
                    let a_next = a.next();
                    if a_next.is_some() && *b == a.next().unwrap() {
                        let new_range = BibleRange::new(a.clone(), b.clone()).unwrap();
                        representations.push(BibleReferenceRepresentation::Range(new_range));
                        representations.remove(j);
                        representations.remove(i);
                        continue 'outer;
                    };
                }
                (
                    BibleReferenceRepresentation::Range(a),
                    BibleReferenceRepresentation::Range(b),
                ) => {
                    if a == b {
                        continue;
                    }
                    if a.end() > b.start() && b.end() > a.end() {
                        let new_range = BibleRange::new(a.start(), b.end()).unwrap();
                        representations.push(BibleReferenceRepresentation::Range(new_range));
                        representations.remove(j);
                        representations.remove(i);
                        continue 'outer;
                    }
                }
                (
                    BibleReferenceRepresentation::Single(a),
                    BibleReferenceRepresentation::Range(b),
                ) => {
                    if a.next().is_some() && a.next().unwrap() == b.start() {
                        let new_range = BibleRange::new(a.clone(), b.end()).unwrap();
                        representations.push(BibleReferenceRepresentation::Range(new_range));
                        representations.remove(j);
                        representations.remove(i);
                        continue 'outer;
                    }
                }
                (
                    BibleReferenceRepresentation::Range(a),
                    BibleReferenceRepresentation::Single(b),
                ) => {
                    if a.end().next().is_some() && a.end().next().unwrap() == *b {
                        let new_range = BibleRange::new(a.start(), b.clone()).unwrap();
                        representations.push(BibleReferenceRepresentation::Range(new_range));
                        representations.remove(j);
                        representations.remove(i);
                        continue 'outer;
                    }
                }
            }
        }

        representations.sort_unstable();
        representations.dedup();
        return representations;
    }
}

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
        let bibleref = BibleVerseReference::new(BibleBook::Matthew, 11, 28);
        assert!(bibleref.is_ok());

        let bibleref = BibleVerseReference::new(BibleBook::Revelation, 23, 8);
        assert!(bibleref.is_err());
    }

    #[test]
    fn test_biblechapterreference_creation() {
        let bibleref = BibleChapterReference::new(BibleBook::Genesis, 1);
        assert!(bibleref.is_ok());

        let bibleref = BibleChapterReference::new(BibleBook::Ruth, 0);
        assert!(bibleref.is_err());
    }

    #[test]
    fn test_biblerefrepresentation_comp() {
        let bibleref1 =
            BibleReference::BibleVerse(BibleVerseReference::new(BibleBook::Genesis, 1, 1).unwrap());
        let bibleref2 =
            BibleReference::BibleVerse(BibleVerseReference::new(BibleBook::Genesis, 1, 2).unwrap());
        let bibleref3 =
            BibleReference::BibleVerse(BibleVerseReference::new(BibleBook::Genesis, 1, 3).unwrap());

        let biblerep1 = BibleReferenceRepresentation::Single(bibleref1.clone());
        let biblerep2 = BibleReferenceRepresentation::Single(bibleref2.clone());
        let biblerep3 = BibleReferenceRepresentation::Single(bibleref3.clone());

        assert!(biblerep1 < biblerep2);
        assert!(biblerep2 < biblerep3);
        assert!(biblerep1 < biblerep3);

        let biblerep4 = BibleReferenceRepresentation::Range(
            BibleRange::new(bibleref1.clone(), bibleref3.clone()).unwrap(),
        );
        assert!(biblerep4 > biblerep1);
        let biblerep5 = BibleReferenceRepresentation::Range(
            BibleRange::new(bibleref1.clone(), bibleref2.clone()).unwrap(),
        );
        dbg!(&biblerep5);
        dbg!(&biblerep3);
        assert!(biblerep5 < biblerep3);
    }

    #[test]
    fn test_biblereference_ordering() {
        let bibleref1 =
            BibleReference::BibleVerse(BibleVerseReference::new(BibleBook::Genesis, 1, 1).unwrap());
        let bibleref2 =
            BibleReference::BibleVerse(BibleVerseReference::new(BibleBook::Genesis, 1, 2).unwrap());
        assert!(bibleref1 < bibleref2);

        let bibleref3 = BibleReference::BibleChapter(
            BibleChapterReference::new(BibleBook::Genesis, 1).unwrap(),
        );
        let bibleref4 = BibleReference::BibleChapter(
            BibleChapterReference::new(BibleBook::Genesis, 2).unwrap(),
        );
        assert!(bibleref3 < bibleref4);

        let bibleref5 = BibleReference::BibleBook(BibleBookReference::new(BibleBook::Genesis));
        let bibleref6 =
            BibleReference::BibleVerse(BibleVerseReference::new(BibleBook::Exodus, 3, 4).unwrap());
        assert!(bibleref5 < bibleref6);
        assert!(bibleref6 > bibleref5);

        let bibleref7 =
            BibleReference::BibleChapter(BibleChapterReference::new(BibleBook::John, 4).unwrap());
        let bibleref8 =
            BibleReference::BibleVerse(BibleVerseReference::new(BibleBook::John, 3, 16).unwrap());
        assert!(bibleref7 > bibleref8);
        assert!(bibleref8 < bibleref7);
    }

    #[test]
    pub fn test_biblerepresentations_aggregation() {
        let bibleref1 =
            BibleReference::BibleVerse(BibleVerseReference::new(BibleBook::Genesis, 1, 1).unwrap());
        let bibleref2 =
            BibleReference::BibleVerse(BibleVerseReference::new(BibleBook::Genesis, 1, 2).unwrap());
        let bibleref3 =
            BibleReference::BibleVerse(BibleVerseReference::new(BibleBook::Genesis, 1, 3).unwrap());

        let biblerep1 = BibleReferenceRepresentation::Single(bibleref1.clone());
        let biblerep2 = BibleReferenceRepresentation::Single(bibleref2.clone());
        let biblerep3 = BibleReferenceRepresentation::Single(bibleref3.clone());

        let biblereps = vec![biblerep1, biblerep2, biblerep3];

        let aggregated = aggregate_bible_representations(biblereps);
        dbg!(&aggregated);
        assert_eq!(aggregated.len(), 1);
        assert_eq!(
            aggregated[0],
            BibleReferenceRepresentation::Range(
                BibleRange::new(bibleref1.clone(), bibleref3.clone()).unwrap()
            )
        );
    }

    #[test]
    pub fn test_biblebooks_length() {
        assert_eq!(BibleBook::all().len(), 66);
    }
}
