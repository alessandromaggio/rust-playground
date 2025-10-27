fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// This implementation is only for Point<f32>
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn generics() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    println!("The x value of integer point is {}", integer.x());
    println!(
        "The distance from origin of float point is {}",
        float.distance_from_origin()
    );
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
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
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// For two traits use +
// pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify_shorthand(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// For two traits use +
// pub fn notify<T: Summary + Display>(item: &T) {}
pub fn notify_longer<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds with where clauses
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: std::fmt::Display + std::fmt::Debug,
    U: std::fmt::Debug + std::clone::Clone,
{
    println!("t is {}", t);
    1
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// This implementation is only for types that implement PartialOrd and Display
impl<T: std::cmp::PartialOrd + std::fmt::Display> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Implement the Summary trait for any type that implements Display
// Blanket implementation
impl<T: std::fmt::Display> Summary for Pair<T> {
    fn summarize(&self) -> String {
        format!("Pair({}, {})", self.x, self.y)
    }
}


fn traits() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the top \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };
    println!("1 new post: {}", post.summarize());

    notify_longer(&article);
    notify_shorthand(&post);
}

// &i32 a reference
// &'a i32 a reference with an explicit lifetime
// &'a mut i32 a mutable reference with an explicit lifetime
// Here we tell the compiler that return value depends on the lifetimes of both parameters, i.e. can borrow of either
// In this case, it means the compiler will assume the shortest lifetime of the two parameters (worst possible case)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // Here the lifetime of the return value is tied to the lifetime of the struct instance
    // i.e. lifetime of self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Static lifetime lives for the entire duration of the program


fn lifetimes() {
    let s:&'static str = "I have a static lifetime";
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("The first sentence is {}", i.part);


}

fn main() {
    generics();
    traits();
    lifetimes();
}
