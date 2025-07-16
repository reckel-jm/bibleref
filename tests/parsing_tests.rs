use bibleref::{
    bible::{
        BibleBook, BibleBookReference, BibleChapterReference, BibleRange, BibleReference,
        BibleReferenceRepresentation, BibleVerseReference,
    },
    parse, translate,
    referencing::{
        language::BookReferenceType,
        parser::{parse_reference, parse_single_reference},
    },
};

// ==========================================
// 1.1 Single Reference Parsing Tests
// ==========================================

#[test]
fn test_parse_book_references_english() {
    // Test parsing book references in English
    let book_references = vec![
        "Genesis", "Exodus", "Leviticus", "Numbers", "Deuteronomy",
        "Joshua", "Judges", "Ruth", "1 Samuel", "2 Samuel",
        "1 Kings", "2 Kings", "1 Chronicles", "2 Chronicles", "Ezra",
        "Nehemiah", "Esther", "Job", "Psalms", "Psalm",
        "Proverbs", "Ecclesiastes", "Song of Solomon", "Isaiah", "Jeremiah",
        "Lamentations", "Ezekiel", "Daniel", "Hosea", "Joel",
        "Amos", "Obadiah", "Jonah", "Micah", "Nahum",
        "Habakkuk", "Zephaniah", "Haggai", "Zechariah", "Malachi",
        "Matthew", "Mark", "Luke", "John", "Acts",
        "Romans", "1 Corinthians", "2 Corinthians", "Galatians", "Ephesians",
        "Philippians", "Colossians", "1 Thessalonians", "2 Thessalonians", "1 Timothy",
        "2 Timothy", "Titus", "Philemon", "Hebrews", "James",
        "1 Peter", "2 Peter", "1 John", "2 John", "3 John",
        "Jude", "Revelation"
    ];

    for book_ref in book_references {
        let result = parse(book_ref);
        assert!(
            result.is_ok(),
            "Failed to parse book reference '{}': {:?}",
            book_ref,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_single(),
            "Expected single reference for '{}', got range",
            book_ref
        );
        
        if let BibleReferenceRepresentation::Single(BibleReference::BibleBook(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected BibleBook reference for '{}', got {:?}",
                book_ref,
                reference
            );
        }
    }
}

#[test]
fn test_parse_book_references_german() {
    // Test parsing book references in German
    let book_references = vec![
        "1. Mose", "2. Mose", "3. Mose", "4. Mose", "5. Mose",
        "Josua", "Richter", "Rut", "1. Samuel", "2. Samuel",
        "1. Könige", "2. Könige", "1. Chronik", "2. Chronik", "Esra",
        "Nehemia", "Ester", "Hiob", "Psalmen", "Psalm",
        "Sprüche", "Prediger", "Hohelied", "Jesaja", "Jeremia",
        "Klagelieder", "Hesekiel", "Daniel", "Hosea", "Joel",
        "Amos", "Obadja", "Jona", "Micha", "Nahum",
        "Habakuk", "Zefanja", "Haggai", "Sacharja", "Maleachi",
        "Matthäus", "Markus", "Lukas", "Johannes", "Apostelgeschichte",
        "Römer", "1. Korinther", "2. Korinther", "Galater", "Epheser",
        "Philipper", "Kolosser", "1. Thessalonicher", "2. Thessalonicher", "1. Timotheus",
        "2. Timotheus", "Titus", "Philemon", "Hebräer", "Jakobus",
        "1. Petrus", "2. Petrus", "1. Johannes", "2. Johannes", "3. Johannes",
        "Judas", "Offenbarung"
    ];

    for book_ref in book_references {
        let result = parse(book_ref);
        assert!(
            result.is_ok(),
            "Failed to parse book reference '{}': {:?}",
            book_ref,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_single(),
            "Expected single reference for '{}', got range",
            book_ref
        );
        
        if let BibleReferenceRepresentation::Single(BibleReference::BibleBook(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected BibleBook reference for '{}', got {:?}",
                book_ref,
                reference
            );
        }
    }
}

