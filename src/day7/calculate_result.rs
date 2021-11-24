#[macro_export]
macro_rules! calculate_result {
    // block
    // expr 用于表达式
    // ident 用于变量名或函数名
    // item
    // pat (模式 pattern)
    // path
    // stmt (语句 statement)
    // tt (标记树 token tree)
    // ty (类型 type)
    ($expression: expr) => {
        $expression
    }
}
