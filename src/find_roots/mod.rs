pub mod find_root_bracketing;
pub mod find_root_iterative;
pub mod find_root_combination;

//Re-exporting the FindRoot trait here if necessary
pub use find_root_bracketing::Bisection;
pub use find_root_bracketing::FalsePosition;
pub use find_root_bracketing::ITP;