#[test]
fn test_parse_book_references_chinese_simplified() {
    // Test parsing book references in Chinese (Simplified)
    let book_references = vec![
        "创世记", "出埃及记", "利未记", "民数记", "申命记",
        "约书亚记", "士师记", "路得记", "撒母耳记上", "撒母耳记下",
        "列王纪上", "列王纪下", "历代志上", "历代志下", "以斯拉记",
        "尼希米记", "以斯帖记", "约伯记", "诗篇", "箴言",
        "传道书", "雅歌", "以赛亚书", "耶利米书", "耶利米哀歌",
        "以西结书", "但以理书", "何西阿书", "约珥书", "阿摩司书",
        "俄巴底亚书", "约拿书", "弥迦书", "那鸿书", "哈巴谷书",
        "西番雅书", "哈该书", "撒迦利亚书", "玛拉基书", "马太福音",
        "马可福音", "路加福音", "约翰福音", "使徒行传", "罗马书",
        "哥林多前书", "哥林多后书", "加拉太书", "以弗所书", "腓立比书",
        "歌罗西书", "帖撒罗尼迦前书", "帖撒罗尼迦后书", "提摩太前书", "提摩太后书",
        "提多书", "腓利门书", "希伯来书", "雅各书", "彼得前书",
        "彼得后书", "约翰一书", "约翰二书", "约翰三书", "犹大书",
        "启示录"
    ];

    for book_ref in book_references {
        let result = parse(book_ref);
        assert!(
            result.is_ok(),
            "Failed to parse book reference '{}': {:?}",
            book_ref,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_single(),
            "Expected single reference for '{}', got range",
            book_ref
        );
        
        if let BibleReferenceRepresentation::Single(BibleReference::BibleBook(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected BibleBook reference for '{}', got {:?}",
                book_ref,
                reference
            );
        }
    }
}

#[test]
fn test_parse_chapter_references_english() {
    // Test parsing chapter references in English
    let chapter_references = vec![
        "Genesis 1", "Exodus 2", "Leviticus 3", "Numbers 4", "Deuteronomy 5",
        "Joshua 6", "Judges 7", "Ruth 1", "1 Samuel 8", "2 Samuel 9",
        "Matthew 10", "Mark 11", "Luke 12", "John 13", "Acts 14",
        "Romans 15", "1 Corinthians 16", "Revelation 1"
    ];

    for chapter_ref in chapter_references {
        let result = parse(chapter_ref);
        assert!(
            result.is_ok(),
            "Failed to parse chapter reference '{}': {:?}",
            chapter_ref,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_single(),
            "Expected single reference for '{}', got range",
            chapter_ref
        );
        
        if let BibleReferenceRepresentation::Single(BibleReference::BibleChapter(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected BibleChapter reference for '{}', got {:?}",
                chapter_ref,
                reference
            );
        }
    }
}

#[test]
fn test_parse_chapter_references_german() {
    // Test parsing chapter references in German
    let chapter_references = vec![
        "1. Mose 1", "2. Mose 2", "3. Mose 3", "4. Mose 4", "5. Mose 5",
        "Josua 6", "Richter 7", "Rut 1", "1. Samuel 8", "2. Samuel 9",
        "Matthäus 10", "Markus 11", "Lukas 12", "Johannes 13", "Apostelgeschichte 14",
        "Römer 15", "1. Korinther 16", "Offenbarung 1"
    ];

    for chapter_ref in chapter_references {
        let result = parse(chapter_ref);
        assert!(
            result.is_ok(),
            "Failed to parse chapter reference '{}': {:?}",
            chapter_ref,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_single(),
            "Expected single reference for '{}', got range",
            chapter_ref
        );
        
        if let BibleReferenceRepresentation::Single(BibleReference::BibleChapter(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected BibleChapter reference for '{}', got {:?}",
                chapter_ref,
                reference
            );
        }
    }
}

