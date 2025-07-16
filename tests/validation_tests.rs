use bibleref::{
    bible::{
        BibleBook, BibleBookReference, BibleChapterReference, BibleRange, BibleReference,
        BibleReferenceRepresentation, BibleVerseReference,
        validate::{get_number_of_chapters, get_number_of_verses},
    },
    parse,
};

// ==========================================
// 2.1 Valid References Tests
// ==========================================

#[test]
fn test_valid_book_references() {
    // Test all valid book references
    for book in BibleBook::all() {
        let book_ref = BibleBookReference::new(book);
        // Just check that we can create a reference, no need to convert to string
        let reference = BibleReference::BibleBook(book_ref);
        assert!(
            matches!(reference, BibleReference::BibleBook(_)),
            "Failed to create valid book reference for {:?}",
            book
        );
    }
}

#[test]
fn test_valid_chapter_references() {
    // Test valid chapter references for various books
    let test_cases = vec![
        (BibleBook::Genesis, 1),     // First chapter of Genesis
        (BibleBook::Genesis, 50),    // Last chapter of Genesis
        (BibleBook::Psalm, 1),       // First chapter of Psalms
        (BibleBook::Psalm, 150),     // Last chapter of Psalms
        (BibleBook::Matthew, 1),     // First chapter of Matthew
        (BibleBook::Matthew, 28),    // Last chapter of Matthew
        (BibleBook::Revelation, 1),  // First chapter of Revelation
        (BibleBook::Revelation, 22), // Last chapter of Revelation
        (BibleBook::Jude, 1),        // Only chapter of Jude
    ];

    for (book, chapter) in test_cases {
        let chapter_ref = BibleChapterReference::new(book, chapter);
        assert!(
            chapter_ref.is_ok(),
            "Failed to create valid chapter reference for {:?} {}",
            book,
            chapter
        );
    }
}

#[test]
fn test_valid_verse_references() {
    // Test valid verse references for various books and chapters
    let test_cases = vec![
        (BibleBook::Genesis, 1, 1),     // First verse of Genesis 1
        (BibleBook::Genesis, 1, 31),    // Last verse of Genesis 1
        (BibleBook::Psalm, 119, 1),     // First verse of Psalm 119
        (BibleBook::Psalm, 119, 176),   // Last verse of Psalm 119
        (BibleBook::John, 3, 16),       // John 3:16
        (BibleBook::Romans, 8, 28),     // Romans 8:28
        (BibleBook::Revelation, 22, 21), // Last verse of the Bible
    ];

    for (book, chapter, verse) in test_cases {
        let verse_ref = BibleVerseReference::new(book, chapter, verse);
        assert!(
            verse_ref.is_ok(),
            "Failed to create valid verse reference for {:?} {}:{}",
            book,
            chapter,
            verse
        );
    }
}

#[test]
fn test_valid_references_parsing() {
    // Test parsing valid references in different formats
    let valid_references = vec![
        // Book references
        "Genesis", "Exodus", "Matthew", "Revelation",
        // Chapter references
        "Genesis 1", "Psalm 23", "John 3", "Revelation 22",
        // Verse references
        "Genesis 1:1", "Psalm 23:1", "John 3:16", "Revelation 22:21",
        // Range references
        "Genesis 1-2", "Psalm 1-3", "John 1-3",
        "Genesis 1:1-10", "Psalm 23:1-6", "John 3:16-18",
        // References in other languages
        "1. Mose 1,1", "Matthäus 5,3", "Johannes 3,16",
        "创世记1：1", "诗篇23：1", "约翰福音3：16",
    ];

    for reference in valid_references {
        let result = parse(reference);
        assert!(
            result.is_ok(),
            "Failed to parse valid reference '{}': {:?}",
            reference,
            result.err()
        );
    }
}

#[test]
fn test_boundary_valid_references() {
    // Test references at the boundaries of validity
    let books = vec![
        BibleBook::Genesis,    // First book
        BibleBook::Revelation, // Last book
        BibleBook::Jude,       // Book with only one chapter
        BibleBook::Psalm,      // Book with many chapters
    ];

    for book in books {
        // Test first chapter
        let first_chapter = BibleChapterReference::new(book, 1);
        assert!(
            first_chapter.is_ok(),
            "Failed to create first chapter reference for {:?}",
            book
        );

        // Test last chapter
        let last_chapter_num = get_number_of_chapters(&book);
        let last_chapter = BibleChapterReference::new(book, last_chapter_num);
        assert!(
            last_chapter.is_ok(),
            "Failed to create last chapter reference for {:?}",
            book
        );

        // Test first verse of first chapter
        let first_verse = BibleVerseReference::new(book, 1, 1);
        assert!(
            first_verse.is_ok(),
            "Failed to create first verse reference for {:?}",
            book
        );

        // Test last verse of last chapter
        let last_verse_num = get_number_of_verses(&book, &last_chapter_num).unwrap();
        let last_verse = BibleVerseReference::new(book, last_chapter_num, last_verse_num);
        assert!(
            last_verse.is_ok(),
            "Failed to create last verse reference for {:?}",
            book
        );
    }
}

// ==========================================
// 2.2 Invalid References Tests
// ==========================================

