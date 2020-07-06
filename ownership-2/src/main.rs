fn main() {
    // slice type
    let s = String::from("Hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];

    let fst = first_word(&s);
    // s.clear();  Error
    println!("first word is: {}", fst);

    // array slices
    let a = [1, 2, 3, 4, 5, 6];
    let a_slice = &a[1..3];
    println!("Arrray slice: {:?}", a_slice);
}

// takes a string a returns the first word it finds in that string
// fn first_word(s: &String) -> &str {
// improved version can use slices as well
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
