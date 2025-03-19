use std::{collections::HashMap, sync::RwLock};
use once_cell::sync::Lazy;

use crate::bible::{BibleBook, BibleReference};

/// A static Read-Write-Lock vector of ReferenceLanguage instances using Lazy. Here, all the languages which are supported by default are loaded and saved in.
/// As this is inside a [RwLock], it is possible to manipulate the languages during runtime.
/// To obtain a mutable reference to the vector, use the `write` function of the RwLock:
/// 
/// ```ignore
/// # use bibleref::referencing::language::REFERENCE_LANGUAGES;
/// let mut languages = REFERENCE_LANGUAGES.write().unwrap();
/// // Generate a language
/// languages.push(your_defined_language);
/// // Add a language or do anything else
/// ```
pub static REFERENCE_LANGUAGES: Lazy<RwLock<Vec<ReferenceLanguage>>> = Lazy::new(|| {
    RwLock::new(vec![
        get_english_reference_language(),
        get_german_reference_language(),
        get_chinese_simplified_reference_language(),
        get_chinese_traditional_reference_language(),
        get_french_reference_language(),
    ])
});

/// A struct representing a human used language where Bible references can be reprsented.
pub struct ReferenceLanguage {
    /// The long name of the language (e.g. English, German, Chinese Simplified, French, etc)
    pub long_language_name: String,

    /// The international language code (eg. en, de, zh_sim)
    pub language_code: String,

    /// The long names of each Bible book represented as a `HashMap`. Each Bible book is assigned a [Vec<String>] which has to contain at least one String with the name of the Bible book in the language.
    /// If multible strings are provided, the first one will be used as default.
    pub long_names: HashMap<BibleBook, Vec<String>>,

    /// Short names (abbreviations) of each Bible book
    pub short_names: HashMap<BibleBook, Vec<String>>,

    /// A list of delimiters splitting the chapter from the verse (most likely ',' or ':')
    pub delimiter: Vec<String>,

    /// Determines whether a simple space should be added between the book name and the chapter. This should be activated for all left-to-right lettered languages with Latin or Cyrillian alphabet, however it should be disabled for Asian languages such as Chinese, Japanese or Korean.
    pub space_sepeeration: bool,
}

impl ReferenceLanguage {
    pub fn create_reference(&self, bible_reference: &BibleReference, book_reference_type: BookReferenceType) -> String {
        match bible_reference {
            BibleReference::BibleBook(book) => match book_reference_type {
                BookReferenceType::Long => self.long_names[&book.book()].first().unwrap().to_string(),
                BookReferenceType::Short => self.short_names[&book.book()].first().unwrap().to_string(),
            }
            BibleReference::BibleChapter(chapter) => format!(
                "{}{}{}",
                match book_reference_type {
                    BookReferenceType::Long => self.long_names[&chapter.book()].first().unwrap().to_string(),
                    BookReferenceType::Short => self.short_names[&chapter.book()].first().unwrap().to_string(),
                },
                match self.space_sepeeration {
                    true => " ",
                    false => ""
                },
                chapter.chapter()
            ),
            BibleReference::BibleVerse(verse) => format!(
                "{}{}{}{}{}",
                match book_reference_type {
                    BookReferenceType::Long => self.long_names[&verse.book()].first().unwrap().to_string(),
                    BookReferenceType::Short => self.short_names[&verse.book()].first().unwrap().to_string(),
                },
                match self.space_sepeeration {
                    true => " ",
                    false => ""
                },
                verse.chapter(),
                self.delimiter.first().unwrap(),
                verse.verse()
            )
        }
    }
}

/// This function creates a Bible reference in a human language.
/// 
/// # Params
/// - `bible_reference`: The Bible reference from which the expression should be created
/// - `language_code`: The language code of the human language in which the reference should be created
/// 
/// # Returns
/// An [Option<String>] which is [Some(bible_reference_string)] if the language specified with the [language_code] exists
/// or [None] if the language can't be found.
pub fn get_reference_in_language(bible_reference: &BibleReference, language_code: &str, book_reference_type: BookReferenceType) -> Option<String> {
    let language_code = language_code.trim().to_lowercase();
    let reference_languages = &*REFERENCE_LANGUAGES.read().unwrap();
    
    for language in reference_languages {
        if language.language_code.to_lowercase().eq(&language_code) {
            return Some(language.create_reference(bible_reference, book_reference_type))
        }
    }
    None
}

/// The type of a book reference in human language
#[derive(Debug)]
pub enum BookReferenceType {
    /// Short versions like "Gen" or "Joh"
    Short,

    /// Long versions like "Genesis" or "John"
    Long
}

