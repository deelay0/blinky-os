cargo build
arm-none-eabi-objcopy -O binary ./target/armv7a-none-eabi/debug/blinky-os ./kernel.img
rm -r ./target
qemu-system-aarch64 -machine raspi3b -semihosting-config enable=on,target=native -kernel ./kernel.img