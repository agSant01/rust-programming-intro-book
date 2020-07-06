fn main() {
    let number = 7;

    if number < 5 {
        println!("Condition was true!");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    // if in let
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // error
    // let number = if condition { 5 } else { "six" };

    println!("the value of number is: {}", number);
}
