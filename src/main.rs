mod find_root_bracketing;
mod find_root_combination;
mod find_root_iterative;

//use find_root_bracketing::{Bisection, FalsePosition, ITP};
//use find_root_iterative::{Secant, Newton, Steffensen};
use std::io::{stdin, Error};
use std::process::Command;

fn main() {
    if let Err(e) = clear_console() {
        eprintln!("Error clearing console: {}", e);
    }
    let mut input = String::new();

    menu();
    if let Err(e) = stdin().read_line(&mut input) {
        eprintln!("Error al leer la línea: {}", e);
        return;
    }

    let opt = input.trim();

    loop {
        match opt {
            "1" => {
                println!("1");
                break;
            }
            "2" => {
                println!("2");
                break;
            }
            "3" => {
                println!("3");
                break;
            }
            "4" => {
                println!("4");
                break;
            }
            _ => {
                println!("_");
                break;
                todo!()
            }
        }
    }
}

fn menu() {
    println!("::: :::--- ROOT-FINDING ALGORITHMS ---::: :::");
    println!("::: :::--- EXAMPLES ---::: :::");
    println!("::: ::: BRACKETING METHODS ::: :::");
    println!("1. Bisection");
    println!("2. False position");
    println!("3. ITP Method");
    println!("::: ::: ITERATIVE METHODS ::: :::");
    println!("4. Newton's method ");
    println!("5. Secant method");
    println!("6. Steffensen's method");
    println!("7. Fixed point iteration method");
    println!("8. Inverse quadratic interpolation");
    println!("::: ::: COMBINATION OF METHODS ::: :::");
    println!("9. Brent's method");
    println!("10. Ridder's method");
    println!("--- --- --- --- ---");
    println!("[0] EXIT");
    println!("--- --- --- --- ---");
    println!("--- --- --- --- ---");
    println!("--- --- --- --- ---");
    println!("Select an option: ");
}



fn clear_console() -> Result<(), Error> {
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/c").arg("cls").status()?;
    } else {
        Command::new("clear").status()?;
    }
    Ok(())
}
