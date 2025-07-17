use bibleref::{
    bible::{
        BibleBook, BibleBookReference, BibleRange, BibleReference,
        BibleReferenceRepresentation,
        validate::{get_number_of_chapters, get_number_of_verses},
    },
    parse,
    referencing::language::{BookReferenceType, get_reference_representation_in_language},
    translate,
};

// ==========================================
// 6.1 Language-Specific Edge Cases
// ==========================================

#[test]
fn test_special_characters_in_languages() {
    // Test handling of special characters in different languages
    let test_cases = vec![
        // German umlauts
        ("Matthäus 5,3", "de", "Matthew 5:3", "en"),
        ("Römer 8,28", "de", "Romans 8:28", "en"),
        // French accents
        ("Genèse 1:1", "fr", "Genesis 1:1", "en"),
        ("Éphésiens 2:8", "fr", "Ephesians 2:8", "en"),
        // Spanish accents and ñ
        ("Génesis 1:1", "es", "Genesis 1:1", "en"),
        ("Mateo 5:3", "es", "Matthew 5:3", "en"),
        // Russian Cyrillic
        ("Бытие 1:1", "ru", "Genesis 1:1", "en"),
        ("От Иоанна 3:16", "ru", "John 3:16", "en"),
        // Chinese characters
        ("创世记1：1", "zh_sim", "Genesis 1:1", "en"),
        ("约翰福音3：16", "zh_sim", "John 3:16", "en"),
    ];

    for (source_ref, source_lang, expected_ref, target_lang) in test_cases {
        let result = translate(source_ref, target_lang);
        assert!(
            result.is_ok(),
            "Failed to translate '{}' from {} to {}: {:?}",
            source_ref,
            source_lang,
            target_lang,
            result.err()
        );

        let translated = result.unwrap();
        assert_eq!(
            translated, expected_ref,
            "Expected '{}' to translate to '{}' in {}, but got '{}'",
            source_ref, expected_ref, target_lang, translated
        );
    }
}

#[test]
fn test_language_specific_formatting() {
    // Test handling of language-specific formatting rules
    let test_cases = vec![
        // English uses colon for chapter-verse delimiter
        ("John 3:16", "en", "John 3:16"),
        // German uses comma for chapter-verse delimiter
        ("Johannes 3,16", "de", "Johannes 3,16"),
        // Chinese uses full-width colon for chapter-verse delimiter
        ("约翰福音3：16", "zh_sim", "约翰福音3：16"),
        // French uses colon for chapter-verse delimiter
        ("Jean 3:16", "fr", "Jean 3:16"),
        // Russian uses colon for chapter-verse delimiter
        ("От Иоанна 3:16", "ru", "От Иоанна 3:16"),
    ];

    for (reference, lang, expected) in test_cases {
        let parsed = parse(reference);
        assert!(
            parsed.is_ok(),
            "Failed to parse '{}' in {}: {:?}",
            reference,
            lang,
            parsed.err()
        );

        let bible_ref = parsed.unwrap();
        let result = get_reference_representation_in_language(
            &bible_ref,
            lang,
            BookReferenceType::Long,
            true,
        );

        assert!(
            result.is_ok(),
            "Failed to format '{}' in {}: {:?}",
            reference,
            lang,
            result.err()
        );

        let formatted = result.unwrap();
        assert_eq!(
            formatted, expected,
            "Expected '{}' to format as '{}' in {}, but got '{}'",
            reference, expected, lang, formatted
        );
    }
}

