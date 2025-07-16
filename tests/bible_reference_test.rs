use bibleref::{
    bible::{
        BibleBook, BibleBookReference, BibleChapterReference, BibleRange, BibleReference,
        BibleReferenceRepresentation, BibleVerseReference,
        validate::{get_number_of_chapters, get_number_of_verses},
    },
    parse,
};

// ==========================================
// 1. BibleReference Creation Tests
// ==========================================

#[test]
fn test_bible_reference_creation() {
    // Test creating valid BibleBookReference
    let book_ref = BibleBookReference::new(BibleBook::Genesis);
    assert_eq!(book_ref.book(), BibleBook::Genesis);
    
    // Test creating valid BibleChapterReference
    let chapter_ref = BibleChapterReference::new(BibleBook::Genesis, 1);
    assert!(chapter_ref.is_ok());
    let chapter_ref = chapter_ref.unwrap();
    assert_eq!(chapter_ref.book(), BibleBook::Genesis);
    assert_eq!(chapter_ref.chapter(), 1);
    
    // Test creating valid BibleVerseReference
    let verse_ref = BibleVerseReference::new(BibleBook::Genesis, 1, 1);
    assert!(verse_ref.is_ok());
    let verse_ref = verse_ref.unwrap();
    assert_eq!(verse_ref.book(), BibleBook::Genesis);
    assert_eq!(verse_ref.chapter(), 1);
    assert_eq!(verse_ref.verse(), 1);
    
    // Test creating invalid BibleChapterReference
    let invalid_chapter_ref = BibleChapterReference::new(BibleBook::Genesis, 100);
    assert!(invalid_chapter_ref.is_err());
    
    // Test creating invalid BibleVerseReference
    let invalid_verse_ref = BibleVerseReference::new(BibleBook::Genesis, 1, 100);
    assert!(invalid_verse_ref.is_err());
}

#[test]
fn test_bible_reference_enum() {
    // Test BibleReference::BibleBook
    let book_ref = BibleBookReference::new(BibleBook::Genesis);
    let bible_ref = BibleReference::BibleBook(book_ref);
    match bible_ref {
        BibleReference::BibleBook(ref book) => {
            assert_eq!(book.book(), BibleBook::Genesis);
        }
        _ => panic!("Expected BibleBook variant"),
    }
    
    // Test BibleReference::BibleChapter
    let chapter_ref = BibleChapterReference::new(BibleBook::Genesis, 1).unwrap();
    let bible_ref = BibleReference::BibleChapter(chapter_ref);
    match bible_ref {
        BibleReference::BibleChapter(ref chapter) => {
            assert_eq!(chapter.book(), BibleBook::Genesis);
            assert_eq!(chapter.chapter(), 1);
        }
        _ => panic!("Expected BibleChapter variant"),
    }
    
    // Test BibleReference::BibleVerse
    let verse_ref = BibleVerseReference::new(BibleBook::Genesis, 1, 1).unwrap();
    let bible_ref = BibleReference::BibleVerse(verse_ref);
    match bible_ref {
        BibleReference::BibleVerse(ref verse) => {
            assert_eq!(verse.book(), BibleBook::Genesis);
            assert_eq!(verse.chapter(), 1);
            assert_eq!(verse.verse(), 1);
        }
        _ => panic!("Expected BibleVerse variant"),
    }
}

// ==========================================
// 2. BibleReference Comparison Tests
// ==========================================

#[test]
fn test_bible_reference_ordering() {
    // Test ordering of BibleBookReference
    let genesis = BibleBookReference::new(BibleBook::Genesis);
    let exodus = BibleBookReference::new(BibleBook::Exodus);
    assert!(genesis < exodus);
    
    // Test ordering of BibleChapterReference
    let genesis_1 = BibleChapterReference::new(BibleBook::Genesis, 1).unwrap();
    let genesis_2 = BibleChapterReference::new(BibleBook::Genesis, 2).unwrap();
    let exodus_1 = BibleChapterReference::new(BibleBook::Exodus, 1).unwrap();
    assert!(genesis_1 < genesis_2);
    assert!(genesis_2 < exodus_1);
    
    // Test ordering of BibleVerseReference
    let genesis_1_1 = BibleVerseReference::new(BibleBook::Genesis, 1, 1).unwrap();
    let genesis_1_2 = BibleVerseReference::new(BibleBook::Genesis, 1, 2).unwrap();
    let genesis_2_1 = BibleVerseReference::new(BibleBook::Genesis, 2, 1).unwrap();
    assert!(genesis_1_1 < genesis_1_2);
    assert!(genesis_1_2 < genesis_2_1);
    
    // Test ordering of mixed BibleReference types
    let book_ref = BibleReference::BibleBook(genesis);
    let chapter_ref = BibleReference::BibleChapter(genesis_1);
    let verse_ref = BibleReference::BibleVerse(genesis_1_1.clone());
    
    assert!(book_ref < chapter_ref);
    assert!(chapter_ref < verse_ref);
    assert!(book_ref < verse_ref);
    
    // Test ordering of BibleReference with different books
    let genesis_verse = BibleReference::BibleVerse(genesis_1_1);
    let exodus_chapter = BibleReference::BibleChapter(exodus_1);
    assert!(genesis_verse < exodus_chapter);
}

