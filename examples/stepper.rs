#![allow(dead_code)]

use rustuino::*;

#[derive(PartialEq, Eq)]
pub enum Stepmode {
  StepmodeNone,
  StepmodeFwd,
  StepmodeBwd,
  StepmodePos,
  StepmodeRef
}
 
pub static mut STEPDIR: bool = false;                         // Richtung des Schrittmotors
pub static mut AKT_POS: u16 = 0;		    					          	// aktuelle Position des Schrittmotors
pub static mut ZIEL_POS: u16 = 0;   							          	// Zielposition des Schrittmotors
pub static mut STEPMODE: Stepmode = Stepmode::StepmodeNone;	  // Modus des Schrittmotors

static mut PATTERN_POS: u16 = 0;

pub fn init_systick(load: u32) {
  // initialisiert SysTick mit load und startet ihn
  let peripherals;
	unsafe {peripherals = stm32f4::stm32f446::Peripherals::steal();}
	let systick = &peripherals.STK;

  systick.load.write(|w| unsafe {w.reload().bits(load)});
  systick.ctrl.modify(|_, w| {
    w.tickint().set_bit();
    w.enable().set_bit()
  });
}
 
pub fn init_stepper() {
  // initialisiert Schrittmotorausgänge GPIOB0...3 und Referenzeingang GPIOB4
  let peripherals;
	unsafe {peripherals = stm32f4::stm32f446::Peripherals::steal();}
	let rcc = &peripherals.RCC;
  let gpiob = &peripherals.GPIOB;

  rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
  gpiob.moder.modify(|_, w| {
    w.moder0().output();
    w.moder1().output();
    w.moder2().output();
    w.moder3().output()
  });

  init_systick((16000000 / 4096) * 6);    // 6s für eine Umdrehung
}

pub fn step_out(step_pattern: u16) {
  // gibt das Schrittmotormuster step_pattern an Motor aus
  let peripherals;
	unsafe {peripherals = stm32f4::stm32f446::Peripherals::steal();}
  let gpiob = &peripherals.GPIOB;

  let pattern_map: [u32; 8] = [0x0001, 0x0003, 0x0002, 0x0006, 0x0004, 0x000C, 0x0008, 0x0009];
  // let pattern_map: [u16; 8] = [0x0009, 0x0008, 0x000C, 0x0004, 0x0006, 0x0002, 0x0003, 0x0001];

  gpiob.odr.reset();
  gpiob.odr.write(|w| unsafe {w.bits(pattern_map[step_pattern as usize])});
}
 
 
/*----------------------------------------------------------------------------
  Interrupt-Handler SysTick
 *----------------------------------------------------------------------------*/
#[allow(non_snake_case)]
#[exception]
fn SysTick() {
  unsafe {
    // Aufgabe 3.3 a)
    if STEPMODE == Stepmode::StepmodeNone {
      // false = rechtslauf, true = linkslauf
      if STEPDIR == true {
        step_out(PATTERN_POS);
        if PATTERN_POS < 7 {PATTERN_POS += 1;}
        else {PATTERN_POS = 0;}
      }
      else {
        step_out(PATTERN_POS);
        if PATTERN_POS > 0 {PATTERN_POS -= 1;}
        else {PATTERN_POS = 7;}
      }
    }
    // Aufgabe 3.3 b)
    else if STEPMODE == Stepmode::StepmodePos {
      // ziel > akt = rechtslauf, ziel < akt = linkslauf
      if ZIEL_POS < AKT_POS {
        step_out(PATTERN_POS);
        if PATTERN_POS < 7 {PATTERN_POS += 1;}
        else {PATTERN_POS = 0;}
        AKT_POS -= 1;
      }
      else if ZIEL_POS > AKT_POS {
        step_out(PATTERN_POS);
        if PATTERN_POS > 0 {PATTERN_POS -= 1;}
        else {PATTERN_POS = 7;}
        AKT_POS += 1;
      }
    }
  }
}
