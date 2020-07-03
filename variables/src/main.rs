const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;

    println!("The value of x is: {}", x);

    x = 6;

    println!("The value of x is: {}", x);

    println!("the constant is: {}", MAX_POINTS);

    // second
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    // not allowed
    // let mut spaces = "   ";
    // spaces = spaces.len();
    println!("Spaces: {}", spaces);

    let _x = 2.0;

    let _y: f32 = 3.0;

    //	addition
    let _sum = 5 + 10;
    //	subtraction
    let _difference = 95.5 - 4.3;
    //	multiplication
    let _product = 4 * 30;
    //	division
    let _quotient = 56.7 / 32.2;
    //	remainder
    let __remainder = 43 % 5;

    let _t = true;

    let _f: bool = false;

    let _tup: (i32, f64, u8) = (54, 65.0, 1);
    let tup = (54, 5.4, 43);
    // tupple destructure
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let tup1st_val = tup.0;
    let tup2nd_val = tup.1;

    println!("The tupple first value is: {}", tup1st_val);
    println!("The tupple second value is: {}", tup2nd_val);

    // Arrays
    let _arr = [1, 2, 3, 4, 4, 5, 67, 8];

    let a: [i32; 3] = [31, 23, 41];

    println!("First: {}", a[0]);
    println!("Second: {}", a[1]);

    // before compile error
    // println!("Second: {}", a[4]);

    another_function();
}

fn another_function() {
    println!("Another Function");
}