// ==========================================
// 3. BibleRange Tests
// ==========================================

#[test]
fn test_bible_range_creation() {
    // Test creating valid BibleRange with books
    let genesis = BibleBookReference::new(BibleBook::Genesis);
    let exodus = BibleBookReference::new(BibleBook::Exodus);
    let book_range = BibleRange::new(
        BibleReference::BibleBook(genesis),
        BibleReference::BibleBook(exodus),
    );
    assert!(book_range.is_ok());
    
    // Test creating valid BibleRange with chapters
    let genesis_1 = BibleChapterReference::new(BibleBook::Genesis, 1).unwrap();
    let genesis_2 = BibleChapterReference::new(BibleBook::Genesis, 2).unwrap();
    let chapter_range = BibleRange::new(
        BibleReference::BibleChapter(genesis_1),
        BibleReference::BibleChapter(genesis_2),
    );
    assert!(chapter_range.is_ok());
    
    // Test creating valid BibleRange with verses
    let genesis_1_1 = BibleVerseReference::new(BibleBook::Genesis, 1, 1).unwrap();
    let genesis_1_2 = BibleVerseReference::new(BibleBook::Genesis, 1, 2).unwrap();
    let verse_range = BibleRange::new(
        BibleReference::BibleVerse(genesis_1_1),
        BibleReference::BibleVerse(genesis_1_2),
    );
    assert!(verse_range.is_ok());
    
    // Test creating invalid BibleRange (end before start)
    let genesis = BibleBookReference::new(BibleBook::Genesis);
    let exodus = BibleBookReference::new(BibleBook::Exodus);
    let invalid_range = BibleRange::new(
        BibleReference::BibleBook(exodus),
        BibleReference::BibleBook(genesis),
    );
    assert!(invalid_range.is_err());
}

#[test]
fn test_bible_range_as_list() {
    // Test BibleRange::as_list for books
    let genesis = BibleBookReference::new(BibleBook::Genesis);
    let exodus = BibleBookReference::new(BibleBook::Exodus);
    let book_range = BibleRange::new(
        BibleReference::BibleBook(genesis),
        BibleReference::BibleBook(exodus),
    )
    .unwrap();
    
    let book_list = book_range.as_list();
    assert_eq!(book_list.len(), 2);
    assert!(matches!(book_list[0], BibleReference::BibleBook(_)));
    assert!(matches!(book_list[1], BibleReference::BibleBook(_)));
    
    // Test BibleRange::as_list for chapters
    let genesis_1 = BibleChapterReference::new(BibleBook::Genesis, 1).unwrap();
    let genesis_3 = BibleChapterReference::new(BibleBook::Genesis, 3).unwrap();
    let chapter_range = BibleRange::new(
        BibleReference::BibleChapter(genesis_1),
        BibleReference::BibleChapter(genesis_3),
    )
    .unwrap();
    
    let chapter_list = chapter_range.as_list();
    assert_eq!(chapter_list.len(), 3);
    assert!(matches!(chapter_list[0], BibleReference::BibleChapter(_)));
    assert!(matches!(chapter_list[1], BibleReference::BibleChapter(_)));
    assert!(matches!(chapter_list[2], BibleReference::BibleChapter(_)));
    
    // Test BibleRange::as_list for verses
    let genesis_1_1 = BibleVerseReference::new(BibleBook::Genesis, 1, 1).unwrap();
    let genesis_1_3 = BibleVerseReference::new(BibleBook::Genesis, 1, 3).unwrap();
    let verse_range = BibleRange::new(
        BibleReference::BibleVerse(genesis_1_1),
        BibleReference::BibleVerse(genesis_1_3),
    )
    .unwrap();
    
    let verse_list = verse_range.as_list();
    assert_eq!(verse_list.len(), 3);
    assert!(matches!(verse_list[0], BibleReference::BibleVerse(_)));
    assert!(matches!(verse_list[1], BibleReference::BibleVerse(_)));
    assert!(matches!(verse_list[2], BibleReference::BibleVerse(_)));
}

