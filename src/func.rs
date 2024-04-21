// Simple wait function
pub fn simple_wait(duration: u32) {
    /*
    For rpi zero 2 w (64-bit Cortex A53 CPU @ 1 GHz)
        - duration approximates to ms (input 1000 -> aprox. 1s wait)
    */
    for _ in 0..duration {
        for _ in 0..1000 {
            unsafe {
                core::ptr::read_volatile(&0);
            }
        }
    }
}
