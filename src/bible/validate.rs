//! This submodule contains some helper function to validate Bible references

use crate::bible::errors::*;
use crate::bible::{BibleBook, BibleChapter, BibleVerse};

pub fn validate_book_chapter(
    book: &BibleBook,
    chapter: &BibleChapter,
) -> Result<(), BibleReferenceValidationError> {
    if *chapter == 0 || chapter > &get_number_of_chapters(book) {
        Err(BibleReferenceValidationError {
            problem: BibleReferenceProblem::ChapterDoesNotExist,
        })
    } else {
        Ok(())
    }
}

/// Returns the number of chapters of a given BibleBook
/// # Params
/// - book: The [BibleBook] as a reference (`&BibleBook`)
/// # Returns
/// A number which can be used as a BibleChapter
#[allow(clippy::all)]
pub fn get_number_of_chapters(book: &BibleBook) -> BibleChapter {
    match *book {
        BibleBook::Genesis => 50,
        BibleBook::Exodus => 40,
        BibleBook::Leviticus => 27,
        BibleBook::Numbers => 36,
        BibleBook::Deuteronomy => 34,
        BibleBook::Joshua => 24,
        BibleBook::Judges => 21,
        BibleBook::Ruth => 4,
        BibleBook::ISamuel => 31,
        BibleBook::IISamuel => 24,
        BibleBook::IKings => 22,
        BibleBook::IIKings => 25,
        BibleBook::IChronicles => 29,
        BibleBook::IIChronicles => 36,
        BibleBook::Ezra => 10,
        BibleBook::Nehemiah => 13,
        BibleBook::Esther => 10,
        BibleBook::Job => 42,
        BibleBook::Psalm => 150,
        BibleBook::Proverbs => 31,
        BibleBook::Ecclesiastes => 12,
        BibleBook::SongofSolomon => 8,
        BibleBook::Isaiah => 66,
        BibleBook::Jeremiah => 52,
        BibleBook::Lamentations => 5,
        BibleBook::Ezekiel => 48,
        BibleBook::Daniel => 12,
        BibleBook::Hosea => 14,
        BibleBook::Joel => 3,
        BibleBook::Amos => 9,
        BibleBook::Obadiah => 1,
        BibleBook::Jonah => 4,
        BibleBook::Micah => 7,
        BibleBook::Nahum => 3,
        BibleBook::Habakkuk => 3,
        BibleBook::Zephaniah => 3,
        BibleBook::Haggai => 2,
        BibleBook::Zechariah => 14,
        BibleBook::Malachi => 4,
        BibleBook::Matthew => 28,
        BibleBook::Mark => 16,
        BibleBook::Luke => 24,
        BibleBook::John => 21,
        BibleBook::Acts => 28,
        BibleBook::Romans => 16,
        BibleBook::ICorinthians => 16,
        BibleBook::IICorinthians => 13,
        BibleBook::Galatians => 6,
        BibleBook::Ephesians => 6,
        BibleBook::Philippians => 4,
        BibleBook::Colossians => 4,
        BibleBook::IThessalonians => 5,
        BibleBook::IIThessalonians => 3,
        BibleBook::ITimothy => 6,
        BibleBook::IITimothy => 4,
        BibleBook::Titus => 3,
        BibleBook::Philemon => 1,
        BibleBook::Hebrews => 13,
        BibleBook::James => 5,
        BibleBook::IPeter => 5,
        BibleBook::IIPeter => 3,
        BibleBook::IJohn => 5,
        BibleBook::IIJohn => 1,
        BibleBook::IIIJohn => 1,
        BibleBook::Jude => 1,
        BibleBook::Revelation => 22,
    }
}

/// Validates whether a reference consisting of a BibleBook, a Chapter and a Verse exists.
pub fn validate_book_chapter_verse(
    book: &BibleBook,
    chapter: &BibleChapter,
    verse: &BibleVerse,
) -> Result<(), BibleReferenceValidationError> {
    if *chapter == 0 {
        return Err(BibleReferenceValidationError {
            problem: (BibleReferenceProblem::ChapterDoesNotExist),
        });
    }
    if *verse == 0 {
        return Err(BibleReferenceValidationError {
            problem: (BibleReferenceProblem::VerseDoesNotExist),
        });
    }

    match get_number_of_verses(book, chapter) {
        Ok(number) => {
            if *verse <= number {
                Ok(())
            } else {
                Err(BibleReferenceValidationError {
                    problem: BibleReferenceProblem::VerseDoesNotExist,
                })
            }
        }
        Err(error) => Err(error),
    }
}

