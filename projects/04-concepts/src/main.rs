fn main() {
    // Shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Scalar Types
    let a: u32 = 42_200; // Unsigned, equal to 42200 but easier to read
    let b: f64 = 3.1415; // 64-bit floating point, default for floating points

    // Operations
    let sum = 5 + 10;
    let difference = 95.5 - 14.2;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    let remainder = 43 % 5;

    // Boolean Type
    let boolean: bool = true;

    // Character Type - use single quote, 4 bytes in size
    let character: char = 'z';
    let emoji: char = '😊';

    // Tuple Type
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple;
    println!("The value of y is: {y}");
    println!("The first value of the tuple is: {}", tuple.0);

    let result = function_with_parameters(5, 6);
    println!("The result of the function is: {result}");

    if_else_demo();
    conditional_assignment();
    loop_demo();
    multiple_loops_demo();
    while_demo();
    array_loop();
    range_demo();
}

fn array_demo() {
    // Array Type - fixed size, allocated to stack
    // Use when you know the number of elements will not change
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let array_of_three = [3; 5]; // same as [3, 3, 3, 3, 3]
    let first = array[0];
    let second = array[1];

    println!("Insert the index of the array element you want to access: ");
    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = array[index];
    println!("The value of the element at index {index} is: {element}");
}

fn function_with_parameters(x: i32, y: i32) -> i32 {
    x + y // No semicolon means this expression is returned
}

fn if_else_demo() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}

fn conditional_assignment() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

fn loop_demo() {
    let mut count = 0;
    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2; // Break with a value
        }
    };
    println!("The result is {result}");
}

fn multiple_loops_demo() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}

fn while_demo() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
}

fn array_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn range_demo() {
    // Reverse range, this is: 3, 2, 1
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}