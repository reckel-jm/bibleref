use bibleref::{
    bible::{
        BibleBook, BibleBookRange, BibleBookReference, BibleChapterRange, BibleChapterReference,
        BibleRange, BibleReference, BibleReferenceRepresentation, BibleVerseRange,
        BibleVerseReference, aggregate_bible_representations,
        validate::{get_number_of_chapters, get_number_of_verses},
    },
    parse,
};

// ==========================================
// 4.1 Upcasting Tests
// ==========================================

#[test]
fn test_upcast_verse_range_to_chapter() {
    // Test upcasting verse ranges that span an entire chapter to chapter references
    let books = vec![
        BibleBook::Genesis,
        BibleBook::Exodus,
        BibleBook::Psalm,
        BibleBook::Matthew,
        BibleBook::John,
        BibleBook::Revelation,
    ];

    for book in books {
        // Test for first chapter
        let chapter_num = 1;
        let verse_count = get_number_of_verses(&book, &chapter_num).unwrap();

        let start = BibleVerseReference::new(book, chapter_num, 1).unwrap();
        let end = BibleVerseReference::new(book, chapter_num, verse_count).unwrap();
        let verse_range = BibleVerseRange::new(start, end).unwrap();

        let verse_range_representation =
            BibleReferenceRepresentation::Range(BibleRange::VerseRange(verse_range));
        let chapter_reference = BibleChapterReference::new(book, chapter_num).unwrap();
        let chapter_representation =
            BibleReferenceRepresentation::Single(BibleReference::BibleChapter(chapter_reference));

        let upcasted = verse_range_representation.try_upcast();
        assert_eq!(
            upcasted, chapter_representation,
            "Failed to upcast verse range for {:?} {} from verses 1-{} to chapter",
            book, chapter_num, verse_count
        );

        // Test for a middle chapter if the book has more than 2 chapters
        if get_number_of_chapters(&book) > 2 {
            let chapter_num = get_number_of_chapters(&book) / 2;
            let verse_count = get_number_of_verses(&book, &chapter_num).unwrap();

            let start = BibleVerseReference::new(book, chapter_num, 1).unwrap();
            let end = BibleVerseReference::new(book, chapter_num, verse_count).unwrap();
            let verse_range = BibleVerseRange::new(start, end).unwrap();

            let verse_range_representation =
                BibleReferenceRepresentation::Range(BibleRange::VerseRange(verse_range));
            let chapter_reference = BibleChapterReference::new(book, chapter_num).unwrap();
            let chapter_representation = BibleReferenceRepresentation::Single(
                BibleReference::BibleChapter(chapter_reference),
            );

            let upcasted = verse_range_representation.try_upcast();
            assert_eq!(
                upcasted, chapter_representation,
                "Failed to upcast verse range for {:?} {} from verses 1-{} to chapter",
                book, chapter_num, verse_count
            );
        }
    }
}

#[test]
fn test_upcast_chapter_range_to_book() {
    // Test upcasting chapter ranges that span an entire book to book references
    let books = vec![
        BibleBook::Genesis,
        BibleBook::Exodus,
        BibleBook::Ruth,
        BibleBook::Psalm,
        BibleBook::Matthew,
        BibleBook::John,
        BibleBook::Jude,
        BibleBook::Revelation,
    ];

    for book in books {
        let chapter_count = get_number_of_chapters(&book);

        let start = BibleChapterReference::new(book, 1).unwrap();
        let end = BibleChapterReference::new(book, chapter_count).unwrap();
        let chapter_range = BibleChapterRange::new(start, end).unwrap();

        let chapter_range_representation =
            BibleReferenceRepresentation::Range(BibleRange::ChapterRange(chapter_range));
        let book_reference = BibleBookReference::new(book);
        let book_representation =
            BibleReferenceRepresentation::Single(BibleReference::BibleBook(book_reference));

        let upcasted = chapter_range_representation.try_upcast();
        assert_eq!(
            upcasted, book_representation,
            "Failed to upcast chapter range for {:?} from chapters 1-{} to book",
            book, chapter_count
        );
    }
}

