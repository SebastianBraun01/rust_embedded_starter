#![no_std]
#![no_main]

use rustuino::*;

#[entry]
fn main() -> ! {

  let pin = pinmode_output(A5).unwrap();

  loop {
    digital_write(&pin, true);
    delay(1000);
    digital_write(&pin, false);
    delay(1000);
  }
}