#[test]
fn test_parse_chapter_references_chinese_simplified() {
    // Test parsing chapter references in Chinese (Simplified)
    let chapter_references = vec![
        "创世记1", "出埃及记2", "利未记3", "民数记4", "申命记5",
        "约书亚记6", "士师记7", "路得记1", "撒母耳记上8", "撒母耳记下9",
        "马太福音10", "马可福音11", "路加福音12", "约翰福音13", "使徒行传14",
        "罗马书15", "哥林多前书16", "启示录1"
    ];

    for chapter_ref in chapter_references {
        let result = parse(chapter_ref);
        assert!(
            result.is_ok(),
            "Failed to parse chapter reference '{}': {:?}",
            chapter_ref,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_single(),
            "Expected single reference for '{}', got range",
            chapter_ref
        );
        
        if let BibleReferenceRepresentation::Single(BibleReference::BibleChapter(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected BibleChapter reference for '{}', got {:?}",
                chapter_ref,
                reference
            );
        }
    }
}

#[test]
fn test_parse_verse_references_english() {
    // Test parsing verse references in English
    let verse_references = vec![
        "Genesis 1:1", "Exodus 2:2", "Leviticus 3:3", "Numbers 4:4", "Deuteronomy 5:5",
        "Joshua 6:6", "Judges 7:7", "Ruth 1:8", "1 Samuel 8:9", "2 Samuel 9:10",
        "Matthew 10:11", "Mark 11:12", "Luke 12:13", "John 13:14", "Acts 14:15",
        "Romans 15:16", "1 Corinthians 16:17", "Revelation 1:18"
    ];

    for verse_ref in verse_references {
        let result = parse(verse_ref);
        assert!(
            result.is_ok(),
            "Failed to parse verse reference '{}': {:?}",
            verse_ref,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_single(),
            "Expected single reference for '{}', got range",
            verse_ref
        );
        
        if let BibleReferenceRepresentation::Single(BibleReference::BibleVerse(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected BibleVerse reference for '{}', got {:?}",
                verse_ref,
                reference
            );
        }
    }
}

#[test]
fn test_parse_verse_references_german() {
    // Test parsing verse references in German
    let verse_references = vec![
        "1. Mose 1,1", "2. Mose 2,2", "3. Mose 3,3", "4. Mose 4,4", "5. Mose 5,5",
        "Josua 6,6", "Richter 7,7", "Rut 1,8", "1. Samuel 8,9", "2. Samuel 9,10",
        "Matthäus 10,11", "Markus 11,12", "Lukas 12,13", "Johannes 13,14", "Apostelgeschichte 14,15",
        "Römer 15,16", "1. Korinther 16,17", "Offenbarung 1,18"
    ];

    for verse_ref in verse_references {
        let result = parse(verse_ref);
        assert!(
            result.is_ok(),
            "Failed to parse verse reference '{}': {:?}",
            verse_ref,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_single(),
            "Expected single reference for '{}', got range",
            verse_ref
        );
        
        if let BibleReferenceRepresentation::Single(BibleReference::BibleVerse(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected BibleVerse reference for '{}', got {:?}",
                verse_ref,
                reference
            );
        }
    }
}

#[test]
fn test_parse_verse_references_chinese_simplified() {
    // Test parsing verse references in Chinese (Simplified)
    let verse_references = vec![
        "创世记1：1", "出埃及记2：2", "利未记3：3", "民数记4：4", "申命记5：5",
        "约书亚记6：6", "士师记7：7", "路得记1：8", "撒母耳记上8：9", "撒母耳记下9：10",
        "马太福音10：11", "马可福音11：12", "路加福音12：13", "约翰福音13：14", "使徒行传14：15",
        "罗马书15：16", "哥林多前书16：17", "启示录1：18"
    ];

    for verse_ref in verse_references {
        let result = parse(verse_ref);
        assert!(
            result.is_ok(),
            "Failed to parse verse reference '{}': {:?}",
            verse_ref,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_single(),
            "Expected single reference for '{}', got range",
            verse_ref
        );
        
        if let BibleReferenceRepresentation::Single(BibleReference::BibleVerse(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected BibleVerse reference for '{}', got {:?}",
                verse_ref,
                reference
            );
        }
    }
}

