#![no_std]    // This is needed an signifies that you need to work the core library because of the embedded target
#![no_main]   // This is also needed in your file and signifies that the main entry of the programm is dependend on the chip

use rustuino::*;  // Include the Rustuino library

// Bacause the compiler doesnt know where to place the programm entry for embedded targets you need to specify it. You do
// that by putting the "entry" attribute on your main function. It doesnt need to be called main, you can choose the name.
// The ! means that the function isnt allowed to end and return. The chip needs to be haltet or loop forever.

#[entry]
fn main() -> ! {
  // This Example will show how make a nice lightshow by only using direct register access.
  //
  // Depending on the register you can either modify, write or read from it:
  // 
  // When you want to set or reset specific fields on a register, you can do that with the modify method. Modify expects a
  // closure that includes the fields of the register you want to modify. In the arguments of the closure you get a read
  // object and a write object. Select only the ones you need to use (see examples below). Be aware that unmodified bits
  // will be left at their CURRENT VALUE.
  //
  // When you want to write a value to a register, use the write method. It is used like the modify method, except you only
  // get a write object. Writing raw bits to a register is unsafe because the value can overflow over the register or
  // reserved fields can be written to, so you need to use an unsafe section (see examples below). Be aware that write will
  // set unmodified bits to their RESET VALUE.
  // 
  // Closures can modify multiple fields at the same time. If you do that open the closure with curly braces.
  // 
  // When you want to read from a register, you can do that with the read method. It can either read a whole register or
  // only specific bits or sections. Read takes no arguments an returns a value type depending on the size of the selection.
  //
  // When you want to reset a register, you can call the reset method on it, to set it to its default state.

  // Get the structs to the RCC, GPIOB, GPIOC and SysTick registers by borrowing it.
  let peripherals = Peripherals::take().unwrap();   // Borrow the set of all peripherals
  let rcc = &peripherals.RCC;       // RCC register struct
  let gpiob = &peripherals.GPIOB;   // GPIOB register struct
  let gpioc = &peripherals.GPIOC;   // GPIOC register struct
  let systick = &peripherals.STK;   // SysTick register struct

  // Enable clock signals for GPIOB and GPIOC 
  rcc.ahb1enr.modify(|_, w| {
    w.gpioben().enabled();  // Enable GPIOB
    w.gpiocen().enabled()   // Enable GPIOC
  });

  // Configure the GPIO ports
  gpiob.moder.modify(|_, w| unsafe {w.bits(0x5555)});   // Set pins 0 to 7 of GPIOB to output
  gpiob.odr.write(|w| unsafe {w.bits(0x1)});            // Enable pin B0
  gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(0x3F))});   // Ensure that pins C0 to C2 are inputs

  // Configure the SysTick timer for a 500ms interrupt interval
  // 2MHz with 1000000 PSC -> 2Hz, 0.5s
  systick.load.write(|w| unsafe {w.reload().bits(1000000)});    // Set the reload value
  systick.val.reset();          // Set the current count value to 0
  systick.ctrl.modify(|_, w| {
    w.tickint().set_bit();      // Enable the SysTick interrupt
    w.enable().set_bit()        // Enable the counter
  });

  // Infinite loop
  loop {}
}

// With Rust, interrupts and exceptions are registered by defining the appropriate function and marking it with "interrupt"
// or "exception" attribute macro (see below). To see whitch function name corrisponds to whitch interrupt see the
// documentation.

#[allow(non_snake_case)]
#[exception]
fn SysTick() {
  // Because interrupts work in a completely different scope than you normal code it is difficult to share data freely
  // between them. Because of this you cannot share peripherals with the interrupt, so you need to unsafely take another
  // instance of all peripherals.

  // Take another instance of all peripherals and borrow the ones you need
  let peripherals;
  unsafe {peripherals = Peripherals::steal();}
  let gpiob = &peripherals.GPIOB;
  let gpioc = &peripherals.GPIOC;
  let systick = &peripherals.STK;

  // Every interrupt the current value of the output pins are read, shifted and written back, to make the lightshow.
  let mut output = gpiob.odr.read().bits();
  if output >= 128 {output = 1;}
  else {output = output << 1;}
  gpiob.odr.write(|w| unsafe {w.bits(output)});

  // Here the three input pins are read and masked and depending on their value the reload value of the SysTick timer is
  // modified to make it faster or slower depending on the input.
  let input = gpioc.idr.read().bits() & 0x3;
  match input {
    0 => systick.load.write(|w| unsafe {w.reload().bits(1000000)}),
    1 => systick.load.write(|w| unsafe {w.reload().bits(750000)}),
    2 => systick.load.write(|w| unsafe {w.reload().bits(500000)}),
    3 => systick.load.write(|w| unsafe {w.reload().bits(250000)}),
    _ => systick.load.write(|w| unsafe {w.reload().bits(1000000)})  // This is the default value if input is invalid
  };
}