fn get_english_reference_language() -> ReferenceLanguage {
    let long_names_vec = vec![
        (BibleBook::Genesis, vec!["Genesis".to_string()]),
        (BibleBook::Exodus, vec!["Exodus".to_string()]),
        (BibleBook::Leviticus, vec!["Leviticus".to_string()]),
        (BibleBook::Numbers, vec!["Numbers".to_string()]),
        (BibleBook::Deuteronomy, vec!["Deuteronomy".to_string()]),
        (BibleBook::Joshua, vec!["Joshua".to_string()]),
        (BibleBook::Judges, vec!["Judges".to_string()]),
        (BibleBook::Ruth, vec!["Ruth".to_string()]),
        (BibleBook::ISamuel, vec!["1 Samuel".to_string()]),
        (BibleBook::IISamuel, vec!["2 Samuel".to_string()]),
        (BibleBook::IKings, vec!["1 Kings".to_string()]),
        (BibleBook::IIKings, vec!["2 Kings".to_string()]),
        (BibleBook::IChronicles, vec!["1 Chronicles".to_string()]),
        (BibleBook::IIChronicles, vec!["2 Chronicles".to_string()]),
        (BibleBook::Ezra, vec!["Ezra".to_string()]),
        (BibleBook::Nehemiah, vec!["Nehemiah".to_string()]),
        (BibleBook::Esther, vec!["Esther".to_string()]),
        (BibleBook::Job, vec!["Job".to_string()]),
        (BibleBook::Psalm, vec!["Psalms".to_string()]), // Commonly plural in English
        (BibleBook::Proverbs, vec!["Proverbs".to_string()]),
        (BibleBook::Ecclesiastes, vec!["Ecclesiastes".to_string()]),
        (BibleBook::SongofSolomon, vec!["Song of Solomon".to_string()]),
        (BibleBook::Isaiah, vec!["Isaiah".to_string()]),
        (BibleBook::Jeremiah, vec!["Jeremiah".to_string()]),
        (BibleBook::Lamentations, vec!["Lamentations".to_string()]),
        (BibleBook::Ezekiel, vec!["Ezekiel".to_string()]),
        (BibleBook::Daniel, vec!["Daniel".to_string()]),
        (BibleBook::Hosea, vec!["Hosea".to_string()]),
        (BibleBook::Joel, vec!["Joel".to_string()]),
        (BibleBook::Amos, vec!["Amos".to_string()]),
        (BibleBook::Obadiah, vec!["Obadiah".to_string()]),
        (BibleBook::Jonah, vec!["Jonah".to_string()]),
        (BibleBook::Micah, vec!["Micah".to_string()]),
        (BibleBook::Nahum, vec!["Nahum".to_string()]),
        (BibleBook::Habakkuk, vec!["Habakkuk".to_string()]),
        (BibleBook::Zephaniah, vec!["Zephaniah".to_string()]),
        (BibleBook::Haggai, vec!["Haggai".to_string()]),
        (BibleBook::Zechariah, vec!["Zechariah".to_string()]),
        (BibleBook::Malachi, vec!["Malachi".to_string()]),
        (BibleBook::Matthew, vec!["Matthew".to_string()]),
        (BibleBook::Mark, vec!["Mark".to_string()]),
        (BibleBook::Luke, vec!["Luke".to_string()]),
        (BibleBook::John, vec!["John".to_string()]),
        (BibleBook::Acts, vec!["Acts".to_string()]),
        (BibleBook::Romans, vec!["Romans".to_string()]),
        (BibleBook::ICorinthians, vec!["1 Corinthians".to_string()]),
        (BibleBook::IICorinthians, vec!["2 Corinthians".to_string()]),
        (BibleBook::Galatians, vec!["Galatians".to_string()]),
        (BibleBook::Ephesians, vec!["Ephesians".to_string()]),
        (BibleBook::Philippians, vec!["Philippians".to_string()]),
        (BibleBook::Colossians, vec!["Colossians".to_string()]),
        (BibleBook::IThessalonians, vec!["1 Thessalonians".to_string()]),
        (BibleBook::IIThessalonians, vec!["2 Thessalonians".to_string()]),
        (BibleBook::ITimothy, vec!["1 Timothy".to_string()]),
        (BibleBook::IITimothy, vec!["2 Timothy".to_string()]),
        (BibleBook::Titus, vec!["Titus".to_string()]),
        (BibleBook::Philemon, vec!["Philemon".to_string()]),
        (BibleBook::Hebrews, vec!["Hebrews".to_string()]),
        (BibleBook::James, vec!["James".to_string()]),
        (BibleBook::IPeter, vec!["1 Peter".to_string()]),
        (BibleBook::IIPeter, vec!["2 Peter".to_string()]),
        (BibleBook::IJohn, vec!["1 John".to_string()]),
        (BibleBook::IIJohn, vec!["2 John".to_string()]),
        (BibleBook::IIIJohn, vec!["3 John".to_string()]),
        (BibleBook::Jude, vec!["Jude".to_string()]),
        (BibleBook::Revelation, vec!["Revelation".to_string()]),
    ];
    let long_names: HashMap<BibleBook, Vec<String>> = long_names_vec.into_iter().collect();

    let short_names_vec = vec![
        (BibleBook::Genesis, vec!["Gen".to_string()]),
        (BibleBook::Exodus, vec!["Exod".to_string()]),
        (BibleBook::Leviticus, vec!["Lev".to_string()]),
        (BibleBook::Numbers, vec!["Num".to_string()]),
        (BibleBook::Deuteronomy, vec!["Deut".to_string()]),
        (BibleBook::Joshua, vec!["Josh".to_string()]),
        (BibleBook::Judges, vec!["Judg".to_string()]),
        (BibleBook::Ruth, vec!["Ruth".to_string()]),
        (BibleBook::ISamuel, vec!["1 Sam".to_string()]),
        (BibleBook::IISamuel, vec!["2 Sam".to_string()]),
        (BibleBook::IKings, vec!["1 Kgs".to_string()]),
        (BibleBook::IIKings, vec!["2 Kgs".to_string()]),
        (BibleBook::IChronicles, vec!["1 Chr".to_string()]),
        (BibleBook::IIChronicles, vec!["2 Chr".to_string()]),
        (BibleBook::Ezra, vec!["Ezra".to_string()]),
        (BibleBook::Nehemiah, vec!["Neh".to_string()]),
        (BibleBook::Esther, vec!["Esth".to_string()]),
        (BibleBook::Job, vec!["Job".to_string()]),
        (BibleBook::Psalm, vec!["Ps".to_string()]), // "Ps" for Psalms
        (BibleBook::Proverbs, vec!["Prov".to_string()]),
        (BibleBook::Ecclesiastes, vec!["Eccl".to_string()]),
        (BibleBook::SongofSolomon, vec!["Song".to_string()]),
        (BibleBook::Isaiah, vec!["Isa".to_string()]),
        (BibleBook::Jeremiah, vec!["Jer".to_string()]),
        (BibleBook::Lamentations, vec!["Lam".to_string()]),
        (BibleBook::Ezekiel, vec!["Ezek".to_string()]),
        (BibleBook::Daniel, vec!["Dan".to_string()]),
        (BibleBook::Hosea, vec!["Hos".to_string()]),
        (BibleBook::Joel, vec!["Joel".to_string()]),
        (BibleBook::Amos, vec!["Amos".to_string()]),
        (BibleBook::Obadiah, vec!["Obad".to_string()]),
        (BibleBook::Jonah, vec!["Jonah".to_string()]),
        (BibleBook::Micah, vec!["Mic".to_string()]),
        (BibleBook::Nahum, vec!["Nah".to_string()]),
        (BibleBook::Habakkuk, vec!["Hab".to_string()]),
        (BibleBook::Zephaniah, vec!["Zeph".to_string()]),
        (BibleBook::Haggai, vec!["Hag".to_string()]),
        (BibleBook::Zechariah, vec!["Zech".to_string()]),
        (BibleBook::Malachi, vec!["Mal".to_string()]),
        (BibleBook::Matthew, vec!["Matt".to_string()]),
        (BibleBook::Mark, vec!["Mark".to_string()]),
        (BibleBook::Luke, vec!["Luke".to_string()]),
        (BibleBook::John, vec!["John".to_string()]),
        (BibleBook::Acts, vec!["Acts".to_string()]),
        (BibleBook::Romans, vec!["Rom".to_string()]),
        (BibleBook::ICorinthians, vec!["1 Cor".to_string()]),
        (BibleBook::IICorinthians, vec!["2 Cor".to_string()]),
        (BibleBook::Galatians, vec!["Gal".to_string()]),
        (BibleBook::Ephesians, vec!["Eph".to_string()]),
        (BibleBook::Philippians, vec!["Phil".to_string()]),
        (BibleBook::Colossians, vec!["Col".to_string()]),
        (BibleBook::IThessalonians, vec!["1 Thess".to_string()]),
        (BibleBook::IIThessalonians, vec!["2 Thess".to_string()]),
        (BibleBook::ITimothy, vec!["1 Tim".to_string()]),
        (BibleBook::IITimothy, vec!["2 Tim".to_string()]),
        (BibleBook::Titus, vec!["Titus".to_string()]),
        (BibleBook::Philemon, vec!["Phlm".to_string()]),
        (BibleBook::Hebrews, vec!["Heb".to_string()]),
        (BibleBook::James, vec!["Jas".to_string()]),
        (BibleBook::IPeter, vec!["1 Pet".to_string()]),
        (BibleBook::IIPeter, vec!["2 Pet".to_string()]),
        (BibleBook::IJohn, vec!["1 John".to_string()]),
        (BibleBook::IIJohn, vec!["2 John".to_string()]),
        (BibleBook::IIIJohn, vec!["3 John".to_string()]),
        (BibleBook::Jude, vec!["Jude".to_string()]),
        (BibleBook::Revelation, vec!["Rev".to_string()]), 
    ];
    let short_names: HashMap<BibleBook, Vec<String>> = short_names_vec.into_iter().collect();

    ReferenceLanguage {
        long_language_name: "English".to_string(),
        language_code: "en".to_string(),
        long_names,
        short_names,
        delimiter: vec![":".to_string()],
        space_sepeeration: true,
    }

}

