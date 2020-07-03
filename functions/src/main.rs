fn main() {
    another_function(34);
    let _x = 5;

    // macro
    let y = {
        let x = 3;
        x + 1
        // returns 3 + 1 as final value
    };

    println!("The value of y is: {}", y);

    println!("the return value of function five() is: {}", five());

    println!("The value of plus_one(23) is: {}", plus_one(23));
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
