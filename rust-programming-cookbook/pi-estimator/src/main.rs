// declaring rounding module
mod rounding;
use printer::pretty_print_pi_approx;

fn main() {
    pretty_print_pi_approx(100_000);
}

mod printer {
    // import a function from an external crate (no more extern decelaration required)
    use rust_plib::monte_carlo_pi;

    // internal crate
    use crate::rounding::round;

    pub fn pretty_print_pi_approx(iterations: usize) {
        let pi = monte_carlo_pi(iterations);
        let places: usize = 2;
        println!(
            "Pi is ~ {} and rounded to {} places {}",
            pi,
            places,
            round(pi, places)
        );
    }
}