fn get_german_reference_language() -> ReferenceLanguage {
    let long_names_vec = vec![
        (BibleBook::Genesis, vec!["1. Mose".to_string(),"Genesis".to_string()]),
        (BibleBook::Exodus, vec!["2. Mose".to_string(),"Exodus".to_string()]),
        (BibleBook::Leviticus, vec!["3. Mose".to_string(),"Levitikus".to_string()]),
        (BibleBook::Numbers, vec!["4. Mose".to_string(),"Numeri".to_string()]),
        (BibleBook::Deuteronomy, vec!["5. Mose".to_string(),"Deuteronomium".to_string()]),
        (BibleBook::Joshua, vec!["Josua".to_string()]),
        (BibleBook::Judges, vec!["Richter".to_string()]),
        (BibleBook::Ruth, vec!["Ruth".to_string()]),
        (BibleBook::ISamuel, vec!["1. Samuel".to_string()]),
        (BibleBook::IISamuel, vec!["2. Samuel".to_string()]),
        (BibleBook::IKings, vec!["1. Könige".to_string()]),
        (BibleBook::IIKings, vec!["2. Könige".to_string()]),
        (BibleBook::IChronicles, vec!["1. Chronik".to_string()]),
        (BibleBook::IIChronicles, vec!["2. Chronik".to_string()]),
        (BibleBook::Ezra, vec!["Esra".to_string()]),
        (BibleBook::Nehemiah, vec!["Nehemia".to_string()]),
        (BibleBook::Esther, vec!["Ester".to_string()]),
        (BibleBook::Job, vec!["Hiob".to_string()]),
        (BibleBook::Psalm, vec!["Psalmen".to_string()]), // Plural in German
        (BibleBook::Proverbs, vec!["Sprüche".to_string()]),
        (BibleBook::Ecclesiastes, vec!["Prediger".to_string()]),
        (BibleBook::SongofSolomon, vec!["Hohelied".to_string()]),
        (BibleBook::Isaiah, vec!["Jesaja".to_string()]),
        (BibleBook::Jeremiah, vec!["Jeremia".to_string()]),
        (BibleBook::Lamentations, vec!["Klagelieder".to_string()]),
        (BibleBook::Ezekiel, vec!["Hesekiel".to_string()]),
        (BibleBook::Daniel, vec!["Daniel".to_string()]),
        (BibleBook::Hosea, vec!["Hosea".to_string()]),
        (BibleBook::Joel, vec!["Joel".to_string()]),
        (BibleBook::Amos, vec!["Amos".to_string()]),
        (BibleBook::Obadiah, vec!["Obadja".to_string()]),
        (BibleBook::Jonah, vec!["Jona".to_string()]),
        (BibleBook::Micah, vec!["Micha".to_string()]),
        (BibleBook::Nahum, vec!["Nahum".to_string()]),
        (BibleBook::Habakkuk, vec!["Habakuk".to_string()]),
        (BibleBook::Zephaniah, vec!["Zephanja".to_string()]),
        (BibleBook::Haggai, vec!["Haggai".to_string()]),
        (BibleBook::Zechariah, vec!["Sacharja".to_string()]),
        (BibleBook::Malachi, vec!["Maleachi".to_string()]),
        (BibleBook::Matthew, vec!["Matthäus".to_string()]),
        (BibleBook::Mark, vec!["Markus".to_string()]),
        (BibleBook::Luke, vec!["Lukas".to_string()]),
        (BibleBook::John, vec!["Johannes".to_string()]),
        (BibleBook::Acts, vec!["Apostelgeschichte".to_string()]),
        (BibleBook::Romans, vec!["Römer".to_string()]),
        (BibleBook::ICorinthians, vec!["1. Korinther".to_string()]),
        (BibleBook::IICorinthians, vec!["2. Korinther".to_string()]),
        (BibleBook::Galatians, vec!["Galater".to_string()]),
        (BibleBook::Ephesians, vec!["Epheser".to_string()]),
        (BibleBook::Philippians, vec!["Philipper".to_string()]),
        (BibleBook::Colossians, vec!["Kolosser".to_string()]),
        (BibleBook::IThessalonians, vec!["1. Thessalonicher".to_string()]),
        (BibleBook::IIThessalonians, vec!["2. Thessalonicher".to_string()]),
        (BibleBook::ITimothy, vec!["1. Timotheus".to_string()]),
        (BibleBook::IITimothy, vec!["2. Timotheus".to_string()]),
        (BibleBook::Titus, vec!["Titus".to_string()]),
        (BibleBook::Philemon, vec!["Philemon".to_string()]),
        (BibleBook::Hebrews, vec!["Hebräer".to_string()]),
        (BibleBook::James, vec!["Jakobus".to_string()]),
        (BibleBook::IPeter, vec!["1. Petrus".to_string()]),
        (BibleBook::IIPeter, vec!["2. Petrus".to_string()]),
        (BibleBook::IJohn, vec!["1. Johannes".to_string()]),
        (BibleBook::IIJohn, vec!["2. Johannes".to_string()]),
        (BibleBook::IIIJohn, vec!["3. Johannes".to_string()]),
        (BibleBook::Jude, vec!["Judas".to_string()]),
        (BibleBook::Revelation, vec!["Offenbarung".to_string()]),
    ];
    let long_names: HashMap<BibleBook, Vec<String>> = long_names_vec.into_iter().collect();

    let short_names_vec = vec![
        (BibleBook::Genesis, vec!["1Mo".to_string()]),
        (BibleBook::Exodus, vec!["2Mo".to_string()]),
        (BibleBook::Leviticus, vec!["3Mo".to_string()]),
        (BibleBook::Numbers, vec!["4Mo".to_string()]),
        (BibleBook::Deuteronomy, vec!["5Mo".to_string()]),
        (BibleBook::Joshua, vec!["Jos".to_string()]),
        (BibleBook::Judges, vec!["Ri".to_string()]),
        (BibleBook::Ruth, vec!["Ruth".to_string()]),
        (BibleBook::ISamuel, vec!["1Sam".to_string()]),
        (BibleBook::IISamuel, vec!["2Sam".to_string()]),
        (BibleBook::IKings, vec!["1Kön".to_string()]),
        (BibleBook::IIKings, vec!["2Kön".to_string()]),
        (BibleBook::IChronicles, vec!["1Chr".to_string()]),
        (BibleBook::IIChronicles, vec!["2Chr".to_string()]),
        (BibleBook::Ezra, vec!["Esr".to_string()]),
        (BibleBook::Nehemiah, vec!["Neh".to_string()]),
        (BibleBook::Esther, vec!["Est".to_string()]),
        (BibleBook::Job, vec!["Hi".to_string()]),
        (BibleBook::Psalm, vec!["Ps".to_string()]), // For "Psalmen"
        (BibleBook::Proverbs, vec!["Spr".to_string()]),
        (BibleBook::Ecclesiastes, vec!["Pred".to_string()]),
        (BibleBook::SongofSolomon, vec!["Hld".to_string()]),
        (BibleBook::Isaiah, vec!["Jes".to_string()]),
        (BibleBook::Jeremiah, vec!["Jer".to_string()]),
        (BibleBook::Lamentations, vec!["Klgl".to_string()]),
        (BibleBook::Ezekiel, vec!["Hes".to_string()]),
        (BibleBook::Daniel, vec!["Dan".to_string()]),
        (BibleBook::Hosea, vec!["Hos".to_string()]),
        (BibleBook::Joel, vec!["Joel".to_string()]),
        (BibleBook::Amos, vec!["Am".to_string()]),
        (BibleBook::Obadiah, vec!["Ob".to_string()]),
        (BibleBook::Jonah, vec!["Jon".to_string()]),
        (BibleBook::Micah, vec!["Mi".to_string()]),
        (BibleBook::Nahum, vec!["Nah".to_string()]),
        (BibleBook::Habakkuk, vec!["Hab".to_string()]),
        (BibleBook::Zephaniah, vec!["Zeph".to_string()]),
        (BibleBook::Haggai, vec!["Hag".to_string()]),
        (BibleBook::Zechariah, vec!["Sach".to_string()]),
        (BibleBook::Malachi, vec!["Mal".to_string()]),
        (BibleBook::Matthew, vec!["Mt".to_string()]),
        (BibleBook::Mark, vec!["Mk".to_string()]),
        (BibleBook::Luke, vec!["Lk".to_string()]),
        (BibleBook::John, vec!["Joh".to_string()]),
        (BibleBook::Acts, vec!["Apg".to_string()]),
        (BibleBook::Romans, vec!["Röm".to_string()]),
        (BibleBook::ICorinthians, vec!["1Kor".to_string()]),
        (BibleBook::IICorinthians, vec!["2Kor".to_string()]),
        (BibleBook::Galatians, vec!["Gal".to_string()]),
        (BibleBook::Ephesians, vec!["Eph".to_string()]),
        (BibleBook::Philippians, vec!["Phil".to_string()]),
        (BibleBook::Colossians, vec!["Kol".to_string()]),
        (BibleBook::IThessalonians, vec!["1Thess".to_string()]),
        (BibleBook::IIThessalonians, vec!["2Thess".to_string()]),
        (BibleBook::ITimothy, vec!["1Tim".to_string()]),
        (BibleBook::IITimothy, vec!["2Tim".to_string()]),
        (BibleBook::Titus, vec!["Tit".to_string()]),
        (BibleBook::Philemon, vec!["Phlm".to_string()]),
        (BibleBook::Hebrews, vec!["Hebr".to_string()]),
        (BibleBook::James, vec!["Jak".to_string()]),
        (BibleBook::IPeter, vec!["1Petr".to_string()]),
        (BibleBook::IIPeter, vec!["2Petr".to_string()]),
        (BibleBook::IJohn, vec!["1Joh".to_string()]),
        (BibleBook::IIJohn, vec!["2Joh".to_string()]),
        (BibleBook::IIIJohn, vec!["3Joh".to_string()]),
        (BibleBook::Jude, vec!["Jud".to_string()]),
        (BibleBook::Revelation, vec!["Offb".to_string()]),
    ];
    let short_names: HashMap<BibleBook, Vec<String>> = short_names_vec.into_iter().collect();

    ReferenceLanguage {
        long_language_name: "German".to_string(),
        language_code: "de".to_string(),
        long_names,
        short_names,
        delimiter: vec![",".to_string(), ":".to_string()],
        space_sepeeration: true,
    }
}


