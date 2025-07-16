# Test Plan for bibleref Crate

This document outlines a plan for adding 300 new tests to the bibleref crate, organized by functionality and test type.

## Test Categories

### 1. Parsing Tests (100 tests)

#### 1.1 Single Reference Parsing (50 tests)
- Test parsing book references in all supported languages (16 tests)
- Test parsing chapter references in all supported languages (16 tests)
- Test parsing verse references in all supported languages (16 tests)
- Test parsing with extra spaces and formatting variations (2 tests)

#### 1.2 Range Reference Parsing (50 tests)
- Test parsing book ranges in all supported languages (16 tests)
- Test parsing chapter ranges in all supported languages (16 tests)
- Test parsing verse ranges in all supported languages (16 tests)
- Test parsing with different range delimiters (2 tests)

### 2. Validation Tests (50 tests)

#### 2.1 Valid References (25 tests)
- Test valid book references (5 tests)
- Test valid chapter references (10 tests)
- Test valid verse references (10 tests)

#### 2.2 Invalid References (25 tests)
- Test invalid book references (5 tests)
- Test invalid chapter references (10 tests)
- Test invalid verse references (10 tests)

### 3. Translation Tests (50 tests)

#### 3.1 Single Reference Translation (25 tests)
- Test translating book references between languages (8 tests)
- Test translating chapter references between languages (8 tests)
- Test translating verse references between languages (9 tests)

#### 3.2 Range Reference Translation (25 tests)
- Test translating book ranges between languages (8 tests)
- Test translating chapter ranges between languages (8 tests)
- Test translating verse ranges between languages (9 tests)

### 4. Upcasting/Downcasting Tests (30 tests)

#### 4.1 Upcasting Tests (15 tests)
- Test upcasting verse ranges to chapters (5 tests)
- Test upcasting chapter ranges to books (5 tests)
- Test upcasting with edge cases (5 tests)

#### 4.2 Downcasting Tests (15 tests)
- Test downcasting books to chapter ranges (5 tests)
- Test downcasting chapters to verse ranges (5 tests)
- Test downcasting with edge cases (5 tests)

### 5. Iteration Tests (30 tests)

#### 5.1 Book Iteration (10 tests)
- Test iterating through all books of the Bible (5 tests)
- Test iterating through specific book ranges (5 tests)

#### 5.2 Chapter Iteration (10 tests)
- Test iterating through all chapters of a book (5 tests)
- Test iterating through specific chapter ranges (5 tests)

#### 5.3 Verse Iteration (10 tests)
- Test iterating through all verses of a chapter (5 tests)
- Test iterating through specific verse ranges (5 tests)

### 6. Edge Case Tests (40 tests)

#### 6.1 Language-Specific Edge Cases (20 tests)
- Test handling of special characters in different languages (10 tests)
- Test handling of language-specific formatting rules (10 tests)

#### 6.2 Reference Edge Cases (20 tests)
- Test books with only one chapter (5 tests)
- Test chapters with varying verse counts (5 tests)
- Test references at the boundaries of the Bible (5 tests)
- Test handling of malformed references (5 tests)

## Implementation Plan

1. Create new test files for each major category:
   - `parsing_tests.rs` - Comprehensive parsing tests
   - `validation_tests.rs` - Tests for reference validation
   - `translation_tests.rs` - Tests for reference translation
   - `casting_tests.rs` - Tests for upcasting and downcasting
   - `iteration_tests.rs` - Tests for iterating through references
   - `edge_case_tests.rs` - Tests for edge cases

2. Implement tests in order of importance:
   - Start with parsing and validation tests
   - Move to translation tests
   - Implement upcasting/downcasting and iteration tests
   - Finish with edge case tests

3. Ensure all tests are well-documented with clear descriptions of what they're testing.

4. Run tests regularly to ensure they pass and don't introduce regressions.