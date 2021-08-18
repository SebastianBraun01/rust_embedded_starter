#![allow(dead_code)]

pub fn init_tim3_pwm() {
  // Initialisierung des Timer 3
  // PA6 Servo, PA7 R, PC8 G, PC9 B
  let peripherals;
	unsafe {peripherals = stm32f4::stm32f446::Peripherals::steal();}
	let rcc = &peripherals.RCC;
	let gpioa = &peripherals.GPIOA;
  let gpioc = &peripherals.GPIOC;
  let tim3 = &peripherals.TIM3;

  // GPIO pin einstellen
  rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
  gpioa.moder.modify(|_, w| {
    w.moder6().alternate();
    w.moder7().alternate()
  });
  gpioa.afrl.modify(|_, w| {
    w.afrl6().af2();
    w.afrl7().af2()
  });

  rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
  gpioc.moder.modify(|_, w| {
    w.moder8().alternate();
    w.moder9().alternate()
  });
  gpioc.afrh.modify(|_, w| {
    w.afrh8().af2();
    w.afrh9().af2()
  });

  // Timer einstellen
  rcc.apb1enr.modify(|_, w| w.tim3en().enabled());

  // Aufgabe 1:
	// ARR: 100 Schritte zwischen 1ms und 2ms f�r 20ms Periodendauer = 2000
	// PRE: 16MHz * (1ms/100) = 160
	// CCR: 100 - 200

  tim3.arr.write(|w| w.arr().bits(2000));
  tim3.psc.write(|w| w.psc().bits(160));
  tim3.ccmr1_output().modify(|_, w| {   // Compare 1 & 2 preload enable + PWM Mode 1
    w.oc1pe().enabled();
    w.oc2pe().enabled();
    w.oc1m().pwm_mode1();
    w.oc2m().pwm_mode1()
  });
  tim3.ccmr2_output().modify(|_, w| {   // Compare 3 & 4 preload enable + PWM Mode 1
    w.oc3pe().enabled();
    w.oc4pe().enabled();
    w.oc3m().pwm_mode1();
    w.oc4m().pwm_mode1()
  });
  tim3.ccer.modify(|_, w| {
    w.cc1e().set_bit();
    w.cc2e().set_bit();
    w.cc3e().set_bit();
    w.cc4e().set_bit()
  });
  tim3.egr.write(|w| w.ug().update());  // Update generieren
  tim3.cr1.modify(|_, w| w.cen().enabled());
}

pub fn tim3_servo(pos: u16) {
  // Servo auf Position fahren (0...100)
  let peripherals;
	unsafe {peripherals = stm32f4::stm32f446::Peripherals::steal();}
  let tim3 = &peripherals.TIM3;

  if pos > 100 {return;}
  tim3.ccr1.write(|w| w.ccr().bits(pos + 100));
}

pub fn tim3_rgb(red: u16, green: u16, blue: u16) {
  // RGB-LED-Helligkeit setzen (0...255)
  let peripherals;
	unsafe {peripherals = stm32f4::stm32f446::Peripherals::steal();}
  let tim3 = &peripherals.TIM3;

  // 2000 / 255 ~= 7 (7 * 255 = 1785 )	gut genug, man kann den unterschied eh nicht sehen
  tim3.ccr2.write(|w| w.ccr().bits(7 * red));       // CCR 2 Register laden
  tim3.ccr3.write(|w| w.ccr().bits(7 * green));     // CCR 3 Register laden
  tim3.ccr4.write(|w| w.ccr().bits(7 * blue));      // CCR 4 Register laden
}
