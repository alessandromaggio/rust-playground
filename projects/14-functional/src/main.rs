use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn closures() {
    let store = Inventory{
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);

    let add_one_closure = |x: i32| x + 1;
    println!("The closure adds one to 5: {}", add_one_closure(5));

    let mut list = vec![1, 2, 3];
    println!("Before calling closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    // This forces move of ownership to the closure even though it is not strictly needed, a way to force transfer of ownership
    thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();

    let mut list = [
        Rectangle{ width: 10, height: 1 },
        Rectangle{ width: 3, height: 5 },
        Rectangle{ width: 7, height: 12 },
    ];

    let mut sort_operations = 0;

    list.sort_by_key(|r| {
        sort_operations += 1;
        r.width
    });
    println!("Sorted by width: {:#?} in {} operations", list, sort_operations);
}

fn iterators() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let mut v1_iter = v1.iter();
    // Iterator trait requires next method to be implemented
    // Calling next changes internal state and hence iterator must be mutable
    // No need when using for loop as it takes care of mutability internally
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
}

fn main() {
    closures();
    iterators();
}
