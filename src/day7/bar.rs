#[macro_export]
macro_rules! bar {
    ($v: ident) => {
        let $v = 3;
    };
}
