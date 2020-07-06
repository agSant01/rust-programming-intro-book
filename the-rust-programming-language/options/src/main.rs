fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    println!("{:?}", absent_number.or(some_number));

    // wont compile
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;
}