fn get_chinese_simplified_reference_language() -> ReferenceLanguage {
    let long_names_vec = vec![
        (BibleBook::Genesis, vec!["创世记".to_string()]),
        (BibleBook::Exodus, vec!["出埃及记".to_string()]),
        (BibleBook::Leviticus, vec!["利未记".to_string()]),
        (BibleBook::Numbers, vec!["民数记".to_string()]),
        (BibleBook::Deuteronomy, vec!["申命记".to_string()]),
        (BibleBook::Joshua, vec!["约书亚记".to_string()]),
        (BibleBook::Judges, vec!["士师记".to_string()]),
        (BibleBook::Ruth, vec!["路得记".to_string()]),
        (BibleBook::ISamuel, vec!["撒母耳记上".to_string()]),
        (BibleBook::IISamuel, vec!["撒母耳记下".to_string()]),
        (BibleBook::IKings, vec!["列王纪上".to_string()]),
        (BibleBook::IIKings, vec!["列王纪下".to_string()]),
        (BibleBook::IChronicles, vec!["历代志上".to_string()]),
        (BibleBook::IIChronicles, vec!["历代志下".to_string()]),
        (BibleBook::Ezra, vec!["以斯拉记".to_string()]),
        (BibleBook::Nehemiah, vec!["尼希米记".to_string()]),
        (BibleBook::Esther, vec!["以斯帖记".to_string()]),
        (BibleBook::Job, vec!["约伯记".to_string()]),
        (BibleBook::Psalm, vec!["诗篇".to_string()]), // Plural in Chinese
        (BibleBook::Proverbs, vec!["箴言".to_string()]),
        (BibleBook::Ecclesiastes, vec!["传道书".to_string()]),
        (BibleBook::SongofSolomon, vec!["雅歌".to_string()]),
        (BibleBook::Isaiah, vec!["以赛亚书".to_string()]),
        (BibleBook::Jeremiah, vec!["耶利米书".to_string()]),
        (BibleBook::Lamentations, vec!["耶利米哀歌".to_string()]),
        (BibleBook::Ezekiel, vec!["以西结书".to_string()]),
        (BibleBook::Daniel, vec!["但以理书".to_string()]),
        (BibleBook::Hosea, vec!["何西阿书".to_string()]),
        (BibleBook::Joel, vec!["约珥书".to_string()]),
        (BibleBook::Amos, vec!["阿摩司书".to_string()]),
        (BibleBook::Obadiah, vec!["俄巴底亚书".to_string()]),
        (BibleBook::Jonah, vec!["约拿书".to_string()]),
        (BibleBook::Micah, vec!["弥迦书".to_string()]),
        (BibleBook::Nahum, vec!["那鸿书".to_string()]),
        (BibleBook::Habakkuk, vec!["哈巴谷书".to_string()]),
        (BibleBook::Zephaniah, vec!["西番雅书".to_string()]),
        (BibleBook::Haggai, vec!["哈该书".to_string()]),
        (BibleBook::Zechariah, vec!["撒迦利亚书".to_string()]),
        (BibleBook::Malachi, vec!["玛拉基书".to_string()]),
        (BibleBook::Matthew, vec!["马太福音".to_string()]),
        (BibleBook::Mark, vec!["马可福音".to_string()]),
        (BibleBook::Luke, vec!["路加福音".to_string()]),
        (BibleBook::John, vec!["约翰福音".to_string()]),
        (BibleBook::Acts, vec!["使徒行传".to_string()]),
        (BibleBook::Romans, vec!["罗马书".to_string()]),
        (BibleBook::ICorinthians, vec!["哥林多前书".to_string()]),
        (BibleBook::IICorinthians, vec!["哥林多后书".to_string()]),
        (BibleBook::Galatians, vec!["加拉太书".to_string()]),
        (BibleBook::Ephesians, vec!["以弗所书".to_string()]),
        (BibleBook::Philippians, vec!["腓立比书".to_string()]),
        (BibleBook::Colossians, vec!["歌罗西书".to_string()]),
        (BibleBook::IThessalonians, vec!["帖撒罗尼迦前书".to_string()]),
        (BibleBook::IIThessalonians, vec!["帖撒罗尼迦后书".to_string()]),
        (BibleBook::ITimothy, vec!["提摩太前书".to_string()]),
        (BibleBook::IITimothy, vec!["提摩太后书".to_string()]),
        (BibleBook::Titus, vec!["提多书".to_string()]),
        (BibleBook::Philemon, vec!["腓利门书".to_string()]),
        (BibleBook::Hebrews, vec!["希伯来书".to_string()]),
        (BibleBook::James, vec!["雅各书".to_string()]),
        (BibleBook::IPeter, vec!["彼得前书".to_string()]),
        (BibleBook::IIPeter, vec!["彼得后书".to_string()]),
        (BibleBook::IJohn, vec!["约翰一书".to_string()]),
        (BibleBook::IIJohn, vec!["约翰二书".to_string()]),
        (BibleBook::IIIJohn, vec!["约翰三书".to_string()]),
        (BibleBook::Jude, vec!["犹大书".to_string()]),
        (BibleBook::Revelation, vec!["启示录".to_string()]),
    ];
    let long_names: HashMap<BibleBook, Vec<String>> = long_names_vec.into_iter().collect();

    let short_names_vec = vec![
        (BibleBook::Genesis, vec!["创".to_string()]),
        (BibleBook::Exodus, vec!["出".to_string()]),
        (BibleBook::Leviticus, vec!["利".to_string()]),
        (BibleBook::Numbers, vec!["民".to_string()]),
        (BibleBook::Deuteronomy, vec!["申".to_string()]),
        (BibleBook::Joshua, vec!["书".to_string()]),
        (BibleBook::Judges, vec!["士".to_string()]),
        (BibleBook::Ruth, vec!["得".to_string()]),
        (BibleBook::ISamuel, vec!["撒上".to_string()]),
        (BibleBook::IISamuel, vec!["撒下".to_string()]),
        (BibleBook::IKings, vec!["王上".to_string()]),
        (BibleBook::IIKings, vec!["王下".to_string()]),
        (BibleBook::IChronicles, vec!["代上".to_string()]),
        (BibleBook::IIChronicles, vec!["代下".to_string()]),
        (BibleBook::Ezra, vec!["拉".to_string()]),
        (BibleBook::Nehemiah, vec!["尼".to_string()]),
        (BibleBook::Esther, vec!["斯".to_string()]),
        (BibleBook::Job, vec!["伯".to_string()]),
        (BibleBook::Psalm, vec!["诗".to_string()]), // For "诗篇"
        (BibleBook::Proverbs, vec!["箴".to_string()]),
        (BibleBook::Ecclesiastes, vec!["传".to_string()]),
        (BibleBook::SongofSolomon, vec!["歌".to_string()]),
        (BibleBook::Isaiah, vec!["赛".to_string()]),
        (BibleBook::Jeremiah, vec!["耶".to_string()]),
        (BibleBook::Lamentations, vec!["哀".to_string()]),
        (BibleBook::Ezekiel, vec!["结".to_string()]),
        (BibleBook::Daniel, vec!["但".to_string()]),
        (BibleBook::Hosea, vec!["何".to_string()]),
        (BibleBook::Joel, vec!["珥".to_string()]),
        (BibleBook::Amos, vec!["摩".to_string()]),
        (BibleBook::Obadiah, vec!["俄".to_string()]),
        (BibleBook::Jonah, vec!["拿".to_string()]),
        (BibleBook::Micah, vec!["弥".to_string()]),
        (BibleBook::Nahum, vec!["鸿".to_string()]),
        (BibleBook::Habakkuk, vec!["哈".to_string()]),
        (BibleBook::Zephaniah, vec!["番".to_string()]),
        (BibleBook::Haggai, vec!["该".to_string()]),
        (BibleBook::Zechariah, vec!["亚".to_string()]),
        (BibleBook::Malachi, vec!["玛".to_string()]),
        (BibleBook::Matthew, vec!["太".to_string()]),
        (BibleBook::Mark, vec!["可".to_string()]),
        (BibleBook::Luke, vec!["路".to_string()]),
        (BibleBook::John, vec!["约".to_string()]),
        (BibleBook::Acts, vec!["徒".to_string()]),
        (BibleBook::Romans, vec!["罗".to_string()]),
        (BibleBook::ICorinthians, vec!["林前".to_string()]),
        (BibleBook::IICorinthians, vec!["林后".to_string()]),
        (BibleBook::Galatians, vec!["加".to_string()]),
        (BibleBook::Ephesians, vec!["弗".to_string()]),
        (BibleBook::Philippians, vec!["腓".to_string()]),
        (BibleBook::Colossians, vec!["西".to_string()]),
        (BibleBook::IThessalonians, vec!["帖前".to_string()]),
        (BibleBook::IIThessalonians, vec!["帖后".to_string()]),
        (BibleBook::ITimothy, vec!["提前".to_string()]),
        (BibleBook::IITimothy, vec!["提后".to_string()]),
        (BibleBook::Titus, vec!["多".to_string()]),
        (BibleBook::Philemon, vec!["门".to_string()]),
        (BibleBook::Hebrews, vec!["来".to_string()]),
        (BibleBook::James, vec!["雅".to_string()]),
        (BibleBook::IPeter, vec!["彼前".to_string()]),
        (BibleBook::IIPeter, vec!["彼后".to_string()]),
        (BibleBook::IJohn, vec!["约一".to_string()]),
        (BibleBook::IIJohn, vec!["约二".to_string()]),
        (BibleBook::IIIJohn, vec!["约三".to_string()]),
        (BibleBook::Jude, vec!["犹".to_string()]),
        (BibleBook::Revelation, vec!["启".to_string()]),
    ];
    let short_names: HashMap<BibleBook, Vec<String>> = short_names_vec.into_iter().collect();

    ReferenceLanguage {
        long_language_name: "Chinese Simplified".to_string(),
        language_code: "zh_sim".to_string(),
        long_names,
        short_names,
        delimiter: vec!["：".to_string()],
        space_sepeeration: false,
    }
}


