//! This module contains data types and structures for handling lists (arrays) of Bible references, multiple verses, multiple chapters or multiple books.

use super::{BibleReference, BibleBookReference, BibleChapterReference, BibleVerseReference};

use super::validate::{get_number_of_chapters,get_number_of_verses};

/// A [Vec<BibleBookReference>] which can be used to represent several Bible books
pub type BibleBookList = Vec<BibleBookReference>;

/// A [Vec<BibleChapterReference>] which can be used to represent several Bible chapters
pub type BibleChapterList = Vec<BibleChapterReference>;

/// A [Vec<BibleVerseReference>] which can be used to represent several Bible verses
pub type BibleVerseList = Vec<BibleVerseReference>;

/// A [Vec<BibleReference>] which can be used to represent several Bible references (books, chapters and verses)
pub type BibleReferenceList = Vec<BibleReference>;


/// Creates a [BibleChapterList] from a given [BibleBookList]
pub fn downcast_to_chapters(bible_books: BibleBookList) -> BibleChapterList {
    let mut bible_chapters: BibleChapterList = vec![];

    for book in bible_books {
        for chapter in 1..=get_number_of_chapters(&book.book()) {
            bible_chapters.push(
                BibleChapterReference::new(book.book(), chapter).unwrap()
            )
        }
    }

    bible_chapters
}

fn downcast_chapter_to_verses(bible_chapter: BibleChapterReference) -> BibleVerseList {
    let mut bible_verses: BibleVerseList = vec![];

    for verse in 1..=get_number_of_verses(&bible_chapter.book(), &bible_chapter.chapter).unwrap() {
        bible_verses.push(
            BibleVerseReference { 
                book: bible_chapter.book(), 
                chapter: bible_chapter.chapter(), 
                verse 
            }
        )
    }

    bible_verses
}

/// Casts a [BibleReferenceList] down to a [BibleVerseList]
pub fn downcast_to_verses(bible_references: BibleReferenceList) -> BibleVerseList {
    let mut bible_verses: BibleVerseList = vec![];

    for bible_reference in bible_references {
        match bible_reference {
            BibleReference::BibleVerse(verse_reference) => bible_verses.push(verse_reference),
            BibleReference::BibleChapter(chapter_reference) => {
                bible_verses.append(
                    &mut downcast_chapter_to_verses(chapter_reference)
                )
            },
            BibleReference::BibleBook(book_reference) => {
                downcast_to_chapters(vec![book_reference])
                    .iter()
                    .for_each(|chapter_reference| bible_verses.append(&mut downcast_chapter_to_verses(chapter_reference.clone())));
            }
        }
    }

    bible_verses
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_downcast_book_to_chapter() {
        let books: BibleBookList = vec![
            BibleBookReference::new(crate::bible::BibleBook::Genesis),
            BibleBookReference::new(crate::bible::BibleBook::Revelation)
        ];

        assert_eq!(
            downcast_to_chapters(books).len(), 72, "Genesis + Relevation should equal 72 chapters"
        );
    }

    #[test]
    fn test_downcast_chapter_to_verses() {
        let chapter: BibleChapterReference = BibleChapterReference::new(crate::bible::BibleBook::Ephesians, 1).unwrap();

        assert_eq!(
            downcast_chapter_to_verses(chapter).len(),
            23,
            "Wrong number of verses in Ephesians 1 (should be 23)"
        )
    }
}