#[test]
fn test_parse_with_extra_spaces() {
    // Test parsing with extra spaces
    let references_with_spaces = vec![
        "Genesis   1:1", "  Exodus 2:2  ", " Leviticus  3 : 3 ",
        "1   Samuel   8 : 9", "Matthew  10 : 11  "
    ];

    for ref_with_spaces in references_with_spaces {
        let result = parse(ref_with_spaces);
        assert!(
            result.is_ok(),
            "Failed to parse reference with extra spaces '{}': {:?}",
            ref_with_spaces,
            result.err()
        );
    }
}

// ==========================================
// 1.2 Range Reference Parsing Tests
// ==========================================

#[test]
fn test_parse_verse_ranges_english() {
    // Test parsing verse ranges in English
    let verse_ranges = vec![
        "Genesis 1:1-3", "Exodus 2:2-4", "Leviticus 3:3-5", "Numbers 4:4-6", "Deuteronomy 5:5-7",
        "Joshua 6:6-8", "Judges 7:7-9", "Ruth 1:8-10", "1 Samuel 8:9-11", "2 Samuel 9:10-12",
        "Matthew 10:11-13", "Mark 11:12-14", "Luke 12:13-15", "John 13:14-16", "Acts 14:15-17",
        "Romans 15:16-18", "1 Corinthians 16:17-19", "Revelation 1:18-20"
    ];

    for verse_range in verse_ranges {
        let result = parse(verse_range);
        assert!(
            result.is_ok(),
            "Failed to parse verse range '{}': {:?}",
            verse_range,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_range(),
            "Expected range reference for '{}', got single",
            verse_range
        );
        
        if let BibleReferenceRepresentation::Range(BibleRange::VerseRange(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected VerseRange reference for '{}', got {:?}",
                verse_range,
                reference
            );
        }
    }
}

#[test]
fn test_parse_verse_ranges_german() {
    // Test parsing verse ranges in German
    let verse_ranges = vec![
        "1. Mose 1,1-3", "2. Mose 2,2-4", "3. Mose 3,3-5", "4. Mose 4,4-6", "5. Mose 5,5-7",
        "Josua 6,6-8", "Richter 7,7-9", "Rut 1,8-10", "1. Samuel 8,9-11", "2. Samuel 9,10-12",
        "Matthäus 10,11-13", "Markus 11,12-14", "Lukas 12,13-15", "Johannes 13,14-16", "Apostelgeschichte 14,15-17",
        "Römer 15,16-18", "1. Korinther 16,17-19", "Offenbarung 1,18-20"
    ];

    for verse_range in verse_ranges {
        let result = parse(verse_range);
        assert!(
            result.is_ok(),
            "Failed to parse verse range '{}': {:?}",
            verse_range,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_range(),
            "Expected range reference for '{}', got single",
            verse_range
        );
        
        if let BibleReferenceRepresentation::Range(BibleRange::VerseRange(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected VerseRange reference for '{}', got {:?}",
                verse_range,
                reference
            );
        }
    }
}