fn get_chinese_traditional_reference_language() -> ReferenceLanguage {
    let long_names_vec = vec![
        (BibleBook::Genesis, vec!["創世記".to_string()]),
        (BibleBook::Exodus, vec!["出埃及記".to_string()]),
        (BibleBook::Leviticus, vec!["利未記".to_string()]),
        (BibleBook::Numbers, vec!["民數記".to_string()]),
        (BibleBook::Deuteronomy, vec!["申命記".to_string()]),
        (BibleBook::Joshua, vec!["約書亞記".to_string()]),
        (BibleBook::Judges, vec!["士師記".to_string()]),
        (BibleBook::Ruth, vec!["路得記".to_string()]),
        (BibleBook::ISamuel, vec!["撒母耳記上".to_string()]),
        (BibleBook::IISamuel, vec!["撒母耳記下".to_string()]),
        (BibleBook::IKings, vec!["列王紀上".to_string()]),
        (BibleBook::IIKings, vec!["列王紀下".to_string()]),
        (BibleBook::IChronicles, vec!["歷代志上".to_string()]),
        (BibleBook::IIChronicles, vec!["歷代志下".to_string()]),
        (BibleBook::Ezra, vec!["以斯拉記".to_string()]),
        (BibleBook::Nehemiah, vec!["尼希米記".to_string()]),
        (BibleBook::Esther, vec!["以斯帖記".to_string()]),
        (BibleBook::Job, vec!["約伯記".to_string()]),
        (BibleBook::Psalm, vec!["詩篇".to_string()]), // Plural in Chinese
        (BibleBook::Proverbs, vec!["箴言".to_string()]),
        (BibleBook::Ecclesiastes, vec!["傳道書".to_string()]),
        (BibleBook::SongofSolomon, vec!["雅歌".to_string()]),
        (BibleBook::Isaiah, vec!["以賽亞書".to_string()]),
        (BibleBook::Jeremiah, vec!["耶利米書".to_string()]),
        (BibleBook::Lamentations, vec!["耶利米哀歌".to_string()]),
        (BibleBook::Ezekiel, vec!["以西結書".to_string()]),
        (BibleBook::Daniel, vec!["但以理書".to_string()]),
        (BibleBook::Hosea, vec!["何西阿書".to_string()]),
        (BibleBook::Joel, vec!["約珥書".to_string()]),
        (BibleBook::Amos, vec!["阿摩司書".to_string()]),
        (BibleBook::Obadiah, vec!["俄巴底亞書".to_string()]),
        (BibleBook::Jonah, vec!["約拿書".to_string()]),
        (BibleBook::Micah, vec!["彌迦書".to_string()]),
        (BibleBook::Nahum, vec!["那鴻書".to_string()]),
        (BibleBook::Habakkuk, vec!["哈巴谷書".to_string()]),
        (BibleBook::Zephaniah, vec!["西番雅書".to_string()]),
        (BibleBook::Haggai, vec!["哈該書".to_string()]),
        (BibleBook::Zechariah, vec!["撒迦利亞書".to_string()]),
        (BibleBook::Malachi, vec!["瑪拉基書".to_string()]),
        (BibleBook::Matthew, vec!["馬太福音".to_string()]),
        (BibleBook::Mark, vec!["馬可福音".to_string()]),
        (BibleBook::Luke, vec!["路加福音".to_string()]),
        (BibleBook::John, vec!["約翰福音".to_string()]),
        (BibleBook::Acts, vec!["使徒行傳".to_string()]),
        (BibleBook::Romans, vec!["羅馬書".to_string()]),
        (BibleBook::ICorinthians, vec!["哥林多前書".to_string()]),
        (BibleBook::IICorinthians, vec!["哥林多後書".to_string()]),
        (BibleBook::Galatians, vec!["加拉太書".to_string()]),
        (BibleBook::Ephesians, vec!["以弗所書".to_string()]),
        (BibleBook::Philippians, vec!["腓立比書".to_string()]),
        (BibleBook::Colossians, vec!["歌羅西書".to_string()]),
        (BibleBook::IThessalonians, vec!["帖撒羅尼迦前書".to_string()]),
        (BibleBook::IIThessalonians, vec!["帖撒羅尼迦後書".to_string()]),
        (BibleBook::ITimothy, vec!["提摩太前書".to_string()]),
        (BibleBook::IITimothy, vec!["提摩太後書".to_string()]),
        (BibleBook::Titus, vec!["提多書".to_string()]),
        (BibleBook::Philemon, vec!["腓利門書".to_string()]),
        (BibleBook::Hebrews, vec!["希伯來書".to_string()]),
        (BibleBook::James, vec!["雅各書".to_string()]),
        (BibleBook::IPeter, vec!["彼得前書".to_string()]),
        (BibleBook::IIPeter, vec!["彼得後書".to_string()]),
        (BibleBook::IJohn, vec!["約翰一書".to_string()]),
        (BibleBook::IIJohn, vec!["約翰二書".to_string()]),
        (BibleBook::IIIJohn, vec!["約翰三書".to_string()]),
        (BibleBook::Jude, vec!["猶大書".to_string()]),
        (BibleBook::Revelation, vec!["啟示錄".to_string()]),
    ];
    let long_names: HashMap<BibleBook, Vec<String>> = long_names_vec.into_iter().collect();

    let short_names_vec = vec![
        (BibleBook::Genesis, vec!["創".to_string()]),
        (BibleBook::Exodus, vec!["出".to_string()]),
        (BibleBook::Leviticus, vec!["利".to_string()]),
        (BibleBook::Numbers, vec!["民".to_string()]),
        (BibleBook::Deuteronomy, vec!["申".to_string()]),
        (BibleBook::Joshua, vec!["書".to_string()]),
        (BibleBook::Judges, vec!["士".to_string()]),
        (BibleBook::Ruth, vec!["得".to_string()]),
        (BibleBook::ISamuel, vec!["撒上".to_string()]),
        (BibleBook::IISamuel, vec!["撒下".to_string()]),
        (BibleBook::IKings, vec!["王上".to_string()]),
        (BibleBook::IIKings, vec!["王下".to_string()]),
        (BibleBook::IChronicles, vec!["代上".to_string()]),
        (BibleBook::IIChronicles, vec!["代下".to_string()]),
        (BibleBook::Ezra, vec!["拉".to_string()]),
        (BibleBook::Nehemiah, vec!["尼".to_string()]),
        (BibleBook::Esther, vec!["斯".to_string()]),
        (BibleBook::Job, vec!["伯".to_string()]),
        (BibleBook::Psalm, vec!["詩".to_string()]),
        (BibleBook::Proverbs, vec!["箴".to_string()]),
        (BibleBook::Ecclesiastes, vec!["傳".to_string()]),
        (BibleBook::SongofSolomon, vec!["歌".to_string()]),
        (BibleBook::Isaiah, vec!["賽".to_string()]),
        (BibleBook::Jeremiah, vec!["耶".to_string()]),
        (BibleBook::Lamentations, vec!["哀".to_string()]),
        (BibleBook::Ezekiel, vec!["結".to_string()]),
        (BibleBook::Daniel, vec!["但".to_string()]),
        (BibleBook::Hosea, vec!["何".to_string()]),
        (BibleBook::Joel, vec!["珥".to_string()]),
        (BibleBook::Amos, vec!["摩".to_string()]),
        (BibleBook::Obadiah, vec!["俄".to_string()]),
        (BibleBook::Jonah, vec!["拿".to_string()]),
        (BibleBook::Micah, vec!["彌".to_string()]),
        (BibleBook::Nahum, vec!["鴻".to_string()]),
        (BibleBook::Habakkuk, vec!["哈".to_string()]),
        (BibleBook::Zephaniah, vec!["番".to_string()]),
        (BibleBook::Haggai, vec!["該".to_string()]),
        (BibleBook::Zechariah, vec!["亞".to_string()]),
        (BibleBook::Malachi, vec!["瑪".to_string()]),
        (BibleBook::Matthew, vec!["太".to_string()]),
        (BibleBook::Mark, vec!["可".to_string()]),
        (BibleBook::Luke, vec!["路".to_string()]),
        (BibleBook::John, vec!["約".to_string()]),
        (BibleBook::Acts, vec!["徒".to_string()]),
        (BibleBook::Romans, vec!["羅".to_string()]),
        (BibleBook::ICorinthians, vec!["林前".to_string()]),
        (BibleBook::IICorinthians, vec!["林後".to_string()]),
        (BibleBook::Galatians, vec!["加".to_string()]),
        (BibleBook::Ephesians, vec!["弗".to_string()]),
        (BibleBook::Philippians, vec!["腓".to_string()]),
        (BibleBook::Colossians, vec!["西".to_string()]),
        (BibleBook::IThessalonians, vec!["帖前".to_string()]),
        (BibleBook::IIThessalonians, vec!["帖後".to_string()]),
        (BibleBook::ITimothy, vec!["提前".to_string()]),
        (BibleBook::IITimothy, vec!["提後".to_string()]),
        (BibleBook::Titus, vec!["多".to_string()]),
        (BibleBook::Philemon, vec!["門".to_string()]),
        (BibleBook::Hebrews, vec!["來".to_string()]),
        (BibleBook::James, vec!["雅".to_string()]),
        (BibleBook::IPeter, vec!["彼前".to_string()]),
        (BibleBook::IIPeter, vec!["彼後".to_string()]),
        (BibleBook::IJohn, vec!["約一".to_string()]),
        (BibleBook::IIJohn, vec!["約二".to_string()]),
        (BibleBook::IIIJohn, vec!["約三".to_string()]),
        (BibleBook::Jude, vec!["猶".to_string()]),
        (BibleBook::Revelation, vec!["啟".to_string()]),
    ];
    let short_names: HashMap<BibleBook, Vec<String>> = short_names_vec.into_iter().collect();

    ReferenceLanguage {
        long_language_name: "Chinese Traditional".to_string(),
        language_code: "zh_trad".to_string(),
        long_names,
        short_names,
        delimiter: vec!["：".to_string()],
        space_sepeeration: false,
    }
}

