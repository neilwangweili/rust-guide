#[macro_export]
macro_rules! hash {
    ($($key: expr => $value: expr),*) => {
        {
            let mut map = HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}
