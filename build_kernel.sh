cargo build --release
arm-none-eabi-objcopy -O binary ./target/armv7a-none-eabi/release/blinky-os ./kernel.img
rm -r ./target