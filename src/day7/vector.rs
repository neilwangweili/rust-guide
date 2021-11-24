#[macro_export]
macro_rules! vector {
    ($($arg: expr),*) => {
        {
            let mut result = Vec::new();
            $(result.push($arg);)*
            result
        }
    };
}