#[test]
fn test_upcast_partial_verse_range() {
    // Test that partial verse ranges (not spanning an entire chapter) are not upcasted
    let test_cases = vec![
        (BibleBook::Genesis, 1, 1, 10), // Genesis 1:1-10 (not the entire chapter)
        (BibleBook::Psalm, 23, 2, 6),   // Psalm 23:2-6 (not from verse 1)
        (BibleBook::John, 3, 16, 18),   // John 3:16-18 (not the entire chapter)
    ];

    for (book, chapter, start_verse, end_verse) in test_cases {
        let start = BibleVerseReference::new(book, chapter, start_verse).unwrap();
        let end = BibleVerseReference::new(book, chapter, end_verse).unwrap();
        let verse_range = BibleVerseRange::new(start, end).unwrap();

        let verse_range_representation =
            BibleReferenceRepresentation::Range(BibleRange::VerseRange(verse_range.clone()));
        let expected_representation =
            BibleReferenceRepresentation::Range(BibleRange::VerseRange(verse_range));

        let upcasted = verse_range_representation.try_upcast();
        assert_eq!(
            upcasted, expected_representation,
            "Incorrectly upcasted partial verse range for {:?} {}:{}-{}",
            book, chapter, start_verse, end_verse
        );
    }
}

#[test]
fn test_upcast_partial_chapter_range() {
    // Test that partial chapter ranges (not spanning an entire book) are not upcasted
    let test_cases = vec![
        (BibleBook::Genesis, 1, 10), // Genesis 1-10 (not the entire book)
        (BibleBook::Psalm, 23, 30),  // Psalm 23-30 (not from chapter 1)
        (BibleBook::John, 3, 5),     // John 3-5 (not the entire book)
    ];

    for (book, start_chapter, end_chapter) in test_cases {
        let start = BibleChapterReference::new(book, start_chapter).unwrap();
        let end = BibleChapterReference::new(book, end_chapter).unwrap();
        let chapter_range = BibleChapterRange::new(start, end).unwrap();

        let chapter_range_representation =
            BibleReferenceRepresentation::Range(BibleRange::ChapterRange(chapter_range.clone()));
        let expected_representation =
            BibleReferenceRepresentation::Range(BibleRange::ChapterRange(chapter_range));

        let upcasted = chapter_range_representation.try_upcast();
        assert_eq!(
            upcasted, expected_representation,
            "Incorrectly upcasted partial chapter range for {:?} {}-{}",
            book, start_chapter, end_chapter
        );
    }
}

#[test]
fn test_upcast_single_verse_to_verse() {
    // Test upcasting a verse range with the same start and end verse to a single verse reference
    let test_cases = vec![
        (BibleBook::Genesis, 1, 1),
        (BibleBook::Psalm, 23, 1),
        (BibleBook::John, 3, 16),
        (BibleBook::Revelation, 22, 21),
    ];

    for (book, chapter, verse) in test_cases {
        let verse_ref = BibleVerseReference::new(book, chapter, verse).unwrap();
        let verse_range = BibleVerseRange::new(verse_ref.clone(), verse_ref.clone()).unwrap();

        let verse_range_representation =
            BibleReferenceRepresentation::Range(BibleRange::VerseRange(verse_range));
        let verse_representation =
            BibleReferenceRepresentation::Single(BibleReference::BibleVerse(verse_ref));

        let upcasted = verse_range_representation.try_upcast();
        assert_eq!(
            upcasted, verse_representation,
            "Failed to upcast single-verse range for {:?} {}:{}",
            book, chapter, verse
        );
    }
}

