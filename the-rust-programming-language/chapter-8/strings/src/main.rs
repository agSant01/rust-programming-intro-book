#![allow(unused_variables)]

fn main() {
    // creating empty string
    let s = String::new();

    // creating a String from a string literal
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    // equivalent to above
    let s = String::from("initial content");

    // updating a string
    let mut s = String::from("initial content");
    s += ", mordvsse content";
    println!("{}", s);

    // Appending to a string
    let mut s = String::from("initial content");
    s.push_str(", more");

    // example of passing a slice to push_str
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    s1 += s2;
    println!("s2 is {}", s2);
    println!("s1 is {}", s1);

    // push only takes a char
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    // concatenation with + and format! (macro)
    let s1 = String::from("Hello, ");
    let s2 = String::from(" world!");
    let s3 = s1 + &s2;

    // we cant use s1 after the line above because we passed s1 and not a reference
    // println!("{}", s1); // compile error, s1 is moved

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{} and {} and {}", s1, s2, s3);
    println!("{}", s);

    // indexing into strings
    let s1 = String::from("hello");
    // let h = s1[0]; // error

    let s = String::from("Здравствуйте");
    println!("Здравствуйте and {}", s);
}
