use std::fs::File;
use std::io::{self, BufReader, Read};
use std::process;
mod sys_detector;
use sys_detector::detector;

fn sistema_no_compatible() {
    println!("Por favor recuerde que este programa está destinado al uso de Windows Server y no es compatible con otros sistemas operativos");
    println!("Por favor, presione 'e' y luego Enter para salir.");
    loop {
        let mut fehler = String::new();
        io::stdin()
            .read_line(&mut fehler)
            .expect("Error al leer la línea");

        let trimmed_fehler = fehler.trim().to_lowercase();

        if trimmed_fehler == "e" {
            process::exit(0);
        } else {
            println!("Tecla no reconocida. Por favor, presione 'e' y luego Enter para salir.");
        };
    };
}

fn modo_debug_p() {
    println!("Estás ejecutando el modo debug (función no implementada completamente).");
}

fn modo_operativo() {
    let det: bool = detector();
    if det {
        println!("Sistema operativo compatible.");
    } else {
        sistema_no_compatible();
    }
}

fn reader(archivo_path: &str) -> io::Result<String> {
    let archivo = File::open(archivo_path)?;
    let mut lector = BufReader::new(archivo);
    let mut contenido = String::new();
    lector.read_to_string(&mut contenido)?;
    Ok(contenido)
}

fn main() -> io::Result<()> {
    let archivo = "src/menu.txt";
    match reader(archivo) {
        Ok(texto) => {
            println!("{}", texto);
        }
        Err(error) => {
            eprintln!("Error al leer el archivo '{}': {}", archivo, error);
        }
    }
    loop {
//      println!("\nIngrese su selección (1-5):");
        let mut menu = String::new();
        io::stdin()
            .read_line(&mut menu)
            .expect("Error al leer la línea");
//pinhe rust
        let opcion = menu.trim().to_lowercase();

        match opcion.as_str() {
            "1" => modo_operativo(),
            "2" => println!("Modo manual seleccionado (función no implementada completamente)."),
            "3" => modo_debug_p(),
            "4" => println!("Más información y documentación (función no implementada completamente)."),
            "5" => {
                println!("Cerrando el programa.");
                process::exit(0);
            }
            _ => println!("Error, ingrese una opción correcta (1-5)."),
        };
    }
}