#[test]
fn test_upcast_single_chapter_to_chapter() {
    // Test upcasting a chapter range with the same start and end chapter to a single chapter reference
    let test_cases = vec![
        (BibleBook::Genesis, 1),
        (BibleBook::Psalm, 23),
        (BibleBook::John, 3),
        (BibleBook::Revelation, 22),
    ];

    for (book, chapter) in test_cases {
        let chapter_ref = BibleChapterReference::new(book, chapter).unwrap();
        let chapter_range =
            BibleChapterRange::new(chapter_ref.clone(), chapter_ref.clone()).unwrap();

        let chapter_range_representation =
            BibleReferenceRepresentation::Range(BibleRange::ChapterRange(chapter_range));
        let chapter_representation =
            BibleReferenceRepresentation::Single(BibleReference::BibleChapter(chapter_ref));

        let upcasted = chapter_range_representation.try_upcast();
        assert_eq!(
            upcasted, chapter_representation,
            "Failed to upcast single-chapter range for {:?} {}",
            book, chapter
        );
    }
}

// ==========================================
// 4.2 Downcasting Tests
// ==========================================

#[test]
fn test_downcast_book_to_chapter_range() {
    // Test downcasting book references to chapter ranges
    let books = vec![
        BibleBook::Genesis,
        BibleBook::Exodus,
        BibleBook::Ruth,
        BibleBook::Psalm,
        BibleBook::Matthew,
        BibleBook::John,
        BibleBook::Jude,
        BibleBook::Revelation,
    ];

    for book in books {
        let book_ref = BibleBookReference::new(book);
        let chapter_count = get_number_of_chapters(&book);

        // Manually create the expected chapter range
        let start = BibleChapterReference::new(book, 1).unwrap();
        let end = BibleChapterReference::new(book, chapter_count).unwrap();
        let expected_chapter_range = BibleChapterRange::new(start, end).unwrap();

        // Create a book range that spans just this book, then get the chapter range
        let book_range = BibleBookRange::new(book_ref.clone(), book_ref.clone()).unwrap();
        let actual_chapter_range = book_range.as_chapter_range();

        assert_eq!(
            actual_chapter_range, expected_chapter_range,
            "Failed to downcast book reference for {:?} to chapter range 1-{}",
            book, chapter_count
        );
    }
}

#[test]
fn test_downcast_chapter_to_verse_range() {
    // Test downcasting chapter references to verse ranges
    let test_cases = vec![
        (BibleBook::Genesis, 1),
        (BibleBook::Psalm, 23),
        (BibleBook::John, 3),
        (BibleBook::Revelation, 22),
    ];

    for (book, chapter) in test_cases {
        let chapter_ref = BibleChapterReference::new(book, chapter).unwrap();
        let verse_count = get_number_of_verses(&book, &chapter).unwrap();

        // Manually create the expected verse range
        let start = BibleVerseReference::new(book, chapter, 1).unwrap();
        let end = BibleVerseReference::new(book, chapter, verse_count).unwrap();
        let expected_verse_range = BibleVerseRange::new(start, end).unwrap();

        // Create a chapter range that spans just this chapter, then get the verse range
        let chapter_range =
            BibleChapterRange::new(chapter_ref.clone(), chapter_ref.clone()).unwrap();
        let actual_verse_range = chapter_range.as_verse_range();

        assert_eq!(
            actual_verse_range, expected_verse_range,
            "Failed to downcast chapter reference for {:?} {} to verse range 1-{}",
            book, chapter, verse_count
        );
    }
}

#[test]
fn test_downcast_book_range_to_chapter_range() {
    // Test downcasting book ranges to chapter ranges
    let test_cases = vec![
        (BibleBook::Genesis, BibleBook::Exodus),
        (BibleBook::Matthew, BibleBook::Mark),
        (BibleBook::Romans, BibleBook::Galatians),
    ];

    for (start_book, end_book) in test_cases {
        let start_book_ref = BibleBookReference::new(start_book);
        let end_book_ref = BibleBookReference::new(end_book);
        let book_range = BibleBookRange::new(start_book_ref, end_book_ref).unwrap();

        // Manually create the expected chapter range
        let start_chapter = BibleChapterReference::new(start_book, 1).unwrap();
        let end_chapter =
            BibleChapterReference::new(end_book, get_number_of_chapters(&end_book)).unwrap();
        let expected_chapter_range = BibleChapterRange::new(start_chapter, end_chapter).unwrap();

        // Get the actual chapter range from the book range
        let actual_chapter_range = book_range.as_chapter_range();

        assert_eq!(
            actual_chapter_range, expected_chapter_range,
            "Failed to downcast book range from {:?} to {:?} to chapter range",
            start_book, end_book
        );
    }
}

