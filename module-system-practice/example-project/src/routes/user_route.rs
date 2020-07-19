// use crate::models::user_model::print_user_model;
use crate::models::user_model::print_user_model as log_user_model;

pub fn print_user_route() {
    // print_user_model();

    // by using the renaming
    log_user_model();

    // lengthy way
    // crate::routes::health_route::print_health_route();

    // short way
    // super::health_route::print_health_route();

    println!("user_route");
}
