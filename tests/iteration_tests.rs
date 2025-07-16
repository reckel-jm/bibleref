use bibleref::bible::{
        BibleBook, BibleBookRange, BibleBookReference, BibleChapterRange, BibleChapterReference, BibleReference, BibleVerseRange,
        BibleVerseReference, get_bible_book_by_number,
        validate::{get_number_of_chapters, get_number_of_verses},
    };

// ==========================================
// 5.1 Book Iteration Tests
// ==========================================

#[test]
fn test_iterate_all_books() {
    // Test iterating through all books of the Bible
    let all_books = BibleBook::all();

    // Check that there are 66 books
    assert_eq!(all_books.len(), 66, "There should be 66 books in the Bible");

    // Check that the first book is Genesis
    assert_eq!(
        all_books[0],
        BibleBook::Genesis,
        "The first book should be Genesis"
    );

    // Check that the last book is Revelation
    assert_eq!(
        all_books[65],
        BibleBook::Revelation,
        "The last book should be Revelation"
    );

    // Check that the books are in the correct order
    for i in 1..all_books.len() {
        assert!(
            all_books[i - 1] < all_books[i],
            "Books should be in ascending order, but {:?} is not less than {:?}",
            all_books[i - 1],
            all_books[i]
        );
    }
}

#[test]
fn test_iterate_old_testament_books() {
    // Test iterating through Old Testament books
    let all_books = BibleBook::all();
    let old_testament_books: Vec<BibleBook> = all_books
        .into_iter()
        .filter(|book| book.is_old_testament())
        .collect();

    // Check that there are 39 books in the Old Testament
    assert_eq!(
        old_testament_books.len(),
        39,
        "There should be 39 books in the Old Testament"
    );

    // Check that the first book is Genesis
    assert_eq!(
        old_testament_books[0],
        BibleBook::Genesis,
        "The first book should be Genesis"
    );

    // Check that the last book is Malachi
    assert_eq!(
        old_testament_books[38],
        BibleBook::Malachi,
        "The last book should be Malachi"
    );

    // Check that all books are indeed Old Testament books
    for book in &old_testament_books {
        assert!(
            book.is_old_testament(),
            "{:?} should be an Old Testament book",
            book
        );
    }
}

#[test]
fn test_iterate_new_testament_books() {
    // Test iterating through New Testament books
    let all_books = BibleBook::all();
    let new_testament_books: Vec<BibleBook> = all_books
        .into_iter()
        .filter(|book| book.is_new_testament())
        .collect();

    // Check that there are 27 books in the New Testament
    assert_eq!(
        new_testament_books.len(),
        27,
        "There should be 27 books in the New Testament"
    );

    // Check that the first book is Matthew
    assert_eq!(
        new_testament_books[0],
        BibleBook::Matthew,
        "The first book should be Matthew"
    );

    // Check that the last book is Revelation
    assert_eq!(
        new_testament_books[26],
        BibleBook::Revelation,
        "The last book should be Revelation"
    );

    // Check that all books are indeed New Testament books
    for book in &new_testament_books {
        assert!(
            book.is_new_testament(),
            "{:?} should be a New Testament book",
            book
        );
    }
}

#[test]
fn test_iterate_book_range() {
    // Test iterating through a range of books
    let test_cases = vec![
        (BibleBook::Genesis, BibleBook::Deuteronomy, 5), // Pentateuch
        (BibleBook::Matthew, BibleBook::John, 4),        // Gospels
        (BibleBook::Romans, BibleBook::Philemon, 13),    // Pauline Epistles
        (BibleBook::Hebrews, BibleBook::Jude, 8),        // General Epistles
    ];

    for (start_book, end_book, expected_count) in test_cases {
        let start_ref = BibleBookReference::new(start_book);
        let end_ref = BibleBookReference::new(end_book);
        let book_range = BibleBookRange::new(start_ref, end_ref).unwrap();

        // Get the list of books in the range
        let book_list = book_range.as_list();

        // Check that the list has the correct length
        assert_eq!(
            book_list.len(),
            expected_count,
            "Book range from {:?} to {:?} should have {} books",
            start_book,
            end_book,
            expected_count
        );

        // Check that the books are in the correct order
        for i in 1..book_list.len() {
            assert!(
                book_list[i - 1].book() < book_list[i].book(),
                "Books should be in ascending order, but {:?} is not less than {:?}",
                book_list[i - 1].book(),
                book_list[i].book()
            );
        }
    }
}