#[test]
fn test_downcast_chapter_range_to_verse_range() {
    // Test downcasting chapter ranges to verse ranges
    let test_cases = vec![
        (BibleBook::Genesis, 1, 2),
        (BibleBook::Psalm, 23, 24),
        (BibleBook::John, 3, 4),
    ];

    for (book, start_chapter, end_chapter) in test_cases {
        let start_chapter_ref = BibleChapterReference::new(book, start_chapter).unwrap();
        let end_chapter_ref = BibleChapterReference::new(book, end_chapter).unwrap();
        let chapter_range = BibleChapterRange::new(start_chapter_ref, end_chapter_ref).unwrap();

        // Manually create the expected verse range
        let start_verse = BibleVerseReference::new(book, start_chapter, 1).unwrap();
        let end_verse = BibleVerseReference::new(
            book,
            end_chapter,
            get_number_of_verses(&book, &end_chapter).unwrap(),
        )
        .unwrap();
        let expected_verse_range = BibleVerseRange::new(start_verse, end_verse).unwrap();

        // Get the actual verse range from the chapter range
        let actual_verse_range = chapter_range.as_verse_range();

        assert_eq!(
            actual_verse_range, expected_verse_range,
            "Failed to downcast chapter range from {:?} {}-{} to verse range",
            book, start_chapter, end_chapter
        );
    }
}

#[test]
fn test_as_list_for_book_range() {
    // Test converting book ranges to lists
    let test_cases = vec![
        (BibleBook::Genesis, BibleBook::Exodus),
        (BibleBook::Matthew, BibleBook::Mark),
        (BibleBook::Romans, BibleBook::Galatians),
    ];

    for (start_book, end_book) in test_cases {
        let start_book_ref = BibleBookReference::new(start_book);
        let end_book_ref = BibleBookReference::new(end_book);
        let book_range = BibleBookRange::new(start_book_ref.clone(), end_book_ref.clone()).unwrap();

        // Get the list from the book range
        let book_list = book_range.as_list();

        // Check that the list starts with the start book and ends with the end book
        assert_eq!(
            book_list.first().unwrap(),
            &start_book_ref,
            "Book list doesn't start with the correct book"
        );
        assert_eq!(
            book_list.last().unwrap(),
            &end_book_ref,
            "Book list doesn't end with the correct book"
        );

        // Check that the list has the correct length
        let expected_length = (end_book.number() - start_book.number() + 1) as usize;
        assert_eq!(
            book_list.len(),
            expected_length,
            "Book list has incorrect length"
        );
    }
}

#[test]
fn test_as_list_for_chapter_range() {
    // Test converting chapter ranges to lists
    let test_cases = vec![
        (BibleBook::Genesis, 1, 5),
        (BibleBook::Psalm, 23, 25),
        (BibleBook::John, 3, 6),
    ];

    for (book, start_chapter, end_chapter) in test_cases {
        let start_chapter_ref = BibleChapterReference::new(book, start_chapter).unwrap();
        let end_chapter_ref = BibleChapterReference::new(book, end_chapter).unwrap();
        let chapter_range =
            BibleChapterRange::new(start_chapter_ref.clone(), end_chapter_ref.clone()).unwrap();

        // Get the list from the chapter range
        let chapter_list = chapter_range.as_list();

        // Check that the list starts with the start chapter and ends with the end chapter
        assert_eq!(
            chapter_list.first().unwrap(),
            &start_chapter_ref,
            "Chapter list doesn't start with the correct chapter"
        );
        assert_eq!(
            chapter_list.last().unwrap(),
            &end_chapter_ref,
            "Chapter list doesn't end with the correct chapter"
        );

        // Check that the list has the correct length
        let expected_length = (end_chapter - start_chapter + 1) as usize;
        assert_eq!(
            chapter_list.len(),
            expected_length,
            "Chapter list has incorrect length"
        );
    }
}

