use std::io::{Error, stdin};
use crate::find_roots::find_root_bracketing::{Bisection, FalsePosition, ITP, FindRoot as FRB};
use crate::find_roots::find_root_iterative::{Newton, Secant, Steffensen, FixedPoint, InverseQuadraticInterpolation, FindRoot as FRI};
use crate::find_roots::find_root_combination::{Brent, Ridder, FindRoot as FRC};
use crate::sodes::sode_explicit_methods::{Euler, RK2, RK4, Functions };
use crate::view::_clear_menu::{clear_menu, clear_console};

pub fn menu() {
    if let Err(e) = clear_console() { eprintln!("Error clearing console: {}", e); }
    menu_principal();
}

fn menu_principal() {
    if let Err(e) = clear_console() { eprintln!("Error clearing console: {}", e); }
    println!("::: ::: ::: ::: NUMERICAL METHODS: EXAMPLES ::: ::: ::: :::");
    println!("1. Solving systems of linear equations");
    println!("2. Root finding algorithms");
    println!("3. Quasi-Newton methods");
    println!("4. Solution of Ordinary Differential Equations (SODE)");
    println!("[0] EXIT");
    println!("::: ::: ::: ::: ::: :::");
    println!("Select an option: ");
    let mut input = String::new();

    if let Err(e) = stdin().read_line(&mut input) {
        eprintln!("Error al leer la línea: {}", e);
        return;
    }

    let opt = input.trim();

    loop {
        match opt {
            "1" => { menu_sle(); break; }
            "2" => { menu_find_root(); break; }
            "3" => { menu_quasi_newton_method(); break; }
            "4" => { menu_sode(); break; }
            "0" => { break; }
            _ => { menu_principal(); break; }
        }
    }
}

fn menu_sle() {
    if let Err(e) = clear_console() { eprintln!("Error clearing console: {}", e); }
    println!("::: :::--- SOLVING SYSTEMS OF LINEAR EQUATIONS ::: ::: :::");
    println!("1. Jacobi iteration method");
    println!("2. Gauss-Jordan method");
    println!("3. Gauss-Seidel method");
    println!("4. Successive over-relaxation method (SOR)");
    println!("[0] BACK");
    println!("::: ::: ::: ::: ::: :::");
    println!("Select an option: ");

    let mut input = String::new();
    if let Err(e) = stdin().read_line(&mut input) {
        eprintln!("Error al leer la línea: {}", e);
        return;
    }

    let opt = input.trim();

    loop {
        match opt {
            "1" => { println!("Choose Jacobi"); break; }
            "2" => { println!("Choose G-J"); break; }
            "3" => { println!("Choose G-S"); break; }
            "4" => { println!("Choose SOR"); break; }
            "0" => { menu_principal(); break; }
            _ => { menu_sle(); break; }
        }
    }
}

fn menu_find_root() {
    if let Err(e) = clear_console() { eprintln!("Error clearing console: {}", e); }
    println!("::: ::: ::: ROOT-FINDING ALGORITHMS ::: ::: :::");
    println!("::: BRACKETING METHODS :::");
    println!("1. Bisection");
    println!("2. False position");
    println!("3. ITP Method");
    println!("::: ITERATIVE METHODS :::");
    println!("4. Newton's method ");
    println!("5. Secant method");
    println!("6. Steffensen's method");
    println!("7. Fixed point iteration method");
    println!("8. Inverse quadratic interpolation");
    println!("::: COMBINATION OF METHODS :::");
    println!("9. Brent's method");
    println!("10. Ridder's method");
    println!("[0] BACK");
    println!("::: ::: ::: ::: ::: :::");
    println!("Select an option: ");
    let mut input = String::new();
    if let Err(e) = stdin().read_line(&mut input) {
        eprintln!("Error al leer la línea: {}", e);
        return;
    }

    let opt = input.trim();

    loop {
        match opt {
            "1" => { clear_menu(); Bisection::info(); break; }
            "2" => { clear_menu(); FalsePosition::info(); break; }
            "3" => { clear_menu(); ITP::info(); break; }
            "4" => { clear_menu(); Newton::info(); break }
            "5" => { clear_menu(); Secant::info(); break }
            "6" => { clear_menu(); Steffensen::info(); break }
            "7" => { clear_menu(); FixedPoint::info(); break }
            "8" => { clear_menu(); InverseQuadraticInterpolation::info(); break }
            "9" => { clear_menu(); Brent::info(); break; }
            "10" => { clear_menu(); Ridder::info(); break; }
            "0" => { menu_principal(); break; }
            &_ => { menu_find_root(); break; }
        }
    }
}

fn menu_quasi_newton_method() {
    if let Err(e) = clear_console() { eprintln!("Error clearing console: {}", e); }
    println!("::: ::: ::: QUASI-NEWTON METHODS ::: ::: :::");
    println!("1. Broyden's method");
    println!("2. Broyden–Fletcher–Goldfarb–Shanno (BFGS) algorithm");
    println!("3. Limited-memory BFGS (LM-BFGS) algorithm");
    println!("4. Davidon–Fletcher–Powell (DFP) formula");
    println!("5. Symmetric rank-one");
    println!("[0] BACK");
    println!("::: ::: ::: ::: ::: :::");
    println!("Select an option: ");
    let mut input = String::new();
    if let Err(e) = stdin().read_line(&mut input) {
        eprintln!("Error al leer la línea: {}", e);
        return;
    }

    let opt = input.trim();
    loop {
        match opt {
            "1" => { todo!() }
            "2" => { todo!() }
            "3" => { todo!() }
            "4" => { todo!() }
            "5" => { todo!() }
            "0" => { menu_principal(); break; }
            &_ => { menu_quasi_newton_method(); break; }
        }
    }
}

fn menu_sode(){
    println!("::: ::: ::: SOLUTION OF ODEs ::: ::: :::");
    println!("::: EXPLICIT METHODS :::");
    println!("1. Euler's method ");
    println!("2. Runge-Kutta-2 method (Midpoint method)");
    println!("3. Runge-Kutta-4 method (Classical fourth order method)");
    println!("::: IMPLICIT METHODS :::");
    println!("4. Implicit Euler's method");
    println!("5. Backward Euler's method");
    println!("6. Crank–Nicolson method");
    println!("::: MULTISTEP METHODS :::");
    println!("7. Trapezoidal method");
    println!("8. Adams-Bashforth method");
    println!("9. Adams-Moulton method");
    println!("::: SHOOTING METHODS :::");
    println!("10. Single shooting method");
    println!("11. Double shooting method");
    println!("::: COLLOCATION METHODS :::");
    println!("12. Hermite collocation");
    println!("13. B-spline collocation");
    println!("[0] BACK");
    println!("::: ::: ::: ::: ::: :::");
    println!("Select an option: ");
    let mut input = String::new();
    if let Err(e) = stdin().read_line(&mut input) {
        eprintln!("Error al leer la línea: {}", e);
        return;
    }

    let opt = input.trim();

    loop {
        match opt {
            "1" => { clear_menu(); Euler::info(); break; }
            "2" => { clear_menu(); RK2::info(); break; }
            "3" => { clear_menu(); RK4::info(); break; }
            "4" => { todo!() }
            "5" => { todo!() }
            "6" => { todo!() }
            "7" => { todo!() }
            "8" => { todo!() }
            "9" => { todo!() }
            "10" => { todo!() }
            "11" => { todo!() }
            "12" => { todo!() }
            "13" => { todo!() }
            "0" => { menu_principal(); break; }
            &_ => { menu_sode(); break; }
        }
    }

}
