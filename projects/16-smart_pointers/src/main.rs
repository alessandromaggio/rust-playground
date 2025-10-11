use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

// Custom box that actually stores the value on the stack, not the heap
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl <T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox");
    }
}

use crate::List::{Cons, Nil};

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    let list = Cons(Rc::new(RefCell::new(1)), Rc::new(Cons(Rc::new(RefCell::new(2)), Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::new(Nil))))));

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); // If we want to assert on the value that y is pointing to, we must use the dereference operator

    let y = Box::new(5); // This set y to a pointer to a copy of x on the heap
    assert_eq!(5, *y); // We can also dereference a Box<T> in the same way

    let y = MyBox::new(5);
    assert_eq!(5, *y); // This works because MyBox<T> implements the Deref trait

    let d = MyBox::new(String::from("Rust"));
    drop(d); // Manually calling drop before the variable goes out of scope

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // This works because of deref coercion, which automatically converts &MyBox<String> to &String, then to &str

    let a = Rc::new(Cons(Rc::new(RefCell::new(5)), Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil)))));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)); // Not a real clone, just increments the reference count
    println!("Count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        println!("Count after creating c in the inner scope = {}", Rc::strong_count(&a));
    }
    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    println!("Count after creating c in the outer scope = {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // Creates a weak reference to branch from child to parent
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // upgrade() converts a Weak<T> to an Option<Rc<T>>
    println!("branch = {:?}", branch);
    println!("leaf = {:?}", leaf);

}