#[test]
fn test_as_list_for_verse_range() {
    // Test converting verse ranges to lists
    let test_cases = vec![
        (BibleBook::Genesis, 1, 1, 5),
        (BibleBook::Psalm, 23, 1, 3),
        (BibleBook::John, 3, 16, 18),
    ];

    for (book, chapter, start_verse, end_verse) in test_cases {
        let start_verse_ref = BibleVerseReference::new(book, chapter, start_verse).unwrap();
        let end_verse_ref = BibleVerseReference::new(book, chapter, end_verse).unwrap();
        let verse_range =
            BibleVerseRange::new(start_verse_ref.clone(), end_verse_ref.clone()).unwrap();

        // Get the list from the verse range
        let verse_list = verse_range.as_list();

        // Check that the list starts with the start verse and ends with the end verse
        assert_eq!(
            verse_list.first().unwrap(),
            &start_verse_ref,
            "Verse list doesn't start with the correct verse"
        );
        assert_eq!(
            verse_list.last().unwrap(),
            &end_verse_ref,
            "Verse list doesn't end with the correct verse"
        );

        // Check that the list has the correct length
        let expected_length = (end_verse - start_verse + 1) as usize;
        assert_eq!(
            verse_list.len(),
            expected_length,
            "Verse list has incorrect length"
        );
    }
}

// ==========================================
// 4.3 Aggregation Tests
// ==========================================

#[test]
fn test_aggregate_adjacent_verses() {
    // Test aggregating adjacent verses
    let book = BibleBook::John;
    let chapter = 3;

    // Create individual verse references
    let verse1 = BibleVerseReference::new(book, chapter, 16).unwrap();
    let verse2 = BibleVerseReference::new(book, chapter, 17).unwrap();
    let verse3 = BibleVerseReference::new(book, chapter, 18).unwrap();

    // Create representations
    let rep1 = BibleReferenceRepresentation::Single(BibleReference::BibleVerse(verse1.clone()));
    let rep2 = BibleReferenceRepresentation::Single(BibleReference::BibleVerse(verse2.clone()));
    let rep3 = BibleReferenceRepresentation::Single(BibleReference::BibleVerse(verse3.clone()));

    // Aggregate them
    let aggregated = aggregate_bible_representations(vec![rep1, rep2, rep3]);

    // Expected result: a single range from verse 16 to 18
    let expected_range = BibleVerseRange::new(verse1, verse3).unwrap();
    let expected = vec![BibleReferenceRepresentation::Range(BibleRange::VerseRange(
        expected_range,
    ))];

    assert_eq!(
        aggregated, expected,
        "Failed to aggregate adjacent verses into a range"
    );
}

#[test]
fn test_aggregate_adjacent_chapters() {
    // Test aggregating adjacent chapters
    let book = BibleBook::Psalm;

    // Create individual chapter references
    let chapter1 = BibleChapterReference::new(book, 23).unwrap();
    let chapter2 = BibleChapterReference::new(book, 24).unwrap();
    let chapter3 = BibleChapterReference::new(book, 25).unwrap();

    // Create representations
    let rep1 = BibleReferenceRepresentation::Single(BibleReference::BibleChapter(chapter1.clone()));
    let rep2 = BibleReferenceRepresentation::Single(BibleReference::BibleChapter(chapter2.clone()));
    let rep3 = BibleReferenceRepresentation::Single(BibleReference::BibleChapter(chapter3.clone()));

    // Aggregate them
    let aggregated = aggregate_bible_representations(vec![rep1, rep2, rep3]);

    // Expected result: a single range from chapter 23 to 25
    let expected_range = BibleChapterRange::new(chapter1, chapter3).unwrap();
    let expected = vec![BibleReferenceRepresentation::Range(
        BibleRange::ChapterRange(expected_range),
    )];

    assert_eq!(
        aggregated, expected,
        "Failed to aggregate adjacent chapters into a range"
    );
}

