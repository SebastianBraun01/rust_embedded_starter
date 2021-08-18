#![no_std]
#![no_main]

use rustuino::*;

#[entry]
fn main() -> ! {
  // Structs für Registerzugriff
  let peripherals = stm32f4::stm32f446::Peripherals::take().unwrap();
  let rcc = &peripherals.RCC;
  let gpiob = &peripherals.GPIOB;
  let gpioc = &peripherals.GPIOC;
  let systick = &peripherals.STK;

  // Clocks für GPIO Bänke einschalten
  rcc.ahb1enr.modify(|_, w| {
    w.gpioben().enabled();
    w.gpiocen().enabled()
  });

  // GPIO pin konfigurieren
  gpiob.moder.modify(|_, w| unsafe {w.bits(0x5555)});
  gpiob.odr.write(|w| unsafe {w.bits(0x1)});
  gpioc.moder.modify(|_, w| unsafe {w.bits(0x3)});

  // Systick Timer konfigurieren
  // 2MHz mit 1000000 PSC -> 2Hz, 0.5s
  systick.load.write(|w| unsafe {w.reload().bits(1000000)});
  systick.val.reset();
  systick.ctrl.modify(|_, w| {
    w.tickint().set_bit();
    w.enable().set_bit()
  });

  // Unendlicher loop
  loop {}
}

#[allow(non_snake_case)]
#[exception]
fn SysTick() {
  let peripherals;
  unsafe {peripherals = stm32f4::stm32f446::Peripherals::steal();}
  let gpiob = &peripherals.GPIOB;
  let gpioc = &peripherals.GPIOC;
  let systick = &peripherals.STK;

  // Output einlesen und lauflicht weiter schieben
  let mut output = gpiob.odr.read().bits();
  if output >= 128 {output = 1;}
  else {output = output << 1;}
  gpiob.odr.write(|w| unsafe {w.bits(output)});

  // Input einlesen und reload wert updaten
  let input = gpioc.idr.read().bits() & 0x3;
  match input {
    0 => systick.load.write(|w| unsafe {w.reload().bits(1000000)}),
    1 => systick.load.write(|w| unsafe {w.reload().bits(750000)}),
    2 => systick.load.write(|w| unsafe {w.reload().bits(500000)}),
    3 => systick.load.write(|w| unsafe {w.reload().bits(250000)}),
    _ => systick.load.write(|w| unsafe {w.reload().bits(1000000)})
  };
}
