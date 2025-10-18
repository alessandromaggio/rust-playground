use std::slice;
use std::ops::Add;

unsafe fn dangerous() {
    println!("This is unsafe code!");
}

// Take a vec of integers and split into two slices at the given index (before and after)
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    // This is how we ensure the midpoint is actually valid
    assert!(mid <= len);

    // This is unsafe because the compiler wouldn't let us borrow the same slice twice normally
    // This is because the compiler does not understand we are borrowing two different parts that are non-overlapping
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn unsafe_rust() {
    // Using raw pointers
    let mut num = 5;
    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        dangerous();

        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    // A safe abstraction wrapping unsafe code
    let (a, b) = split_at_mut(&mut v, 3);
    println!("First part: {:?}", a);
    println!("Second part: {:?}", b);

}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    // Associated type set to a specific value, this associated type is defined by the Add trait
    // We can potentially add Point to other types if we wanted to by changing this associated type (for return) and use impl Add<OtherType> for Point
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn operator_overloading() {
    assert_eq!(
        Point { x: 1, y: 2 } + Point { x: 3, y: 4 },
        Point { x: 4, y: 6 }
    );
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn return_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn advanced_functions() {
    let result = do_twice(add_one, 5);
    println!("Result: {}", result);

    let closure_result = return_closure();
    println!("Closure Result: {}", closure_result(10));
}

#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[derive(HelloMacro)]
struct Pancakes {}

fn macros() {
    let v = my_vec![1, 2, 3, 4, 5];
    println!("My vec: {:?}", v);
    Pancakes::hello_macro();
}

fn main() {
    macros();
    advanced_functions();
    operator_overloading();
    unsafe_rust();
}
