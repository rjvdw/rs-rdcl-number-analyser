//! Does some basic analysis on numbers.

use std::{env, process};

use crate::number::Number;

mod number;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <nr>", args[0]);
        process::exit(1);
    }

    let number = match args[1].parse::<Number>() {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Unable to parse number: {}", e);
            process::exit(1);
        }
    };

    write_as_powers_of_two(number);
    find_closest_powers_of_two(number);
}

/// Write this number as the sum of powers of two.
///
/// For example:
/// * 5 = 1 + 4
/// * 15 = 1 + 2 + 4 + 8
/// * 32 = 32
/// * -3 = -(1 + 2)
fn write_as_powers_of_two(mut number: Number) {
    print!("This number can be written as: ");
    if number.is_negative {
        print!("-(");
    }
    let mut first = true;
    let mut power: u128 = 1;
    while number.value != 0 {
        if number.value % 2 == 1 {
            if !first {
                print!(" + ");
            }
            first = false;
            print!("{}", power);
        }
        power <<= 1;
        number.value >>= 1;
    }
    if number.is_negative {
        print!(")");
    }
    println!();
}

/// Find nearby powers of two.
fn find_closest_powers_of_two(number: Number) {
    if number.value.is_power_of_two() {
        println!("The absolute value of this number is a power of two");
    } else {
        let largest_power_of_two = u128::MAX / 2 + 1;
        if number.value > largest_power_of_two {
            println!(
                "The nearest power of two (without overflowing) is {}",
                largest_power_of_two
            );
        } else {
            let next_power_of_two = number.value.next_power_of_two();
            println!("The next power of two is: {}", next_power_of_two);
            println!("The previous power of two is: {}", next_power_of_two / 2);
        }
    }
}
