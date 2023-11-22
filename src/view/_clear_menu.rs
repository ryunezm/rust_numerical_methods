use std::io::Error;
use std::process::Command;

pub fn clear_menu(){
    if let Err(e) = clear_console() { eprintln!("Error clearing console: {}", e); }
}
pub fn clear_console() -> Result<(), Error> {
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/c").arg("cls").status()?;
    } else {
        Command::new("clear").status().or_else(|_| {
            Command::new("tput").arg("clear").status()
        })?;
    }
    Ok(())
}
