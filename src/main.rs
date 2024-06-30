extern crate math_lib;

use math_lib::{add, cos, divide, multiply, power, sin, sqrt, subtract, tan, to_radians};

fn main() {
    println!("2 + 3 = {}", add(2.0, 3.0));
    println!("5 - 3 = {}", subtract(5.0, 3.0));
    println!("2 * 3 = {}", multiply(2.0, 3.0));
    println!("6 / 3 = {}", divide(6.0, 3.0).unwrap());
    println!("2^3 = {}", power(2.0, 3.0));
    println!("sqrt(9) = {}", sqrt(9.0).unwrap());
    println!("sin(30 degrees) = {}", sin(to_radians(30.0)));
    println!("cos(60 degrees) = {}", cos(to_radians(60.0)));
    println!("tan(45 degrees) = {}", tan(to_radians(45.0)));
}
