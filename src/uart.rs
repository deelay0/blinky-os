//Base UART register address:
/* Usage:
*  To enable UART, write 1 to RX_enable, TX_enable and UART_enable in the CR (control) register
*/
const UART: u32 = 0x3F20_1000;
/* UART registers offsets: */
//Data register
const DR_OFFSET: u32 = 0x0;
/* Data register bytes */
const DATA_START: u32 = 0;
const DATA_END: u32 = 7;

//Integer baud rate divisor
const IBRD_OFFSET: usize = 0x24;
//Fractional baud rate  divisor
const FBRD_OFFSET: usize = 0x28;

//Control register
const CR_OFFSET: usize = 0x30;
/* UART Control bytes */
const RX_ENABLE: u32 = 9;
const TX_ENABLE: u32 = 8;
const UART_ENABLE: u32 = 0;

//System clock frequency (Hz)
const SYS_CLK: u32 = 250_000_000;

pub unsafe fn init(baud_rate: u32) {
    let uart_base = UART as *mut u32;
    //Disable UART before meaking any config. changes
    uart_base.add(CR_OFFSET).write_volatile(0 << UART_ENABLE);
    //Set baud rate
    set_baud_rate(baud_rate);
    //UART Configuration before enabling UART
    /* Explanation: (TXE = 8, RXE = 9, UARTEN = 0)
    *   1 << UARTEN = 1000000000...
    *   1 << TXE    = 0000000010...
    *   1 << RXE    = 0000000001...
    +   (a | b | c) = 1000000011... --> Enable UART, RX and TX
    */
    uart_base
        .add(CR_OFFSET)
        .write_volatile((1 << UART_ENABLE) | (1 << TX_ENABLE) | (1 << RX_ENABLE));
}

pub fn set_baud_rate(baud_rate: u32) {
    let uart_base = UART as *mut u32;
    let baud_divisor = SYS_CLK as f32 / (16.0 * baud_rate as f32);

    let ibrd: u32 = baud_divisor as u32;
    let fbrd: u32 = ((baud_divisor - ibrd as f32) * 64_f32 + 0.5_f32) as u32;

    //Write the values in register
    unsafe {
        uart_base.add(IBRD_OFFSET).write_volatile(ibrd);
        uart_base.add(FBRD_OFFSET).write_volatile(fbrd);
    }
}

pub unsafe fn send_byte(byte: u8) {
    let dr_base = (UART + DR_OFFSET) as *mut u32;
    dr_base.write_volatile((byte as u32) << DATA_START);
}
