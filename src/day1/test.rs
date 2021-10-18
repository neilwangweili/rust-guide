#[cfg(test)]
mod tests {
    use crate::day1::const_shadowing::const_shadowing;
    use crate::day1::demo1hello_world::hello_world;
    use crate::day1::demo2compare_number::compare_number;
    use crate::day1::demo3type_change::type_change;
    use crate::day1::demo4guess_number::guess_number;
    use crate::day1::demo5variable_readable::variable_readable;
    use crate::day1::demo6data_type::data_type;

    #[test]
    fn hello_world_should_return_hello_world() {
        assert_eq!(hello_world(), "Hello world");
    }

    #[test]
    fn should_guess_number_5_return_true() {
        assert_eq!(compare_number(5), true);
    }

    #[test]
    fn should_guess_number_3_return_false() {
        assert_eq!(compare_number(3), false);
    }

    #[test]
    fn should_guess_number_10_return_false() {
        assert_eq!(compare_number(10), false);
    }

    #[test]
    fn should_change_type_from_string_to_int() {
        assert_eq!(type_change("5"), 5);
    }

    #[test]
    #[should_panic(expected = "Fail to convert to number.")]
    fn should_not_change_type_from_illegal_string_to_int() {
        type_change("a");
    }

    #[test]
    fn should_guess_number_match_secret_number() {
        assert_eq!(guess_number(), false);
    }

    #[test]
    fn should_variable_with_mut_tag_edit_its_value() {
        assert_eq!(variable_readable(), 6);
    }

    #[test]
    fn should_shadowing_variable() {
        assert_eq!(const_shadowing(5), 6);
    }

    #[test]
    fn show_data_types() {
        data_type();
    }

}