#[test]
fn test_book_name_variations() {
    // Test handling of book name variations in different languages
    let test_cases = vec![
        // English variations
        ("Gen 1:1", "Genesis 1:1"),
        ("Gn 1:1", "Genesis 1:1"),
        ("Mt 5:3", "Matthew 5:3"),
        ("Matt 5:3", "Matthew 5:3"),
        ("Ps 23:1", "Psalms 23:1"),
        ("Psa 23:1", "Psalms 23:1"),
        ("Psalm 23:1", "Psalms 23:1"),
        ("Rev 1:1", "Revelation 1:1"),
        ("Rv 1:1", "Revelation 1:1"),
        // German variations
        ("1 Mo 1,1", "1. Mose 1,1"),
        ("Mt 5,3", "Matthäus 5,3"),
        ("Ps 23,1", "Psalmen 23,1"),
        ("Offb 1,1", "Offenbarung 1,1"),
    ];

    for (short_ref, expected_ref) in test_cases {
        let parsed = parse(short_ref);
        assert!(
            parsed.is_ok(),
            "Failed to parse short reference '{}': {:?}",
            short_ref,
            parsed.err()
        );

        let bible_ref = parsed.unwrap();
        let lang = if short_ref.contains(",") { "de" } else { "en" };

        let result = get_reference_representation_in_language(
            &bible_ref,
            lang,
            BookReferenceType::Long,
            true,
        );

        assert!(
            result.is_ok(),
            "Failed to format '{}' in {}: {:?}",
            short_ref,
            lang,
            result.err()
        );

        let formatted = result.unwrap();
        assert_eq!(
            formatted, expected_ref,
            "Expected '{}' to format as '{}' in {}, but got '{}'",
            short_ref, expected_ref, lang, formatted
        );
    }
}

#[test]
fn test_whitespace_handling() {
    // Test handling of whitespace in references
    let test_cases = vec![
        // Extra spaces
        ("Genesis   1:1", "Genesis 1:1"),
        ("Genesis 1  :  1", "Genesis 1:1"),
        ("  Genesis 1:1  ", "Genesis 1:1"),
        // No spaces
        ("Genesis1:1", "Genesis 1:1"),
        // Tabs and newlines
        ("Genesis\t1:1", "Genesis 1:1"),
        ("Genesis\n1:1", "Genesis 1:1"),
    ];

    for (input_ref, expected_ref) in test_cases {
        let parsed = parse(input_ref);
        assert!(
            parsed.is_ok(),
            "Failed to parse reference with whitespace '{}': {:?}",
            input_ref,
            parsed.err()
        );

        let bible_ref = parsed.unwrap();
        let result = get_reference_representation_in_language(
            &bible_ref,
            "en",
            BookReferenceType::Long,
            true,
        );

        assert!(
            result.is_ok(),
            "Failed to format '{}': {:?}",
            input_ref,
            result.err()
        );

        let formatted = result.unwrap();
        assert_eq!(
            formatted, expected_ref,
            "Expected '{}' to format as '{}', but got '{}'",
            input_ref, expected_ref, formatted
        );
    }
}

#[test]
fn test_case_insensitivity() {
    // Test case insensitivity in book names
    let test_cases = vec![
        // All lowercase
        ("genesis 1:1", "Genesis 1:1"),
        ("matthew 5:3", "Matthew 5:3"),
        ("revelation 1:1", "Revelation 1:1"),
        // All uppercase
        ("GENESIS 1:1", "Genesis 1:1"),
        ("MATTHEW 5:3", "Matthew 5:3"),
        ("REVELATION 1:1", "Revelation 1:1"),
        // Mixed case
        ("GeNeSiS 1:1", "Genesis 1:1"),
        ("MaTtHeW 5:3", "Matthew 5:3"),
        ("ReVeLaTiOn 1:1", "Revelation 1:1"),
    ];

    for (input_ref, expected_ref) in test_cases {
        let parsed = parse(input_ref);
        assert!(
            parsed.is_ok(),
            "Failed to parse reference with case variations '{}': {:?}",
            input_ref,
            parsed.err()
        );

        let bible_ref = parsed.unwrap();
        let result = get_reference_representation_in_language(
            &bible_ref,
            "en",
            BookReferenceType::Long,
            true,
        );

        assert!(
            result.is_ok(),
            "Failed to format '{}': {:?}",
            input_ref,
            result.err()
        );

        let formatted = result.unwrap();
        assert_eq!(
            formatted, expected_ref,
            "Expected '{}' to format as '{}', but got '{}'",
            input_ref, expected_ref, formatted
        );
    }
}

// ==========================================
// 6.2 Reference Edge Cases
// ==========================================

