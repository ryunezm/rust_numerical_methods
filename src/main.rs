use std::io::{Error, stdin};
use std::process::Command;

mod find_roots;

fn main() {

    if let Err(e) = clear_console() {
        eprintln!("Error clearing console: {}", e);
    }
    menu_principal();

    let mut input = String::new();

    if let Err(e) = stdin().read_line(&mut input) {
        eprintln!("Error al leer la línea: {}", e);
        return;
    }

    let opt = input.trim();

    loop {
        match opt {
            "1" => {
                menu_sle();
                break;
            }
            "2" => {
                menu_find_root();
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

fn menu_principal() {
    println!("::: ::: ::: ::: NUMERICAL METHODS: EXAMPLES ::: ::: ::: :::");
    println!("1. Solving systems of linear equations");
    println!("2. Root finding algorithms");
    println!("3. Quasi-Newton methods");
    println!("[0] EXIT");
    println!("::: ::: ::: ::: ::: :::");
    println!("Select an option: ");
}

fn menu_sle(){
    println!("::: :::--- SOLVING SYSTEMS OF LINEAR EQUATIONS ::: ::: :::");
    println!("1. Jacobi iteration method");
    println!("2. Gauss-Jordan method");
    println!("3. Gauss-Seidel method");
    println!("4. Successive over-relaxation method (SOR)")
}
fn menu_find_root(){
    println!("::: ::: ::: ROOT-FINDING ALGORITHMS ::: ::: :::");
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
}

fn menu_quasi_newton_method(){
    println!("::: ::: ::: QUASI-NEWTON METHODS ::: ::: :::");
    println!("1. Broyden's method");
    println!("2. Broyden–Fletcher–Goldfarb–Shanno (BFGS) algorithm");
    println!("3. Limited-memory BFGS (LM-BFGS) algorithm");
    println!("4. Davidon–Fletcher–Powell (DFP) formula");
    println!("5. Symmetric rank-one")
}

fn clear_console() -> Result<(), Error> {
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/c").arg("cls").status()?;
    } else {
        Command::new("clear").status()?;
    }
    Ok(())
}
