fn main() {
    let header = "On the ";
    let footer = " day of Christmas my true love sent to me";
    let ordinals = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let items = [
        " partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];
    println!("{header}{}{footer}", ordinals[0]);
    println!("A{}", items[0]);
    for i in 1..=11 {
        println!("{header}{}{footer}", ordinals[i]);
        for j in (0..=i).rev() {
            if j == 0 {
                println!("And a{}", items[j]);
                continue;
            }
            println!("{}", items[j]);
        }
    }
}
