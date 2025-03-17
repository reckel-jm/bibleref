//! This module contains data types and structures for handling lists (arrays) of Bible references, multiple verses, multiple chapters or multiple books.

use super::{BibleReference, BibleBookReference, BibleChapterReference, BibleVerseReference, BibleChapter};

use super::validate::get_number_of_chapters;

/// A [Vec<BibleBookReference>] which can be used to represent several Bible books
pub type BibleBookList = Vec<BibleBookReference>;

pub type BibleChapterList = Vec<BibleChapterReference>;

pub type BibleVerseList = Vec<BibleVerseReference>;


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
}