#[test]
fn test_book_next_previous() {
    // Test the next() and previous() methods for books
    let books = BibleBook::all();

    for i in 0..books.len() {
        let book = books[i];
        let book_ref = BibleReference::BibleBook(BibleBookReference::new(book));

        // Test next()
        if i < books.len() - 1 {
            let next_book = books[i + 1];
            let expected_next = BibleReference::BibleBook(BibleBookReference::new(next_book));
            assert_eq!(
                book_ref.next(),
                Some(expected_next),
                "The next book after {:?} should be {:?}",
                book,
                next_book
            );
        } else {
            assert_eq!(
                book_ref.next(),
                None,
                "There should be no next book after {:?}",
                book
            );
        }

        // Test previous()
        if i > 0 {
            let prev_book = books[i - 1];
            let expected_prev = BibleReference::BibleBook(BibleBookReference::new(prev_book));
            assert_eq!(
                book_ref.previous(),
                Some(expected_prev),
                "The previous book before {:?} should be {:?}",
                book,
                prev_book
            );
        } else {
            assert_eq!(
                book_ref.previous(),
                None,
                "There should be no previous book before {:?}",
                book
            );
        }
    }
}

// ==========================================
// 5.2 Chapter Iteration Tests
// ==========================================

#[test]
fn test_iterate_chapters_of_book() {
    // Test iterating through all chapters of a book
    let test_cases = vec![
        (BibleBook::Genesis, 50),    // Genesis has 50 chapters
        (BibleBook::Psalm, 150),     // Psalms has 150 chapters
        (BibleBook::John, 21),       // John has 21 chapters
        (BibleBook::Jude, 1),        // Jude has 1 chapter
        (BibleBook::Revelation, 22), // Revelation has 22 chapters
    ];

    for (book, expected_chapters) in test_cases {
        // Check that the number of chapters is correct
        let chapter_count = get_number_of_chapters(&book);
        assert_eq!(
            chapter_count, expected_chapters,
            "{:?} should have {} chapters",
            book, expected_chapters
        );

        // Create a book reference and create a book range that spans just this book
        let book_ref = BibleBookReference::new(book);
        let book_range = BibleBookRange::new(book_ref.clone(), book_ref.clone()).unwrap();
        let chapter_range = book_range.as_chapter_range();

        // Get the list of chapters
        let chapter_list = chapter_range.as_list();

        // Check that the list has the correct length
        assert_eq!(
            chapter_list.len(),
            expected_chapters as usize,
            "{:?} should have {} chapters",
            book,
            expected_chapters
        );

        // Check that the chapters are in the correct order
        for i in 1..chapter_list.len() {
            assert!(
                chapter_list[i - 1].chapter() < chapter_list[i].chapter(),
                "Chapters should be in ascending order, but {} is not less than {}",
                chapter_list[i - 1].chapter(),
                chapter_list[i].chapter()
            );
        }
    }
}

#[test]
fn test_iterate_chapter_range() {
    // Test iterating through a range of chapters
    let test_cases = vec![
        (BibleBook::Genesis, 1, 5, 5),      // Genesis 1-5 (5 chapters)
        (BibleBook::Psalm, 23, 25, 3),      // Psalm 23-25 (3 chapters)
        (BibleBook::Matthew, 5, 7, 3),      // Matthew 5-7 (Sermon on the Mount, 3 chapters)
        (BibleBook::Revelation, 20, 22, 3), // Revelation 20-22 (3 chapters)
    ];

    for (book, start_chapter, end_chapter, expected_count) in test_cases {
        let start_ref = BibleChapterReference::new(book, start_chapter).unwrap();
        let end_ref = BibleChapterReference::new(book, end_chapter).unwrap();
        let chapter_range = BibleChapterRange::new(start_ref, end_ref).unwrap();

        // Get the list of chapters in the range
        let chapter_list = chapter_range.as_list();

        // Check that the list has the correct length
        assert_eq!(
            chapter_list.len(),
            expected_count,
            "Chapter range from {:?} {} to {} should have {} chapters",
            book,
            start_chapter,
            end_chapter,
            expected_count
        );

        // Check that the chapters are in the correct order
        for i in 1..chapter_list.len() {
            assert!(
                chapter_list[i - 1].chapter() < chapter_list[i].chapter(),
                "Chapters should be in ascending order, but {} is not less than {}",
                chapter_list[i - 1].chapter(),
                chapter_list[i].chapter()
            );
        }
    }
}

