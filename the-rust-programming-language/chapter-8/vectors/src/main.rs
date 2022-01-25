fn main() {
    let v: Vec<i32> = Vec::new();

    // vec! macro for convinience
    // the compiler infers the i32 data type
    let mut v = vec![1, 2, 3];

    let mut v = Vec::new();

    // in order to push to a vector it needs to be mutable
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v2 = vec![1, 2, 3, 4];

        // do stuff with this v
        // scope of inner v
    } // <- v goes out of scope and is freed

    // Reading Elements of Vectors
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[3];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // will panic at runtime, because index is not valid.
    // let does_not_exist = &v[100];

    // will not panic at runtime, but return an Option::None
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // will not compile
    // trying to mutate v while holding an inmutable reference to one
    // of its element
    // and then trying to use it afer
    // v.push(6);

    println!("The first elemt is: {}", first);

    // Iterating over vec[] without mutating items
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 56];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    // Using an Enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
