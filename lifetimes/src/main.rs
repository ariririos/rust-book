use std::fmt::Display;

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where T: Display {
    println!("Announcement: {}", ann);
    if x.len() > y.len() { x } else { y }
}
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Announcement: {}", announcement);
        self.part
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let str1 = String::from("Short string");
    let mut result: &str;
    {
        let str2 = "longer string";
        result = longest(str1.as_str(), str2);
    }
    println!("Longer string is {}", result);
    let str3 = "testing testing 123";
    result = longest(str1.as_str(), str3);
    println!("New longer string is {}", result);
    let str4 = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = str4.split('.').next().unwrap();
    // drop(str4);
    let i = ImportantExcerpt {
        part: first_sentence
    };
    println!("struct str: {}", i.part);
    let part = i.announce_and_return_part("test");
    println!("struct str again: {}", part);
    let longest = longest_with_announcement(&str1, str3, "testing");
    println!("Longest between str1 and str3 is: {}", longest);
}