#[test]
fn test_chapter_next_previous() {
    // Test the next() and previous() methods for chapters
    let test_cases = vec![
        BibleBook::Genesis,
        BibleBook::Psalm,
        BibleBook::Matthew,
        BibleBook::Revelation,
    ];

    for book in test_cases {
        let chapter_count = get_number_of_chapters(&book);

        for chapter_num in 1..=chapter_count {
            let chapter_ref = BibleReference::BibleChapter(
                BibleChapterReference::new(book, chapter_num).unwrap(),
            );

            // Test next()
            if chapter_num < chapter_count {
                let expected_next = BibleReference::BibleChapter(
                    BibleChapterReference::new(book, chapter_num + 1).unwrap(),
                );
                assert_eq!(
                    chapter_ref.next(),
                    Some(expected_next),
                    "The next chapter after {:?} {} should be {:?} {}",
                    book,
                    chapter_num,
                    book,
                    chapter_num + 1
                );
            } else if book != BibleBook::Revelation {
                // If this is the last chapter of a book (except Revelation), the next reference should be the first chapter of the next book
                let next_book = get_bible_book_by_number(book.number() + 1).unwrap();
                let expected_next = BibleReference::BibleBook(BibleBookReference::new(next_book));
                assert_eq!(
                    chapter_ref.next(),
                    Some(expected_next),
                    "The next reference after the last chapter of {:?} should be {:?}",
                    book,
                    next_book
                );
            } else {
                // If this is the last chapter of Revelation, there should be no next reference
                assert_eq!(
                    chapter_ref.next(),
                    None,
                    "There should be no next reference after the last chapter of Revelation"
                );
            }

            // Test previous()
            if chapter_num > 1 {
                let expected_prev = BibleReference::BibleChapter(
                    BibleChapterReference::new(book, chapter_num - 1).unwrap(),
                );
                assert_eq!(
                    chapter_ref.previous(),
                    Some(expected_prev),
                    "The previous chapter before {:?} {} should be {:?} {}",
                    book,
                    chapter_num,
                    book,
                    chapter_num - 1
                );
            } else if book != BibleBook::Genesis {
                // If this is the first chapter of a book (except Genesis), the previous reference should be the last chapter of the previous book
                let prev_book = get_bible_book_by_number(book.number() - 1).unwrap();
                let expected_prev = BibleReference::BibleBook(BibleBookReference::new(prev_book));
                assert_eq!(
                    chapter_ref.previous(),
                    Some(expected_prev),
                    "The previous reference before the first chapter of {:?} should be {:?}",
                    book,
                    prev_book
                );
            } else {
                // If this is the first chapter of Genesis, there should be no previous reference
                assert_eq!(
                    chapter_ref.previous(),
                    None,
                    "There should be no previous reference before the first chapter of Genesis"
                );
            }
        }
    }
}

// ==========================================
// 5.3 Verse Iteration Tests
// ==========================================

#[test]
fn test_iterate_verses_of_chapter() {
    // Test iterating through all verses of a chapter
    let test_cases = vec![
        (BibleBook::Genesis, 1, 31),     // Genesis 1 has 31 verses
        (BibleBook::Psalm, 119, 176),    // Psalm 119 has 176 verses
        (BibleBook::John, 3, 36),        // John 3 has 36 verses
        (BibleBook::Revelation, 22, 21), // Revelation 22 has 21 verses
    ];

    for (book, chapter, expected_verses) in test_cases {
        // Check that the number of verses is correct
        let verse_count = get_number_of_verses(&book, &chapter).unwrap();
        assert_eq!(
            verse_count, expected_verses,
            "{:?} {} should have {} verses",
            book, chapter, expected_verses
        );

        // Create a chapter reference and create a chapter range that spans just this chapter
        let chapter_ref = BibleChapterReference::new(book, chapter).unwrap();
        let chapter_range =
            BibleChapterRange::new(chapter_ref.clone(), chapter_ref.clone()).unwrap();
        let verse_range = chapter_range.as_verse_range();

        // Get the list of verses
        let verse_list = verse_range.as_list();

        // Check that the list has the correct length
        assert_eq!(
            verse_list.len(),
            expected_verses.into(),
            "{:?} {} should have {} verses",
            book,
            chapter,
            expected_verses
        );

        // Check that the verses are in the correct order
        for i in 1..verse_list.len() {
            assert!(
                verse_list[i - 1].verse() < verse_list[i].verse(),
                "Verses should be in ascending order, but {} is not less than {}",
                verse_list[i - 1].verse(),
                verse_list[i].verse()
            );
        }
    }
}

#[test]
fn test_iterate_verse_range() {
    // Test iterating through a range of verses
    let test_cases = vec![
        (BibleBook::Genesis, 1, 1, 5, 5),       // Genesis 1:1-5 (5 verses)
        (BibleBook::Psalm, 23, 1, 6, 6),        // Psalm 23:1-6 (6 verses)
        (BibleBook::John, 3, 16, 18, 3),        // John 3:16-18 (3 verses)
        (BibleBook::Revelation, 22, 18, 21, 4), // Revelation 22:18-21 (4 verses)
    ];

    for (book, chapter, start_verse, end_verse, expected_count) in test_cases {
        let start_ref = BibleVerseReference::new(book, chapter, start_verse).unwrap();
        let end_ref = BibleVerseReference::new(book, chapter, end_verse).unwrap();
        let verse_range = BibleVerseRange::new(start_ref, end_ref).unwrap();

        // Get the list of verses in the range
        let verse_list = verse_range.as_list();

        // Check that the list has the correct length
        assert_eq!(
            verse_list.len(),
            expected_count,
            "Verse range from {:?} {}:{} to {}:{} should have {} verses",
            book,
            chapter,
            start_verse,
            chapter,
            end_verse,
            expected_count
        );

        // Check that the verses are in the correct order
        for i in 1..verse_list.len() {
            assert!(
                verse_list[i - 1].verse() < verse_list[i].verse(),
                "Verses should be in ascending order, but {} is not less than {}",
                verse_list[i - 1].verse(),
                verse_list[i].verse()
            );
        }
    }
}

