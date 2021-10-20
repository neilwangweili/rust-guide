pub fn christmas_song() -> String {
    let christmas_array = [
        [String::from("First"), String::from(""), String::from("A song and a Christmas tree.\n")],
        [String::from("Second"), String::from("Two candy canes\n"), String::from("And a song for the Christmas tree.\n")],
        [String::from("Third"), String::from("Three boughs of holly\n"), String::from("And a song for the Christmas tree.\n")],
        [String::from("Fourth"), String::from("Four colored lights\n"), String::from("And a song for the Christmas tree.\n")],
        [String::from("Fifth"), String::from("A shining star\n"), String::from("And a song for the Christmas tree.\n")],
        [String::from("Sixth"), String::from("Little silver bells\n"), String::from("And a song for the Christmas tree.\n")],
        [String::from("Seventh"), String::from("Candles a-glowing\n"), String::from("And a song for the Christmas tree.\n")],
        [String::from("Eighth"), String::from("Gold and silver tinsel\n"), String::from("And a song for the Christmas tree.\n")],
        [String::from("Ninth"), String::from("A guardian angel\n"), String::from("And a song for the Christmas tree.\n")],
        [String::from("Tenth"), String::from("Some mistletoe\n"), String::from("And a song for the Christmas tree.\n")],
        [String::from("Eleventh"), String::from("Gifts for one and all\n"), String::from("And a song for the Christmas tree.\n")],
        [String::from("Twelfth"), String::from("All their good wishes\n"), String::from("And a song for the Christmas tree.")]
    ];
    let mut result = String::new();
    for i in 1..=12 {
        result += &(String::from("On the ") + &christmas_array[i - 1][0] + &String::from(" day of Christmas\n\
        My good friends brought to me\n"));
        result += &calculate_gift(&christmas_array, i);
        result += &christmas_array[i - 1][2];
    }
    return result;
}

fn calculate_gift(christmas_array: &[[String; 3]; 12], i: usize) -> String {
    let mut j = i;
    let mut gift = String::new();
    let gifts = loop {
        gift += &christmas_array[j - 1][1];
        j -= 1;
        if j == 0 {
            break gift;
        }
    };
    gifts
}
