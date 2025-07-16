
#[test]
fn test_range_parsing() {
    let ranges = ["Joh 3-4",
        "Mt 5,2-7",
        "Klagelieder 1-2",
        "Ps 1-2",
        "罗3：1-2"];
    ranges
        .iter()
        .for_each(|range| match bibleref::parse(range) {
            Ok(reference) => assert!(
                reference.is_range(),
                "'{}' expected a range reference",
                range
            ),
            Err(error) => panic!("'{}' failed to parse: {}", range, error),
        });
}
