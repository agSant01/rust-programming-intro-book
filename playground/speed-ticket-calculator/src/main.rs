const SPEED_LIMIT: i32 = 60;
const EXCESSIVE_SPEED: i32 = 80;
const BIRTHDAY_INCREASE: i32 = 5;

#[derive(Debug)]
enum SpeedingTicket {
    NoTicket = 0,
    RegularTicket = 1,
    BigTicket = 2,
}

fn speeding_ticket(speed: i32, birthday: bool) -> SpeedingTicket {
    let speed_inc = if birthday { BIRTHDAY_INCREASE } else { 0 };

    if speed > EXCESSIVE_SPEED + speed_inc {
        SpeedingTicket::BigTicket
    } else if speed > SPEED_LIMIT + speed_inc {
        SpeedingTicket::RegularTicket
    } else {
        SpeedingTicket::NoTicket
    }
}

fn main() {
    println!("Hello, world!");

    let r = speeding_ticket(60, false);
    println!("{:?}", r);
    let r = speeding_ticket(65, false);
    println!("{:?}", r);
    let r = speeding_ticket(65, true);
    println!("{:?}", r);
}
