use rust_guide::dojo::grep::grep::Grep;

#[test]
fn should_init_grep() {
    assert_eq!(
        Grep::of("./tests/dojo/file_for_test_grep/test1.txt")
            .about("hello")
            .len(),
        8
    );
}