#[test]
fn test_parse_verse_ranges_chinese_simplified() {
    // Test parsing verse ranges in Chinese (Simplified)
    let verse_ranges = vec![
        "创世记1：1-3", "出埃及记2：2-4", "利未记3：3-5", "民数记4：4-6", "申命记5：5-7",
        "约书亚记6：6-8", "士师记7：7-9", "路得记1：8-10", "撒母耳记上8：9-11", "撒母耳记下9：10-12",
        "马太福音10：11-13", "马可福音11：12-14", "路加福音12：13-15", "约翰福音13：14-16", "使徒行传14：15-17",
        "罗马书15：16-18", "哥林多前书16：17-19", "启示录1：18-20"
    ];

    for verse_range in verse_ranges {
        let result = parse(verse_range);
        assert!(
            result.is_ok(),
            "Failed to parse verse range '{}': {:?}",
            verse_range,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_range(),
            "Expected range reference for '{}', got single",
            verse_range
        );
        
        if let BibleReferenceRepresentation::Range(BibleRange::VerseRange(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected VerseRange reference for '{}', got {:?}",
                verse_range,
                reference
            );
        }
    }
}

#[test]
fn test_parse_chapter_ranges_english() {
    // Test parsing chapter ranges in English
    let chapter_ranges = vec![
        "Genesis 1-3", "Exodus 2-4", "Leviticus 3-5", "Numbers 4-6", "Deuteronomy 5-7",
        "Joshua 6-8", "Judges 7-9", "Ruth 1-2", "1 Samuel 8-10", "2 Samuel 9-11",
        "Matthew 10-12", "Mark 11-13", "Luke 12-14", "John 13-15", "Acts 14-16",
        "Romans 15-16", "1 Corinthians 1-3", "Revelation 1-3"
    ];

    for chapter_range in chapter_ranges {
        let result = parse(chapter_range);
        assert!(
            result.is_ok(),
            "Failed to parse chapter range '{}': {:?}",
            chapter_range,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_range(),
            "Expected range reference for '{}', got single",
            chapter_range
        );
        
        if let BibleReferenceRepresentation::Range(BibleRange::ChapterRange(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected ChapterRange reference for '{}', got {:?}",
                chapter_range,
                reference
            );
        }
    }
}

#[test]
fn test_parse_chapter_ranges_german() {
    // Test parsing chapter ranges in German
    let chapter_ranges = vec![
        "1. Mose 1-3", "2. Mose 2-4", "3. Mose 3-5", "4. Mose 4-6", "5. Mose 5-7",
        "Josua 6-8", "Richter 7-9", "Rut 1-2", "1. Samuel 8-10", "2. Samuel 9-11",
        "Matthäus 10-12", "Markus 11-13", "Lukas 12-14", "Johannes 13-15", "Apostelgeschichte 14-16",
        "Römer 15-16", "1. Korinther 1-3", "Offenbarung 1-3"
    ];

    for chapter_range in chapter_ranges {
        let result = parse(chapter_range);
        assert!(
            result.is_ok(),
            "Failed to parse chapter range '{}': {:?}",
            chapter_range,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_range(),
            "Expected range reference for '{}', got single",
            chapter_range
        );
        
        if let BibleReferenceRepresentation::Range(BibleRange::ChapterRange(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected ChapterRange reference for '{}', got {:?}",
                chapter_range,
                reference
            );
        }
    }
}

#[test]
fn test_parse_chapter_ranges_chinese_simplified() {
    // Test parsing chapter ranges in Chinese (Simplified)
    let chapter_ranges = vec![
        "创世记1-3", "出埃及记2-4", "利未记3-5", "民数记4-6", "申命记5-7",
        "约书亚记6-8", "士师记7-9", "路得记1-2", "撒母耳记上8-10", "撒母耳记下9-11",
        "马太福音10-12", "马可福音11-13", "路加福音12-14", "约翰福音13-15", "使徒行传14-16",
        "罗马书15-16", "哥林多前书1-3", "启示录1-3"
    ];

    for chapter_range in chapter_ranges {
        let result = parse(chapter_range);
        assert!(
            result.is_ok(),
            "Failed to parse chapter range '{}': {:?}",
            chapter_range,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_range(),
            "Expected range reference for '{}', got single",
            chapter_range
        );
        
        if let BibleReferenceRepresentation::Range(BibleRange::ChapterRange(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected ChapterRange reference for '{}', got {:?}",
                chapter_range,
                reference
            );
        }
    }
}

