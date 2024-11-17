//! The Bible module includes the data structure around a Bible, including books, chapters and verses. 
//! It also handles the parsing process which determins the validity of Bible references.

use serde::{Serialize, Deserialize};

/// This struct contains a Bible reference which is valid (can be found in a real Bible, consisting of a book, a chapter and a verse.
/// Please note the following: There are some differences concerning the number of verses of certain chapters depending on some Bible versions, e.g. in English Bible translations, Psalms may have one verse more as in most German translations–because the introduction words at the beginning of some Psalms are counted as a seperate verse, while other translations might render them as the preface (or a verse 0). In this crate, we are always assuming the **maximum amount** of verses, so that all translations and versions can be used.
/// In the new testament, the Textus Receptus is used as template for determining the numbers of chapters and vereses.
/// Some books (like the book of Jude) may only have one Chapter. Normally, in human languages people would only quote the verse and leave the chapter out (e.g. Jude 13)–however, this will be parsed as Jude 1:13 technically.
#[derive(PartialEq, Serialize, Deserialize, Debug)]
struct BibleVerseReference {
    book: BibleBook,
    chapter: BibleChapter,
    verse: BibleVerse,
}

/// The struct BibleBook contains all books of the Bible in their order.
#[derive(PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
enum BibleBook {
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
    pub fn is_old_testament(&self) -> bool {
        self < &BibleBook::Matthew
    }
    
    /// This function determines whether the current Bible book is part of the New Testament.
    /// # Parameters
    /// - No parameter
    /// # Returns
    /// `true` if the book is part of the New Testament, `false` if it is part of the Old Testament.
    /// # Note
    //7 This function is per definition the inverse of `is_old_testament`.
    pub fn is_new_testament(&self) -> bool {
        self >= &BibleBook::Matthew
    }
}

/// An unsigned positive number which represents the chapter of a Bible reference
type BibleChapter = u8;

/// An unsigned positive number which represents the verse of a Bible reference
type BibleVerse = u8;


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
}
