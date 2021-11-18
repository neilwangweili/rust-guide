pub fn largest<T>(list: &[T]) -> T where T: PartialOrd + Copy {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
