use std::fmt::{ Display, Debug };

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

pub fn notify_no_sugar<T: Summary>(item: &T) {
    println!("Breaking news: {}", item.summarize());
}

pub fn notify_two_bounds(item: &(impl Summary + Display)) {
    println!("Breaking news: {}; as Display: {}", item.summarize(), item);
}

pub fn notify_two_bounds_no_sugar<T: Summary + Display>(item: &T) {
    println!("Breaking news: {}; as Display: {}", item.summarize(), item);
}

#[allow(unused)]
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

#[allow(unused)]
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you already know, people"),
        reply: false,
        repost: false
    }
}

#[allow(unused)]
struct Pair<T> {
    x: T,
    y: T
}

#[allow(unused)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[allow(unused)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x)
        }
        else {
            println!("The largest member is y = {}", self.y)
        }
    }
}