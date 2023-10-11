fn main() {
    println!("The twelve days of Christmas");
    println!();

    let phrases = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];

    let mut counter = 0;
    while counter < phrases.len() {
        println!("On the {} day of Christmas, my true love sent to me", days[counter]);
        for index in (0..counter+1).rev() {
            println!("{}", phrases[index])
        }
        counter += 1;
        println!()
    }
}
