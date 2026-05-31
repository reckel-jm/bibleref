use bibleref::{
    self,
    bible::{
        BibleBook, BibleChapterReference, BibleRange, BibleReference,
        BibleReferenceRepresentation, BibleVerseRange, BibleVerseReference,
    },
};

#[test]
fn test_parse_dot_separated_verses() {
    // "Johannes 3,4.8" should parse as John 3:4 and John 3:8
    let result = bibleref::parse("Johannes 3,4.8").unwrap();
    assert!(result.is_multi_part(), "Expected multi-part reference");
    if let BibleReferenceRepresentation::MultiPart(parts) = &result {
        assert_eq!(parts.len(), 2);
        assert_eq!(
            parts[0],
            BibleReferenceRepresentation::Single(BibleReference::BibleVerse(
                BibleVerseReference::new(BibleBook::John, 3, 4).unwrap()
            ))
        );
        assert_eq!(
            parts[1],
            BibleReferenceRepresentation::Single(BibleReference::BibleVerse(
                BibleVerseReference::new(BibleBook::John, 3, 8).unwrap()
            ))
        );
    }
}

#[test]
fn test_parse_dot_separated_multiple_verses() {
    // "Johannes 3,4.8.16" should parse as John 3:4, John 3:8, and John 3:16
    let result = bibleref::parse("Johannes 3,4.8.16").unwrap();
    assert!(result.is_multi_part());
    if let BibleReferenceRepresentation::MultiPart(parts) = &result {
        assert_eq!(parts.len(), 3);
        assert_eq!(
            parts[0],
            BibleReferenceRepresentation::Single(BibleReference::BibleVerse(
                BibleVerseReference::new(BibleBook::John, 3, 4).unwrap()
            ))
        );
        assert_eq!(
            parts[1],
            BibleReferenceRepresentation::Single(BibleReference::BibleVerse(
                BibleVerseReference::new(BibleBook::John, 3, 8).unwrap()
            ))
        );
        assert_eq!(
            parts[2],
            BibleReferenceRepresentation::Single(BibleReference::BibleVerse(
                BibleVerseReference::new(BibleBook::John, 3, 16).unwrap()
            ))
        );
    }
}

#[test]
fn test_parse_plus_separated_chapters() {
    // "Hebräer 3+7" should parse as Hebrews 3 and Hebrews 7
    let result = bibleref::parse("Hebräer 3+7").unwrap();
    assert!(result.is_multi_part());
    if let BibleReferenceRepresentation::MultiPart(parts) = &result {
        assert_eq!(parts.len(), 2);
        assert_eq!(
            parts[0],
            BibleReferenceRepresentation::Single(BibleReference::BibleChapter(
                BibleChapterReference::new(BibleBook::Hebrews, 3).unwrap()
            ))
        );
        assert_eq!(
            parts[1],
            BibleReferenceRepresentation::Single(BibleReference::BibleChapter(
                BibleChapterReference::new(BibleBook::Hebrews, 7).unwrap()
            ))
        );
    }
}

#[test]
fn test_parse_range_still_works() {
    // "Hebräer 1-8" should still parse as a chapter range
    let result = bibleref::parse("Hebräer 1-8").unwrap();
    assert!(result.is_range());
}

#[test]
fn test_parse_simple_reference_still_works() {
    // Single references should still work
    let result = bibleref::parse("Johannes 3,16").unwrap();
    assert!(result.is_single());
    assert_eq!(
        result,
        BibleReferenceRepresentation::Single(BibleReference::BibleVerse(
            BibleVerseReference::new(BibleBook::John, 3, 16).unwrap()
        ))
    );
}

#[test]
fn test_upcast_full_chapter_from_verse_range() {
    // "Ps 1,1-6" (Psalm 1 has 6 verses) should upcast to Psalm 1
    let result = bibleref::parse("Ps 1,1-6").unwrap();
    let upcasted = result.try_upcast();
    assert_eq!(
        upcasted,
        BibleReferenceRepresentation::Single(BibleReference::BibleChapter(
            BibleChapterReference::new(BibleBook::Psalm, 1).unwrap()
        ))
    );
}