fn get_french_reference_language() -> ReferenceLanguage {
    let long_names_vec = vec![
        (BibleBook::Genesis, vec!["Genèse".to_string()]),
        (BibleBook::Exodus, vec!["Exode".to_string()]),
        (BibleBook::Leviticus, vec!["Lévitique".to_string()]),
        (BibleBook::Numbers, vec!["Nombres".to_string()]),
        (BibleBook::Deuteronomy, vec!["Deutéronome".to_string()]),
        (BibleBook::Joshua, vec!["Josué".to_string()]),
        (BibleBook::Judges, vec!["Juges".to_string()]),
        (BibleBook::Ruth, vec!["Ruth".to_string()]),
        (BibleBook::ISamuel, vec!["1 Samuel".to_string()]),
        (BibleBook::IISamuel, vec!["2 Samuel".to_string()]),
        (BibleBook::IKings, vec!["1 Rois".to_string()]),
        (BibleBook::IIKings, vec!["2 Rois".to_string()]),
        (BibleBook::IChronicles, vec!["1 Chroniques".to_string()]),
        (BibleBook::IIChronicles, vec!["2 Chroniques".to_string()]),
        (BibleBook::Ezra, vec!["Esdras".to_string()]),
        (BibleBook::Nehemiah, vec!["Néhémie".to_string()]),
        (BibleBook::Esther, vec!["Esther".to_string()]),
        (BibleBook::Job, vec!["Job".to_string()]),
        (BibleBook::Psalm, vec!["Psaumes".to_string()]), // Plural in French
        (BibleBook::Proverbs, vec!["Proverbes".to_string()]),
        (BibleBook::Ecclesiastes, vec!["Ecclésiaste".to_string()]),
        (BibleBook::SongofSolomon, vec!["Cantique des Cantiques".to_string()]),
        (BibleBook::Isaiah, vec!["Ésaïe".to_string()]),
        (BibleBook::Jeremiah, vec!["Jérémie".to_string()]),
        (BibleBook::Lamentations, vec!["Lamentations".to_string()]),
        (BibleBook::Ezekiel, vec!["Ézéchiel".to_string()]),
        (BibleBook::Daniel, vec!["Daniel".to_string()]),
        (BibleBook::Hosea, vec!["Osée".to_string()]),
        (BibleBook::Joel, vec!["Joël".to_string()]),
        (BibleBook::Amos, vec!["Amos".to_string()]),
        (BibleBook::Obadiah, vec!["Abdias".to_string()]),
        (BibleBook::Jonah, vec!["Jonas".to_string()]),
        (BibleBook::Micah, vec!["Michée".to_string()]),
        (BibleBook::Nahum, vec!["Nahum".to_string()]),
        (BibleBook::Habakkuk, vec!["Habacuc".to_string()]),
        (BibleBook::Zephaniah, vec!["Sophonie".to_string()]),
        (BibleBook::Haggai, vec!["Aggée".to_string()]),
        (BibleBook::Zechariah, vec!["Zacharie".to_string()]),
        (BibleBook::Malachi, vec!["Malachie".to_string()]),
        (BibleBook::Matthew, vec!["Matthieu".to_string()]),
        (BibleBook::Mark, vec!["Marc".to_string()]),
        (BibleBook::Luke, vec!["Luc".to_string()]),
        (BibleBook::John, vec!["Jean".to_string()]),
        (BibleBook::Acts, vec!["Actes".to_string()]),
        (BibleBook::Romans, vec!["Romains".to_string()]),
        (BibleBook::ICorinthians, vec!["1 Corinthiens".to_string()]),
        (BibleBook::IICorinthians, vec!["2 Corinthiens".to_string()]),
        (BibleBook::Galatians, vec!["Galates".to_string()]),
        (BibleBook::Ephesians, vec!["Éphésiens".to_string()]),
        (BibleBook::Philippians, vec!["Philippiens".to_string()]),
        (BibleBook::Colossians, vec!["Colossiens".to_string()]),
        (BibleBook::IThessalonians, vec!["1 Thessaloniciens".to_string()]),
        (BibleBook::IIThessalonians, vec!["2 Thessaloniciens".to_string()]),
        (BibleBook::ITimothy, vec!["1 Timothée".to_string()]),
        (BibleBook::IITimothy, vec!["2 Timothée".to_string()]),
        (BibleBook::Titus, vec!["Tite".to_string()]),
        (BibleBook::Philemon, vec!["Philémon".to_string()]),
        (BibleBook::Hebrews, vec!["Hébreux".to_string()]),
        (BibleBook::James, vec!["Jacques".to_string()]),
        (BibleBook::IPeter, vec!["1 Pierre".to_string()]),
        (BibleBook::IIPeter, vec!["2 Pierre".to_string()]),
        (BibleBook::IJohn, vec!["1 Jean".to_string()]),
        (BibleBook::IIJohn, vec!["2 Jean".to_string()]),
        (BibleBook::IIIJohn, vec!["3 Jean".to_string()]),
        (BibleBook::Jude, vec!["Jude".to_string()]),
        (BibleBook::Revelation, vec!["Apocalypse".to_string()]),
    ];
    let long_names: HashMap<BibleBook, Vec<String>> = long_names_vec.into_iter().collect();

    let short_names_vec = vec![
        (BibleBook::Genesis, vec!["Gn".to_string()]),
        (BibleBook::Exodus, vec!["Ex".to_string()]),
        (BibleBook::Leviticus, vec!["Lv".to_string()]), // Alternative to "Lév"
        (BibleBook::Numbers, vec!["Nb".to_string()]),
        (BibleBook::Deuteronomy, vec!["Dt".to_string()]),
        (BibleBook::Joshua, vec!["Jos".to_string()]),
        (BibleBook::Judges, vec!["Jg".to_string()]),
        (BibleBook::Ruth, vec!["Rt".to_string()]),
        (BibleBook::ISamuel, vec!["1 S".to_string()]),
        (BibleBook::IISamuel, vec!["2 S".to_string()]),
        (BibleBook::IKings, vec!["1 R".to_string()]),
        (BibleBook::IIKings, vec!["2 R".to_string()]),
        (BibleBook::IChronicles, vec!["1 Ch".to_string()]),
        (BibleBook::IIChronicles, vec!["2 Ch".to_string()]),
        (BibleBook::Ezra, vec!["Esd".to_string()]),
        (BibleBook::Nehemiah, vec!["Né".to_string()]),
        (BibleBook::Esther, vec!["Est".to_string()]),
        (BibleBook::Job, vec!["Jb".to_string()]),
        (BibleBook::Psalm, vec!["Ps".to_string()]),
        (BibleBook::Proverbs, vec!["Pr".to_string()]),
        (BibleBook::Ecclesiastes, vec!["Ec".to_string()]),
        (BibleBook::SongofSolomon, vec!["Ct".to_string()]),
        (BibleBook::Isaiah, vec!["És".to_string()]),
        (BibleBook::Jeremiah, vec!["Jr".to_string()]),
        (BibleBook::Lamentations, vec!["Lm".to_string()]),
        (BibleBook::Ezekiel, vec!["Éz".to_string()]),
        (BibleBook::Daniel, vec!["Dn".to_string()]),
        (BibleBook::Hosea, vec!["Os".to_string()]),
        (BibleBook::Joel, vec!["Jl".to_string()]),
        (BibleBook::Amos, vec!["Am".to_string()]),
        (BibleBook::Obadiah, vec!["Ab".to_string()]),
        (BibleBook::Jonah, vec!["Jon".to_string()]),
        (BibleBook::Micah, vec!["Mi".to_string()]),
        (BibleBook::Nahum, vec!["Na".to_string()]),
        (BibleBook::Habakkuk, vec!["Ha".to_string()]),
        (BibleBook::Zephaniah, vec!["So".to_string()]),
        (BibleBook::Haggai, vec!["Ag".to_string()]),
        (BibleBook::Zechariah, vec!["Za".to_string()]),
        (BibleBook::Malachi, vec!["Mal".to_string()]),
        (BibleBook::Matthew, vec!["Mt".to_string()]),
        (BibleBook::Mark, vec!["Mc".to_string()]),
        (BibleBook::Luke, vec!["Lc".to_string()]),
        (BibleBook::John, vec!["Jn".to_string()]),
        (BibleBook::Acts, vec!["Ac".to_string()]),
        (BibleBook::Romans, vec!["Rm".to_string()]),
        (BibleBook::ICorinthians, vec!["1 Co".to_string()]),
        (BibleBook::IICorinthians, vec!["2 Co".to_string()]),
        (BibleBook::Galatians, vec!["Ga".to_string()]),
        (BibleBook::Ephesians, vec!["Ép".to_string()]),
        (BibleBook::Philippians, vec!["Ph".to_string()]),
        (BibleBook::Colossians, vec!["Col".to_string()]),
        (BibleBook::IThessalonians, vec!["1 Th".to_string()]),
        (BibleBook::IIThessalonians, vec!["2 Th".to_string()]),
        (BibleBook::ITimothy, vec!["1 Tm".to_string()]),
        (BibleBook::IITimothy, vec!["2 Tm".to_string()]),
        (BibleBook::Titus, vec!["Tt".to_string()]),
        (BibleBook::Philemon, vec!["Phm".to_string()]),
        (BibleBook::Hebrews, vec!["Hé".to_string()]),
        (BibleBook::James, vec!["Jc".to_string()]),
        (BibleBook::IPeter, vec!["1 P".to_string()]),
        (BibleBook::IIPeter, vec!["2 P".to_string()]),
        (BibleBook::IJohn, vec!["1 Jn".to_string()]),
        (BibleBook::IIJohn, vec!["2 Jn".to_string()]),
        (BibleBook::IIIJohn, vec!["3 Jn".to_string()]),
        (BibleBook::Jude, vec!["Jd".to_string()]),
        (BibleBook::Revelation, vec!["Ap".to_string()]),
    ];
    let short_names: HashMap<BibleBook, Vec<String>> = short_names_vec.into_iter().collect();

    ReferenceLanguage {
        long_language_name: "French".to_string(),
        language_code: "fr".to_string(),
        long_names,
        short_names,
        delimiter: vec![":".to_string(), ",".to_string()],
        space_sepeeration: true,
    }
}

#[cfg(test)]
mod tests {
    use crate::bible::BibleVerseReference;

    use super::*;

    #[test]
    fn test_references_to_human_language() {
        // Test John 3:16 in multiple languages
        let reference1: BibleReference = BibleReference::BibleVerse(
            BibleVerseReference::new(BibleBook::John, 3, 16).unwrap()
        );
        assert_eq!(
            get_reference_in_language(&reference1, "en", BookReferenceType::Long).unwrap(),
            "John 3:16".to_string(),
        );
        assert_eq!(
            get_reference_in_language(&reference1, "de", BookReferenceType::Long).unwrap(),
            "Johannes 3,16".to_string()
        );
        assert_eq!(
            get_reference_in_language(&reference1, "de", BookReferenceType::Short).unwrap(),
            "Joh 3,16".to_string()
        );
        assert_eq!(
            get_reference_in_language(&reference1, "zh_sim", BookReferenceType::Long).unwrap(),
            "约翰福音3：16".to_string()
        )
    }
}