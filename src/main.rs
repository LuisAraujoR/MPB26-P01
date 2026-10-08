use std::thread::sleep;
use std::time::Duration;

use esp_idf_svc::hal::gpio::{PinDriver, Pull};
use esp_idf_svc::hal::peripherals::Peripherals;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Iniciando Porton Electrico...");

    let peripherals = Peripherals::take()?;

    // Salida
    let mut cerrar = PinDriver::output(peripherals.pins.gpio17)?;
    let mut luz = PinDriver::output(peripherals.pins.gpio16)?;
    let mut abrir = PinDriver::output(peripherals.pins.gpio15)?;

    // Entrada
    let control = PinDriver::input(peripherals.pins.gpio18, Pull::Up)?;
    let comp_cerrado = PinDriver::input(peripherals.pins.gpio8, Pull::Up)?;
    let comp_abierto = PinDriver::input(peripherals.pins.gpio3, Pull::Up)?;
    let sensor = PinDriver::input(peripherals.pins.gpio46, Pull::Up)?;

    // Asegurarse de que inicien apagados
    cerrar.set_low()?;
    luz.set_low()?;
    abrir.set_low()?;

    let mut estado_porton: u8 = 0;
    let mut tiempo: u8 = 0;

    log::info!("Porton listo");

    loop {
        if estado_porton == 0 {
            // estado Cerrado
            abrir.set_low()?;
            cerrar.set_low()?;
            luz.set_low()?;

            if control.is_low() {
                estado_porton = 1;
            }
        } else if estado_porton == 1 {
            // estado Abriendo
            abrir.set_high()?;
            cerrar.set_low()?;
            luz.set_high()?;

            if comp_abierto.is_low() {
                estado_porton = 2;
            }
        } else if estado_porton == 2 {
            // estado Abierto
            abrir.set_low()?;
            cerrar.set_high()?;
            luz.set_high()?;

            if tiempo < 10 {
                tiempo = tiempo + 1;
                sleep(Duration::from_secs(1));
                log::info!("tiempo restante: {}", tiempo);
            } else {
                estado_porton = 3;
                tiempo = 0;
            }
        } else {
            // estado Cerrando
            abrir.set_low()?;
            cerrar.set_high()?;
            luz.set_high()?;

            if sensor.is_low() {
                estado_porton = 1;
            } else if comp_cerrado.is_low() {
                estado_porton = 0;
            }
        }
    }
}