/// Returns the number of Bible verses of a chapter (specified by book and chapter)
#[allow(clippy::all)]
pub fn get_number_of_verses(
    book: &BibleBook,
    chapter: &BibleChapter,
) -> Result<BibleVerse, BibleReferenceValidationError> {
    if book == &BibleBook::Genesis && *chapter == 1 {
        Ok(31)
    } else if book == &BibleBook::Genesis && *chapter == 2 {
        Ok(25)
    } else if book == &BibleBook::Genesis && *chapter == 3 {
        Ok(24)
    } else if book == &BibleBook::Genesis && *chapter == 4 {
        Ok(26)
    } else if book == &BibleBook::Genesis && *chapter == 5 {
        Ok(32)
    } else if book == &BibleBook::Genesis && *chapter == 6 {
        Ok(22)
    } else if book == &BibleBook::Genesis && *chapter == 7 {
        Ok(24)
    } else if book == &BibleBook::Genesis && *chapter == 8 {
        Ok(22)
    } else if book == &BibleBook::Genesis && *chapter == 9 {
        Ok(29)
    } else if book == &BibleBook::Genesis && *chapter == 10 {
        Ok(32)
    } else if book == &BibleBook::Genesis && *chapter == 11 {
        Ok(32)
    } else if book == &BibleBook::Genesis && *chapter == 12 {
        Ok(20)
    } else if book == &BibleBook::Genesis && *chapter == 13 {
        Ok(18)
    } else if book == &BibleBook::Genesis && *chapter == 14 {
        Ok(24)
    } else if book == &BibleBook::Genesis && *chapter == 15 {
        Ok(21)
    } else if book == &BibleBook::Genesis && *chapter == 16 {
        Ok(16)
    } else if book == &BibleBook::Genesis && *chapter == 17 {
        Ok(27)
    } else if book == &BibleBook::Genesis && *chapter == 18 {
        Ok(33)
    } else if book == &BibleBook::Genesis && *chapter == 19 {
        Ok(38)
    } else if book == &BibleBook::Genesis && *chapter == 20 {
        Ok(18)
    } else if book == &BibleBook::Genesis && *chapter == 21 {
        Ok(34)
    } else if book == &BibleBook::Genesis && *chapter == 22 {
        Ok(24)
    } else if book == &BibleBook::Genesis && *chapter == 23 {
        Ok(20)
    } else if book == &BibleBook::Genesis && *chapter == 24 {
        Ok(67)
    } else if book == &BibleBook::Genesis && *chapter == 25 {
        Ok(34)
    } else if book == &BibleBook::Genesis && *chapter == 26 {
        Ok(35)
    } else if book == &BibleBook::Genesis && *chapter == 27 {
        Ok(46)
    } else if book == &BibleBook::Genesis && *chapter == 28 {
        Ok(22)
    } else if book == &BibleBook::Genesis && *chapter == 29 {
        Ok(35)
    } else if book == &BibleBook::Genesis && *chapter == 30 {
        Ok(43)
    } else if book == &BibleBook::Genesis && *chapter == 31 {
        Ok(55)
    } else if book == &BibleBook::Genesis && *chapter == 32 {
        Ok(32)
    } else if book == &BibleBook::Genesis && *chapter == 33 {
        Ok(20)
    } else if book == &BibleBook::Genesis && *chapter == 34 {
        Ok(31)
    } else if book == &BibleBook::Genesis && *chapter == 35 {
        Ok(29)
    } else if book == &BibleBook::Genesis && *chapter == 36 {
        Ok(43)
    } else if book == &BibleBook::Genesis && *chapter == 37 {
        Ok(36)
    } else if book == &BibleBook::Genesis && *chapter == 38 {
        Ok(30)
    } else if book == &BibleBook::Genesis && *chapter == 39 {
        Ok(23)
    } else if book == &BibleBook::Genesis && *chapter == 40 {
        Ok(23)
    } else if book == &BibleBook::Genesis && *chapter == 41 {
        Ok(57)
    } else if book == &BibleBook::Genesis && *chapter == 42 {
        Ok(38)
    } else if book == &BibleBook::Genesis && *chapter == 43 {
        Ok(34)
    } else if book == &BibleBook::Genesis && *chapter == 44 {
        Ok(34)
    } else if book == &BibleBook::Genesis && *chapter == 45 {
        Ok(28)
    } else if book == &BibleBook::Genesis && *chapter == 46 {
        Ok(34)
    } else if book == &BibleBook::Genesis && *chapter == 47 {
        Ok(31)
    } else if book == &BibleBook::Genesis && *chapter == 48 {
        Ok(22)
    } else if book == &BibleBook::Genesis && *chapter == 49 {
        Ok(33)
    } else if book == &BibleBook::Genesis && *chapter == 50 {
        Ok(26)
    } else if book == &BibleBook::Exodus && *chapter == 1 {
        Ok(22)
    } else if book == &BibleBook::Exodus && *chapter == 2 {
        Ok(25)
    } else if book == &BibleBook::Exodus && *chapter == 3 {
        Ok(22)
    } else if book == &BibleBook::Exodus && *chapter == 4 {
        Ok(31)
    } else if book == &BibleBook::Exodus && *chapter == 5 {
        Ok(23)
    } else if book == &BibleBook::Exodus && *chapter == 6 {
        Ok(30)
    } else if book == &BibleBook::Exodus && *chapter == 7 {
        Ok(25)
    } else if book == &BibleBook::Exodus && *chapter == 8 {
        Ok(32)
    } else if book == &BibleBook::Exodus && *chapter == 9 {
        Ok(35)
    } else if book == &BibleBook::Exodus && *chapter == 10 {
        Ok(29)
    } else if book == &BibleBook::Exodus && *chapter == 11 {
        Ok(10)
    } else if book == &BibleBook::Exodus && *chapter == 12 {
        Ok(51)
    } else if book == &BibleBook::Exodus && *chapter == 13 {
        Ok(22)
    } else if book == &BibleBook::Exodus && *chapter == 14 {
        Ok(31)
    } else if book == &BibleBook::Exodus && *chapter == 15 {
        Ok(27)
    } else if book == &BibleBook::Exodus && *chapter == 16 {
        Ok(36)
    } else if book == &BibleBook::Exodus && *chapter == 17 {
        Ok(16)
    } else if book == &BibleBook::Exodus && *chapter == 18 {
        Ok(27)
    } else if book == &BibleBook::Exodus && *chapter == 19 {
        Ok(25)
    } else if book == &BibleBook::Exodus && *chapter == 20 {
        Ok(26)
    } else if book == &BibleBook::Exodus && *chapter == 21 {
        Ok(36)
    } else if book == &BibleBook::Exodus && *chapter == 22 {
        Ok(31)
    } else if book == &BibleBook::Exodus && *chapter == 23 {
        Ok(33)
    } else if book == &BibleBook::Exodus && *chapter == 24 {
        Ok(18)
    } else if book == &BibleBook::Exodus && *chapter == 25 {
        Ok(40)
    } else if book == &BibleBook::Exodus && *chapter == 26 {
        Ok(37)
    } else if book == &BibleBook::Exodus && *chapter == 27 {
        Ok(21)
    } else if book == &BibleBook::Exodus && *chapter == 28 {
        Ok(43)
    } else if book == &BibleBook::Exodus && *chapter == 29 {
        Ok(46)
    } else if book == &BibleBook::Exodus && *chapter == 30 {
        Ok(38)
    } else if book == &BibleBook::Exodus && *chapter == 31 {
        Ok(18)
    } else if book == &BibleBook::Exodus && *chapter == 32 {
        Ok(35)
    } else if book == &BibleBook::Exodus && *chapter == 33 {
        Ok(23)
    } else if book == &BibleBook::Exodus && *chapter == 34 {
        Ok(35)
    } else if book == &BibleBook::Exodus && *chapter == 35 {
        Ok(35)
    } else if book == &BibleBook::Exodus && *chapter == 36 {
        Ok(38)
    } else if book == &BibleBook::Exodus && *chapter == 37 {
        Ok(29)
    } else if book == &BibleBook::Exodus && *chapter == 38 {
        Ok(31)
    } else if book == &BibleBook::Exodus && *chapter == 39 {
        Ok(43)
    } else if book == &BibleBook::Exodus && *chapter == 40 {
        Ok(38)
    } else if book == &BibleBook::Leviticus && *chapter == 1 {
        Ok(17)
    } else if book == &BibleBook::Leviticus && *chapter == 2 {
        Ok(16)
    } else if book == &BibleBook::Leviticus && *chapter == 3 {
        Ok(17)
    } else if book == &BibleBook::Leviticus && *chapter == 4 {
        Ok(35)
    } else if book == &BibleBook::Leviticus && *chapter == 5 {
        Ok(19)
    } else if book == &BibleBook::Leviticus && *chapter == 6 {
        Ok(30)
    } else if book == &BibleBook::Leviticus && *chapter == 7 {
        Ok(38)
    } else if book == &BibleBook::Leviticus && *chapter == 8 {
        Ok(36)
    } else if book == &BibleBook::Leviticus && *chapter == 9 {
        Ok(24)
    } else if book == &BibleBook::Leviticus && *chapter == 10 {
        Ok(20)
    } else if book == &BibleBook::Leviticus && *chapter == 11 {
        Ok(47)
    } else if book == &BibleBook::Leviticus && *chapter == 12 {
        Ok(8)
    } else if book == &BibleBook::Leviticus && *chapter == 13 {
        Ok(59)
    } else if book == &BibleBook::Leviticus && *chapter == 14 {
        Ok(57)
    } else if book == &BibleBook::Leviticus && *chapter == 15 {
        Ok(33)
    } else if book == &BibleBook::Leviticus && *chapter == 16 {
        Ok(34)
    } else if book == &BibleBook::Leviticus && *chapter == 17 {
        Ok(16)
    } else if book == &BibleBook::Leviticus && *chapter == 18 {
        Ok(30)
    } else if book == &BibleBook::Leviticus && *chapter == 19 {
        Ok(37)
    } else if book == &BibleBook::Leviticus && *chapter == 20 {
        Ok(27)
    } else if book == &BibleBook::Leviticus && *chapter == 21 {
        Ok(24)
    } else if book == &BibleBook::Leviticus && *chapter == 22 {
        Ok(33)
    } else if book == &BibleBook::Leviticus && *chapter == 23 {
        Ok(44)
    } else if book == &BibleBook::Leviticus && *chapter == 24 {
        Ok(23)
    } else if book == &BibleBook::Leviticus && *chapter == 25 {
        Ok(55)
    } else if book == &BibleBook::Leviticus && *chapter == 26 {
        Ok(46)
    } else if book == &BibleBook::Leviticus && *chapter == 27 {
        Ok(34)
    } else if book == &BibleBook::Numbers && *chapter == 1 {
        Ok(54)
    } else if book == &BibleBook::Numbers && *chapter == 2 {
        Ok(34)
    } else if book == &BibleBook::Numbers && *chapter == 3 {
        Ok(51)
    } else if book == &BibleBook::Numbers && *chapter == 4 {
        Ok(49)
    } else if book == &BibleBook::Numbers && *chapter == 5 {
        Ok(31)
    } else if book == &BibleBook::Numbers && *chapter == 6 {
        Ok(27)
    } else if book == &BibleBook::Numbers && *chapter == 7 {
        Ok(89)
    } else if book == &BibleBook::Numbers && *chapter == 8 {
        Ok(26)
    } else if book == &BibleBook::Numbers && *chapter == 9 {
        Ok(23)
    } else if book == &BibleBook::Numbers && *chapter == 10 {
        Ok(36)
    } else if book == &BibleBook::Numbers && *chapter == 11 {
        Ok(35)
    } else if book == &BibleBook::Numbers && *chapter == 12 {
        Ok(16)
    } else if book == &BibleBook::Numbers && *chapter == 13 {
        Ok(33)
    } else if book == &BibleBook::Numbers && *chapter == 14 {
        Ok(45)
    } else if book == &BibleBook::Numbers && *chapter == 15 {
        Ok(41)
    } else if book == &BibleBook::Numbers && *chapter == 16 {
        Ok(50)
    } else if book == &BibleBook::Numbers && *chapter == 17 {
        Ok(13)
    } else if book == &BibleBook::Numbers && *chapter == 18 {
        Ok(32)
    } else if book == &BibleBook::Numbers && *chapter == 19 {
        Ok(22)
    } else if book == &BibleBook::Numbers && *chapter == 20 {
        Ok(29)
    } else if book == &BibleBook::Numbers && *chapter == 21 {
        Ok(35)
    } else if book == &BibleBook::Numbers && *chapter == 22 {
        Ok(41)
    } else if book == &BibleBook::Numbers && *chapter == 23 {
        Ok(30)
    } else if book == &BibleBook::Numbers && *chapter == 24 {
        Ok(25)
    } else if book == &BibleBook::Numbers && *chapter == 25 {
        Ok(18)
    } else if book == &BibleBook::Numbers && *chapter == 26 {
        Ok(65)
    } else if book == &BibleBook::Numbers && *chapter == 27 {
        Ok(23)
    } else if book == &BibleBook::Numbers && *chapter == 28 {
        Ok(31)
    } else if book == &BibleBook::Numbers && *chapter == 29 {
        Ok(40)
    } else if book == &BibleBook::Numbers && *chapter == 30 {
        Ok(16)
    } else if book == &BibleBook::Numbers && *chapter == 31 {
        Ok(54)
    } else if book == &BibleBook::Numbers && *chapter == 32 {
        Ok(42)
    } else if book == &BibleBook::Numbers && *chapter == 33 {
        Ok(56)
    } else if book == &BibleBook::Numbers && *chapter == 34 {
        Ok(29)
    } else if book == &BibleBook::Numbers && *chapter == 35 {
        Ok(34)
    } else if book == &BibleBook::Numbers && *chapter == 36 {
        Ok(13)
    } else if book == &BibleBook::Deuteronomy && *chapter == 1 {
        Ok(46)
    } else if book == &BibleBook::Deuteronomy && *chapter == 2 {
        Ok(37)
    } else if book == &BibleBook::Deuteronomy && *chapter == 3 {
        Ok(29)
    } else if book == &BibleBook::Deuteronomy && *chapter == 4 {
        Ok(49)
    } else if book == &BibleBook::Deuteronomy && *chapter == 5 {
        Ok(33)
    } else if book == &BibleBook::Deuteronomy && *chapter == 6 {
        Ok(25)
    } else if book == &BibleBook::Deuteronomy && *chapter == 7 {
        Ok(26)
    } else if book == &BibleBook::Deuteronomy && *chapter == 8 {
        Ok(20)
    } else if book == &BibleBook::Deuteronomy && *chapter == 9 {
        Ok(29)
    } else if book == &BibleBook::Deuteronomy && *chapter == 10 {
        Ok(22)
    } else if book == &BibleBook::Deuteronomy && *chapter == 11 {
        Ok(32)
    } else if book == &BibleBook::Deuteronomy && *chapter == 12 {
        Ok(32)
    } else if book == &BibleBook::Deuteronomy && *chapter == 13 {
        Ok(18)
    } else if book == &BibleBook::Deuteronomy && *chapter == 14 {
        Ok(29)
    } else if book == &BibleBook::Deuteronomy && *chapter == 15 {
        Ok(23)
    } else if book == &BibleBook::Deuteronomy && *chapter == 16 {
        Ok(22)
    } else if book == &BibleBook::Deuteronomy && *chapter == 17 {
        Ok(20)
    } else if book == &BibleBook::Deuteronomy && *chapter == 18 {
        Ok(22)
    } else if book == &BibleBook::Deuteronomy && *chapter == 19 {
        Ok(21)
    } else if book == &BibleBook::Deuteronomy && *chapter == 20 {
        Ok(20)
    } else if book == &BibleBook::Deuteronomy && *chapter == 21 {
        Ok(23)
    } else if book == &BibleBook::Deuteronomy && *chapter == 22 {
        Ok(30)
    } else if book == &BibleBook::Deuteronomy && *chapter == 23 {
        Ok(25)
    } else if book == &BibleBook::Deuteronomy && *chapter == 24 {
        Ok(22)
    } else if book == &BibleBook::Deuteronomy && *chapter == 25 {
        Ok(19)
    } else if book == &BibleBook::Deuteronomy && *chapter == 26 {
        Ok(19)
    } else if book == &BibleBook::Deuteronomy && *chapter == 27 {
        Ok(26)
    } else if book == &BibleBook::Deuteronomy && *chapter == 28 {
        Ok(68)
    } else if book == &BibleBook::Deuteronomy && *chapter == 29 {
        Ok(29)
    } else if book == &BibleBook::Deuteronomy && *chapter == 30 {
        Ok(20)
    } else if book == &BibleBook::Deuteronomy && *chapter == 31 {
        Ok(30)
    } else if book == &BibleBook::Deuteronomy && *chapter == 32 {
        Ok(52)
    } else if book == &BibleBook::Deuteronomy && *chapter == 33 {
        Ok(29)
    } else if book == &BibleBook::Deuteronomy && *chapter == 34 {
        Ok(12)
    } else if book == &BibleBook::Joshua && *chapter == 1 {
        Ok(18)
    } else if book == &BibleBook::Joshua && *chapter == 2 {
        Ok(24)
    } else if book == &BibleBook::Joshua && *chapter == 3 {
        Ok(17)
    } else if book == &BibleBook::Joshua && *chapter == 4 {
        Ok(24)
    } else if book == &BibleBook::Joshua && *chapter == 5 {
        Ok(15)
    } else if book == &BibleBook::Joshua && *chapter == 6 {
        Ok(27)
    } else if book == &BibleBook::Joshua && *chapter == 7 {
        Ok(26)
    } else if book == &BibleBook::Joshua && *chapter == 8 {
        Ok(35)
    } else if book == &BibleBook::Joshua && *chapter == 9 {
        Ok(27)
    } else if book == &BibleBook::Joshua && *chapter == 10 {
        Ok(43)
    } else if book == &BibleBook::Joshua && *chapter == 11 {
        Ok(23)
    } else if book == &BibleBook::Joshua && *chapter == 12 {
        Ok(24)
    } else if book == &BibleBook::Joshua && *chapter == 13 {
        Ok(33)
    } else if book == &BibleBook::Joshua && *chapter == 14 {
        Ok(15)
    } else if book == &BibleBook::Joshua && *chapter == 15 {
        Ok(63)
    } else if book == &BibleBook::Joshua && *chapter == 16 {
        Ok(10)
    } else if book == &BibleBook::Joshua && *chapter == 17 {
        Ok(18)
    } else if book == &BibleBook::Joshua && *chapter == 18 {
        Ok(28)
    } else if book == &BibleBook::Joshua && *chapter == 19 {
        Ok(51)
    } else if book == &BibleBook::Joshua && *chapter == 20 {
        Ok(9)
    } else if book == &BibleBook::Joshua && *chapter == 21 {
        Ok(45)
    } else if book == &BibleBook::Joshua && *chapter == 22 {
        Ok(34)
    } else if book == &BibleBook::Joshua && *chapter == 23 {
        Ok(16)
    } else if book == &BibleBook::Joshua && *chapter == 24 {
        Ok(33)
    } else if book == &BibleBook::Judges && *chapter == 1 {
        Ok(36)
    } else if book == &BibleBook::Judges && *chapter == 2 {
        Ok(23)
    } else if book == &BibleBook::Judges && *chapter == 3 {
        Ok(31)
    } else if book == &BibleBook::Judges && *chapter == 4 {
        Ok(24)
    } else if book == &BibleBook::Judges && *chapter == 5 {
        Ok(31)
    } else if book == &BibleBook::Judges && *chapter == 6 {
        Ok(40)
    } else if book == &BibleBook::Judges && *chapter == 7 {
        Ok(25)
    } else if book == &BibleBook::Judges && *chapter == 8 {
        Ok(35)
    } else if book == &BibleBook::Judges && *chapter == 9 {
        Ok(57)
    } else if book == &BibleBook::Judges && *chapter == 10 {
        Ok(18)
    } else if book == &BibleBook::Judges && *chapter == 11 {
        Ok(40)
    } else if book == &BibleBook::Judges && *chapter == 12 {
        Ok(15)
    } else if book == &BibleBook::Judges && *chapter == 13 {
        Ok(25)
    } else if book == &BibleBook::Judges && *chapter == 14 {
        Ok(20)
    } else if book == &BibleBook::Judges && *chapter == 15 {
        Ok(20)
    } else if book == &BibleBook::Judges && *chapter == 16 {
        Ok(31)
    } else if book == &BibleBook::Judges && *chapter == 17 {
        Ok(13)
    } else if book == &BibleBook::Judges && *chapter == 18 {
        Ok(31)
    } else if book == &BibleBook::Judges && *chapter == 19 {
        Ok(30)
    } else if book == &BibleBook::Judges && *chapter == 20 {
        Ok(48)
    } else if book == &BibleBook::Judges && *chapter == 21 {
        Ok(25)
    } else if book == &BibleBook::Ruth && *chapter == 1 {
        Ok(22)
    } else if book == &BibleBook::Ruth && *chapter == 2 {
        Ok(23)
    } else if book == &BibleBook::Ruth && *chapter == 3 {
        Ok(18)
    } else if book == &BibleBook::Ruth && *chapter == 4 {
        Ok(22)
    } else if book == &BibleBook::ISamuel && *chapter == 1 {
        Ok(28)
    } else if book == &BibleBook::ISamuel && *chapter == 2 {
        Ok(36)
    } else if book == &BibleBook::ISamuel && *chapter == 3 {
        Ok(21)
    } else if book == &BibleBook::ISamuel && *chapter == 4 {
        Ok(22)
    } else if book == &BibleBook::ISamuel && *chapter == 5 {
        Ok(12)
    } else if book == &BibleBook::ISamuel && *chapter == 6 {
        Ok(21)
    } else if book == &BibleBook::ISamuel && *chapter == 7 {
        Ok(17)
    } else if book == &BibleBook::ISamuel && *chapter == 8 {
        Ok(22)
    } else if book == &BibleBook::ISamuel && *chapter == 9 {
        Ok(27)
    } else if book == &BibleBook::ISamuel && *chapter == 10 {
        Ok(27)
    } else if book == &BibleBook::ISamuel && *chapter == 11 {
        Ok(15)
    } else if book == &BibleBook::ISamuel && *chapter == 12 {
        Ok(25)
    } else if book == &BibleBook::ISamuel && *chapter == 13 {
        Ok(23)
    } else if book == &BibleBook::ISamuel && *chapter == 14 {
        Ok(52)
    } else if book == &BibleBook::ISamuel && *chapter == 15 {
        Ok(35)
    } else if book == &BibleBook::ISamuel && *chapter == 16 {
        Ok(23)
    } else if book == &BibleBook::ISamuel && *chapter == 17 {
        Ok(58)
    } else if book == &BibleBook::ISamuel && *chapter == 18 {
        Ok(30)
    } else if book == &BibleBook::ISamuel && *chapter == 19 {
        Ok(24)
    } else if book == &BibleBook::ISamuel && *chapter == 20 {
        Ok(42)
    } else if book == &BibleBook::ISamuel && *chapter == 21 {
        Ok(15)
    } else if book == &BibleBook::ISamuel && *chapter == 22 {
        Ok(23)
    } else if book == &BibleBook::ISamuel && *chapter == 23 {
        Ok(29)
    } else if book == &BibleBook::ISamuel && *chapter == 24 {
        Ok(22)
    } else if book == &BibleBook::ISamuel && *chapter == 25 {
        Ok(44)
    } else if book == &BibleBook::ISamuel && *chapter == 26 {
        Ok(25)
    } else if book == &BibleBook::ISamuel && *chapter == 27 {
        Ok(12)
    } else if book == &BibleBook::ISamuel && *chapter == 28 {
        Ok(25)
    } else if book == &BibleBook::ISamuel && *chapter == 29 {
        Ok(11)
    } else if book == &BibleBook::ISamuel && *chapter == 30 {
        Ok(31)
    } else if book == &BibleBook::ISamuel && *chapter == 31 {
        Ok(13)
    } else if book == &BibleBook::IISamuel && *chapter == 1 {
        Ok(27)
    } else if book == &BibleBook::IISamuel && *chapter == 2 {
        Ok(32)
    } else if book == &BibleBook::IISamuel && *chapter == 3 {
        Ok(39)
    } else if book == &BibleBook::IISamuel && *chapter == 4 {
        Ok(12)
    } else if book == &BibleBook::IISamuel && *chapter == 5 {
        Ok(25)
    } else if book == &BibleBook::IISamuel && *chapter == 6 {
        Ok(23)
    } else if book == &BibleBook::IISamuel && *chapter == 7 {
        Ok(29)
    } else if book == &BibleBook::IISamuel && *chapter == 8 {
        Ok(18)
    } else if book == &BibleBook::IISamuel && *chapter == 9 {
        Ok(13)
    } else if book == &BibleBook::IISamuel && *chapter == 10 {
        Ok(19)
    } else if book == &BibleBook::IISamuel && *chapter == 11 {
        Ok(27)
    } else if book == &BibleBook::IISamuel && *chapter == 12 {
        Ok(31)
    } else if book == &BibleBook::IISamuel && *chapter == 13 {
        Ok(39)
    } else if book == &BibleBook::IISamuel && *chapter == 14 {
        Ok(33)
    } else if book == &BibleBook::IISamuel && *chapter == 15 {
        Ok(37)
    } else if book == &BibleBook::IISamuel && *chapter == 16 {
        Ok(23)
    } else if book == &BibleBook::IISamuel && *chapter == 17 {
        Ok(29)
    } else if book == &BibleBook::IISamuel && *chapter == 18 {
        Ok(33)
    } else if book == &BibleBook::IISamuel && *chapter == 19 {
        Ok(43)
    } else if book == &BibleBook::IISamuel && *chapter == 20 {
        Ok(26)
    } else if book == &BibleBook::IISamuel && *chapter == 21 {
        Ok(22)
    } else if book == &BibleBook::IISamuel && *chapter == 22 {
        Ok(51)
    } else if book == &BibleBook::IISamuel && *chapter == 23 {
        Ok(39)
    } else if book == &BibleBook::IISamuel && *chapter == 24 {
        Ok(25)
    } else if book == &BibleBook::IKings && *chapter == 1 {
        Ok(53)
    } else if book == &BibleBook::IKings && *chapter == 2 {
        Ok(46)
    } else if book == &BibleBook::IKings && *chapter == 3 {
        Ok(28)
    } else if book == &BibleBook::IKings && *chapter == 4 {
        Ok(34)
    } else if book == &BibleBook::IKings && *chapter == 5 {
        Ok(18)
    } else if book == &BibleBook::IKings && *chapter == 6 {
        Ok(38)
    } else if book == &BibleBook::IKings && *chapter == 7 {
        Ok(51)
    } else if book == &BibleBook::IKings && *chapter == 8 {
        Ok(66)
    } else if book == &BibleBook::IKings && *chapter == 9 {
        Ok(28)
    } else if book == &BibleBook::IKings && *chapter == 10 {
        Ok(29)
    } else if book == &BibleBook::IKings && *chapter == 11 {
        Ok(43)
    } else if book == &BibleBook::IKings && *chapter == 12 {
        Ok(33)
    } else if book == &BibleBook::IKings && *chapter == 13 {
        Ok(34)
    } else if book == &BibleBook::IKings && *chapter == 14 {
        Ok(31)
    } else if book == &BibleBook::IKings && *chapter == 15 {
        Ok(34)
    } else if book == &BibleBook::IKings && *chapter == 16 {
        Ok(34)
    } else if book == &BibleBook::IKings && *chapter == 17 {
        Ok(24)
    } else if book == &BibleBook::IKings && *chapter == 18 {
        Ok(46)
    } else if book == &BibleBook::IKings && *chapter == 19 {
        Ok(21)
    } else if book == &BibleBook::IKings && *chapter == 20 {
        Ok(43)
    } else if book == &BibleBook::IKings && *chapter == 21 {
        Ok(29)
    } else if book == &BibleBook::IKings && *chapter == 22 {
        Ok(53)
    } else if book == &BibleBook::IIKings && *chapter == 1 {
        Ok(18)
    } else if book == &BibleBook::IIKings && *chapter == 2 {
        Ok(25)
    } else if book == &BibleBook::IIKings && *chapter == 3 {
        Ok(27)
    } else if book == &BibleBook::IIKings && *chapter == 4 {
        Ok(44)
    } else if book == &BibleBook::IIKings && *chapter == 5 {
        Ok(27)
    } else if book == &BibleBook::IIKings && *chapter == 6 {
        Ok(33)
    } else if book == &BibleBook::IIKings && *chapter == 7 {
        Ok(20)
    } else if book == &BibleBook::IIKings && *chapter == 8 {
        Ok(29)
    } else if book == &BibleBook::IIKings && *chapter == 9 {
        Ok(37)
    } else if book == &BibleBook::IIKings && *chapter == 10 {
        Ok(36)
    } else if book == &BibleBook::IIKings && *chapter == 11 {
        Ok(21)
    } else if book == &BibleBook::IIKings && *chapter == 12 {
        Ok(21)
    } else if book == &BibleBook::IIKings && *chapter == 13 {
        Ok(25)
    } else if book == &BibleBook::IIKings && *chapter == 14 {
        Ok(29)
    } else if book == &BibleBook::IIKings && *chapter == 15 {
        Ok(38)
    } else if book == &BibleBook::IIKings && *chapter == 16 {
        Ok(20)
    } else if book == &BibleBook::IIKings && *chapter == 17 {
        Ok(41)
    } else if book == &BibleBook::IIKings && *chapter == 18 {
        Ok(37)
    } else if book == &BibleBook::IIKings && *chapter == 19 {
        Ok(37)
    } else if book == &BibleBook::IIKings && *chapter == 20 {
        Ok(21)
    } else if book == &BibleBook::IIKings && *chapter == 21 {
        Ok(26)
    } else if book == &BibleBook::IIKings && *chapter == 22 {
        Ok(20)
    } else if book == &BibleBook::IIKings && *chapter == 23 {
        Ok(37)
    } else if book == &BibleBook::IIKings && *chapter == 24 {
        Ok(20)
    } else if book == &BibleBook::IIKings && *chapter == 25 {
        Ok(30)
    } else if book == &BibleBook::IChronicles && *chapter == 1 {
        Ok(54)
    } else if book == &BibleBook::IChronicles && *chapter == 2 {
        Ok(55)
    } else if book == &BibleBook::IChronicles && *chapter == 3 {
        Ok(24)
    } else if book == &BibleBook::IChronicles && *chapter == 4 {
        Ok(43)
    } else if book == &BibleBook::IChronicles && *chapter == 5 {
        Ok(26)
    } else if book == &BibleBook::IChronicles && *chapter == 6 {
        Ok(81)
    } else if book == &BibleBook::IChronicles && *chapter == 7 {
        Ok(40)
    } else if book == &BibleBook::IChronicles && *chapter == 8 {
        Ok(40)
    } else if book == &BibleBook::IChronicles && *chapter == 9 {
        Ok(44)
    } else if book == &BibleBook::IChronicles && *chapter == 10 {
        Ok(14)
    } else if book == &BibleBook::IChronicles && *chapter == 11 {
        Ok(47)
    } else if book == &BibleBook::IChronicles && *chapter == 12 {
        Ok(40)
    } else if book == &BibleBook::IChronicles && *chapter == 13 {
        Ok(14)
    } else if book == &BibleBook::IChronicles && *chapter == 14 {
        Ok(17)
    } else if book == &BibleBook::IChronicles && *chapter == 15 {
        Ok(29)
    } else if book == &BibleBook::IChronicles && *chapter == 16 {
        Ok(43)
    } else if book == &BibleBook::IChronicles && *chapter == 17 {
        Ok(27)
    } else if book == &BibleBook::IChronicles && *chapter == 18 {
        Ok(17)
    } else if book == &BibleBook::IChronicles && *chapter == 19 {
        Ok(19)
    } else if book == &BibleBook::IChronicles && *chapter == 20 {
        Ok(8)
    } else if book == &BibleBook::IChronicles && *chapter == 21 {
        Ok(30)
    } else if book == &BibleBook::IChronicles && *chapter == 22 {
        Ok(19)
    } else if book == &BibleBook::IChronicles && *chapter == 23 {
        Ok(32)
    } else if book == &BibleBook::IChronicles && *chapter == 24 {
        Ok(31)
    } else if book == &BibleBook::IChronicles && *chapter == 25 {
        Ok(31)
    } else if book == &BibleBook::IChronicles && *chapter == 26 {
        Ok(32)
    } else if book == &BibleBook::IChronicles && *chapter == 27 {
        Ok(34)
    } else if book == &BibleBook::IChronicles && *chapter == 28 {
        Ok(21)
    } else if book == &BibleBook::IChronicles && *chapter == 29 {
        Ok(30)
    } else if book == &BibleBook::IIChronicles && *chapter == 1 {
        Ok(17)
    } else if book == &BibleBook::IIChronicles && *chapter == 2 {
        Ok(18)
    } else if book == &BibleBook::IIChronicles && *chapter == 3 {
        Ok(17)
    } else if book == &BibleBook::IIChronicles && *chapter == 4 {
        Ok(22)
    } else if book == &BibleBook::IIChronicles && *chapter == 5 {
        Ok(14)
    } else if book == &BibleBook::IIChronicles && *chapter == 6 {
        Ok(42)
    } else if book == &BibleBook::IIChronicles && *chapter == 7 {
        Ok(22)
    } else if book == &BibleBook::IIChronicles && *chapter == 8 {
        Ok(18)
    } else if book == &BibleBook::IIChronicles && *chapter == 9 {
        Ok(31)
    } else if book == &BibleBook::IIChronicles && *chapter == 10 {
        Ok(19)
    } else if book == &BibleBook::IIChronicles && *chapter == 11 {
        Ok(23)
    } else if book == &BibleBook::IIChronicles && *chapter == 12 {
        Ok(16)
    } else if book == &BibleBook::IIChronicles && *chapter == 13 {
        Ok(22)
    } else if book == &BibleBook::IIChronicles && *chapter == 14 {
        Ok(15)
    } else if book == &BibleBook::IIChronicles && *chapter == 15 {
        Ok(19)
    } else if book == &BibleBook::IIChronicles && *chapter == 16 {
        Ok(14)
    } else if book == &BibleBook::IIChronicles && *chapter == 17 {
        Ok(19)
    } else if book == &BibleBook::IIChronicles && *chapter == 18 {
        Ok(34)
    } else if book == &BibleBook::IIChronicles && *chapter == 19 {
        Ok(11)
    } else if book == &BibleBook::IIChronicles && *chapter == 20 {
        Ok(37)
    } else if book == &BibleBook::IIChronicles && *chapter == 21 {
        Ok(20)
    } else if book == &BibleBook::IIChronicles && *chapter == 22 {
        Ok(12)
    } else if book == &BibleBook::IIChronicles && *chapter == 23 {
        Ok(21)
    } else if book == &BibleBook::IIChronicles && *chapter == 24 {
        Ok(27)
    } else if book == &BibleBook::IIChronicles && *chapter == 25 {
        Ok(28)
    } else if book == &BibleBook::IIChronicles && *chapter == 26 {
        Ok(23)
    } else if book == &BibleBook::IIChronicles && *chapter == 27 {
        Ok(9)
    } else if book == &BibleBook::IIChronicles && *chapter == 28 {
        Ok(27)
    } else if book == &BibleBook::IIChronicles && *chapter == 29 {
        Ok(36)
    } else if book == &BibleBook::IIChronicles && *chapter == 30 {
        Ok(27)
    } else if book == &BibleBook::IIChronicles && *chapter == 31 {
        Ok(21)
    } else if book == &BibleBook::IIChronicles && *chapter == 32 {
        Ok(33)
    } else if book == &BibleBook::IIChronicles && *chapter == 33 {
        Ok(25)
    } else if book == &BibleBook::IIChronicles && *chapter == 34 {
        Ok(33)
    } else if book == &BibleBook::IIChronicles && *chapter == 35 {
        Ok(27)
    } else if book == &BibleBook::IIChronicles && *chapter == 36 {
        Ok(23)
    } else if book == &BibleBook::Ezra && *chapter == 1 {
        Ok(11)
    } else if book == &BibleBook::Ezra && *chapter == 2 {
        Ok(70)
    } else if book == &BibleBook::Ezra && *chapter == 3 {
        Ok(13)
    } else if book == &BibleBook::Ezra && *chapter == 4 {
        Ok(24)
    } else if book == &BibleBook::Ezra && *chapter == 5 {
        Ok(17)
    } else if book == &BibleBook::Ezra && *chapter == 6 {
        Ok(22)
    } else if book == &BibleBook::Ezra && *chapter == 7 {
        Ok(28)
    } else if book == &BibleBook::Ezra && *chapter == 8 {
        Ok(36)
    } else if book == &BibleBook::Ezra && *chapter == 9 {
        Ok(15)
    } else if book == &BibleBook::Ezra && *chapter == 10 {
        Ok(44)
    } else if book == &BibleBook::Nehemiah && *chapter == 1 {
        Ok(11)
    } else if book == &BibleBook::Nehemiah && *chapter == 2 {
        Ok(20)
    } else if book == &BibleBook::Nehemiah && *chapter == 3 {
        Ok(32)
    } else if book == &BibleBook::Nehemiah && *chapter == 4 {
        Ok(23)
    } else if book == &BibleBook::Nehemiah && *chapter == 5 {
        Ok(19)
    } else if book == &BibleBook::Nehemiah && *chapter == 6 {
        Ok(19)
    } else if book == &BibleBook::Nehemiah && *chapter == 7 {
        Ok(73)
    } else if book == &BibleBook::Nehemiah && *chapter == 8 {
        Ok(18)
    } else if book == &BibleBook::Nehemiah && *chapter == 9 {
        Ok(38)
    } else if book == &BibleBook::Nehemiah && *chapter == 10 {
        Ok(39)
    } else if book == &BibleBook::Nehemiah && *chapter == 11 {
        Ok(36)
    } else if book == &BibleBook::Nehemiah && *chapter == 12 {
        Ok(47)
    } else if book == &BibleBook::Nehemiah && *chapter == 13 {
        Ok(31)
    } else if book == &BibleBook::Esther && *chapter == 1 {
        Ok(22)
    } else if book == &BibleBook::Esther && *chapter == 2 {
        Ok(23)
    } else if book == &BibleBook::Esther && *chapter == 3 {
        Ok(15)
    } else if book == &BibleBook::Esther && *chapter == 4 {
        Ok(17)
    } else if book == &BibleBook::Esther && *chapter == 5 {
        Ok(14)
    } else if book == &BibleBook::Esther && *chapter == 6 {
        Ok(14)
    } else if book == &BibleBook::Esther && *chapter == 7 {
        Ok(10)
    } else if book == &BibleBook::Esther && *chapter == 8 {
        Ok(17)
    } else if book == &BibleBook::Esther && *chapter == 9 {
        Ok(32)
    } else if book == &BibleBook::Esther && *chapter == 10 {
        Ok(3)
    } else if book == &BibleBook::Job && *chapter == 1 {
        Ok(22)
    } else if book == &BibleBook::Job && *chapter == 2 {
        Ok(13)
    } else if book == &BibleBook::Job && *chapter == 3 {
        Ok(26)
    } else if book == &BibleBook::Job && *chapter == 4 {
        Ok(21)
    } else if book == &BibleBook::Job && *chapter == 5 {
        Ok(27)
    } else if book == &BibleBook::Job && *chapter == 6 {
        Ok(30)
    } else if book == &BibleBook::Job && *chapter == 7 {
        Ok(21)
    } else if book == &BibleBook::Job && *chapter == 8 {
        Ok(22)
    } else if book == &BibleBook::Job && *chapter == 9 {
        Ok(35)
    } else if book == &BibleBook::Job && *chapter == 10 {
        Ok(22)
    } else if book == &BibleBook::Job && *chapter == 11 {
        Ok(20)
    } else if book == &BibleBook::Job && *chapter == 12 {
        Ok(25)
    } else if book == &BibleBook::Job && *chapter == 13 {
        Ok(28)
    } else if book == &BibleBook::Job && *chapter == 14 {
        Ok(22)
    } else if book == &BibleBook::Job && *chapter == 15 {
        Ok(35)
    } else if book == &BibleBook::Job && *chapter == 16 {
        Ok(22)
    } else if book == &BibleBook::Job && *chapter == 17 {
        Ok(16)
    } else if book == &BibleBook::Job && *chapter == 18 {
        Ok(21)
    } else if book == &BibleBook::Job && *chapter == 19 {
        Ok(29)
    } else if book == &BibleBook::Job && *chapter == 20 {
        Ok(29)
    } else if book == &BibleBook::Job && *chapter == 21 {
        Ok(34)
    } else if book == &BibleBook::Job && *chapter == 22 {
        Ok(30)
    } else if book == &BibleBook::Job && *chapter == 23 {
        Ok(17)
    } else if book == &BibleBook::Job && *chapter == 24 {
        Ok(25)
    } else if book == &BibleBook::Job && *chapter == 25 {
        Ok(6)
    } else if book == &BibleBook::Job && *chapter == 26 {
        Ok(14)
    } else if book == &BibleBook::Job && *chapter == 27 {
        Ok(23)
    } else if book == &BibleBook::Job && *chapter == 28 {
        Ok(28)
    } else if book == &BibleBook::Job && *chapter == 29 {
        Ok(25)
    } else if book == &BibleBook::Job && *chapter == 30 {
        Ok(31)
    } else if book == &BibleBook::Job && *chapter == 31 {
        Ok(40)
    } else if book == &BibleBook::Job && *chapter == 32 {
        Ok(22)
    } else if book == &BibleBook::Job && *chapter == 33 {
        Ok(33)
    } else if book == &BibleBook::Job && *chapter == 34 {
        Ok(37)
    } else if book == &BibleBook::Job && *chapter == 35 {
        Ok(16)
    } else if book == &BibleBook::Job && *chapter == 36 {
        Ok(33)
    } else if book == &BibleBook::Job && *chapter == 37 {
        Ok(24)
    } else if book == &BibleBook::Job && *chapter == 38 {
        Ok(41)
    } else if book == &BibleBook::Job && *chapter == 39 {
        Ok(30)
    } else if book == &BibleBook::Job && *chapter == 40 {
        Ok(24)
    } else if book == &BibleBook::Job && *chapter == 41 {
        Ok(34)
    } else if book == &BibleBook::Job && *chapter == 42 {
        Ok(17)
    } else if book == &BibleBook::Psalm && *chapter == 1 {
        Ok(6)
    } else if book == &BibleBook::Psalm && *chapter == 2 {
        Ok(12)
    } else if book == &BibleBook::Psalm && *chapter == 3 {
        Ok(8)
    } else if book == &BibleBook::Psalm && *chapter == 4 {
        Ok(8)
    } else if book == &BibleBook::Psalm && *chapter == 5 {
        Ok(12)
    } else if book == &BibleBook::Psalm && *chapter == 6 {
        Ok(10)
    } else if book == &BibleBook::Psalm && *chapter == 7 {
        Ok(17)
    } else if book == &BibleBook::Psalm && *chapter == 8 {
        Ok(9)
    } else if book == &BibleBook::Psalm && *chapter == 9 {
        Ok(20)
    } else if book == &BibleBook::Psalm && *chapter == 10 {
        Ok(18)
    } else if book == &BibleBook::Psalm && *chapter == 11 {
        Ok(7)
    } else if book == &BibleBook::Psalm && *chapter == 12 {
        Ok(8)
    } else if book == &BibleBook::Psalm && *chapter == 13 {
        Ok(6)
    } else if book == &BibleBook::Psalm && *chapter == 14 {
        Ok(7)
    } else if book == &BibleBook::Psalm && *chapter == 15 {
        Ok(5)
    } else if book == &BibleBook::Psalm && *chapter == 16 {
        Ok(11)
    } else if book == &BibleBook::Psalm && *chapter == 17 {
        Ok(15)
    } else if book == &BibleBook::Psalm && *chapter == 18 {
        Ok(50)
    } else if book == &BibleBook::Psalm && *chapter == 19 {
        Ok(14)
    } else if book == &BibleBook::Psalm && *chapter == 20 {
        Ok(9)
    } else if book == &BibleBook::Psalm && *chapter == 21 {
        Ok(13)
    } else if book == &BibleBook::Psalm && *chapter == 22 {
        Ok(31)
    } else if book == &BibleBook::Psalm && *chapter == 23 {
        Ok(6)
    } else if book == &BibleBook::Psalm && *chapter == 24 {
        Ok(10)
    } else if book == &BibleBook::Psalm && *chapter == 25 {
        Ok(22)
    } else if book == &BibleBook::Psalm && *chapter == 26 {
        Ok(12)
    } else if book == &BibleBook::Psalm && *chapter == 27 {
        Ok(14)
    } else if book == &BibleBook::Psalm && *chapter == 28 {
        Ok(9)
    } else if book == &BibleBook::Psalm && *chapter == 29 {
        Ok(11)
    } else if book == &BibleBook::Psalm && *chapter == 30 {
        Ok(12)
    } else if book == &BibleBook::Psalm && *chapter == 31 {
        Ok(24)
    } else if book == &BibleBook::Psalm && *chapter == 32 {
        Ok(11)
    } else if book == &BibleBook::Psalm && *chapter == 33 {
        Ok(22)
    } else if book == &BibleBook::Psalm && *chapter == 34 {
        Ok(22)
    } else if book == &BibleBook::Psalm && *chapter == 35 {
        Ok(28)
    } else if book == &BibleBook::Psalm && *chapter == 36 {
        Ok(12)
    } else if book == &BibleBook::Psalm && *chapter == 37 {
        Ok(40)
    } else if book == &BibleBook::Psalm && *chapter == 38 {
        Ok(22)
    } else if book == &BibleBook::Psalm && *chapter == 39 {
        Ok(13)
    } else if book == &BibleBook::Psalm && *chapter == 40 {
        Ok(17)
    } else if book == &BibleBook::Psalm && *chapter == 41 {
        Ok(13)
    } else if book == &BibleBook::Psalm && *chapter == 42 {
        Ok(11)
    } else if book == &BibleBook::Psalm && *chapter == 43 {
        Ok(5)
    } else if book == &BibleBook::Psalm && *chapter == 44 {
        Ok(26)
    } else if book == &BibleBook::Psalm && *chapter == 45 {
        Ok(17)
    } else if book == &BibleBook::Psalm && *chapter == 46 {
        Ok(11)
    } else if book == &BibleBook::Psalm && *chapter == 47 {
        Ok(9)
    } else if book == &BibleBook::Psalm && *chapter == 48 {
        Ok(14)
    } else if book == &BibleBook::Psalm && *chapter == 49 {
        Ok(20)
    } else if book == &BibleBook::Psalm && *chapter == 50 {
        Ok(23)
    } else if book == &BibleBook::Psalm && *chapter == 51 {
        Ok(19)
    } else if book == &BibleBook::Psalm && *chapter == 52 {
        Ok(9)
    } else if book == &BibleBook::Psalm && *chapter == 53 {
        Ok(6)
    } else if book == &BibleBook::Psalm && *chapter == 54 {
        Ok(7)
    } else if book == &BibleBook::Psalm && *chapter == 55 {
        Ok(23)
    } else if book == &BibleBook::Psalm && *chapter == 56 {
        Ok(13)
    } else if book == &BibleBook::Psalm && *chapter == 57 {
        Ok(11)
    } else if book == &BibleBook::Psalm && *chapter == 58 {
        Ok(11)
    } else if book == &BibleBook::Psalm && *chapter == 59 {
        Ok(17)
    } else if book == &BibleBook::Psalm && *chapter == 60 {
        Ok(12)
    } else if book == &BibleBook::Psalm && *chapter == 61 {
        Ok(8)
    } else if book == &BibleBook::Psalm && *chapter == 62 {
        Ok(12)
    } else if book == &BibleBook::Psalm && *chapter == 63 {
        Ok(11)
    } else if book == &BibleBook::Psalm && *chapter == 64 {
        Ok(10)
    } else if book == &BibleBook::Psalm && *chapter == 65 {
        Ok(13)
    } else if book == &BibleBook::Psalm && *chapter == 66 {
        Ok(20)
    } else if book == &BibleBook::Psalm && *chapter == 67 {
        Ok(7)
    } else if book == &BibleBook::Psalm && *chapter == 68 {
        Ok(35)
    } else if book == &BibleBook::Psalm && *chapter == 69 {
        Ok(36)
    } else if book == &BibleBook::Psalm && *chapter == 70 {
        Ok(5)
    } else if book == &BibleBook::Psalm && *chapter == 71 {
        Ok(24)
    } else if book == &BibleBook::Psalm && *chapter == 72 {
        Ok(20)
    } else if book == &BibleBook::Psalm && *chapter == 73 {
        Ok(28)
    } else if book == &BibleBook::Psalm && *chapter == 74 {
        Ok(23)
    } else if book == &BibleBook::Psalm && *chapter == 75 {
        Ok(10)
    } else if book == &BibleBook::Psalm && *chapter == 76 {
        Ok(12)
    } else if book == &BibleBook::Psalm && *chapter == 77 {
        Ok(20)
    } else if book == &BibleBook::Psalm && *chapter == 78 {
        Ok(72)
    } else if book == &BibleBook::Psalm && *chapter == 79 {
        Ok(13)
    } else if book == &BibleBook::Psalm && *chapter == 80 {
        Ok(19)
    } else if book == &BibleBook::Psalm && *chapter == 81 {
        Ok(16)
    } else if book == &BibleBook::Psalm && *chapter == 82 {
        Ok(8)
    } else if book == &BibleBook::Psalm && *chapter == 83 {
        Ok(18)
    } else if book == &BibleBook::Psalm && *chapter == 84 {
        Ok(12)
    } else if book == &BibleBook::Psalm && *chapter == 85 {
        Ok(13)
    } else if book == &BibleBook::Psalm && *chapter == 86 {
        Ok(17)
    } else if book == &BibleBook::Psalm && *chapter == 87 {
        Ok(7)
    } else if book == &BibleBook::Psalm && *chapter == 88 {
        Ok(18)
    } else if book == &BibleBook::Psalm && *chapter == 89 {
        Ok(52)
    } else if book == &BibleBook::Psalm && *chapter == 90 {
        Ok(17)
    } else if book == &BibleBook::Psalm && *chapter == 91 {
        Ok(16)
    } else if book == &BibleBook::Psalm && *chapter == 92 {
        Ok(15)
    } else if book == &BibleBook::Psalm && *chapter == 93 {
        Ok(5)
    } else if book == &BibleBook::Psalm && *chapter == 94 {
        Ok(23)
    } else if book == &BibleBook::Psalm && *chapter == 95 {
        Ok(11)
    } else if book == &BibleBook::Psalm && *chapter == 96 {
        Ok(13)
    } else if book == &BibleBook::Psalm && *chapter == 97 {
        Ok(12)
    } else if book == &BibleBook::Psalm && *chapter == 98 {
        Ok(9)
    } else if book == &BibleBook::Psalm && *chapter == 99 {
        Ok(9)
    } else if book == &BibleBook::Psalm && *chapter == 100 {
        Ok(5)
    } else if book == &BibleBook::Psalm && *chapter == 101 {
        Ok(8)
    } else if book == &BibleBook::Psalm && *chapter == 102 {
        Ok(28)
    } else if book == &BibleBook::Psalm && *chapter == 103 {
        Ok(22)
    } else if book == &BibleBook::Psalm && *chapter == 104 {
        Ok(35)
    } else if book == &BibleBook::Psalm && *chapter == 105 {
        Ok(45)
    } else if book == &BibleBook::Psalm && *chapter == 106 {
        Ok(48)
    } else if book == &BibleBook::Psalm && *chapter == 107 {
        Ok(43)
    } else if book == &BibleBook::Psalm && *chapter == 108 {
        Ok(13)
    } else if book == &BibleBook::Psalm && *chapter == 109 {
        Ok(31)
    } else if book == &BibleBook::Psalm && *chapter == 110 {
        Ok(7)
    } else if book == &BibleBook::Psalm && *chapter == 111 {
        Ok(10)
    } else if book == &BibleBook::Psalm && *chapter == 112 {
        Ok(10)
    } else if book == &BibleBook::Psalm && *chapter == 113 {
        Ok(9)
    } else if book == &BibleBook::Psalm && *chapter == 114 {
        Ok(8)
    } else if book == &BibleBook::Psalm && *chapter == 115 {
        Ok(18)
    } else if book == &BibleBook::Psalm && *chapter == 116 {
        Ok(19)
    } else if book == &BibleBook::Psalm && *chapter == 117 {
        Ok(2)
    } else if book == &BibleBook::Psalm && *chapter == 118 {
        Ok(29)
    } else if book == &BibleBook::Psalm && *chapter == 119 {
        Ok(176)
    } else if book == &BibleBook::Psalm && *chapter == 120 {
        Ok(7)
    } else if book == &BibleBook::Psalm && *chapter == 121 {
        Ok(8)
    } else if book == &BibleBook::Psalm && *chapter == 122 {
        Ok(9)
    } else if book == &BibleBook::Psalm && *chapter == 123 {
        Ok(4)
    } else if book == &BibleBook::Psalm && *chapter == 124 {
        Ok(8)
    } else if book == &BibleBook::Psalm && *chapter == 125 {
        Ok(5)
    } else if book == &BibleBook::Psalm && *chapter == 126 {
        Ok(6)
    } else if book == &BibleBook::Psalm && *chapter == 127 {
        Ok(5)
    } else if book == &BibleBook::Psalm && *chapter == 128 {
        Ok(6)
    } else if book == &BibleBook::Psalm && *chapter == 129 {
        Ok(8)
    } else if book == &BibleBook::Psalm && *chapter == 130 {
        Ok(8)
    } else if book == &BibleBook::Psalm && *chapter == 131 {
        Ok(3)
    } else if book == &BibleBook::Psalm && *chapter == 132 {
        Ok(18)
    } else if book == &BibleBook::Psalm && *chapter == 133 {
        Ok(3)
    } else if book == &BibleBook::Psalm && *chapter == 134 {
        Ok(3)
    } else if book == &BibleBook::Psalm && *chapter == 135 {
        Ok(21)
    } else if book == &BibleBook::Psalm && *chapter == 136 {
        Ok(26)
    } else if book == &BibleBook::Psalm && *chapter == 137 {
        Ok(9)
    } else if book == &BibleBook::Psalm && *chapter == 138 {
        Ok(8)
    } else if book == &BibleBook::Psalm && *chapter == 139 {
        Ok(24)
    } else if book == &BibleBook::Psalm && *chapter == 140 {
        Ok(13)
    } else if book == &BibleBook::Psalm && *chapter == 141 {
        Ok(10)
    } else if book == &BibleBook::Psalm && *chapter == 142 {
        Ok(7)
    } else if book == &BibleBook::Psalm && *chapter == 143 {
        Ok(12)
    } else if book == &BibleBook::Psalm && *chapter == 144 {
        Ok(15)
    } else if book == &BibleBook::Psalm && *chapter == 145 {
        Ok(21)
    } else if book == &BibleBook::Psalm && *chapter == 146 {
        Ok(10)
    } else if book == &BibleBook::Psalm && *chapter == 147 {
        Ok(20)
    } else if book == &BibleBook::Psalm && *chapter == 148 {
        Ok(14)
    } else if book == &BibleBook::Psalm && *chapter == 149 {
        Ok(9)
    } else if book == &BibleBook::Psalm && *chapter == 150 {
        Ok(6)
    } else if book == &BibleBook::Proverbs && *chapter == 1 {
        Ok(33)
    } else if book == &BibleBook::Proverbs && *chapter == 2 {
        Ok(22)
    } else if book == &BibleBook::Proverbs && *chapter == 3 {
        Ok(35)
    } else if book == &BibleBook::Proverbs && *chapter == 4 {
        Ok(27)
    } else if book == &BibleBook::Proverbs && *chapter == 5 {
        Ok(23)
    } else if book == &BibleBook::Proverbs && *chapter == 6 {
        Ok(35)
    } else if book == &BibleBook::Proverbs && *chapter == 7 {
        Ok(27)
    } else if book == &BibleBook::Proverbs && *chapter == 8 {
        Ok(36)
    } else if book == &BibleBook::Proverbs && *chapter == 9 {
        Ok(18)
    } else if book == &BibleBook::Proverbs && *chapter == 10 {
        Ok(32)
    } else if book == &BibleBook::Proverbs && *chapter == 11 {
        Ok(31)
    } else if book == &BibleBook::Proverbs && *chapter == 12 {
        Ok(28)
    } else if book == &BibleBook::Proverbs && *chapter == 13 {
        Ok(25)
    } else if book == &BibleBook::Proverbs && *chapter == 14 {
        Ok(35)
    } else if book == &BibleBook::Proverbs && *chapter == 15 {
        Ok(33)
    } else if book == &BibleBook::Proverbs && *chapter == 16 {
        Ok(33)
    } else if book == &BibleBook::Proverbs && *chapter == 17 {
        Ok(28)
    } else if book == &BibleBook::Proverbs && *chapter == 18 {
        Ok(24)
    } else if book == &BibleBook::Proverbs && *chapter == 19 {
        Ok(29)
    } else if book == &BibleBook::Proverbs && *chapter == 20 {
        Ok(30)
    } else if book == &BibleBook::Proverbs && *chapter == 21 {
        Ok(31)
    } else if book == &BibleBook::Proverbs && *chapter == 22 {
        Ok(29)
    } else if book == &BibleBook::Proverbs && *chapter == 23 {
        Ok(35)
    } else if book == &BibleBook::Proverbs && *chapter == 24 {
        Ok(34)
    } else if book == &BibleBook::Proverbs && *chapter == 25 {
        Ok(28)
    } else if book == &BibleBook::Proverbs && *chapter == 26 {
        Ok(28)
    } else if book == &BibleBook::Proverbs && *chapter == 27 {
        Ok(27)
    } else if book == &BibleBook::Proverbs && *chapter == 28 {
        Ok(28)
    } else if book == &BibleBook::Proverbs && *chapter == 29 {
        Ok(27)
    } else if book == &BibleBook::Proverbs && *chapter == 30 {
        Ok(33)
    } else if book == &BibleBook::Proverbs && *chapter == 31 {
        Ok(31)
    } else if book == &BibleBook::Ecclesiastes && *chapter == 1 {
        Ok(18)
    } else if book == &BibleBook::Ecclesiastes && *chapter == 2 {
        Ok(26)
    } else if book == &BibleBook::Ecclesiastes && *chapter == 3 {
        Ok(22)
    } else if book == &BibleBook::Ecclesiastes && *chapter == 4 {
        Ok(16)
    } else if book == &BibleBook::Ecclesiastes && *chapter == 5 {
        Ok(20)
    } else if book == &BibleBook::Ecclesiastes && *chapter == 6 {
        Ok(12)
    } else if book == &BibleBook::Ecclesiastes && *chapter == 7 {
        Ok(29)
    } else if book == &BibleBook::Ecclesiastes && *chapter == 8 {
        Ok(17)
    } else if book == &BibleBook::Ecclesiastes && *chapter == 9 {
        Ok(18)
    } else if book == &BibleBook::Ecclesiastes && *chapter == 10 {
        Ok(20)
    } else if book == &BibleBook::Ecclesiastes && *chapter == 11 {
        Ok(10)
    } else if book == &BibleBook::Ecclesiastes && *chapter == 12 {
        Ok(14)
    } else if book == &BibleBook::SongofSolomon && *chapter == 1 {
        Ok(17)
    } else if book == &BibleBook::SongofSolomon && *chapter == 2 {
        Ok(17)
    } else if book == &BibleBook::SongofSolomon && *chapter == 3 {
        Ok(11)
    } else if book == &BibleBook::SongofSolomon && *chapter == 4 {
        Ok(16)
    } else if book == &BibleBook::SongofSolomon && *chapter == 5 {
        Ok(16)
    } else if book == &BibleBook::SongofSolomon && *chapter == 6 {
        Ok(13)
    } else if book == &BibleBook::SongofSolomon && *chapter == 7 {
        Ok(13)
    } else if book == &BibleBook::SongofSolomon && *chapter == 8 {
        Ok(14)
    } else if book == &BibleBook::Isaiah && *chapter == 1 {
        Ok(31)
    } else if book == &BibleBook::Isaiah && *chapter == 2 {
        Ok(22)
    } else if book == &BibleBook::Isaiah && *chapter == 3 {
        Ok(26)
    } else if book == &BibleBook::Isaiah && *chapter == 4 {
        Ok(6)
    } else if book == &BibleBook::Isaiah && *chapter == 5 {
        Ok(30)
    } else if book == &BibleBook::Isaiah && *chapter == 6 {
        Ok(13)
    } else if book == &BibleBook::Isaiah && *chapter == 7 {
        Ok(25)
    } else if book == &BibleBook::Isaiah && *chapter == 8 {
        Ok(22)
    } else if book == &BibleBook::Isaiah && *chapter == 9 {
        Ok(21)
    } else if book == &BibleBook::Isaiah && *chapter == 10 {
        Ok(34)
    } else if book == &BibleBook::Isaiah && *chapter == 11 {
        Ok(16)
    } else if book == &BibleBook::Isaiah && *chapter == 12 {
        Ok(6)
    } else if book == &BibleBook::Isaiah && *chapter == 13 {
        Ok(22)
    } else if book == &BibleBook::Isaiah && *chapter == 14 {
        Ok(32)
    } else if book == &BibleBook::Isaiah && *chapter == 15 {
        Ok(9)
    } else if book == &BibleBook::Isaiah && *chapter == 16 {
        Ok(14)
    } else if book == &BibleBook::Isaiah && *chapter == 17 {
        Ok(14)
    } else if book == &BibleBook::Isaiah && *chapter == 18 {
        Ok(7)
    } else if book == &BibleBook::Isaiah && *chapter == 19 {
        Ok(25)
    } else if book == &BibleBook::Isaiah && *chapter == 20 {
        Ok(6)
    } else if book == &BibleBook::Isaiah && *chapter == 21 {
        Ok(17)
    } else if book == &BibleBook::Isaiah && *chapter == 22 {
        Ok(25)
    } else if book == &BibleBook::Isaiah && *chapter == 23 {
        Ok(18)
    } else if book == &BibleBook::Isaiah && *chapter == 24 {
        Ok(23)
    } else if book == &BibleBook::Isaiah && *chapter == 25 {
        Ok(12)
    } else if book == &BibleBook::Isaiah && *chapter == 26 {
        Ok(21)
    } else if book == &BibleBook::Isaiah && *chapter == 27 {
        Ok(13)
    } else if book == &BibleBook::Isaiah && *chapter == 28 {
        Ok(29)
    } else if book == &BibleBook::Isaiah && *chapter == 29 {
        Ok(24)
    } else if book == &BibleBook::Isaiah && *chapter == 30 {
        Ok(33)
    } else if book == &BibleBook::Isaiah && *chapter == 31 {
        Ok(9)
    } else if book == &BibleBook::Isaiah && *chapter == 32 {
        Ok(20)
    } else if book == &BibleBook::Isaiah && *chapter == 33 {
        Ok(24)
    } else if book == &BibleBook::Isaiah && *chapter == 34 {
        Ok(17)
    } else if book == &BibleBook::Isaiah && *chapter == 35 {
        Ok(10)
    } else if book == &BibleBook::Isaiah && *chapter == 36 {
        Ok(22)
    } else if book == &BibleBook::Isaiah && *chapter == 37 {
        Ok(38)
    } else if book == &BibleBook::Isaiah && *chapter == 38 {
        Ok(22)
    } else if book == &BibleBook::Isaiah && *chapter == 39 {
        Ok(8)
    } else if book == &BibleBook::Isaiah && *chapter == 40 {
        Ok(31)
    } else if book == &BibleBook::Isaiah && *chapter == 41 {
        Ok(29)
    } else if book == &BibleBook::Isaiah && *chapter == 42 {
        Ok(25)
    } else if book == &BibleBook::Isaiah && *chapter == 43 {
        Ok(28)
    } else if book == &BibleBook::Isaiah && *chapter == 44 {
        Ok(28)
    } else if book == &BibleBook::Isaiah && *chapter == 45 {
        Ok(25)
    } else if book == &BibleBook::Isaiah && *chapter == 46 {
        Ok(13)
    } else if book == &BibleBook::Isaiah && *chapter == 47 {
        Ok(15)
    } else if book == &BibleBook::Isaiah && *chapter == 48 {
        Ok(22)
    } else if book == &BibleBook::Isaiah && *chapter == 49 {
        Ok(26)
    } else if book == &BibleBook::Isaiah && *chapter == 50 {
        Ok(11)
    } else if book == &BibleBook::Isaiah && *chapter == 51 {
        Ok(23)
    } else if book == &BibleBook::Isaiah && *chapter == 52 {
        Ok(15)
    } else if book == &BibleBook::Isaiah && *chapter == 53 {
        Ok(12)
    } else if book == &BibleBook::Isaiah && *chapter == 54 {
        Ok(17)
    } else if book == &BibleBook::Isaiah && *chapter == 55 {
        Ok(13)
    } else if book == &BibleBook::Isaiah && *chapter == 56 {
        Ok(12)
    } else if book == &BibleBook::Isaiah && *chapter == 57 {
        Ok(21)
    } else if book == &BibleBook::Isaiah && *chapter == 58 {
        Ok(14)
    } else if book == &BibleBook::Isaiah && *chapter == 59 {
        Ok(21)
    } else if book == &BibleBook::Isaiah && *chapter == 60 {
        Ok(22)
    } else if book == &BibleBook::Isaiah && *chapter == 61 {
        Ok(11)
    } else if book == &BibleBook::Isaiah && *chapter == 62 {
        Ok(12)
    } else if book == &BibleBook::Isaiah && *chapter == 63 {
        Ok(19)
    } else if book == &BibleBook::Isaiah && *chapter == 64 {
        Ok(12)
    } else if book == &BibleBook::Isaiah && *chapter == 65 {
        Ok(25)
    } else if book == &BibleBook::Isaiah && *chapter == 66 {
        Ok(24)
    } else if book == &BibleBook::Jeremiah && *chapter == 1 {
        Ok(19)
    } else if book == &BibleBook::Jeremiah && *chapter == 2 {
        Ok(37)
    } else if book == &BibleBook::Jeremiah && *chapter == 3 {
        Ok(25)
    } else if book == &BibleBook::Jeremiah && *chapter == 4 {
        Ok(31)
    } else if book == &BibleBook::Jeremiah && *chapter == 5 {
        Ok(31)
    } else if book == &BibleBook::Jeremiah && *chapter == 6 {
        Ok(30)
    } else if book == &BibleBook::Jeremiah && *chapter == 7 {
        Ok(34)
    } else if book == &BibleBook::Jeremiah && *chapter == 8 {
        Ok(22)
    } else if book == &BibleBook::Jeremiah && *chapter == 9 {
        Ok(26)
    } else if book == &BibleBook::Jeremiah && *chapter == 10 {
        Ok(25)
    } else if book == &BibleBook::Jeremiah && *chapter == 11 {
        Ok(23)
    } else if book == &BibleBook::Jeremiah && *chapter == 12 {
        Ok(17)
    } else if book == &BibleBook::Jeremiah && *chapter == 13 {
        Ok(27)
    } else if book == &BibleBook::Jeremiah && *chapter == 14 {
        Ok(22)
    } else if book == &BibleBook::Jeremiah && *chapter == 15 {
        Ok(21)
    } else if book == &BibleBook::Jeremiah && *chapter == 16 {
        Ok(21)
    } else if book == &BibleBook::Jeremiah && *chapter == 17 {
        Ok(27)
    } else if book == &BibleBook::Jeremiah && *chapter == 18 {
        Ok(23)
    } else if book == &BibleBook::Jeremiah && *chapter == 19 {
        Ok(15)
    } else if book == &BibleBook::Jeremiah && *chapter == 20 {
        Ok(18)
    } else if book == &BibleBook::Jeremiah && *chapter == 21 {
        Ok(14)
    } else if book == &BibleBook::Jeremiah && *chapter == 22 {
        Ok(30)
    } else if book == &BibleBook::Jeremiah && *chapter == 23 {
        Ok(40)
    } else if book == &BibleBook::Jeremiah && *chapter == 24 {
        Ok(10)
    } else if book == &BibleBook::Jeremiah && *chapter == 25 {
        Ok(38)
    } else if book == &BibleBook::Jeremiah && *chapter == 26 {
        Ok(24)
    } else if book == &BibleBook::Jeremiah && *chapter == 27 {
        Ok(22)
    } else if book == &BibleBook::Jeremiah && *chapter == 28 {
        Ok(17)
    } else if book == &BibleBook::Jeremiah && *chapter == 29 {
        Ok(32)
    } else if book == &BibleBook::Jeremiah && *chapter == 30 {
        Ok(24)
    } else if book == &BibleBook::Jeremiah && *chapter == 31 {
        Ok(40)
    } else if book == &BibleBook::Jeremiah && *chapter == 32 {
        Ok(44)
    } else if book == &BibleBook::Jeremiah && *chapter == 33 {
        Ok(26)
    } else if book == &BibleBook::Jeremiah && *chapter == 34 {
        Ok(22)
    } else if book == &BibleBook::Jeremiah && *chapter == 35 {
        Ok(19)
    } else if book == &BibleBook::Jeremiah && *chapter == 36 {
        Ok(32)
    } else if book == &BibleBook::Jeremiah && *chapter == 37 {
        Ok(21)
    } else if book == &BibleBook::Jeremiah && *chapter == 38 {
        Ok(28)
    } else if book == &BibleBook::Jeremiah && *chapter == 39 {
        Ok(18)
    } else if book == &BibleBook::Jeremiah && *chapter == 40 {
        Ok(16)
    } else if book == &BibleBook::Jeremiah && *chapter == 41 {
        Ok(18)
    } else if book == &BibleBook::Jeremiah && *chapter == 42 {
        Ok(22)
    } else if book == &BibleBook::Jeremiah && *chapter == 43 {
        Ok(13)
    } else if book == &BibleBook::Jeremiah && *chapter == 44 {
        Ok(30)
    } else if book == &BibleBook::Jeremiah && *chapter == 45 {
        Ok(5)
    } else if book == &BibleBook::Jeremiah && *chapter == 46 {
        Ok(28)
    } else if book == &BibleBook::Jeremiah && *chapter == 47 {
        Ok(7)
    } else if book == &BibleBook::Jeremiah && *chapter == 48 {
        Ok(47)
    } else if book == &BibleBook::Jeremiah && *chapter == 49 {
        Ok(39)
    } else if book == &BibleBook::Jeremiah && *chapter == 50 {
        Ok(46)
    } else if book == &BibleBook::Jeremiah && *chapter == 51 {
        Ok(64)
    } else if book == &BibleBook::Jeremiah && *chapter == 52 {
        Ok(34)
    } else if book == &BibleBook::Lamentations && *chapter == 1 {
        Ok(22)
    } else if book == &BibleBook::Lamentations && *chapter == 2 {
        Ok(22)
    } else if book == &BibleBook::Lamentations && *chapter == 3 {
        Ok(66)
    } else if book == &BibleBook::Lamentations && *chapter == 4 {
        Ok(22)
    } else if book == &BibleBook::Lamentations && *chapter == 5 {
        Ok(22)
    } else if book == &BibleBook::Ezekiel && *chapter == 1 {
        Ok(28)
    } else if book == &BibleBook::Ezekiel && *chapter == 2 {
        Ok(10)
    } else if book == &BibleBook::Ezekiel && *chapter == 3 {
        Ok(27)
    } else if book == &BibleBook::Ezekiel && *chapter == 4 {
        Ok(17)
    } else if book == &BibleBook::Ezekiel && *chapter == 5 {
        Ok(17)
    } else if book == &BibleBook::Ezekiel && *chapter == 6 {
        Ok(14)
    } else if book == &BibleBook::Ezekiel && *chapter == 7 {
        Ok(27)
    } else if book == &BibleBook::Ezekiel && *chapter == 8 {
        Ok(18)
    } else if book == &BibleBook::Ezekiel && *chapter == 9 {
        Ok(11)
    } else if book == &BibleBook::Ezekiel && *chapter == 10 {
        Ok(22)
    } else if book == &BibleBook::Ezekiel && *chapter == 11 {
        Ok(25)
    } else if book == &BibleBook::Ezekiel && *chapter == 12 {
        Ok(28)
    } else if book == &BibleBook::Ezekiel && *chapter == 13 {
        Ok(23)
    } else if book == &BibleBook::Ezekiel && *chapter == 14 {
        Ok(23)
    } else if book == &BibleBook::Ezekiel && *chapter == 15 {
        Ok(8)
    } else if book == &BibleBook::Ezekiel && *chapter == 16 {
        Ok(63)
    } else if book == &BibleBook::Ezekiel && *chapter == 17 {
        Ok(24)
    } else if book == &BibleBook::Ezekiel && *chapter == 18 {
        Ok(32)
    } else if book == &BibleBook::Ezekiel && *chapter == 19 {
        Ok(14)
    } else if book == &BibleBook::Ezekiel && *chapter == 20 {
        Ok(49)
    } else if book == &BibleBook::Ezekiel && *chapter == 21 {
        Ok(32)
    } else if book == &BibleBook::Ezekiel && *chapter == 22 {
        Ok(31)
    } else if book == &BibleBook::Ezekiel && *chapter == 23 {
        Ok(49)
    } else if book == &BibleBook::Ezekiel && *chapter == 24 {
        Ok(27)
    } else if book == &BibleBook::Ezekiel && *chapter == 25 {
        Ok(17)
    } else if book == &BibleBook::Ezekiel && *chapter == 26 {
        Ok(21)
    } else if book == &BibleBook::Ezekiel && *chapter == 27 {
        Ok(36)
    } else if book == &BibleBook::Ezekiel && *chapter == 28 {
        Ok(26)
    } else if book == &BibleBook::Ezekiel && *chapter == 29 {
        Ok(21)
    } else if book == &BibleBook::Ezekiel && *chapter == 30 {
        Ok(26)
    } else if book == &BibleBook::Ezekiel && *chapter == 31 {
        Ok(18)
    } else if book == &BibleBook::Ezekiel && *chapter == 32 {
        Ok(32)
    } else if book == &BibleBook::Ezekiel && *chapter == 33 {
        Ok(33)
    } else if book == &BibleBook::Ezekiel && *chapter == 34 {
        Ok(31)
    } else if book == &BibleBook::Ezekiel && *chapter == 35 {
        Ok(15)
    } else if book == &BibleBook::Ezekiel && *chapter == 36 {
        Ok(38)
    } else if book == &BibleBook::Ezekiel && *chapter == 37 {
        Ok(28)
    } else if book == &BibleBook::Ezekiel && *chapter == 38 {
        Ok(23)
    } else if book == &BibleBook::Ezekiel && *chapter == 39 {
        Ok(29)
    } else if book == &BibleBook::Ezekiel && *chapter == 40 {
        Ok(49)
    } else if book == &BibleBook::Ezekiel && *chapter == 41 {
        Ok(26)
    } else if book == &BibleBook::Ezekiel && *chapter == 42 {
        Ok(20)
    } else if book == &BibleBook::Ezekiel && *chapter == 43 {
        Ok(27)
    } else if book == &BibleBook::Ezekiel && *chapter == 44 {
        Ok(31)
    } else if book == &BibleBook::Ezekiel && *chapter == 45 {
        Ok(25)
    } else if book == &BibleBook::Ezekiel && *chapter == 46 {
        Ok(24)
    } else if book == &BibleBook::Ezekiel && *chapter == 47 {
        Ok(23)
    } else if book == &BibleBook::Ezekiel && *chapter == 48 {
        Ok(35)
    } else if book == &BibleBook::Daniel && *chapter == 1 {
        Ok(21)
    } else if book == &BibleBook::Daniel && *chapter == 2 {
        Ok(49)
    } else if book == &BibleBook::Daniel && *chapter == 3 {
        Ok(30)
    } else if book == &BibleBook::Daniel && *chapter == 4 {
        Ok(37)
    } else if book == &BibleBook::Daniel && *chapter == 5 {
        Ok(31)
    } else if book == &BibleBook::Daniel && *chapter == 6 {
        Ok(28)
    } else if book == &BibleBook::Daniel && *chapter == 7 {
        Ok(28)
    } else if book == &BibleBook::Daniel && *chapter == 8 {
        Ok(27)
    } else if book == &BibleBook::Daniel && *chapter == 9 {
        Ok(27)
    } else if book == &BibleBook::Daniel && *chapter == 10 {
        Ok(21)
    } else if book == &BibleBook::Daniel && *chapter == 11 {
        Ok(45)
    } else if book == &BibleBook::Daniel && *chapter == 12 {
        Ok(13)
    } else if book == &BibleBook::Hosea && *chapter == 1 {
        Ok(11)
    } else if book == &BibleBook::Hosea && *chapter == 2 {
        Ok(23)
    } else if book == &BibleBook::Hosea && *chapter == 3 {
        Ok(5)
    } else if book == &BibleBook::Hosea && *chapter == 4 {
        Ok(19)
    } else if book == &BibleBook::Hosea && *chapter == 5 {
        Ok(15)
    } else if book == &BibleBook::Hosea && *chapter == 6 {
        Ok(11)
    } else if book == &BibleBook::Hosea && *chapter == 7 {
        Ok(16)
    } else if book == &BibleBook::Hosea && *chapter == 8 {
        Ok(14)
    } else if book == &BibleBook::Hosea && *chapter == 9 {
        Ok(17)
    } else if book == &BibleBook::Hosea && *chapter == 10 {
        Ok(15)
    } else if book == &BibleBook::Hosea && *chapter == 11 {
        Ok(12)
    } else if book == &BibleBook::Hosea && *chapter == 12 {
        Ok(14)
    } else if book == &BibleBook::Hosea && *chapter == 13 {
        Ok(16)
    } else if book == &BibleBook::Hosea && *chapter == 14 {
        Ok(9)
    } else if book == &BibleBook::Joel && *chapter == 1 {
        Ok(20)
    } else if book == &BibleBook::Joel && *chapter == 2 {
        Ok(32)
    } else if book == &BibleBook::Joel && *chapter == 3 {
        Ok(21)
    } else if book == &BibleBook::Amos && *chapter == 1 {
        Ok(15)
    } else if book == &BibleBook::Amos && *chapter == 2 {
        Ok(16)
    } else if book == &BibleBook::Amos && *chapter == 3 {
        Ok(15)
    } else if book == &BibleBook::Amos && *chapter == 4 {
        Ok(13)
    } else if book == &BibleBook::Amos && *chapter == 5 {
        Ok(27)
    } else if book == &BibleBook::Amos && *chapter == 6 {
        Ok(14)
    } else if book == &BibleBook::Amos && *chapter == 7 {
        Ok(17)
    } else if book == &BibleBook::Amos && *chapter == 8 {
        Ok(14)
    } else if book == &BibleBook::Amos && *chapter == 9 {
        Ok(15)
    } else if book == &BibleBook::Obadiah && *chapter == 1 {
        Ok(21)
    } else if book == &BibleBook::Jonah && *chapter == 1 {
        Ok(17)
    } else if book == &BibleBook::Jonah && *chapter == 2 {
        Ok(10)
    } else if book == &BibleBook::Jonah && *chapter == 3 {
        Ok(10)
    } else if book == &BibleBook::Jonah && *chapter == 4 {
        Ok(11)
    } else if book == &BibleBook::Micah && *chapter == 1 {
        Ok(16)
    } else if book == &BibleBook::Micah && *chapter == 2 {
        Ok(13)
    } else if book == &BibleBook::Micah && *chapter == 3 {
        Ok(12)
    } else if book == &BibleBook::Micah && *chapter == 4 {
        Ok(13)
    } else if book == &BibleBook::Micah && *chapter == 5 {
        Ok(15)
    } else if book == &BibleBook::Micah && *chapter == 6 {
        Ok(16)
    } else if book == &BibleBook::Micah && *chapter == 7 {
        Ok(20)
    } else if book == &BibleBook::Nahum && *chapter == 1 {
        Ok(15)
    } else if book == &BibleBook::Nahum && *chapter == 2 {
        Ok(13)
    } else if book == &BibleBook::Nahum && *chapter == 3 {
        Ok(19)
    } else if book == &BibleBook::Habakkuk && *chapter == 1 {
        Ok(17)
    } else if book == &BibleBook::Habakkuk && *chapter == 2 {
        Ok(20)
    } else if book == &BibleBook::Habakkuk && *chapter == 3 {
        Ok(19)
    } else if book == &BibleBook::Zephaniah && *chapter == 1 {
        Ok(18)
    } else if book == &BibleBook::Zephaniah && *chapter == 2 {
        Ok(15)
    } else if book == &BibleBook::Zephaniah && *chapter == 3 {
        Ok(20)
    } else if book == &BibleBook::Haggai && *chapter == 1 {
        Ok(15)
    } else if book == &BibleBook::Haggai && *chapter == 2 {
        Ok(23)
    } else if book == &BibleBook::Zechariah && *chapter == 1 {
        Ok(21)
    } else if book == &BibleBook::Zechariah && *chapter == 2 {
        Ok(13)
    } else if book == &BibleBook::Zechariah && *chapter == 3 {
        Ok(10)
    } else if book == &BibleBook::Zechariah && *chapter == 4 {
        Ok(14)
    } else if book == &BibleBook::Zechariah && *chapter == 5 {
        Ok(11)
    } else if book == &BibleBook::Zechariah && *chapter == 6 {
        Ok(15)
    } else if book == &BibleBook::Zechariah && *chapter == 7 {
        Ok(14)
    } else if book == &BibleBook::Zechariah && *chapter == 8 {
        Ok(23)
    } else if book == &BibleBook::Zechariah && *chapter == 9 {
        Ok(17)
    } else if book == &BibleBook::Zechariah && *chapter == 10 {
        Ok(12)
    } else if book == &BibleBook::Zechariah && *chapter == 11 {
        Ok(17)
    } else if book == &BibleBook::Zechariah && *chapter == 12 {
        Ok(14)
    } else if book == &BibleBook::Zechariah && *chapter == 13 {
        Ok(9)
    } else if book == &BibleBook::Zechariah && *chapter == 14 {
        Ok(21)
    } else if book == &BibleBook::Malachi && *chapter == 1 {
        Ok(14)
    } else if book == &BibleBook::Malachi && *chapter == 2 {
        Ok(17)
    } else if book == &BibleBook::Malachi && *chapter == 3 {
        Ok(18)
    } else if book == &BibleBook::Malachi && *chapter == 4 {
        Ok(6)
    } else if book == &BibleBook::Matthew && *chapter == 1 {
        Ok(25)
    } else if book == &BibleBook::Matthew && *chapter == 2 {
        Ok(23)
    } else if book == &BibleBook::Matthew && *chapter == 3 {
        Ok(17)
    } else if book == &BibleBook::Matthew && *chapter == 4 {
        Ok(25)
    } else if book == &BibleBook::Matthew && *chapter == 5 {
        Ok(48)
    } else if book == &BibleBook::Matthew && *chapter == 6 {
        Ok(34)
    } else if book == &BibleBook::Matthew && *chapter == 7 {
        Ok(29)
    } else if book == &BibleBook::Matthew && *chapter == 8 {
        Ok(34)
    } else if book == &BibleBook::Matthew && *chapter == 9 {
        Ok(38)
    } else if book == &BibleBook::Matthew && *chapter == 10 {
        Ok(42)
    } else if book == &BibleBook::Matthew && *chapter == 11 {
        Ok(30)
    } else if book == &BibleBook::Matthew && *chapter == 12 {
        Ok(50)
    } else if book == &BibleBook::Matthew && *chapter == 13 {
        Ok(58)
    } else if book == &BibleBook::Matthew && *chapter == 14 {
        Ok(36)
    } else if book == &BibleBook::Matthew && *chapter == 15 {
        Ok(39)
    } else if book == &BibleBook::Matthew && *chapter == 16 {
        Ok(28)
    } else if book == &BibleBook::Matthew && *chapter == 17 {
        Ok(27)
    } else if book == &BibleBook::Matthew && *chapter == 18 {
        Ok(35)
    } else if book == &BibleBook::Matthew && *chapter == 19 {
        Ok(30)
    } else if book == &BibleBook::Matthew && *chapter == 20 {
        Ok(34)
    } else if book == &BibleBook::Matthew && *chapter == 21 {
        Ok(46)
    } else if book == &BibleBook::Matthew && *chapter == 22 {
        Ok(46)
    } else if book == &BibleBook::Matthew && *chapter == 23 {
        Ok(39)
    } else if book == &BibleBook::Matthew && *chapter == 24 {
        Ok(51)
    } else if book == &BibleBook::Matthew && *chapter == 25 {
        Ok(46)
    } else if book == &BibleBook::Matthew && *chapter == 26 {
        Ok(75)
    } else if book == &BibleBook::Matthew && *chapter == 27 {
        Ok(66)
    } else if book == &BibleBook::Matthew && *chapter == 28 {
        Ok(20)
    } else if book == &BibleBook::Mark && *chapter == 1 {
        Ok(45)
    } else if book == &BibleBook::Mark && *chapter == 2 {
        Ok(28)
    } else if book == &BibleBook::Mark && *chapter == 3 {
        Ok(35)
    } else if book == &BibleBook::Mark && *chapter == 4 {
        Ok(41)
    } else if book == &BibleBook::Mark && *chapter == 5 {
        Ok(43)
    } else if book == &BibleBook::Mark && *chapter == 6 {
        Ok(56)
    } else if book == &BibleBook::Mark && *chapter == 7 {
        Ok(37)
    } else if book == &BibleBook::Mark && *chapter == 8 {
        Ok(38)
    } else if book == &BibleBook::Mark && *chapter == 9 {
        Ok(50)
    } else if book == &BibleBook::Mark && *chapter == 10 {
        Ok(52)
    } else if book == &BibleBook::Mark && *chapter == 11 {
        Ok(33)
    } else if book == &BibleBook::Mark && *chapter == 12 {
        Ok(44)
    } else if book == &BibleBook::Mark && *chapter == 13 {
        Ok(37)
    } else if book == &BibleBook::Mark && *chapter == 14 {
        Ok(72)
    } else if book == &BibleBook::Mark && *chapter == 15 {
        Ok(47)
    } else if book == &BibleBook::Mark && *chapter == 16 {
        Ok(20)
    } else if book == &BibleBook::Luke && *chapter == 1 {
        Ok(80)
    } else if book == &BibleBook::Luke && *chapter == 2 {
        Ok(52)
    } else if book == &BibleBook::Luke && *chapter == 3 {
        Ok(38)
    } else if book == &BibleBook::Luke && *chapter == 4 {
        Ok(44)
    } else if book == &BibleBook::Luke && *chapter == 5 {
        Ok(39)
    } else if book == &BibleBook::Luke && *chapter == 6 {
        Ok(49)
    } else if book == &BibleBook::Luke && *chapter == 7 {
        Ok(50)
    } else if book == &BibleBook::Luke && *chapter == 8 {
        Ok(56)
    } else if book == &BibleBook::Luke && *chapter == 9 {
        Ok(62)
    } else if book == &BibleBook::Luke && *chapter == 10 {
        Ok(42)
    } else if book == &BibleBook::Luke && *chapter == 11 {
        Ok(54)
    } else if book == &BibleBook::Luke && *chapter == 12 {
        Ok(59)
    } else if book == &BibleBook::Luke && *chapter == 13 {
        Ok(35)
    } else if book == &BibleBook::Luke && *chapter == 14 {
        Ok(35)
    } else if book == &BibleBook::Luke && *chapter == 15 {
        Ok(32)
    } else if book == &BibleBook::Luke && *chapter == 16 {
        Ok(31)
    } else if book == &BibleBook::Luke && *chapter == 17 {
        Ok(37)
    } else if book == &BibleBook::Luke && *chapter == 18 {
        Ok(43)
    } else if book == &BibleBook::Luke && *chapter == 19 {
        Ok(48)
    } else if book == &BibleBook::Luke && *chapter == 20 {
        Ok(47)
    } else if book == &BibleBook::Luke && *chapter == 21 {
        Ok(38)
    } else if book == &BibleBook::Luke && *chapter == 22 {
        Ok(71)
    } else if book == &BibleBook::Luke && *chapter == 23 {
        Ok(56)
    } else if book == &BibleBook::Luke && *chapter == 24 {
        Ok(53)
    } else if book == &BibleBook::John && *chapter == 1 {
        Ok(51)
    } else if book == &BibleBook::John && *chapter == 2 {
        Ok(25)
    } else if book == &BibleBook::John && *chapter == 3 {
        Ok(36)
    } else if book == &BibleBook::John && *chapter == 4 {
        Ok(54)
    } else if book == &BibleBook::John && *chapter == 5 {
        Ok(47)
    } else if book == &BibleBook::John && *chapter == 6 {
        Ok(71)
    } else if book == &BibleBook::John && *chapter == 7 {
        Ok(53)
    } else if book == &BibleBook::John && *chapter == 8 {
        Ok(59)
    } else if book == &BibleBook::John && *chapter == 9 {
        Ok(41)
    } else if book == &BibleBook::John && *chapter == 10 {
        Ok(42)
    } else if book == &BibleBook::John && *chapter == 11 {
        Ok(57)
    } else if book == &BibleBook::John && *chapter == 12 {
        Ok(50)
    } else if book == &BibleBook::John && *chapter == 13 {
        Ok(38)
    } else if book == &BibleBook::John && *chapter == 14 {
        Ok(31)
    } else if book == &BibleBook::John && *chapter == 15 {
        Ok(27)
    } else if book == &BibleBook::John && *chapter == 16 {
        Ok(33)
    } else if book == &BibleBook::John && *chapter == 17 {
        Ok(26)
    } else if book == &BibleBook::John && *chapter == 18 {
        Ok(40)
    } else if book == &BibleBook::John && *chapter == 19 {
        Ok(42)
    } else if book == &BibleBook::John && *chapter == 20 {
        Ok(31)
    } else if book == &BibleBook::John && *chapter == 21 {
        Ok(25)
    } else if book == &BibleBook::Acts && *chapter == 1 {
        Ok(26)
    } else if book == &BibleBook::Acts && *chapter == 2 {
        Ok(47)
    } else if book == &BibleBook::Acts && *chapter == 3 {
        Ok(26)
    } else if book == &BibleBook::Acts && *chapter == 4 {
        Ok(37)
    } else if book == &BibleBook::Acts && *chapter == 5 {
        Ok(42)
    } else if book == &BibleBook::Acts && *chapter == 6 {
        Ok(15)
    } else if book == &BibleBook::Acts && *chapter == 7 {
        Ok(60)
    } else if book == &BibleBook::Acts && *chapter == 8 {
        Ok(40)
    } else if book == &BibleBook::Acts && *chapter == 9 {
        Ok(43)
    } else if book == &BibleBook::Acts && *chapter == 10 {
        Ok(48)
    } else if book == &BibleBook::Acts && *chapter == 11 {
        Ok(30)
    } else if book == &BibleBook::Acts && *chapter == 12 {
        Ok(25)
    } else if book == &BibleBook::Acts && *chapter == 13 {
        Ok(52)
    } else if book == &BibleBook::Acts && *chapter == 14 {
        Ok(28)
    } else if book == &BibleBook::Acts && *chapter == 15 {
        Ok(41)
    } else if book == &BibleBook::Acts && *chapter == 16 {
        Ok(40)
    } else if book == &BibleBook::Acts && *chapter == 17 {
        Ok(34)
    } else if book == &BibleBook::Acts && *chapter == 18 {
        Ok(28)
    } else if book == &BibleBook::Acts && *chapter == 19 {
        Ok(41)
    } else if book == &BibleBook::Acts && *chapter == 20 {
        Ok(38)
    } else if book == &BibleBook::Acts && *chapter == 21 {
        Ok(40)
    } else if book == &BibleBook::Acts && *chapter == 22 {
        Ok(30)
    } else if book == &BibleBook::Acts && *chapter == 23 {
        Ok(35)
    } else if book == &BibleBook::Acts && *chapter == 24 {
        Ok(27)
    } else if book == &BibleBook::Acts && *chapter == 25 {
        Ok(27)
    } else if book == &BibleBook::Acts && *chapter == 26 {
        Ok(32)
    } else if book == &BibleBook::Acts && *chapter == 27 {
        Ok(44)
    } else if book == &BibleBook::Acts && *chapter == 28 {
        Ok(31)
    } else if book == &BibleBook::Romans && *chapter == 1 {
        Ok(32)
    } else if book == &BibleBook::Romans && *chapter == 2 {
        Ok(29)
    } else if book == &BibleBook::Romans && *chapter == 3 {
        Ok(31)
    } else if book == &BibleBook::Romans && *chapter == 4 {
        Ok(25)
    } else if book == &BibleBook::Romans && *chapter == 5 {
        Ok(21)
    } else if book == &BibleBook::Romans && *chapter == 6 {
        Ok(23)
    } else if book == &BibleBook::Romans && *chapter == 7 {
        Ok(25)
    } else if book == &BibleBook::Romans && *chapter == 8 {
        Ok(39)
    } else if book == &BibleBook::Romans && *chapter == 9 {
        Ok(33)
    } else if book == &BibleBook::Romans && *chapter == 10 {
        Ok(21)
    } else if book == &BibleBook::Romans && *chapter == 11 {
        Ok(36)
    } else if book == &BibleBook::Romans && *chapter == 12 {
        Ok(21)
    } else if book == &BibleBook::Romans && *chapter == 13 {
        Ok(14)
    } else if book == &BibleBook::Romans && *chapter == 14 {
        Ok(23)
    } else if book == &BibleBook::Romans && *chapter == 15 {
        Ok(33)
    } else if book == &BibleBook::Romans && *chapter == 16 {
        Ok(27)
    } else if book == &BibleBook::ICorinthians && *chapter == 1 {
        Ok(31)
    } else if book == &BibleBook::ICorinthians && *chapter == 2 {
        Ok(16)
    } else if book == &BibleBook::ICorinthians && *chapter == 3 {
        Ok(23)
    } else if book == &BibleBook::ICorinthians && *chapter == 4 {
        Ok(21)
    } else if book == &BibleBook::ICorinthians && *chapter == 5 {
        Ok(13)
    } else if book == &BibleBook::ICorinthians && *chapter == 6 {
        Ok(20)
    } else if book == &BibleBook::ICorinthians && *chapter == 7 {
        Ok(40)
    } else if book == &BibleBook::ICorinthians && *chapter == 8 {
        Ok(13)
    } else if book == &BibleBook::ICorinthians && *chapter == 9 {
        Ok(27)
    } else if book == &BibleBook::ICorinthians && *chapter == 10 {
        Ok(33)
    } else if book == &BibleBook::ICorinthians && *chapter == 11 {
        Ok(34)
    } else if book == &BibleBook::ICorinthians && *chapter == 12 {
        Ok(31)
    } else if book == &BibleBook::ICorinthians && *chapter == 13 {
        Ok(13)
    } else if book == &BibleBook::ICorinthians && *chapter == 14 {
        Ok(40)
    } else if book == &BibleBook::ICorinthians && *chapter == 15 {
        Ok(58)
    } else if book == &BibleBook::ICorinthians && *chapter == 16 {
        Ok(24)
    } else if book == &BibleBook::IICorinthians && *chapter == 1 {
        Ok(24)
    } else if book == &BibleBook::IICorinthians && *chapter == 2 {
        Ok(17)
    } else if book == &BibleBook::IICorinthians && *chapter == 3 {
        Ok(18)
    } else if book == &BibleBook::IICorinthians && *chapter == 4 {
        Ok(18)
    } else if book == &BibleBook::IICorinthians && *chapter == 5 {
        Ok(21)
    } else if book == &BibleBook::IICorinthians && *chapter == 6 {
        Ok(18)
    } else if book == &BibleBook::IICorinthians && *chapter == 7 {
        Ok(16)
    } else if book == &BibleBook::IICorinthians && *chapter == 8 {
        Ok(24)
    } else if book == &BibleBook::IICorinthians && *chapter == 9 {
        Ok(15)
    } else if book == &BibleBook::IICorinthians && *chapter == 10 {
        Ok(18)
    } else if book == &BibleBook::IICorinthians && *chapter == 11 {
        Ok(33)
    } else if book == &BibleBook::IICorinthians && *chapter == 12 {
        Ok(21)
    } else if book == &BibleBook::IICorinthians && *chapter == 13 {
        Ok(14)
    } else if book == &BibleBook::Galatians && *chapter == 1 {
        Ok(24)
    } else if book == &BibleBook::Galatians && *chapter == 2 {
        Ok(21)
    } else if book == &BibleBook::Galatians && *chapter == 3 {
        Ok(29)
    } else if book == &BibleBook::Galatians && *chapter == 4 {
        Ok(31)
    } else if book == &BibleBook::Galatians && *chapter == 5 {
        Ok(26)
    } else if book == &BibleBook::Galatians && *chapter == 6 {
        Ok(18)
    } else if book == &BibleBook::Ephesians && *chapter == 1 {
        Ok(23)
    } else if book == &BibleBook::Ephesians && *chapter == 2 {
        Ok(22)
    } else if book == &BibleBook::Ephesians && *chapter == 3 {
        Ok(21)
    } else if book == &BibleBook::Ephesians && *chapter == 4 {
        Ok(32)
    } else if book == &BibleBook::Ephesians && *chapter == 5 {
        Ok(33)
    } else if book == &BibleBook::Ephesians && *chapter == 6 {
        Ok(24)
    } else if book == &BibleBook::Philippians && *chapter == 1 {
        Ok(30)
    } else if book == &BibleBook::Philippians && *chapter == 2 {
        Ok(30)
    } else if book == &BibleBook::Philippians && *chapter == 3 {
        Ok(21)
    } else if book == &BibleBook::Philippians && *chapter == 4 {
        Ok(23)
    } else if book == &BibleBook::Colossians && *chapter == 1 {
        Ok(29)
    } else if book == &BibleBook::Colossians && *chapter == 2 {
        Ok(23)
    } else if book == &BibleBook::Colossians && *chapter == 3 {
        Ok(25)
    } else if book == &BibleBook::Colossians && *chapter == 4 {
        Ok(18)
    } else if book == &BibleBook::IThessalonians && *chapter == 1 {
        Ok(10)
    } else if book == &BibleBook::IThessalonians && *chapter == 2 {
        Ok(20)
    } else if book == &BibleBook::IThessalonians && *chapter == 3 {
        Ok(13)
    } else if book == &BibleBook::IThessalonians && *chapter == 4 {
        Ok(18)
    } else if book == &BibleBook::IThessalonians && *chapter == 5 {
        Ok(28)
    } else if book == &BibleBook::IIThessalonians && *chapter == 1 {
        Ok(12)
    } else if book == &BibleBook::IIThessalonians && *chapter == 2 {
        Ok(17)
    } else if book == &BibleBook::IIThessalonians && *chapter == 3 {
        Ok(18)
    } else if book == &BibleBook::ITimothy && *chapter == 1 {
        Ok(20)
    } else if book == &BibleBook::ITimothy && *chapter == 2 {
        Ok(15)
    } else if book == &BibleBook::ITimothy && *chapter == 3 {
        Ok(16)
    } else if book == &BibleBook::ITimothy && *chapter == 4 {
        Ok(16)
    } else if book == &BibleBook::ITimothy && *chapter == 5 {
        Ok(25)
    } else if book == &BibleBook::ITimothy && *chapter == 6 {
        Ok(21)
    } else if book == &BibleBook::IITimothy && *chapter == 1 {
        Ok(18)
    } else if book == &BibleBook::IITimothy && *chapter == 2 {
        Ok(26)
    } else if book == &BibleBook::IITimothy && *chapter == 3 {
        Ok(17)
    } else if book == &BibleBook::IITimothy && *chapter == 4 {
        Ok(22)
    } else if book == &BibleBook::Titus && *chapter == 1 {
        Ok(16)
    } else if book == &BibleBook::Titus && *chapter == 2 {
        Ok(15)
    } else if book == &BibleBook::Titus && *chapter == 3 {
        Ok(15)
    } else if book == &BibleBook::Philemon && *chapter == 1 {
        Ok(25)
    } else if book == &BibleBook::Hebrews && *chapter == 1 {
        Ok(14)
    } else if book == &BibleBook::Hebrews && *chapter == 2 {
        Ok(18)
    } else if book == &BibleBook::Hebrews && *chapter == 3 {
        Ok(19)
    } else if book == &BibleBook::Hebrews && *chapter == 4 {
        Ok(16)
    } else if book == &BibleBook::Hebrews && *chapter == 5 {
        Ok(14)
    } else if book == &BibleBook::Hebrews && *chapter == 6 {
        Ok(20)
    } else if book == &BibleBook::Hebrews && *chapter == 7 {
        Ok(28)
    } else if book == &BibleBook::Hebrews && *chapter == 8 {
        Ok(13)
    } else if book == &BibleBook::Hebrews && *chapter == 9 {
        Ok(28)
    } else if book == &BibleBook::Hebrews && *chapter == 10 {
        Ok(39)
    } else if book == &BibleBook::Hebrews && *chapter == 11 {
        Ok(40)
    } else if book == &BibleBook::Hebrews && *chapter == 12 {
        Ok(29)
    } else if book == &BibleBook::Hebrews && *chapter == 13 {
        Ok(25)
    } else if book == &BibleBook::James && *chapter == 1 {
        Ok(27)
    } else if book == &BibleBook::James && *chapter == 2 {
        Ok(26)
    } else if book == &BibleBook::James && *chapter == 3 {
        Ok(18)
    } else if book == &BibleBook::James && *chapter == 4 {
        Ok(17)
    } else if book == &BibleBook::James && *chapter == 5 {
        Ok(20)
    } else if book == &BibleBook::IPeter && *chapter == 1 {
        Ok(25)
    } else if book == &BibleBook::IPeter && *chapter == 2 {
        Ok(25)
    } else if book == &BibleBook::IPeter && *chapter == 3 {
        Ok(22)
    } else if book == &BibleBook::IPeter && *chapter == 4 {
        Ok(19)
    } else if book == &BibleBook::IPeter && *chapter == 5 {
        Ok(14)
    } else if book == &BibleBook::IIPeter && *chapter == 1 {
        Ok(21)
    } else if book == &BibleBook::IIPeter && *chapter == 2 {
        Ok(22)
    } else if book == &BibleBook::IIPeter && *chapter == 3 {
        Ok(18)
    } else if book == &BibleBook::IJohn && *chapter == 1 {
        Ok(10)
    } else if book == &BibleBook::IJohn && *chapter == 2 {
        Ok(29)
    } else if book == &BibleBook::IJohn && *chapter == 3 {
        Ok(24)
    } else if book == &BibleBook::IJohn && *chapter == 4 {
        Ok(21)
    } else if book == &BibleBook::IJohn && *chapter == 5 {
        Ok(21)
    } else if book == &BibleBook::IIJohn && *chapter == 1 {
        Ok(13)
    } else if book == &BibleBook::IIIJohn && *chapter == 1 {
        Ok(14)
    } else if book == &BibleBook::Jude && *chapter == 1 {
        Ok(25)
    } else if book == &BibleBook::Revelation && *chapter == 1 {
        Ok(20)
    } else if book == &BibleBook::Revelation && *chapter == 2 {
        Ok(29)
    } else if book == &BibleBook::Revelation && *chapter == 3 {
        Ok(22)
    } else if book == &BibleBook::Revelation && *chapter == 4 {
        Ok(11)
    } else if book == &BibleBook::Revelation && *chapter == 5 {
        Ok(14)
    } else if book == &BibleBook::Revelation && *chapter == 6 {
        Ok(17)
    } else if book == &BibleBook::Revelation && *chapter == 7 {
        Ok(17)
    } else if book == &BibleBook::Revelation && *chapter == 8 {
        Ok(13)
    } else if book == &BibleBook::Revelation && *chapter == 9 {
        Ok(21)
    } else if book == &BibleBook::Revelation && *chapter == 10 {
        Ok(11)
    } else if book == &BibleBook::Revelation && *chapter == 11 {
        Ok(19)
    } else if book == &BibleBook::Revelation && *chapter == 12 {
        Ok(17)
    } else if book == &BibleBook::Revelation && *chapter == 13 {
        Ok(18)
    } else if book == &BibleBook::Revelation && *chapter == 14 {
        Ok(20)
    } else if book == &BibleBook::Revelation && *chapter == 15 {
        Ok(8)
    } else if book == &BibleBook::Revelation && *chapter == 16 {
        Ok(21)
    } else if book == &BibleBook::Revelation && *chapter == 17 {
        Ok(18)
    } else if book == &BibleBook::Revelation && *chapter == 18 {
        Ok(24)
    } else if book == &BibleBook::Revelation && *chapter == 19 {
        Ok(21)
    } else if book == &BibleBook::Revelation && *chapter == 20 {
        Ok(15)
    } else if book == &BibleBook::Revelation && *chapter == 21 {
        Ok(27)
    } else if book == &BibleBook::Revelation && *chapter == 22 {
        Ok(21)
    } else {
        Err(BibleReferenceValidationError {
            problem: BibleReferenceProblem::ChapterDoesNotExist,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bibleverse_exists() {
        assert!(validate_book_chapter_verse(&BibleBook::John, &3, &16).is_ok());
        assert!(
            validate_book_chapter_verse(&BibleBook::Psalm, &1, &7)
                .err()
                .unwrap()
                .problem
                == BibleReferenceProblem::VerseDoesNotExist
        );
        assert!(
            validate_book_chapter_verse(&BibleBook::Psalm, &171, &1)
                .err()
                .unwrap()
                .problem
                == BibleReferenceProblem::ChapterDoesNotExist
        );

        assert!(
            validate_book_chapter_verse(&BibleBook::Genesis, &0, &1)
                .err()
                .unwrap()
                .problem
                == BibleReferenceProblem::ChapterDoesNotExist
        );
    }
}