// ==========================================
// 4. BibleReferenceRepresentation Tests
// ==========================================

#[test]
fn test_bible_reference_representation() {
    // Test BibleReferenceRepresentation::Single
    let genesis_1_1 = BibleVerseReference::new(BibleBook::Genesis, 1, 1).unwrap();
    let single_ref = BibleReferenceRepresentation::Single(BibleReference::BibleVerse(genesis_1_1));
    assert!(single_ref.is_single());
    assert!(!single_ref.is_range());
    
    // Test BibleReferenceRepresentation::Range
    let genesis_1_1 = BibleVerseReference::new(BibleBook::Genesis, 1, 1).unwrap();
    let genesis_1_3 = BibleVerseReference::new(BibleBook::Genesis, 1, 3).unwrap();
    let range = BibleRange::new(
        BibleReference::BibleVerse(genesis_1_1),
        BibleReference::BibleVerse(genesis_1_3),
    )
    .unwrap();
    let range_ref = BibleReferenceRepresentation::Range(range);
    assert!(!range_ref.is_single());
    assert!(range_ref.is_range());
}

#[test]
fn test_bible_reference_representation_try_upcast() {
    // Test upcasting a verse range that spans a whole chapter
    let book = BibleBook::Genesis;
    let chapter = 1;
    let verse_count = get_number_of_verses(&book, &chapter).unwrap();
    
    let start_verse = BibleVerseReference::new(book, chapter, 1).unwrap();
    let end_verse = BibleVerseReference::new(book, chapter, verse_count).unwrap();
    
    let verse_range = BibleRange::new(
        BibleReference::BibleVerse(start_verse),
        BibleReference::BibleVerse(end_verse),
    )
    .unwrap();
    
    let range_ref = BibleReferenceRepresentation::Range(verse_range);
    let upcasted = range_ref.try_upcast();
    
    // Should be upcasted to a chapter reference
    assert!(upcasted.is_single());
    if let BibleReferenceRepresentation::Single(BibleReference::BibleChapter(chapter_ref)) = upcasted {
        assert_eq!(chapter_ref.book(), book);
        assert_eq!(chapter_ref.chapter(), chapter);
    } else {
        panic!("Expected upcasted reference to be a chapter reference");
    }
    
    // Test upcasting a single verse reference (should remain the same)
    let verse_ref = BibleVerseReference::new(book, chapter, 1).unwrap();
    let single_ref = BibleReferenceRepresentation::Single(BibleReference::BibleVerse(verse_ref));
    let upcasted = single_ref.try_upcast();
    
    assert_eq!(upcasted, single_ref);
}

// ==========================================
// 5. BibleReference Navigation Tests
// ==========================================

