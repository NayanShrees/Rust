const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Won't work with immutable variables
    //     let x = 5;
    //     println!("The value of x is {x}");
    //     x = 6;
    //     println!("The value of x is {x}");

    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // Constants
    println!("The value of a constant THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x is {x}");

    // Tuples
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (_, y, _) = tuple;

    println!("The value of index 0 is: {}", tuple.0);
    println!("The value of y is: {y}");
    println!("The value of index 2 is: {}", tuple.2);

    // Arrays
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

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