#[test]
fn test_parse_book_ranges_english() {
    // Test parsing book ranges in English
    let book_ranges = vec![
        "Genesis-Exodus", "Leviticus-Numbers", "Joshua-Judges",
        "1 Samuel-2 Samuel", "1 Kings-2 Kings", "1 Chronicles-2 Chronicles",
        "Matthew-Mark", "Luke-John", "Romans-1 Corinthians",
        "1 Thessalonians-2 Thessalonians", "1 Timothy-2 Timothy", "1 Peter-2 Peter"
    ];

    for book_range in book_ranges {
        let result = parse(book_range);
        assert!(
            result.is_ok(),
            "Failed to parse book range '{}': {:?}",
            book_range,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_range(),
            "Expected range reference for '{}', got single",
            book_range
        );
        
        if let BibleReferenceRepresentation::Range(BibleRange::BookRange(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected BookRange reference for '{}', got {:?}",
                book_range,
                reference
            );
        }
    }
}

#[test]
fn test_parse_book_ranges_german() {
    // Test parsing book ranges in German
    let book_ranges = vec![
        "1. Mose-2. Mose", "3. Mose-4. Mose", "Josua-Richter",
        "1. Samuel-2. Samuel", "1. Könige-2. Könige", "1. Chronik-2. Chronik",
        "Matthäus-Markus", "Lukas-Johannes", "Römer-1. Korinther",
        "1. Thessalonicher-2. Thessalonicher", "1. Timotheus-2. Timotheus", "1. Petrus-2. Petrus"
    ];

    for book_range in book_ranges {
        let result = parse(book_range);
        assert!(
            result.is_ok(),
            "Failed to parse book range '{}': {:?}",
            book_range,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_range(),
            "Expected range reference for '{}', got single",
            book_range
        );
        
        if let BibleReferenceRepresentation::Range(BibleRange::BookRange(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected BookRange reference for '{}', got {:?}",
                book_range,
                reference
            );
        }
    }
}

#[test]
fn test_parse_book_ranges_chinese_simplified() {
    // Test parsing book ranges in Chinese (Simplified)
    let book_ranges = vec![
        "创世记-出埃及记", "利未记-民数记", "约书亚记-士师记",
        "撒母耳记上-撒母耳记下", "列王纪上-列王纪下", "历代志上-历代志下",
        "马太福音-马可福音", "路加福音-约翰福音", "罗马书-哥林多前书",
        "帖撒罗尼迦前书-帖撒罗尼迦后书", "提摩太前书-提摩太后书", "彼得前书-彼得后书"
    ];

    for book_range in book_ranges {
        let result = parse(book_range);
        assert!(
            result.is_ok(),
            "Failed to parse book range '{}': {:?}",
            book_range,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_range(),
            "Expected range reference for '{}', got single",
            book_range
        );
        
        if let BibleReferenceRepresentation::Range(BibleRange::BookRange(_)) = reference {
            // This is the expected case
        } else {
            panic!(
                "Expected BookRange reference for '{}', got {:?}",
                book_range,
                reference
            );
        }
    }
}

#[test]
fn test_parse_with_different_range_delimiters() {
    // Test parsing with different range delimiters
    let ranges_with_delimiters = vec![
        // English uses hyphen
        "Genesis 1:1-3",
        // German uses hyphen
        "1. Mose 1,1-3",
        // Chinese uses hyphen
        "创世记1：1-3",
        // Some might use tilde
        "Genesis 1:1~3",
        // Some might use en dash
        "Genesis 1:1–3"
    ];

    for range_with_delimiter in ranges_with_delimiters {
        let result = parse(range_with_delimiter);
        assert!(
            result.is_ok(),
            "Failed to parse range with delimiter '{}': {:?}",
            range_with_delimiter,
            result.err()
        );
        
        let reference = result.unwrap();
        assert!(
            reference.is_range(),
            "Expected range reference for '{}', got single",
            range_with_delimiter
        );
    }
}