#[test]
fn test_bible_reference_next_previous() {
    // Test next() for BibleBookReference
    let genesis = BibleBookReference::new(BibleBook::Genesis);
    let genesis_ref = BibleReference::BibleBook(genesis);
    let next_ref = genesis_ref.next().unwrap();
    
    if let BibleReference::BibleBook(book_ref) = next_ref {
        assert_eq!(book_ref.book(), BibleBook::Exodus);
    } else {
        panic!("Expected next reference to be a book reference");
    }
    
    // Test previous() for BibleBookReference
    let exodus = BibleBookReference::new(BibleBook::Exodus);
    let exodus_ref = BibleReference::BibleBook(exodus);
    let prev_ref = exodus_ref.previous().unwrap();
    
    if let BibleReference::BibleBook(book_ref) = prev_ref {
        assert_eq!(book_ref.book(), BibleBook::Genesis);
    } else {
        panic!("Expected previous reference to be a book reference");
    }
    
    // Test next() for BibleChapterReference
    let genesis_1 = BibleChapterReference::new(BibleBook::Genesis, 1).unwrap();
    let chapter_ref = BibleReference::BibleChapter(genesis_1);
    let next_ref = chapter_ref.next().unwrap();
    
    if let BibleReference::BibleChapter(chapter_ref) = next_ref {
        assert_eq!(chapter_ref.book(), BibleBook::Genesis);
        assert_eq!(chapter_ref.chapter(), 2);
    } else {
        panic!("Expected next reference to be a chapter reference");
    }
    
    // Test previous() for BibleChapterReference
    let genesis_2 = BibleChapterReference::new(BibleBook::Genesis, 2).unwrap();
    let chapter_ref = BibleReference::BibleChapter(genesis_2);
    let prev_ref = chapter_ref.previous().unwrap();
    
    if let BibleReference::BibleChapter(chapter_ref) = prev_ref {
        assert_eq!(chapter_ref.book(), BibleBook::Genesis);
        assert_eq!(chapter_ref.chapter(), 1);
    } else {
        panic!("Expected previous reference to be a chapter reference");
    }
    
    // Test next() for BibleVerseReference
    let genesis_1_1 = BibleVerseReference::new(BibleBook::Genesis, 1, 1).unwrap();
    let verse_ref = BibleReference::BibleVerse(genesis_1_1);
    let next_ref = verse_ref.next().unwrap();
    
    if let BibleReference::BibleVerse(verse_ref) = next_ref {
        assert_eq!(verse_ref.book(), BibleBook::Genesis);
        assert_eq!(verse_ref.chapter(), 1);
        assert_eq!(verse_ref.verse(), 2);
    } else {
        panic!("Expected next reference to be a verse reference");
    }
    
    // Test previous() for BibleVerseReference
    let genesis_1_2 = BibleVerseReference::new(BibleBook::Genesis, 1, 2).unwrap();
    let verse_ref = BibleReference::BibleVerse(genesis_1_2);
    let prev_ref = verse_ref.previous().unwrap();
    
    if let BibleReference::BibleVerse(verse_ref) = prev_ref {
        assert_eq!(verse_ref.book(), BibleBook::Genesis);
        assert_eq!(verse_ref.chapter(), 1);
        assert_eq!(verse_ref.verse(), 1);
    } else {
        panic!("Expected previous reference to be a verse reference");
    }
    
    // Test next() at chapter boundary
    let genesis_1_31 = BibleVerseReference::new(BibleBook::Genesis, 1, 31).unwrap();
    let verse_ref = BibleReference::BibleVerse(genesis_1_31);
    let next_ref = verse_ref.next().unwrap();
    
    if let BibleReference::BibleChapter(chapter_ref) = next_ref {
        assert_eq!(chapter_ref.book(), BibleBook::Genesis);
        assert_eq!(chapter_ref.chapter(), 2);
    } else {
        panic!("Expected next reference to be a chapter reference");
    }
    
    // Test previous() at chapter boundary
    let genesis_2_1 = BibleVerseReference::new(BibleBook::Genesis, 2, 1).unwrap();
    let verse_ref = BibleReference::BibleVerse(genesis_2_1);
    let prev_ref = verse_ref.previous().unwrap();
    
    if let BibleReference::BibleChapter(chapter_ref) = prev_ref {
        assert_eq!(chapter_ref.book(), BibleBook::Genesis);
        assert_eq!(chapter_ref.chapter(), 1);
    } else {
        panic!("Expected previous reference to be a chapter reference");
    }
    
    // Test next() at book boundary
    let genesis_50 = BibleChapterReference::new(BibleBook::Genesis, 50).unwrap();
    let chapter_ref = BibleReference::BibleChapter(genesis_50);
    let next_ref = chapter_ref.next().unwrap();
    
    if let BibleReference::BibleBook(book_ref) = next_ref {
        assert_eq!(book_ref.book(), BibleBook::Exodus);
    } else {
        panic!("Expected next reference to be a book reference");
    }
    
    // Test previous() at book boundary
    let exodus_1 = BibleChapterReference::new(BibleBook::Exodus, 1).unwrap();
    let chapter_ref = BibleReference::BibleChapter(exodus_1);
    let prev_ref = chapter_ref.previous().unwrap();
    
    if let BibleReference::BibleBook(book_ref) = prev_ref {
        assert_eq!(book_ref.book(), BibleBook::Genesis);
    } else {
        panic!("Expected previous reference to be a book reference");
    }
    
    // Test next() at Bible boundary (Revelation)
    let revelation = BibleBookReference::new(BibleBook::Revelation);
    let revelation_ref = BibleReference::BibleBook(revelation);
    let next_ref = revelation_ref.next();
    assert_eq!(next_ref, None);
    
    // Test previous() at Bible boundary (Genesis)
    let genesis = BibleBookReference::new(BibleBook::Genesis);
    let genesis_ref = BibleReference::BibleBook(genesis);
    let prev_ref = genesis_ref.previous();
    assert_eq!(prev_ref, None);
}

// ==========================================
// 6. BibleBook Tests
// ==========================================

