cargo build --release
arm-none-eabi-objcopy -O binary ./target/armv7a-none-eabi/release/blinky-os ./kernel.img
#Uncomment to automatically delete /target after building kernel
#rm -r ./target