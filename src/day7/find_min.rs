#[macro_export]
macro_rules! find_min {
    ($a :expr) => {
        $a
    };
    ($a: expr, $b: expr) => {
        if $a <= $b { $a } else { $b }
    };
    ($a: expr, $b: expr, $($c: expr),+) => {
        find_min!(find_min!($a, $b), $($c),+);
    };
}
