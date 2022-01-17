#![no_std]
#![no_main]

use rustuino::*;
use rustuino::uart::UART;

#[entry]
fn main() -> ! {
  rtt_init_print!();

  let pin1 = pinmode_output(C8).unwrap();
  let pin2 = pinmode_output(C9).unwrap();
  let pin_in = pinmode_input(A1).unwrap();
  let uart = UART::new(2, A2, A3, 115200).unwrap();

  let mut state: bool;

  loop {
    digital_write(&pin1, true);
    rprintln!("delay start");
    delay(250);
    digital_write(&pin2, true);
    delay(250);
    digital_write(&pin1, false);
    delay(250);
    digital_write(&pin2, false);
    state = digital_read(&pin_in);
    rprintln!("Input state: {}", state);
    uart.println_str("Test").ok();
    delay(250);
  }
}