#[test]
fn test_bible_book_properties() {
    // Test BibleBook::all()
    let all_books = BibleBook::all();
    assert_eq!(all_books.len(), 66);
    assert_eq!(all_books[0], BibleBook::Genesis);
    assert_eq!(all_books[65], BibleBook::Revelation);
    
    // Test BibleBook::is_old_testament()
    assert!(BibleBook::Genesis.is_old_testament());
    assert!(BibleBook::Malachi.is_old_testament());
    assert!(!BibleBook::Matthew.is_old_testament());
    assert!(!BibleBook::Revelation.is_old_testament());
    
    // Test BibleBook::is_new_testament()
    assert!(!BibleBook::Genesis.is_new_testament());
    assert!(!BibleBook::Malachi.is_new_testament());
    assert!(BibleBook::Matthew.is_new_testament());
    assert!(BibleBook::Revelation.is_new_testament());
    
    // Test BibleBook::number()
    assert_eq!(BibleBook::Genesis.number(), 1);
    assert_eq!(BibleBook::Exodus.number(), 2);
    assert_eq!(BibleBook::Matthew.number(), 40);
    assert_eq!(BibleBook::Revelation.number(), 66);
    
    // Test get_bible_book_by_number()
    assert_eq!(bibleref::bible::get_bible_book_by_number(1), Some(BibleBook::Genesis));
    assert_eq!(bibleref::bible::get_bible_book_by_number(40), Some(BibleBook::Matthew));
    assert_eq!(bibleref::bible::get_bible_book_by_number(66), Some(BibleBook::Revelation));
    assert_eq!(bibleref::bible::get_bible_book_by_number(0), None);
    assert_eq!(bibleref::bible::get_bible_book_by_number(67), None);
}

// ==========================================
// 7. Validation Tests
// ==========================================

#[test]
fn test_bible_reference_validation() {
    // Test valid references
    assert!(BibleChapterReference::new(BibleBook::Genesis, 1).is_ok());
    assert!(BibleVerseReference::new(BibleBook::Genesis, 1, 1).is_ok());
    
    // Test invalid chapter
    assert!(BibleChapterReference::new(BibleBook::Genesis, 0).is_err());
    assert!(BibleChapterReference::new(BibleBook::Genesis, 51).is_err());
    
    // Test invalid verse
    assert!(BibleVerseReference::new(BibleBook::Genesis, 1, 0).is_err());
    assert!(BibleVerseReference::new(BibleBook::Genesis, 1, 32).is_err());
    
    // Test invalid chapter-verse combination
    assert!(BibleVerseReference::new(BibleBook::Genesis, 51, 1).is_err());
    
    // Test get_number_of_chapters
    assert_eq!(get_number_of_chapters(&BibleBook::Genesis), 50);
    assert_eq!(get_number_of_chapters(&BibleBook::Psalm), 150);
    assert_eq!(get_number_of_chapters(&BibleBook::John), 21);
    assert_eq!(get_number_of_chapters(&BibleBook::Jude), 1);
    
    // Test get_number_of_verses
    assert_eq!(get_number_of_verses(&BibleBook::Genesis, &1).unwrap(), 31);
    assert_eq!(get_number_of_verses(&BibleBook::Psalm, &119).unwrap(), 176);
    assert_eq!(get_number_of_verses(&BibleBook::John, &3).unwrap(), 36);
    assert_eq!(get_number_of_verses(&BibleBook::Jude, &1).unwrap(), 25);
    
    // Test get_number_of_verses with invalid chapter
    assert!(get_number_of_verses(&BibleBook::Genesis, &51).is_err());
}

// ==========================================
// 8. Parsing Tests
// ==========================================

#[test]
fn test_parse_function() {
    // Test parsing book references
    let result = parse("Genesis");
    assert!(result.is_ok());
    let reference = result.unwrap();
    assert!(reference.is_single());
    
    // Test parsing chapter references
    let result = parse("Genesis 1");
    assert!(result.is_ok());
    let reference = result.unwrap();
    assert!(reference.is_single());
    
    // Test parsing verse references
    let result = parse("Genesis 1:1");
    assert!(result.is_ok());
    let reference = result.unwrap();
    assert!(reference.is_single());
    
    // Test parsing range references
    let result = parse("Genesis 1:1-3");
    assert!(result.is_ok());
    let reference = result.unwrap();
    assert!(reference.is_range());
    
    // Test parsing chapter range references
    let result = parse("Genesis 1-3");
    assert!(result.is_ok());
    let reference = result.unwrap();
    assert!(reference.is_range());
    
    // Test parsing book range references
    let result = parse("Genesis-Exodus");
    assert!(result.is_ok());
    let reference = result.unwrap();
    assert!(reference.is_range());
    
    // Test parsing invalid references
    let result = parse("Genesis 51");
    assert!(result.is_err());
    
    let result = parse("Genesis 1:32");
    assert!(result.is_err());
    
    let result = parse("NonexistentBook 1:1");
    assert!(result.is_err());
}