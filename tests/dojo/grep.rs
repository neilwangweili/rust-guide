use rust_guide::dojo::grep::grep::Grep;

#[test]
fn should_init_grep() {
    let grep = Grep::of("./tests/dojo/file_for_test_grep/test1.txt").about("hello");
    assert_eq!(grep.len(), 8);
}

#[test]
fn should_print_grep_line() {
    let grep = Grep::of("./tests/dojo/file_for_test_grep/test1.txt").about("hello");
    assert_eq!(true, grep[0].show().contains("hello 1"));
}

#[test]
#[should_panic]
fn should_error() {
    Grep::of("./tests/dojo/file_for_test_grep/no_result.txt");
}

#[test]
fn should_match_lowercase() {
    let grep = Grep::of("./tests/dojo/file_for_test_grep/test2.txt").about("hello");
    assert_eq!(grep.len(), 2);
}
