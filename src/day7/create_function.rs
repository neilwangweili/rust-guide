#[macro_export]
macro_rules! create_function {
    ($function_name:ident) => {
        fn $function_name() -> String {
            format!("You called {:?}()", stringify!($function_name))
        }
    };
}
