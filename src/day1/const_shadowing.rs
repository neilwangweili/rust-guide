pub fn const_shadowing(x: i32) -> i32 {
    // This is a const.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    //这个程序首先将 x 绑定到值 5 上。接着通过 let x = 隐藏 x，获取初始值并加 1，这样 x 的值就变成 6 了。
    // 注意，传入的x不是mut的，为什么仍可以修改值呢？因为运用了[隐藏]特性。
    // 例如：
    // Correct:
    // let spaces = "   ";
    // let spaces = spaces.len();

    // Error:
    // let mut spaces = "   ";
    // spaces = spaces.len(); [类型不符合，将i32类型赋值给str类型会报错]
    print!("{}", THREE_HOURS_IN_SECONDS);
    let x = x + 1;
    {
        //然后，在内部作用域内，第三个 let 语句也隐藏了 x，将之前的值乘以 2，x 得到的值是 12。
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    // 当该作用域结束时，内部 shadowing 的作用域也结束了，x 又返回到 6
    x
}