#[test]
fn test_invalid_chapter_references() {
    // Test invalid chapter references
    let test_cases = vec![
        (BibleBook::Genesis, 0),      // Chapter 0 doesn't exist
        (BibleBook::Genesis, 51),     // Genesis only has 50 chapters
        (BibleBook::Psalm, 151),      // Psalms only has 150 chapters
        (BibleBook::Matthew, 29),     // Matthew only has 28 chapters
        (BibleBook::Revelation, 23),  // Revelation only has 22 chapters
        (BibleBook::Jude, 2),         // Jude only has 1 chapter
    ];

    for (book, chapter) in test_cases {
        let chapter_ref = BibleChapterReference::new(book, chapter);
        assert!(
            chapter_ref.is_err(),
            "Expected error for invalid chapter reference {:?} {}, but got Ok",
            book,
            chapter
        );
    }
}

#[test]
fn test_invalid_verse_references() {
    // Test invalid verse references
    let test_cases = vec![
        (BibleBook::Genesis, 1, 0),      // Verse 0 doesn't exist
        (BibleBook::Genesis, 1, 32),     // Genesis 1 only has 31 verses
        (BibleBook::Psalm, 119, 177),    // Psalm 119 only has 176 verses
        (BibleBook::John, 3, 37),        // John 3 doesn't have 37 verses
        (BibleBook::Revelation, 22, 22), // Revelation 22 only has 21 verses
    ];

    for (book, chapter, verse) in test_cases {
        let verse_ref = BibleVerseReference::new(book, chapter, verse);
        assert!(
            verse_ref.is_err(),
            "Expected error for invalid verse reference {:?} {}:{}, but got Ok",
            book,
            chapter,
            verse
        );
    }
}

#[test]
fn test_invalid_references_parsing() {
    // Test parsing invalid references
    let invalid_references = vec![
        // Invalid book
        "GenesisX", "Exoduss", "Mattheww", "Revelations",
        // Invalid chapter
        "Genesis 51", "Psalm 151", "John 22", "Revelation 23",
        // Invalid verse
        "Genesis 1:32", "Psalm 23:7", "John 3:37", "Revelation 22:22",
        // Invalid format
        "Genesis:1", "Psalm:23", "John:3", "Revelation:22",
        // Empty reference
        "",
        // Nonsense
        "xyz", "123", "abc123",
    ];

    for reference in invalid_references {
        let result = parse(reference);
        assert!(
            result.is_err(),
            "Expected error for invalid reference '{}', but got Ok",
            reference
        );
    }
}

#[test]
fn test_invalid_range_references() {
    // Test invalid range references
    let invalid_ranges = vec![
        // End chapter after start chapter
        "Genesis 2-1",
        // End verse after start verse
        "Genesis 1:2-1",
        // Invalid start chapter
        "Genesis 51-52",
        // Invalid end chapter
        "Genesis 1-51",
        // Invalid start verse
        "Genesis 1:32-33",
        // Invalid end verse
        "Genesis 1:1-32",
        // Invalid format
        "Genesis 1:-2", "Genesis 1:1-", "-Genesis 2",
    ];

    for range in invalid_ranges {
        let result = parse(range);
        assert!(
            result.is_err(),
            "Expected error for invalid range '{}', but got Ok",
            range
        );
    }
}

#[test]
fn test_boundary_invalid_references() {
    // Test references just outside the boundaries of validity
    let books = vec![
        BibleBook::Genesis,    // First book
        BibleBook::Revelation, // Last book
        BibleBook::Jude,       // Book with only one chapter
        BibleBook::Psalm,      // Book with many chapters
    ];

    for book in books {
        // Test chapter 0 (invalid)
        let chapter_zero = BibleChapterReference::new(book, 0);
        assert!(
            chapter_zero.is_err(),
            "Expected error for chapter 0 of {:?}, but got Ok",
            book
        );

        // Test chapter beyond last chapter (invalid)
        let beyond_last_chapter = BibleChapterReference::new(book, get_number_of_chapters(&book) + 1);
        assert!(
            beyond_last_chapter.is_err(),
            "Expected error for chapter beyond last chapter of {:?}, but got Ok",
            book
        );

        // Test verse 0 (invalid)
        let verse_zero = BibleVerseReference::new(book, 1, 0);
        assert!(
            verse_zero.is_err(),
            "Expected error for verse 0 of {:?} 1, but got Ok",
            book
        );

        // Test verse beyond last verse (invalid)
        let last_chapter_num = get_number_of_chapters(&book);
        let last_verse_num = get_number_of_verses(&book, &last_chapter_num).unwrap();
        let beyond_last_verse = BibleVerseReference::new(book, last_chapter_num, last_verse_num + 1);
        assert!(
            beyond_last_verse.is_err(),
            "Expected error for verse beyond last verse of {:?} {}, but got Ok",
            book,
            last_chapter_num
        );
    }
}

#[test]
fn test_malformed_references() {
    // Test malformed references
    let malformed_references = vec![
        // Missing parts
        "Genesis 1:", "Genesis :1", ": Genesis 1",
        // Extra delimiters
        "Genesis 1:1:1", "Genesis 1,1,1", "Genesis 1-1-1",
        // Mixed delimiters
        "Genesis 1:1,2", "Genesis 1,1:2", "Genesis 1:1-2-3",
        // Invalid characters
        "Genesis 1:a", "Genesis a:1", "Genesis @#$",
        // Incomplete ranges
        "Genesis 1-", "-Genesis 1", "Genesis 1:-",
    ];

    for reference in malformed_references {
        let result = parse(reference);
        assert!(
            result.is_err(),
            "Expected error for malformed reference '{}', but got Ok",
            reference
        );
    }
}