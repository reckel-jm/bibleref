use bibleref::{
    bible::{
        BibleBook, BibleChapterReference, BibleRange, BibleReference, BibleReferenceRepresentation,
        BibleVerseRange, BibleVerseReference,
        validate::{get_number_of_chapters, get_number_of_verses},
    },
    referencing::language::{BookReferenceType, get_reference_representation_in_language},
};
use bibleref::bible::BibleBookReference;

#[test]
fn upcast_chapters() {
    // Go through each book of the Bible
    for book in BibleBook::all() {
        for chapter in 1..=get_number_of_chapters(&book) {
            let start = BibleVerseReference::new(book, chapter, 1).unwrap();
            let end = BibleVerseReference::new(
                book,
                chapter,
                get_number_of_verses(&book, &chapter).unwrap(),
            )
            .unwrap();
            let chapter_reference = BibleChapterReference::new(book, chapter).unwrap();
            let verse_range = BibleRange::VerseRange(BibleVerseRange::new(start, end).unwrap());

            let verse_range_representation = BibleReferenceRepresentation::Range(verse_range);
            let chapter_reference_representation = BibleReferenceRepresentation::Single(
                BibleReference::BibleChapter(chapter_reference.clone()),
            );
            let upcasted = verse_range_representation.try_upcast();
            println!(
                "Upcasted: {} to {}",
                get_reference_representation_in_language(
                    &verse_range_representation,
                    "de",
                    BookReferenceType::Long,
                    true
                )
                .unwrap(),
                get_reference_representation_in_language(
                    &upcasted,
                    "de",
                    BookReferenceType::Long,
                    true
                )
                .unwrap()
            );
            if get_number_of_chapters(&chapter_reference.book()) > 1 {
                assert_eq!(upcasted, chapter_reference_representation);
            } else {
                // If the book has only one chapter, we assert an upcast to a BibleBookReference
                let bible_book_representation = BibleReferenceRepresentation::Single(
                    BibleReference::BibleBook(
                        BibleBookReference::new(chapter_reference.book())
                    )
                );
                assert_eq!(upcasted, bible_book_representation);
            }
            
        }
    }
}
