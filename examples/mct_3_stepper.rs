#![no_std]
#![no_main]

mod usart2;
mod tim3_pwm;
mod stepper;

use usart2::*;
use tim3_pwm::*;
use stepper::*;
use rustuino::*;


/*----------------------------------------------------------------------------
  Main Program
 *----------------------------------------------------------------------------*/
#[entry]
fn main() -> ! {
	init_usart2();
	init_tim3_pwm();
	init_stepper();
 
	write_string("\n\r\n\rV3.1 Nucleo\n\r");
 
	loop {
		unsafe {
				if CMD_FLAG == true {		// anliegendes Eingabekommando	
				execute_cmd();
				CMD_FLAG = false;
			}
	  } 
	}
}


fn execute_cmd() {
	static mut RED: u16 = 0;
	static mut GREEN: u16 = 0;
	static mut BLUE: u16 = 0;

	// Ausführung des eingegebenen Befehls
	match unsafe {INPUT_BUFFER.chars().nth(0).unwrap()} {
		's' => {
			// Servo auf Position fahren
			let wert;
			unsafe {wert = txt_to_num(&INPUT_BUFFER).unwrap();}
			tim3_servo(wert);
			write_string("Neue Servo Position...\r\n");
		},
		'r' => {
			// RGB LED farbe komplett einstellen "r{}g{}b{}"
			let string: &str;
			unsafe {string = &INPUT_BUFFER;}
			let (sred, buffer) = string.split_once('g').unwrap();
			let (sgreen, sblue) = buffer.split_once('b').unwrap();

			let r = txt_to_num(&sred).unwrap();
			let g = txt_to_num(&sgreen).unwrap();
			let b = txt_to_num(&sblue).unwrap();

			tim3_rgb(r, g, b);
			write_string("Neuer RGB Wert...\r\n");
		},
		'l' => {
			// RGB LED farben individuel einstellen
			unsafe {
				match INPUT_BUFFER.chars().nth(1).unwrap() {
					'r' => RED = txt_to_num(&INPUT_BUFFER).unwrap(),
					'g' => GREEN = txt_to_num(&INPUT_BUFFER).unwrap(),
					'b' => BLUE = txt_to_num(&INPUT_BUFFER).unwrap(),
					_   => write_string("Not a valid led color index!\r\n")
				};
				tim3_rgb(RED, GREEN, BLUE);
			}
			write_string("Neuer RGB Wert...\r\n");
		},
		'+' => {
			// Schrittmotor rechtslauf
			unsafe {
				STEPDIR = false;
				STEPMODE = Stepmode::StepmodeNone;
			}
			write_string("Schrittmotor Rechtslauf...\r\n");
		},
		'-' => {
			// Schrittmotor linkslauf
			unsafe {
				STEPDIR = true;
				STEPMODE = Stepmode::StepmodeNone;
			}
			write_string("Schrittmotor Rechtslauf...\r\n");
		},
		'm' => {
			// Schrittmotor auf Position fahren bzw Referenz finden
			unsafe {
				STEPMODE = Stepmode::StepmodePos;
				if INPUT_BUFFER.chars().nth(1).unwrap() == 'r' {ZIEL_POS = 0;}
				else {
					ZIEL_POS = txt_to_num(&INPUT_BUFFER).unwrap();
				}
			}	
			write_string("Schrittmotor auf Referenz...\r\n");
		},
		'h' => help_display(),
		_   => help_display()
	};

	unsafe {INPUT_BUFFER.clear();}
}

fn help_display() {
	// Anzeige des Hilfetextes
	write_string("Kommandos:\n\r");
	write_string("\tp[pos]w[wert]:\t\tSinustabelle: an Stelle pos den Wert wert setzen\n\r");
	write_string("\tf[freq]a[amp]:\t\tSinustabelle: Frequenz und Amplitude [1-9]  setzen\n\r");
	write_string("\ts[pos]:\t\t\tServo auf Position pos fahren\n\r");
	write_string("\tr[r]g[g]b[b]\t\tRGB-LED auf Farbe r/g/b setzen [0...255]\n\r");
	write_string("\tl[r/g/b][wert]:\t\tBei RGB-LED Farbe r/g/b auf Wert wert setzen\n\r");
	write_string("\t+:\t\t\tSchrittmotor dreht vorwaerts\n\r");
	write_string("\t-:\t\t\tSchrittmotor dreht rueckwaerts\n\r");
	write_string("\tm[pos]\t\t\tSchrittmotor auf Position pos fahren\n\r");
	write_string("\tmr\t\t\tReferenzfahrt Schrittmotor\n\r");
}