#[test]
fn test_dot_with_range_in_part() {
    // "Johannes 3,4.8-10" should parse as John 3:4 and John 3:8-10
    let result = bibleref::parse("Johannes 3,4.8-10").unwrap();
    assert!(result.is_multi_part());
    if let BibleReferenceRepresentation::MultiPart(parts) = &result {
        assert_eq!(parts.len(), 2);
        assert_eq!(
            parts[0],
            BibleReferenceRepresentation::Single(BibleReference::BibleVerse(
                BibleVerseReference::new(BibleBook::John, 3, 4).unwrap()
            ))
        );
        assert_eq!(
            parts[1],
            BibleReferenceRepresentation::Range(BibleRange::VerseRange(
                BibleVerseRange::new(
                    BibleVerseReference::new(BibleBook::John, 3, 8).unwrap(),
                    BibleVerseReference::new(BibleBook::John, 3, 10).unwrap()
                )
                .unwrap()
            ))
        );
    }
}

#[test]
fn test_english_dot_separated_verses() {
    // "John 3:4.8" should also work in English
    let result = bibleref::parse("John 3:4.8").unwrap();
    assert!(result.is_multi_part());
    if let BibleReferenceRepresentation::MultiPart(parts) = &result {
        assert_eq!(parts.len(), 2);
        assert_eq!(
            parts[0],
            BibleReferenceRepresentation::Single(BibleReference::BibleVerse(
                BibleVerseReference::new(BibleBook::John, 3, 4).unwrap()
            ))
        );
        assert_eq!(
            parts[1],
            BibleReferenceRepresentation::Single(BibleReference::BibleVerse(
                BibleVerseReference::new(BibleBook::John, 3, 8).unwrap()
            ))
        );
    }
}

#[test]
fn test_plus_separated_verses() {
    // "Johannes 3,4+8" should also work with + delimiter
    let result = bibleref::parse("Johannes 3,4+8").unwrap();
    assert!(result.is_multi_part());
    if let BibleReferenceRepresentation::MultiPart(parts) = &result {
        assert_eq!(parts.len(), 2);
        assert_eq!(
            parts[0],
            BibleReferenceRepresentation::Single(BibleReference::BibleVerse(
                BibleVerseReference::new(BibleBook::John, 3, 4).unwrap()
            ))
        );
        assert_eq!(
            parts[1],
            BibleReferenceRepresentation::Single(BibleReference::BibleVerse(
                BibleVerseReference::new(BibleBook::John, 3, 8).unwrap()
            ))
        );
    }
}

#[test]
fn test_upcast_multi_part() {
    // Upcasting multi-part references should upcast each part
    let result = bibleref::parse("Johannes 3,4.8").unwrap();
    let upcasted = result.try_upcast();
    // Parts should remain as verses (no simplification possible)
    assert!(upcasted.is_multi_part());
}

#[test]
fn test_range_verse_reference_with_dot() {
    // "Römer 8,28.31-39" should parse as Romans 8:28 and Romans 8:31-39
    let result = bibleref::parse("Römer 8,28.31-39").unwrap();
    assert!(result.is_multi_part());
    if let BibleReferenceRepresentation::MultiPart(parts) = &result {
        assert_eq!(parts.len(), 2);
        assert_eq!(
            parts[0],
            BibleReferenceRepresentation::Single(BibleReference::BibleVerse(
                BibleVerseReference::new(BibleBook::Romans, 8, 28).unwrap()
            ))
        );
        assert_eq!(
            parts[1],
            BibleReferenceRepresentation::Range(BibleRange::VerseRange(
                BibleVerseRange::new(
                    BibleVerseReference::new(BibleBook::Romans, 8, 31).unwrap(),
                    BibleVerseReference::new(BibleBook::Romans, 8, 39).unwrap()
                )
                .unwrap()
            ))
        );
    }
}
