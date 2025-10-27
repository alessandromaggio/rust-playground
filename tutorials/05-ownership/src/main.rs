fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // This will print `hello, world!`

    let mut s2 = s; // s is moved to s2. s is no longer valid here.
    println!("{s2}"); // This will print `hello, world!`
    s2 = String::from("hello, Rust!"); // s2 is reassigned to a new String. The original string is dropped
    println!("{s2}"); // This will print `hello, Rust!`
    take_ownership(s2); // s2 is moved to the function. s2 is no longer valid here.
    let mut s2 = gives_ownership();
    println!("Length of '{}' is {}", s2, calculate_length(&s2)); // This will print `hello`
    change(&mut s2);
    println!("After change: {s2}");
    println!("First word: {}", first_world(&s2));
}

fn take_ownership(some_string: String) {
    // some_string comes into scope
    println!("Taking ownership of: {some_string}");
} // some_string goes out of scope and `drop` is called. The backing memory is freed

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(" [changed]");
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}