#[test]
fn test_books_with_one_chapter() {
    // Test handling of books with only one chapter
    let one_chapter_books = vec![
        BibleBook::Obadiah,
        BibleBook::Philemon,
        BibleBook::IIJohn,
        BibleBook::IIIJohn,
        BibleBook::Jude,
    ];

    for book in one_chapter_books {
        // Verify that the book has only one chapter
        assert_eq!(
            get_number_of_chapters(&book),
            1,
            "{:?} should have exactly 1 chapter",
            book
        );

        // Test parsing with and without chapter number
        let book_name = match book {
            BibleBook::Obadiah => "Obadiah",
            BibleBook::Philemon => "Philemon",
            BibleBook::IIJohn => "2 John",
            BibleBook::IIIJohn => "3 John",
            BibleBook::Jude => "Jude",
            _ => unreachable!(),
        };

        // Test with chapter number
        let with_chapter = format!("{} 1:1", book_name);
        let parsed_with_chapter = parse(&with_chapter);
        assert!(
            parsed_with_chapter.is_ok(),
            "Failed to parse '{}': {:?}",
            with_chapter,
            parsed_with_chapter.err()
        );

        // Test without chapter number (e.g., "Jude 1" instead of "Jude 1:1")
        let without_chapter = format!("{} 1", book_name);
        let parsed_without_chapter = parse(&without_chapter);
        assert!(
            parsed_without_chapter.is_ok(),
            "Failed to parse '{}': {:?}",
            without_chapter,
            parsed_without_chapter.err()
        );

        // Test verse reference without chapter (e.g., "Jude 5" meaning "Jude 1:5")
        let verse_only = format!("{} 5", book_name);
        let parsed_verse_only = parse(&verse_only);

        // This should be interpreted as chapter 1, verse 5
        assert!(
            parsed_verse_only.is_ok(),
            "Failed to parse '{}': {:?}",
            verse_only,
            parsed_verse_only.err()
        );

        if let Ok(BibleReferenceRepresentation::Single(BibleReference::BibleVerse(verse_ref))) =
            parsed_verse_only
        {
            assert_eq!(
                verse_ref.chapter(),
                1,
                "Expected chapter 1 for '{}', got {}",
                verse_only,
                verse_ref.chapter()
            );
            assert_eq!(
                verse_ref.verse(),
                5,
                "Expected verse 5 for '{}', got {}",
                verse_only,
                verse_ref.verse()
            );
        } else {
            panic!("Expected BibleVerse reference for '{}'", verse_only);
        }
    }
}

#[test]
fn test_chapters_with_varying_verse_counts() {
    // Test handling of chapters with varying verse counts
    let test_cases = vec![
        // Chapters with many verses
        (BibleBook::Psalm, 119, 176), // Psalm 119 has 176 verses
        (BibleBook::Numbers, 7, 89),  // Numbers 7 has 89 verses
        // Chapters with few verses
        (BibleBook::Genesis, 2, 25), // Genesis 2 has 25 verses
        (BibleBook::John, 11, 57),   // John 11 has 57 verses
        // Last verses of chapters
        (BibleBook::Genesis, 1, 31),     // Genesis 1 has 31 verses
        (BibleBook::Revelation, 22, 21), // Revelation 22 has 21 verses
    ];

    for (book, chapter, verse_count) in test_cases {
        // Verify that the chapter has the expected number of verses
        assert_eq!(
            get_number_of_verses(&book, &chapter).unwrap(),
            verse_count,
            "{:?} {} should have exactly {} verses",
            book,
            chapter,
            verse_count
        );

        // Test parsing the last verse of the chapter
        let last_verse_ref = format!("{:?} {}:{}", book, chapter, verse_count);
        let parsed_last_verse = parse(&last_verse_ref);
        assert!(
            parsed_last_verse.is_ok(),
            "Failed to parse last verse '{}': {:?}",
            last_verse_ref,
            parsed_last_verse.err()
        );

        // Test parsing a verse beyond the last verse (should fail)
        let beyond_last_verse_ref = format!("{:?} {}:{}", book, chapter, verse_count + 1);
        let parsed_beyond_last_verse = parse(&beyond_last_verse_ref);
        assert!(
            parsed_beyond_last_verse.is_err(),
            "Expected error for verse beyond last verse '{}', but got Ok",
            beyond_last_verse_ref
        );
    }
}

