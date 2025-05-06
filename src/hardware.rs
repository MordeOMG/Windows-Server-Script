//use std::{process::Command, result};
//#[cfg(target_os = "windows")]
//fn obtener_info_dispositivos() -> Result<String, std::io::Error> {
//    let output = Command::new("systeminfo").output()?;
//    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
//}

//#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
//fn obtener_info_dispositivos() -> Result<String, std::io::Error> {
//    Ok("Sistema operativo no soportado para la detección completa de dispositivos.".to_string())
//}

//pub fn mifuncion() -> result<(), std::io::Error> {
//    let info = obtener_info_dispositivos()?;
//    println!("{}", info);
//}