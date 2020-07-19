use rand::random;

pub fn print_health_route() {
    // let random_number: u8 = rand::random();
    let random_number: u8 = random();

    println!("{}", random_number);
    println!("health_route");
}