#[test]
fn test_verse_next_previous() {
    // Test the next() and previous() methods for verses
    let test_cases = vec![
        (BibleBook::Genesis, 1, 31), // Genesis 1 has 31 verses
        (BibleBook::John, 3, 36),    // John 3 has 36 verses
    ];

    for (book, chapter, verse_count) in test_cases {
        for verse_num in 1..=verse_count {
            let verse_ref = BibleReference::BibleVerse(
                BibleVerseReference::new(book, chapter, verse_num).unwrap(),
            );

            // Test next()
            if verse_num < verse_count {
                let expected_next = BibleReference::BibleVerse(
                    BibleVerseReference::new(book, chapter, verse_num + 1).unwrap(),
                );
                assert_eq!(
                    verse_ref.next(),
                    Some(expected_next),
                    "The next verse after {:?} {}:{} should be {:?} {}:{}",
                    book,
                    chapter,
                    verse_num,
                    book,
                    chapter,
                    verse_num + 1
                );
            } else {
                // If this is the last verse of a chapter, the next reference should be the next chapter
                let next_chapter = chapter + 1;
                if next_chapter <= get_number_of_chapters(&book) {
                    let expected_next = BibleReference::BibleChapter(
                        BibleChapterReference::new(book, next_chapter).unwrap(),
                    );
                    assert_eq!(
                        verse_ref.next(),
                        Some(expected_next),
                        "The next reference after the last verse of {:?} {} should be {:?} {}",
                        book,
                        chapter,
                        book,
                        next_chapter
                    );
                } else {
                    // If this is the last verse of the last chapter, the next reference should be the next book
                    if book != BibleBook::Revelation {
                        let next_book = get_bible_book_by_number(book.number() + 1).unwrap();
                        let expected_next =
                            BibleReference::BibleBook(BibleBookReference::new(next_book));
                        assert_eq!(
                            verse_ref.next(),
                            Some(expected_next),
                            "The next reference after the last verse of the last chapter of {:?} should be {:?}",
                            book,
                            next_book
                        );
                    } else {
                        // If this is the last verse of Revelation, there should be no next reference
                        assert_eq!(
                            verse_ref.next(),
                            None,
                            "There should be no next reference after the last verse of Revelation"
                        );
                    }
                }
            }

            // Test previous()
            if verse_num > 1 {
                let expected_prev = BibleReference::BibleVerse(
                    BibleVerseReference::new(book, chapter, verse_num - 1).unwrap(),
                );
                assert_eq!(
                    verse_ref.previous(),
                    Some(expected_prev),
                    "The previous verse before {:?} {}:{} should be {:?} {}:{}",
                    book,
                    chapter,
                    verse_num,
                    book,
                    chapter,
                    verse_num - 1
                );
            } else {
                // If this is the first verse of a chapter, the previous reference should be the previous chapter
                if chapter > 1 {
                    let prev_chapter = chapter - 1;
                    let expected_prev = BibleReference::BibleChapter(
                        BibleChapterReference::new(book, prev_chapter).unwrap(),
                    );
                    assert_eq!(
                        verse_ref.previous(),
                        Some(expected_prev),
                        "The previous reference before the first verse of {:?} {} should be {:?} {}",
                        book,
                        chapter,
                        book,
                        prev_chapter
                    );
                } else {
                    // If this is the first verse of the first chapter, the previous reference should be the previous book
                    if book != BibleBook::Genesis {
                        let prev_book = get_bible_book_by_number(book.number() - 1).unwrap();
                        let expected_prev =
                            BibleReference::BibleBook(BibleBookReference::new(prev_book));
                        assert_eq!(
                            verse_ref.previous(),
                            Some(expected_prev),
                            "The previous reference before the first verse of the first chapter of {:?} should be {:?}",
                            book,
                            prev_book
                        );
                    } else {
                        // If this is the first verse of Genesis, there should be no previous reference
                        assert_eq!(
                            verse_ref.previous(),
                            None,
                            "There should be no previous reference before the first verse of Genesis"
                        );
                    }
                }
            }
        }
    }
}
