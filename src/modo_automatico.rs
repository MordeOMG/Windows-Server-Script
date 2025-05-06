use std::process::Command;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("powershell") // Especificamos el ejecutable de PowerShell
        .args(&["ipconfig", "Get-Process"]) // Pasamos el comando y sus argumentos
        .output()?; // Ejecutamos el comando y esperamos la salida

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Salida de PowerShell:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error al ejecutar PowerShell:\n{}", stderr);
    };

    Ok(())
}