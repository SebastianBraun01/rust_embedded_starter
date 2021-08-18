#![allow(dead_code)]

use rustuino::*;

pub static mut DIR: bool = false;
pub static mut INPUT_BUFFER: String<64> = String::new();
pub static mut CMD_FLAG: bool = false;

pub fn init_usart2() {
	let peripherals;
	unsafe {peripherals = stm32f4::stm32f446::Peripherals::steal();}
	let rcc = &peripherals.RCC;
	let gpioa = &peripherals.GPIOA;
	let usart2 = &peripherals.USART2;

	// GPIO pins einstellen
	rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
	gpioa.moder.modify(|_, w| {
		w.moder2().alternate();
		w.moder3().alternate()
	});
	gpioa.afrl.modify(|_, w| {
		w.afrl2().af7();
		w.afrl3().af7()
	});

	// USART2 einstellen
	rcc.apb1enr.modify(|_, w| w.usart2en().enabled());
	usart2.cr1.modify(|_, w| {
		w.re().enabled();
		w.te().enabled();
		w.ue().enabled()
	});
	usart2.brr.write(|w| unsafe {w.bits(0x682)});
	unsafe {NVIC::unmask(Interrupt::USART2);}
}

pub fn write_char(c: char) {
	// Schreibt zeichen c an USART2
	let peripherals;
	unsafe {peripherals = stm32f4::stm32f446::Peripherals::steal();}
	let usart2 = &peripherals.USART2;

	usart2.dr.write(|w| w.dr().bits(c as u16));
	while usart2.sr.read().tc().bit_is_clear() == true {}
}

pub fn write_string(string: &str) {
	// Schreibt ganzen string an USART2
	for c in string.chars() {
		write_char(c);
	}
}

pub fn read_char() -> char {
	// Liest zeichen c von USART2
	let peripherals;
	unsafe {peripherals = stm32f4::stm32f446::Peripherals::steal();}
	let usart2 = &peripherals.USART2;

	while usart2.sr.read().rxne().bit_is_clear() {}
	return usart2.dr.read().dr().bits() as u8 as char;
}

pub fn txt_to_num(string: &str) -> Option<u16> {
  let mut faktor: u16 = 1;
  let mut buffer: u16 = 0;

  if string.is_ascii() == false {return None;}
  let s = string.trim_matches(|c: char| c.is_ascii_alphabetic());
  for c in s.chars().rev() {
    buffer += c.to_digit(10).unwrap() as u16 * faktor;
    faktor *= 10;
  }

  return Some(buffer);
}

	
/*----------------------------------------------------------------------------
  Interrupt-Handler USART2 (2. Aufgabe)
 *----------------------------------------------------------------------------*/
// #[allow(non_snake_case)]
// #[interrupt]
// fn USART2() {
// 	let peripherals;
// 	unsafe {peripherals = stm32f4::stm32f446::Peripherals::steal();}
// 	let usart2 = &peripherals.USART2;
// 	let systick = &peripherals.STK;

// 	let char_buffer: char;
// 	static string_buffer: String<4> = String::new();

// 	char_buffer = usart2.dr.read().dr().bits() as u8 as char;
// 	write_char(char_buffer);

// 	if char_buffer == '.' {
// 		// Befehl ausführen
// 		match string_buffer.chars().nth(0).unwrap() {
// 			'w' => {
// 				let value = txt_to_num(&string_buffer).unwrap();
// 				systick.load.write(|w| unsafe {w.reload().bits((16000 / 8) * value)});
// 				write_string("\r\n");
// 			},
// 			'r' => {
// 				let value = txt_to_num(&string_buffer).unwrap();
// 				if value == 0 {
// 					DIR = false;
// 					write_string("\r\n");
// 				}
// 				else if value == 1 {
// 					DIR = true;
// 					write_string("\r\n");
// 				}
// 				else {write_string("\n\rFehler: Richtung nicht definiert!\r\n");}
// 			},
// 			_ => write_string("\n\rFehler: Befehl unbekannt!\r\n")
// 		};

// 		string_buffer.clear();
// 	}
// 	else {
// 		// Prüfen ob Befehl zu lang ist und Zeichen in Puffer laden
// 		if string_buffer.chars().count() > 3 {write_string("\n\rFehler:Befehl Puffer voll!\r\n");}
// 		else {string_buffer.push(char_buffer).unwrap();}
// 	}
// }


/*----------------------------------------------------------------------------
  Interrupt-Handler USART2 (3. Aufgabe)
 *----------------------------------------------------------------------------*/
#[allow(non_snake_case)]
#[interrupt]
fn USART2() {
	let peripherals;
	unsafe {peripherals = stm32f4::stm32f446::Peripherals::steal();}
	let usart2 = &peripherals.USART2;

	let input_char: char;							// Eingelesenes Zeichen

	input_char = usart2.dr.read().dr().bits() as u8 as char;
	write_char(input_char);

	unsafe {
		if input_char == 0x7F as char && INPUT_BUFFER.chars().count() > 0 {		// Backspace
			INPUT_BUFFER.pop().unwrap();
		}
		else {INPUT_BUFFER.push(input_char).unwrap();}
	}

	// Prüfen ob Punkt oder Entertaste eingelesen wurde
	if input_char == '\r' || input_char == '.' {
		// falls ja, ist die Zeileneingabe beendet
		unsafe {CMD_FLAG = true;}						// Kommandoausführung im Hauptprogramm
		write_string("\n\r");								// neue Zeile
	}
}
