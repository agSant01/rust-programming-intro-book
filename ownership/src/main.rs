fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let x = 66;
    let y = x;

    println!("x is: {} and y is: {}", x, y);

    let s1 = String::from("Hood");
    // incorrect
    let s2 = s1.clone();
    println!("s1: {} and s2: {}", s1, s2);

    let s = String::from("ownership Test");
    takes_ownership(s);
    // no longer available here
    // print!("{}", s);

    let x = 5;
    makes_copy(x);

    // giving ownership
    let s4 = gives_ownership();
    println!("{}", s4);

    // giving and taking
    let s5 = String::from("TakingBack");
    let s6 = takes_and_gives_back(s5);
    println!("Gave back s5: {}", s6);

    // sharing ownership temporarily
    let sharing = String::from("hello");
    let (returned, len) = calculate_length(sharing);
    println!("The length of '{}' is {}.", returned, len);

    // references
    let referenced = String::from("it's me Referenced");
    let len = calculate_length_wthreferenc(&referenced);
    println!("The length of '{}' is {}", referenced, len);

    // mutable references
    let mut mutable = String::from("mutable");
    // not possible
    // let m1 = &mut mutable;
    // let m2 = &mut mutable;

    // possible
    {
        let _m1 = &mut mutable;
    }
    let _m2 = &mut mutable;

    // let mut s = String::from("hello");
    // not possible
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;
    // println!("{},{}, and {}", r1, r2, r3);

    let mut s = String::from("hello mutable");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello giving ownership");
    some_string
}
// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_wthreferenc(s: &String) -> usize {
    s.len()
}
