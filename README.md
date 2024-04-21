--WORK IN PROGRESS--
 - End goal is a simple OS for a Raspberry Pi Zero 2 W made fully in Rust
 - Currently have basic GPIO functionality and trying to make UART work

Configuration changes made (project):
    - Added build target and linker script at compilation (at ./.cargo/config):
        
        [build]
        target = "armv7a-none-eabi"
        [target.'cfg(all(target_arch = "arm", target_os = "none"))']
        rustflags = ["-C", "link-arg=-Tlinker.ld",]
    
    - Added rust-analyzer settings to only check for our target, removing false-positive errors (at ./.vscode/settings.json):
        
        "rust-analyzer.cargo.target": "armv7a-none-eabi",
        "rust-analyzer.checkOnSave.allTargets": false
    
    - Added flags to disable stack unwinding when building for release & dev (at ./Cargo.toml):
        
        [profile.dev]
        panic = "abort"
        [profile.release]
        panic = "abort"

Changes in 'config.txt' in SD Card:
    - Added 

ELF to binary image:
    $ arm-none-eabi-objcopy -O binary ./target/armv7a-none-eabi/release/blinky-os ./kernel.img