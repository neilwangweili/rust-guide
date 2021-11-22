use rust_guide::day1practise::christmas_song::christmas_song;
use rust_guide::day1practise::degree_convert::degree_convert;
use rust_guide::day1practise::fibonacci_number::fibonacci_number;

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

#[test]
fn print_christmas_song() {
    let text: &str = "On the First day of Christmas
My good friends brought to me
A song and a Christmas tree.
On the Second day of Christmas
My good friends brought to me
Two candy canes
And a song for the Christmas tree.
On the Third day of Christmas
My good friends brought to me
Three boughs of holly
Two candy canes
And a song for the Christmas tree.
On the Fourth day of Christmas
My good friends brought to me
Four colored lights
Three boughs of holly
Two candy canes
And a song for the Christmas tree.
On the Fifth day of Christmas
My good friends brought to me
A shining star
Four colored lights
Three boughs of holly
Two candy canes
And a song for the Christmas tree.
On the Sixth day of Christmas
My good friends brought to me
Little silver bells
A shining star
Four colored lights
Three boughs of holly
Two candy canes
And a song for the Christmas tree.
On the Seventh day of Christmas
My good friends brought to me
Candles a-glowing
Little silver bells
A shining star
Four colored lights
Three boughs of holly
Two candy canes
And a song for the Christmas tree.
On the Eighth day of Christmas
My good friends brought to me
Gold and silver tinsel
Candles a-glowing
Little silver bells
A shining star
Four colored lights
Three boughs of holly
Two candy canes
And a song for the Christmas tree.
On the Ninth day of Christmas
My good friends brought to me
A guardian angel
Gold and silver tinsel
Candles a-glowing
Little silver bells
A shining star
Four colored lights
Three boughs of holly
Two candy canes
And a song for the Christmas tree.
On the Tenth day of Christmas
My good friends brought to me
Some mistletoe
A guardian angel
Gold and silver tinsel
Candles a-glowing
Little silver bells
A shining star
Four colored lights
Three boughs of holly
Two candy canes
And a song for the Christmas tree.
On the Eleventh day of Christmas
My good friends brought to me
Gifts for one and all
Some mistletoe
A guardian angel
Gold and silver tinsel
Candles a-glowing
Little silver bells
A shining star
Four colored lights
Three boughs of holly
Two candy canes
And a song for the Christmas tree.
On the Twelfth day of Christmas
My good friends brought to me
All their good wishes
Gifts for one and all
Some mistletoe
A guardian angel
Gold and silver tinsel
Candles a-glowing
Little silver bells
A shining star
Four colored lights
Three boughs of holly
Two candy canes
And a song for the Christmas tree.";
    assert_eq!(christmas_song(), text);
}
