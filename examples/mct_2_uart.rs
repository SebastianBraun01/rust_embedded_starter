#![no_std]
#![no_main]

mod usart2;

use rustuino::*;
use usart2::*;


/*----------------------------------------------------------------------------
  Main Program
 *----------------------------------------------------------------------------*/
#[entry]
fn main() -> ! {
	let peripherals;
	unsafe {peripherals = stm32f4::stm32f446::Peripherals::steal();}
	let rcc = &peripherals.RCC;
	let gpiob = &peripherals.GPIOB;
	let systick = &peripherals.STK;

	// GPIO pins einstellen
	rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
	gpiob.moder.modify(|_, w| {
		w.moder0().output();
		w.moder1().output();
		w.moder2().output();
		w.moder3().output();
		w.moder4().output();
		w.moder5().output();
		w.moder6().output();
		w.moder7().output()
	});
	gpiob.odr.write(|w| unsafe {w.bits(0x1)});

	// Systick timer einstellen
	systick.load.write(|w| unsafe {w.reload().bits((16000 / 8) * 100)});
	systick.ctrl.modify(|_, w| {
		w.tickint().set_bit();
		w.enable().set_bit()
	});

	init_usart2();
	write_string("\n\r\n\rV2.3 Nucleo\n\r");

	loop {}
}


/*----------------------------------------------------------------------------
  SysTick-Handler
 *----------------------------------------------------------------------------*/
#[allow(non_snake_case)]
#[exception]
fn SysTick() {
	let peripherals;
	unsafe {peripherals = stm32f4::stm32f446::Peripherals::steal();}
	let gpiob = &peripherals.GPIOB;

	static mut OUTPUT: u32 = 0x1;									// Leuchtmuster LED

	unsafe {
		if DIR == true {
			OUTPUT = OUTPUT >> 1;											// nach rechts schieben
			if OUTPUT < 0x1 {OUTPUT = 0x80;}					// Überlauf
		}
		else {
			OUTPUT = OUTPUT << 1;											// nach links schieben
			if OUTPUT > 0x80 {OUTPUT = 0x1;}					// Überlauf
		}
	}
	
	gpiob.odr.write(|w| unsafe {w.bits(OUTPUT)});	// LED ausgeben
}
