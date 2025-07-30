fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("y is {y}");

    let x = five();
    println!("x is {x}");

    let x = plus_one(five());
    println!("x is {x}");

    control_flow();

    let f = 68.0;
    let c = convert_f_to_c(f);
    println!("f is {f} and c is {c}");

    let c = 20.0;
    let f = other_way_around(c);
    println!("c is {c} and f is {f}");

    let n = 40;
    let result = fib(n);
    println!("fib({n}) is {result}");

    let n = 40;
    let result = fib_but_faster(n);
    println!("fib_but_faster({n}) is {result}");
}

fn another_function(x: i32) {
    println!("Do NOT fuck with me. I will cry.");
    println!("x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

/*
bruh
*/
// what
// the heck
// i thought this language had no multi line comments

fn plus_1(x: i32) -> i32 {
    return x + 1;
}

fn control_flow() {
    let nubmer = 6;

    if nubmer < 5 {
        println!("condition was true");
    } else if nubmer % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    }

    let condition = true;
    let big_number = if condition { 5 } else { 6 };
    println!("big_number is {big_number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        println!("again!");
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result is {result}");

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
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("Litty!!!");
    
    
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("Litty2!!!");

    let fake_a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", fake_a[index]);
        index += 1;
    }

    // while index < 6 {
    //     println!("the value is: {}", fake_a[index]);
    //     index += 1;
    // }

}

fn convert_f_to_c(f: f64) -> f64 {
    (f - 32.0) * (5.0/9.0)
}
fn other_way_around(c: f64) -> f64 {
    (c * (9.0/5.0)) + 32.0
}

fn fib(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

fn fib_but_faster(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}