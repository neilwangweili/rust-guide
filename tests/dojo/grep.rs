use rust_guide::dojo::grep::grep::Grep;

#[test]
fn should_init_grep() {
    assert_eq!(Grep::of().print().len(), 8);
}
