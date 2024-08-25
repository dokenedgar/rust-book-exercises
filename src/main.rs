fn main() {
    let carol_array: [&str; 12] = [
        "And a partridge in a pear tree!",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese-a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming",
    ];
    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let mut counter: usize = 0;
    for x in 0..12 {
        let mut carol: String = String::from("On the ");
        carol.push_str(days[x]);
        carol.push_str(" day of Christmas,\nmy true love gave to me");
        counter = 0;
        for index in (0..x + 1).rev() {
            let carol_line: &str = carol_array[index];
            carol.push_str("\n");
            carol.push_str(carol_line);
        }
        println!("{carol}\n");
    }
}
