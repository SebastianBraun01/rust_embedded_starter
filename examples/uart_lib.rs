#![no_std]
#![no_main]

use rustuino::*;
use rustuino::uart::UART;		// We need to import the UART functions for this example

#[entry]
fn main() -> ! {
	// In this example, we use the UART library functions to establish a serial connection that excepts instructions to toggle
	// an LED.
	//
	// To use this example you will need a serial console like putty or minicom installed an set to a baudrate of 115200.
	//
	// When you type characters into the console they will be echoed back, so you can see what you typed and if you type "lon"
	// and hit enter, the inbuilt LED should light up. On "loff" the LED should go out.

	// Configure a UART connection with USART2 on pins A2 and A3 with a baudrate of 115200
	let uart = UART::new(2, A2, A3, 115200).unwrap();
	let led = pinmode_output(A5).unwrap();						// Set inbuilt LED to output

	uart.println_str("\n\r\n\rV2.3 Nucleo\n\r").ok();			// Send an inital message

	let mut buffer: char;		// Buffer variable for incoming char
	let mut storage: String<32> = String::new();		// String object that will hold the typed chars for the commands

	loop {
		// This loop is used to wait continuously for incoming chars. All chars will be pushed into the string object until
		// enter is pressed. When that happens the loop is exited and the content of the string object is processed.
		loop {
			buffer = uart.read_char().unwrap();		// Wait until char can be read
			uart.print_char(buffer).ok();					// Return the char
			if buffer == '\n' {break;}						// If enter key is read break the loop
			if storage.chars().count() < 32 {storage.push(buffer).ok();}	// Store the char in the String object
		}

		// Here the String object is read as a string and matched for a command
		match storage.as_str() {
			"lon" => digital_write(&led, true),
			"loff" => digital_write(&led, false),
			_ => uart.println_str("Command not recognized!").unwrap()		// This is the default option
		};

		storage.clear();		// String object is emptied
	}
}
