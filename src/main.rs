use std::thread::sleep;
use std::time::Duration;

use esp_idf_svc::hal::gpio::{PinDriver, Pull};
use esp_idf_svc::hal::peripherals::Peripherals;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Iniciando sistema de semáforo ESP32-S3...");

    let peripherals = Peripherals::take()?;

    // Configurar pines GPIO de los LEDs como salidas
    // Rojo: GPIO 17, Amarillo: GPIO 16, Verde: GPIO 15
    let mut led_rojo = PinDriver::output(peripherals.pins.gpio17)?;
    let mut led_amarillo = PinDriver::output(peripherals.pins.gpio16)?;
    let mut led_verde = PinDriver::output(peripherals.pins.gpio15)?;

    // Configurar pulsador en GPIO 18 con resistencia Pull-Up interna
    // Con Pull-Up: presionado = LOW (0), sin presionar = HIGH (1)
    let boton = PinDriver::input(peripherals.pins.gpio18, Pull::Up)?;

    // Asegurarse de que inicien apagados
    led_rojo.set_low()?;
    led_amarillo.set_low()?;
    led_verde.set_low()?;

    log::info!("Semáforo listo. Comenzando secuencia cíclica.");

    loop {
        // 1. Verde encendido por 10 segundos
        log::info!("VERDE (GPIO 15) ENCENDIDO - 10 segundos");
        led_verde.set_high()?;
        led_amarillo.set_low()?;
        led_rojo.set_low()?;
        sleep(Duration::from_secs(10));

        // Chequear estado del pulsador antes de pasar a la fase intermedia (amarillo o intermitente)
        if boton.is_low() {
            // Botón presionado: Verde intermitente a 2 Hz durante 3 segundos (6 ciclos de 500 ms)
            log::info!("BOTÓN PRESIONADO: Verde intermitente a 2 Hz (3 segundos)");
            led_amarillo.set_low()?;
            led_rojo.set_low()?;

            for _ in 0..6 {
                led_verde.set_high()?;
                sleep(Duration::from_millis(250));
                led_verde.set_low()?;
                sleep(Duration::from_millis(250));
            }
        } else {
            // Botón no presionado: Amarillo encendido normalmente por 3 segundos
            log::info!("AMARILLO (GPIO 16) ENCENDIDO - 3 segundos");
            led_verde.set_low()?;
            led_amarillo.set_high()?;
            led_rojo.set_low()?;
            sleep(Duration::from_secs(3));
        }

        // 3. Rojo encendido por 20 segundos
        log::info!("ROJO (GPIO 17) ENCENDIDO - 20 segundos");
        led_verde.set_low()?;
        led_amarillo.set_low()?;
        led_rojo.set_high()?;
        sleep(Duration::from_secs(20));
    }
}
