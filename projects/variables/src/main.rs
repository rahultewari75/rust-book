use std::io;

fn main() {
    // let x = 5;
    let mut x = 5;
    println!("x is {x}");

    x = 6;
    println!("x is {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");


    let y = 5;

    let y = y + 1;
    
    {
        let y = y * 2;
        println!("y is {y}");
    }

    println!("y is {y}");

    // let spaces = "   ";
    // let spaces = spaces.len();

    // let mut spaces = "   ";
    // spaces = spaces.len();

    let guess: u32 = "42".parse().expect("Not a number");
    // let guess = "42".parse().expect("Not a number");

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // -1

    // modulus
    let remainder = 43 % 5;

    println!("sum is {sum}");
    println!("difference is {difference}");
    println!("product is {product}");
    println!("quotient is {quotient}");
    println!("remainder is {remainder}");
    println!("truncated is {truncated}");


    let t = true;
    let f: bool = false;
    println!("t is {t}");
    println!("f is {f}");

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c is {c}");
    println!("z is {z}");
    println!("heart_eyed_cat is {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x is {x}");
    println!("y is {y}");
    println!("z is {z}");
    let zero = tup.0;
    println!("tup[0] is {zero}");

    let a = [1, 2, 3, 4, 5];
    println!("a[0] is {}", a[0]);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a[0] is {}", a[0]);

    let a = [3; 5];
    println!("a[0] is {}", a[0]);
    println!("a[4] is {}", a[4]);
    // println!("a[5] is {}", a[5]);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
