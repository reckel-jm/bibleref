use bibleref::translate;

// ==========================================
// 3.1 Single Reference Translation Tests
// ==========================================

#[test]
fn test_translate_book_references() {
    // Test translating book references between languages
    let test_cases = vec![
        // English to German
        ("Genesis", "en", "1. Mose", "de"),
        ("Exodus", "en", "2. Mose", "de"),
        ("Matthew", "en", "Matthäus", "de"),
        ("Revelation", "en", "Offenbarung", "de"),
        // German to English
        ("1. Mose", "de", "Genesis", "en"),
        ("2. Mose", "de", "Exodus", "en"),
        ("Matthäus", "de", "Matthew", "en"),
        ("Offenbarung", "de", "Revelation", "en"),
        // English to Chinese (Simplified)
        ("Genesis", "en", "创世记", "zh_sim"),
        ("Exodus", "en", "出埃及记", "zh_sim"),
        ("Matthew", "en", "马太福音", "zh_sim"),
        ("Revelation", "en", "启示录", "zh_sim"),
        // Chinese (Simplified) to English
        ("创世记", "zh_sim", "Genesis", "en"),
        ("出埃及记", "zh_sim", "Exodus", "en"),
        ("马太福音", "zh_sim", "Matthew", "en"),
        ("启示录", "zh_sim", "Revelation", "en"),
        // German to Chinese (Simplified)
        ("1. Mose", "de", "创世记", "zh_sim"),
        ("2. Mose", "de", "出埃及记", "zh_sim"),
        ("Matthäus", "de", "马太福音", "zh_sim"),
        ("Offenbarung", "de", "启示录", "zh_sim"),
        // Chinese (Simplified) to German
        ("创世记", "zh_sim", "1. Mose", "de"),
        ("出埃及记", "zh_sim", "2. Mose", "de"),
        ("马太福音", "zh_sim", "Matthäus", "de"),
        ("启示录", "zh_sim", "Offenbarung", "de"),
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
fn test_translate_chapter_references() {
    // Test translating chapter references between languages
    let test_cases = vec![
        // English to German
        ("Genesis 1", "en", "1. Mose 1", "de"),
        ("Exodus 2", "en", "2. Mose 2", "de"),
        ("Matthew 5", "en", "Matthäus 5", "de"),
        ("Revelation 22", "en", "Offenbarung 22", "de"),
        // German to English
        ("1. Mose 1", "de", "Genesis 1", "en"),
        ("2. Mose 2", "de", "Exodus 2", "en"),
        ("Matthäus 5", "de", "Matthew 5", "en"),
        ("Offenbarung 22", "de", "Revelation 22", "en"),
        // English to Chinese (Simplified)
        ("Genesis 1", "en", "创世记1", "zh_sim"),
        ("Exodus 2", "en", "出埃及记2", "zh_sim"),
        ("Matthew 5", "en", "马太福音5", "zh_sim"),
        ("Revelation 22", "en", "启示录22", "zh_sim"),
        // Chinese (Simplified) to English
        ("创世记1", "zh_sim", "Genesis 1", "en"),
        ("出埃及记2", "zh_sim", "Exodus 2", "en"),
        ("马太福音5", "zh_sim", "Matthew 5", "en"),
        ("启示录22", "zh_sim", "Revelation 22", "en"),
        // German to Chinese (Simplified)
        ("1. Mose 1", "de", "创世记1", "zh_sim"),
        ("2. Mose 2", "de", "出埃及记2", "zh_sim"),
        ("Matthäus 5", "de", "马太福音5", "zh_sim"),
        ("Offenbarung 22", "de", "启示录22", "zh_sim"),
        // Chinese (Simplified) to German
        ("创世记1", "zh_sim", "1. Mose 1", "de"),
        ("出埃及记2", "zh_sim", "2. Mose 2", "de"),
        ("马太福音5", "zh_sim", "Matthäus 5", "de"),
        ("启示录22", "zh_sim", "Offenbarung 22", "de"),
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
fn test_translate_verse_references() {
    // Test translating verse references between languages
    let test_cases = vec![
        // English to German
        ("Genesis 1:1", "en", "1. Mose 1,1", "de"),
        ("Exodus 2:2", "en", "2. Mose 2,2", "de"),
        ("Matthew 5:3", "en", "Matthäus 5,3", "de"),
        ("John 3:16", "en", "Johannes 3,16", "de"),
        ("Revelation 22:21", "en", "Offenbarung 22,21", "de"),
        // German to English
        ("1. Mose 1,1", "de", "Genesis 1:1", "en"),
        ("2. Mose 2,2", "de", "Exodus 2:2", "en"),
        ("Matthäus 5,3", "de", "Matthew 5:3", "en"),
        ("Johannes 3,16", "de", "John 3:16", "en"),
        ("Offenbarung 22,21", "de", "Revelation 22:21", "en"),
        // English to Chinese (Simplified)
        ("Genesis 1:1", "en", "创世记1：1", "zh_sim"),
        ("Exodus 2:2", "en", "出埃及记2：2", "zh_sim"),
        ("Matthew 5:3", "en", "马太福音5：3", "zh_sim"),
        ("John 3:16", "en", "约翰福音3：16", "zh_sim"),
        ("Revelation 22:21", "en", "启示录22：21", "zh_sim"),
        // Chinese (Simplified) to English
        ("创世记1：1", "zh_sim", "Genesis 1:1", "en"),
        ("出埃及记2：2", "zh_sim", "Exodus 2:2", "en"),
        ("马太福音5：3", "zh_sim", "Matthew 5:3", "en"),
        ("约翰福音3：16", "zh_sim", "John 3:16", "en"),
        ("启示录22：21", "zh_sim", "Revelation 22:21", "en"),
        // German to Chinese (Simplified)
        ("1. Mose 1,1", "de", "创世记1：1", "zh_sim"),
        ("2. Mose 2,2", "de", "出埃及记2：2", "zh_sim"),
        ("Matthäus 5,3", "de", "马太福音5：3", "zh_sim"),
        ("Johannes 3,16", "de", "约翰福音3：16", "zh_sim"),
        ("Offenbarung 22,21", "de", "启示录22：21", "zh_sim"),
        // Chinese (Simplified) to German
        ("创世记1：1", "zh_sim", "1. Mose 1,1", "de"),
        ("出埃及记2：2", "zh_sim", "2. Mose 2,2", "de"),
        ("马太福音5：3", "zh_sim", "Matthäus 5,3", "de"),
        ("约翰福音3：16", "zh_sim", "Johannes 3,16", "de"),
        ("启示录22：21", "zh_sim", "Offenbarung 22,21", "de"),
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
fn test_translate_to_other_languages() {
    // Test translating to other supported languages
    let test_cases = vec![
        // English to French
        ("Genesis 1:1", "en", "Genèse 1:1", "fr"),
        ("John 3:16", "en", "Jean 3:16", "fr"),
        // English to Spanish
        ("Genesis 1:1", "en", "Génesis 1:1", "es"),
        ("John 3:16", "en", "Juan 3:16", "es"),
        // English to Italian
        ("Genesis 1:1", "en", "Genesi 1:1", "it"),
        ("John 3:16", "en", "Giovanni 3:16", "it"),
        // English to Dutch
        ("Genesis 1:1", "en", "Genesis 1:1", "nl"),
        ("John 3:16", "en", "Johannes 3:16", "nl"),
        // English to Russian
        ("Genesis 1:1", "en", "Бытие 1:1", "ru"),
        ("John 3:16", "en", "От Иоанна 3:16", "ru"),
        // English to Polish
        ("Genesis 1:1", "en", "Księga Rodzaju 1:1", "pl"),
        ("John 3:16", "en", "Ewangelia Jana 3:16", "pl"),
        // English to Czech
        ("Genesis 1:1", "en", "Genesis 1:1", "cs"),
        ("John 3:16", "en", "Jan 3:16", "cs"),
        // English to Hungarian
        ("Genesis 1:1", "en", "1 Mózes 1:1", "hu"),
        ("John 3:16", "en", "János 3:16", "hu"),
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

// ==========================================
// 3.2 Range Reference Translation Tests
// ==========================================

#[test]
fn test_translate_verse_ranges() {
    // Test translating verse ranges between languages
    let test_cases = vec![
        // English to German
        ("Genesis 1:1-3", "en", "1. Mose 1,1-3", "de"),
        ("John 3:16-18", "en", "Johannes 3,16-18", "de"),
        ("Revelation 22:18-21", "en", "Offenbarung 22,18-21", "de"),
        // German to English
        ("1. Mose 1,1-3", "de", "Genesis 1:1-3", "en"),
        ("Johannes 3,16-18", "de", "John 3:16-18", "en"),
        ("Offenbarung 22,18-21", "de", "Revelation 22:18-21", "en"),
        // English to Chinese (Simplified)
        ("Genesis 1:1-3", "en", "创世记1：1-3", "zh_sim"),
        ("John 3:16-18", "en", "约翰福音3：16-18", "zh_sim"),
        ("Revelation 22:18-21", "en", "启示录22：18-21", "zh_sim"),
        // Chinese (Simplified) to English
        ("创世记1：1-3", "zh_sim", "Genesis 1:1-3", "en"),
        ("约翰福音3：16-18", "zh_sim", "John 3:16-18", "en"),
        ("启示录22：18-21", "zh_sim", "Revelation 22:18-21", "en"),
        // German to Chinese (Simplified)
        ("1. Mose 1,1-3", "de", "创世记1：1-3", "zh_sim"),
        ("Johannes 3,16-18", "de", "约翰福音3：16-18", "zh_sim"),
        ("Offenbarung 22,18-21", "de", "启示录22：18-21", "zh_sim"),
        // Chinese (Simplified) to German
        ("创世记1：1-3", "zh_sim", "1. Mose 1,1-3", "de"),
        ("约翰福音3：16-18", "zh_sim", "Johannes 3,16-18", "de"),
        ("启示录22：18-21", "zh_sim", "Offenbarung 22,18-21", "de"),
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
fn test_translate_chapter_ranges() {
    // Test translating chapter ranges between languages
    let test_cases = vec![
        // English to German
        ("Genesis 1-3", "en", "1. Mose 1-3", "de"),
        ("Psalms 1-5", "en", "Psalmen 1-5", "de"),
        ("Matthew 5-7", "en", "Matthäus 5-7", "de"),
        // German to English
        ("1. Mose 1-3", "de", "Genesis 1-3", "en"),
        ("Psalmen 1-5", "de", "Psalms 1-5", "en"),
        ("Matthäus 5-7", "de", "Matthew 5-7", "en"),
        // English to Chinese (Simplified)
        ("Genesis 1-3", "en", "创世记1-3", "zh_sim"),
        ("Psalms 1-5", "en", "诗篇1-5", "zh_sim"),
        ("Matthew 5-7", "en", "马太福音5-7", "zh_sim"),
        // Chinese (Simplified) to English
        ("创世记1-3", "zh_sim", "Genesis 1-3", "en"),
        ("诗篇1-5", "zh_sim", "Psalms 1-5", "en"),
        ("马太福音5-7", "zh_sim", "Matthew 5-7", "en"),
        // German to Chinese (Simplified)
        ("1. Mose 1-3", "de", "创世记1-3", "zh_sim"),
        ("Psalmen 1-5", "de", "诗篇1-5", "zh_sim"),
        ("Matthäus 5-7", "de", "马太福音5-7", "zh_sim"),
        // Chinese (Simplified) to German
        ("创世记1-3", "zh_sim", "1. Mose 1-3", "de"),
        ("诗篇1-5", "zh_sim", "Psalmen 1-5", "de"),
        ("马太福音5-7", "zh_sim", "Matthäus 5-7", "de"),
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
fn test_translate_book_ranges() {
    // Test translating book ranges between languages
    let test_cases = vec![
        // English to German
        ("Genesis-Exodus", "en", "1. Mose-2. Mose", "de"),
        ("Matthew-Mark", "en", "Matthäus-Markus", "de"),
        (
            "1 Corinthians-2 Corinthians",
            "en",
            "1. Korinther-2. Korinther",
            "de",
        ),
        // German to English
        ("1. Mose-2. Mose", "de", "Genesis-Exodus", "en"),
        ("Matthäus-Markus", "de", "Matthew-Mark", "en"),
        (
            "1. Korinther-2. Korinther",
            "de",
            "1 Corinthians-2 Corinthians",
            "en",
        ),
        // English to Chinese (Simplified)
        ("Genesis-Exodus", "en", "创世记-出埃及记", "zh_sim"),
        ("Matthew-Mark", "en", "马太福音-马可福音", "zh_sim"),
        (
            "1 Corinthians-2 Corinthians",
            "en",
            "哥林多前书-哥林多后书",
            "zh_sim",
        ),
        // Chinese (Simplified) to English
        ("创世记-出埃及记", "zh_sim", "Genesis-Exodus", "en"),
        ("马太福音-马可福音", "zh_sim", "Matthew-Mark", "en"),
        (
            "哥林多前书-哥林多后书",
            "zh_sim",
            "1 Corinthians-2 Corinthians",
            "en",
        ),
        // German to Chinese (Simplified)
        ("1. Mose-2. Mose", "de", "创世记-出埃及记", "zh_sim"),
        ("Matthäus-Markus", "de", "马太福音-马可福音", "zh_sim"),
        (
            "1. Korinther-2. Korinther",
            "de",
            "哥林多前书-哥林多后书",
            "zh_sim",
        ),
        // Chinese (Simplified) to German
        ("创世记-出埃及记", "zh_sim", "1. Mose-2. Mose", "de"),
        ("马太福音-马可福音", "zh_sim", "Matthäus-Markus", "de"),
        (
            "哥林多前书-哥林多后书",
            "zh_sim",
            "1. Korinther-2. Korinther",
            "de",
        ),
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
fn test_translate_range_to_other_languages() {
    // Test translating ranges to other supported languages
    let test_cases = vec![
        // English to French
        ("Genesis 1:1-3", "en", "Genèse 1:1-3", "fr"),
        ("John 3:16-18", "en", "Jean 3:16-18", "fr"),
        // English to Spanish
        ("Genesis 1:1-3", "en", "Génesis 1:1-3", "es"),
        ("John 3:16-18", "en", "Juan 3:16-18", "es"),
        // English to Italian
        ("Genesis 1:1-3", "en", "Genesi 1:1-3", "it"),
        ("John 3:16-18", "en", "Giovanni 3:16-18", "it"),
        // English to Dutch
        ("Genesis 1:1-3", "en", "Genesis 1:1-3", "nl"),
        ("John 3:16-18", "en", "Johannes 3:16-18", "nl"),
        // English to Russian
        ("Genesis 1:1-3", "en", "Бытие 1:1-3", "ru"),
        ("John 3:16-18", "en", "От Иоанна 3:16-18", "ru"),
        // English to Polish
        ("Genesis 1:1-3", "en", "Księga Rodzaju 1:1-3", "pl"),
        ("John 3:16-18", "en", "Ewangelia Jana 3:16-18", "pl"),
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
fn test_translate_invalid_references() {
    // Test translating invalid references
    let invalid_references = vec![
        // Invalid book
        "GenesisX",
        "Exoduss",
        "Mattheww",
        "Revelations",
        // Invalid chapter
        "Genesis 51",
        "Psalm 151",
        "John 22",
        "Revelation 23",
        // Invalid verse
        "Genesis 1:32",
        "Psalm 23:7",
        "John 3:37",
        "Revelation 22:22",
        // Invalid format
        "Genesis:1",
        "Psalm:23",
        "John:3",
        "Revelation:22",
        // Empty reference
        "",
        // Nonsense
        "xyz",
        "123",
        "abc123",
    ];

    for reference in invalid_references {
        let result = translate(reference, "de");
        assert!(
            result.is_err(),
            "Expected error for invalid reference '{}', but got Ok",
            reference
        );
    }
}

#[test]
fn test_translate_to_invalid_language() {
    // Test translating to an invalid language
    let valid_references = vec![
        "Genesis 1:1",
        "Exodus 2:2",
        "Matthew 5:3",
        "John 3:16",
        "Revelation 22:21",
    ];

    for reference in valid_references {
        let result = translate(reference, "invalid_language");
        assert!(
            result.is_err(),
            "Expected error for invalid language 'invalid_language', but got Ok",
        );
    }
}