#[test]
fn test_aggregate_adjacent_books() {
    // Test aggregating adjacent books

    // Create individual book references
    let book1 = BibleBookReference::new(BibleBook::Matthew);
    let book2 = BibleBookReference::new(BibleBook::Mark);
    let book3 = BibleBookReference::new(BibleBook::Luke);

    // Create representations
    let rep1 = BibleReferenceRepresentation::Single(BibleReference::BibleBook(book1.clone()));
    let rep2 = BibleReferenceRepresentation::Single(BibleReference::BibleBook(book2.clone()));
    let rep3 = BibleReferenceRepresentation::Single(BibleReference::BibleBook(book3.clone()));

    // Aggregate them
    let aggregated = aggregate_bible_representations(vec![rep1, rep2, rep3]);

    // Expected result: a single range from Matthew to Luke
    let expected_range = BibleBookRange::new(book1, book3).unwrap();
    let expected = vec![BibleReferenceRepresentation::Range(BibleRange::BookRange(
        expected_range,
    ))];

    assert_eq!(
        aggregated, expected,
        "Failed to aggregate adjacent books into a range"
    );
}

#[test]
fn test_aggregate_overlapping_ranges() {
    // Test aggregating overlapping ranges
    let book = BibleBook::John;
    let chapter = 3;

    // Create overlapping verse ranges
    let range1_start = BibleVerseReference::new(book, chapter, 16).unwrap();
    let range1_end = BibleVerseReference::new(book, chapter, 18).unwrap();
    let range1 = BibleVerseRange::new(range1_start, range1_end).unwrap();

    let range2_start = BibleVerseReference::new(book, chapter, 17).unwrap();
    let range2_end = BibleVerseReference::new(book, chapter, 19).unwrap();
    let range2 = BibleVerseRange::new(range2_start, range2_end).unwrap();

    // Create representations
    let rep1 = BibleReferenceRepresentation::Range(BibleRange::VerseRange(range1));
    let rep2 = BibleReferenceRepresentation::Range(BibleRange::VerseRange(range2));

    // Aggregate them
    let aggregated = aggregate_bible_representations(vec![rep1, rep2]);

    // Expected result: a single range from verse 16 to 19
    let expected_start = BibleVerseReference::new(book, chapter, 16).unwrap();
    let expected_end = BibleVerseReference::new(book, chapter, 19).unwrap();
    let expected_range = BibleVerseRange::new(expected_start, expected_end).unwrap();
    let expected = vec![BibleReferenceRepresentation::Range(BibleRange::VerseRange(
        expected_range,
    ))];

    assert_eq!(
        aggregated, expected,
        "Failed to aggregate overlapping verse ranges"
    );
}

#[test]
fn test_aggregate_mixed_references() {
    // Test aggregating a mix of single references and ranges
    let book = BibleBook::John;
    let chapter = 3;

    // Create a verse reference and a verse range
    let verse1 = BibleVerseReference::new(book, chapter, 16).unwrap();
    let range_start = BibleVerseReference::new(book, chapter, 17).unwrap();
    let range_end = BibleVerseReference::new(book, chapter, 18).unwrap();
    let verse_range = BibleVerseRange::new(range_start, range_end).unwrap();

    // Create representations
    let rep1 = BibleReferenceRepresentation::Single(BibleReference::BibleVerse(verse1.clone()));
    let rep2 = BibleReferenceRepresentation::Range(BibleRange::VerseRange(verse_range));

    // Aggregate them
    let aggregated = aggregate_bible_representations(vec![rep1, rep2]);

    // Expected result: a single range from verse 16 to 18
    let expected_start = BibleVerseReference::new(book, chapter, 16).unwrap();
    let expected_end = BibleVerseReference::new(book, chapter, 18).unwrap();
    let expected_range = BibleVerseRange::new(expected_start, expected_end).unwrap();
    let expected = vec![BibleReferenceRepresentation::Range(BibleRange::VerseRange(
        expected_range,
    ))];

    assert_eq!(
        aggregated, expected,
        "Failed to aggregate mixed verse references"
    );
}
