// Addresses of BCM2837 pins (./BCM2837-ARM-Peripherals.pdf)
// BCM2837 --> 53 GPIO Pins

/* 53 Pins, 10 pins per register, reg addr separated by 4 bytes (32bits x reg)
*  GPFSEL0 -> 0x3F2_0000  => pins 10-19
*  GPFSEL1 -> 0x3F2_0004  => pins 20-29
*  GPFSEL2 -> 0x3F2_0008  => pins 30-39
*  GPFSEL3 -> 0x3F2_000C  => pins 40-49
*  GPFSEL4 -> 0x3F2_0010  => pins 50-53
*  GPFSEL5 -> 0x3F2_0014  => pins 0-9    ??
*
*  TLDR: to get to the correct register: GPFSEL0.add(offset*(pin_num/10-1))
*/
//GPIO function select base
const GPFSEL0: usize = 0x3F20_0000; //Pins 0-9 (10-19 at GPFSEL1, 20-29 at GPFSEL2, etc.)

//GPIO output set (turn on) base
const GPSET0: usize = 0x3F20_001C; //Pins 0-31 (32-53 @ GPSET0.add(OFFSET) = 0x3F20_0020 => GPSET1)

//GPIO output clear (turn off) base
const GPCLR0: usize = 0x3F20_0028; //Pins 0-31 (32-53 @ GPCLR0.add(OFFSET) = 0x3F20_002C => GPCLR1)

//GPIO pin level
const GPLEV0: usize = 0x3F20_0034; //Pins 0-31 (32-53 @ GPLEV0.add(OFFSET) = 0x3F20_0038 => GPLEV1)

//GPIO event detect status
const GPEDS0: usize = 0x3F20_0040; //Pins 0-31 (32-53 @ GPEDS0.add(OFFSET) = 0x3F20_0044 => GPEDS1)

//GPIO single register offset
const OFFSET: usize = 0x4;


pub unsafe fn funsel(pin: usize, fun: u32) {
    /* Pin function selection:  (0 -> input , 1 -> output)
        fun = 0 -> input
        fun = 1 -> output
        fun = 2 -> alt function 5
        fun = 3 -> alt function 4
        fun = 4 -> alt function 0
        fun = 5 -> alt function 1
        fun = 6 -> alt function 2
        fun = 7 -> alt function 3
    Alternate functions table in BCM2837 Peripherals manual, page 102
    */
    let pin_bit = (pin % 10) * 3;
    let register_num = (pin / 10) - 1;
    let gpfsel = (GPFSEL0 + (OFFSET * register_num)) as *mut u32;

    let current_val = gpfsel.read_volatile();
    let clear_mask = !(0b111 << pin_bit);
    let cleared_value = current_val & clear_mask;

    gpfsel.write_volatile(cleared_value | (fun << pin_bit));
}

// Set (turn on) an output pin
pub unsafe fn output_pin_set(pin: usize) {
    let gpset = if pin < 32 {GPSET0 as *mut u32} else {(GPSET0 + OFFSET) as *mut u32};
    let pin_index = if pin < 32 {pin} else {pin - 32};
    
    gpset.write_volatile(1 << pin_index);
}

// Clear (turn off) an output pin
pub unsafe fn output_pin_clear(pin: usize) {
    let gpclr = if pin < 32 {GPCLR0 as *mut u32} else {(GPCLR0 + OFFSET) as *mut u32};
    let pin_index = if pin < 32 {pin} else {pin - 32};
    
    gpclr.write_volatile(1 << pin_index);
}

//Read pin level (Output: true -> High , false -> Low)
pub unsafe fn pin_level(pin: usize) -> bool {
    let gplev = if pin < 32 {GPLEV0 as *const u32} else {(GPLEV0 + OFFSET) as *const u32};
    let pin_index = if pin < 32 { pin } else { pin - 32 };
    return gplev.read_volatile() & (1 << pin_index) != 0;
}

//Read and clear a pin's event detect status (Output: true -> event detected , false -> event not detected)
pub unsafe fn pin_eds(pin: usize) -> bool {
    let gpeds = if pin < 32 {GPEDS0 as *mut u32} else {(GPEDS0 + OFFSET) as *mut u32};
    let pin_index = if pin < 32 { pin } else { pin - 32 };
    let current_val = gpeds.read_volatile();
    let clear_mask = !(1 << pin_index);
    let cleared_value = current_val | clear_mask;

    gpeds.write_volatile(cleared_value);

    return current_val & (1 << pin_index) != 0;
}

//Read event detect status of pin (no clearing after reading)
pub unsafe fn pin_eds_read(pin: usize) -> bool {
    let gpeds = if pin < 32 {GPEDS0 as *mut u32} else {(GPEDS0 + OFFSET) as *mut u32};
    let pin_index = if pin < 32 { pin } else { pin - 32 };

    return gpeds.read_volatile() & (1 << pin_index) != 0;
}

//Clear a pin's event detect status
pub unsafe fn pin_eds_clr(pin: usize) {
    let gpeds = if pin < 32 {GPEDS0 as *mut u32} else {(GPEDS0 + OFFSET) as *mut u32};
    let pin_index = if pin < 32 { pin } else { pin - 32 };
    let current_val = gpeds.read_volatile();
    let clear_mask = !(1 << pin_index);
    let cleared_val = current_val | clear_mask;

    gpeds.write_volatile(cleared_val);
}
