#[cfg(test)]
mod test {
    use crate::day1practise::degree_convert::degree_convert;
    use crate::day1practise::fibonacci_number::fibonacci_number;

    #[test]
    fn should_0_fahrenheit_equal_neg_17_88_celsius() {
        assert_eq!(degree_convert(0.0, "FAHRENHEIT", "CELSIUS"), "-17.78");
    }

    #[test]
    fn should_1_fahrenheit_equal_neg_17_22_celsius() {
        assert_eq!(degree_convert(1.0, "FAHRENHEIT", "CELSIUS"), "-17.22");
    }

    #[test]
    fn should_1_celsius_equal_33_80_fahrenheit() {
        assert_eq!(degree_convert(1.0, "CELSIUS", "FAHRENHEIT"), "33.80");
    }

    #[test]
    fn should_fibonacci_number_5_return_5() {
        assert_eq!(fibonacci_number(5), 5);
    }
}
