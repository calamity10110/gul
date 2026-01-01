use gul_gpio::PinDriver;
use std::{thread, time::Duration};

fn main() {
    // Spawn a thread with explicit 20KB stack size to avoid main task (3KB default) overflow
    let handle = thread::Builder::new()
        .stack_size(20 * 1024)
        .spawn(|| {
            let mut pin = PinDriver::new_output(17);
            println!("Hello, Embedded World! Flashing successful via explicit thread.");

            loop {
                println!("Toggling pin 17 high for 1 second...");
                pin.high();
                thread::sleep(Duration::from_millis(1000));

                println!("Toggling pin 17 low for 1 second...");
                pin.low();
                thread::sleep(Duration::from_millis(1000));
            }
        })
        .unwrap();

    handle.join().unwrap();
}