#[test]
fn test_references_at_boundaries() {
    // Test references at the boundaries of the Bible

    // First verse of the Bible
    let first_verse = "Genesis 1:1";
    let parsed_first_verse = parse(first_verse);
    assert!(
        parsed_first_verse.is_ok(),
        "Failed to parse first verse of the Bible '{}': {:?}",
        first_verse,
        parsed_first_verse.err()
    );

    // Last verse of the Bible
    let last_verse = "Revelation 22:21";
    let parsed_last_verse = parse(last_verse);
    assert!(
        parsed_last_verse.is_ok(),
        "Failed to parse last verse of the Bible '{}': {:?}",
        last_verse,
        parsed_last_verse.err()
    );

    // First verse of the New Testament
    let first_nt_verse = "Matthew 1:1";
    let parsed_first_nt_verse = parse(first_nt_verse);
    assert!(
        parsed_first_nt_verse.is_ok(),
        "Failed to parse first verse of the New Testament '{}': {:?}",
        first_nt_verse,
        parsed_first_nt_verse.err()
    );

    // Last verse of the Old Testament
    let last_ot_verse = "Malachi 4:6";
    let parsed_last_ot_verse = parse(last_ot_verse);
    assert!(
        parsed_last_ot_verse.is_ok(),
        "Failed to parse last verse of the Old Testament '{}': {:?}",
        last_ot_verse,
        parsed_last_ot_verse.err()
    );

    // Test that there is no book before Genesis
    let genesis_book_ref = BibleReference::BibleBook(BibleBookReference::new(BibleBook::Genesis));
    assert_eq!(
        genesis_book_ref.previous(),
        None,
        "There should be no book before Genesis"
    );

    // Test that there is no book after Revelation
    let revelation_book_ref =
        BibleReference::BibleBook(BibleBookReference::new(BibleBook::Revelation));
    assert_eq!(
        revelation_book_ref.next(),
        None,
        "There should be no book after Revelation"
    );
}

#[test]
#[ignore]
fn test_malformed_references() {
    // Test handling of malformed references
    let malformed_references = vec![
        // Missing parts
        "Genesis :1",
        ": Genesis 1",
        // Extra delimiters
        "Genesis 1:1:1",
        "Genesis 1,1,1",
        "Genesis 1-1-1",
        // Mixed delimiters
        "Genesis 1:1,2",
        "Genesis 1,1:2",
        "Genesis 1:1-2-3",
        // Invalid characters
        "Genesis 1:a",
        "Genesis a:1",
        "Genesis @#$",
        // Incomplete ranges
        "Genesis 1-",
        "-Genesis 1",
        "Genesis 1:-",
        // Non-existent books
        "Hezekiah 1:1",
        "3 Corinthians 1:1",
        "Book of Mormon 1:1",
        // Non-existent chapters
        "Genesis 51:1",
        "Revelation 23:1",
        "Jude 2:1",
        // Non-existent verses
        "Genesis 1:32",
        "Psalm 23:7",
        "John 3:37",
        "Revelation 22:22",
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

#[test]
fn test_cross_chapter_verse_ranges() {
    // Test handling of verse ranges that cross chapter boundaries
    let test_cases = vec![
        // These should be parsed as separate chapters, not as verse ranges
        "Matthew 5-6",      // Matthew chapters 5-6
        "John 3-4",         // John chapters 3-4
        "Revelation 21-22", // Revelation chapters 21-22
    ];

    for reference in test_cases {
        let result = parse(reference);
        assert!(
            result.is_ok(),
            "Failed to parse cross-chapter reference '{}': {:?}",
            reference,
            result.err()
        );

        let parsed = result.unwrap();
        assert!(
            parsed.is_range(),
            "Expected range reference for '{}', got single",
            reference
        );

        if let BibleReferenceRepresentation::Range(BibleRange::ChapterRange(_)) = parsed {
            // This is the expected case
        } else {
            panic!(
                "Expected ChapterRange reference for '{}', got {:?}",
                reference, parsed
            );
        }
    }
}

#[test]
fn test_cross_book_ranges() {
    // Test handling of ranges that cross book boundaries
    let test_cases = vec![
        // These should be parsed as book ranges
        "Genesis-Exodus",
        "Matthew-Mark",
        "1 Corinthians-2 Corinthians",
    ];

    for reference in test_cases {
        let result = parse(reference);
        assert!(
            result.is_ok(),
            "Failed to parse cross-book reference '{}': {:?}",
            reference,
            result.err()
        );

        let parsed = result.unwrap();
        assert!(
            parsed.is_range(),
            "Expected range reference for '{}', got single",
            reference
        );

        if let BibleReferenceRepresentation::Range(BibleRange::BookRange(_)) = parsed {
            // This is the expected case
        } else {
            panic!(
                "Expected BookRange reference for '{}', got {:?}",
                reference, parsed
            );
        }
    }
}
