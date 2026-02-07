mod spherical;

use std::io::{self, Write};
use std::process;

use spherical::{Polar, Rectangular};

fn clean_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn show_menu() -> i32 {
    let mut choice: i32 = 0;

    while choice < 1 || choice > 3 {
        clean_screen();

        println!("MENU\n");
        println!("1) Rectangular to Spherical");
        println!("2) Spherical to Rectangular");
        println!("3) Exit\n");
        print!("Enter choice: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        match input.trim().parse::<i32>() {
            Ok(n) => choice = n,
            Err(_) => choice = 0,
        }
    }

    choice
}

fn get_value(prompt: &str) -> f64 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<f64>().unwrap_or(0.0)
}

fn evaluate_rectangular() {
    clean_screen();

    println!("RECTANGULAR → SPHERICAL\n");

    let x = get_value("Enter x: ");
    let y = get_value("Enter y: ");
    let z = get_value("Enter z: ");

    let r = Rectangular::new(x, y, z);
    let p = r.to_spherical();

    println!("\n{}", r);
    println!("\n{}", p);

    pause();
}

fn evaluate_spherical() {
    clean_screen();

    println!("SPHERICAL → RECTANGULAR\n");

    let r = get_value("Enter r: ");
    let theta = get_value("Enter theta (radians): ");
    let phi = get_value("Enter phi (radians): ");

    let p = Polar::new(r, theta, phi);
    let r2 = p.to_rectangular();

    println!("\n{}", p);
    println!("\n{}", r2);

    pause();
}

fn pause() {
    println!("\nPress ENTER to continue...");
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();
}

fn main() {
    loop {
        match show_menu() {
            1 => evaluate_rectangular(),
            2 => evaluate_spherical(),
            3 => process::exit(0),
            _ => {}
        }
    }
}
