# PROJECT MOVED

This project has been moved to cpdt/trustzone-m-rs: https://gitee.com/cpdt/trustzone-m-rs .

# trustzone-m-rs

ARM TrustZone-M example application in Rust, both secure world side and non-secure world side;
projects are modified from generated result of cortex-m-quickstart.

This project is based on guide _Writing secure applications using Rust and TrustZone-M, Version 1.0_
by ARM, (c) 2022 Arm Limited.

## Features

This project illustrates how to:

- Write and run TrustZone-M application in Rust
- Enter Non-Secure entry function in Secure World
- Handle secure faults
- Call secure function in Non-Secure World (using veneers.o)
- Check permission in secure functions with pointers

## Run

You need to install rustc target, using:

```shell
rustup target add thumbv8m.main-none-eabi
```

Additionally, you should check if your computer have `arm-none-eabi-ld` installed
by `arm-none-eabi-ld --version 2`. This special ARM linker is used on veneer generation;
veneer is an important media where non-secure applications get a list of
supported functions provided by secure world side.

Clone repository and run:

```shell
git clone git@github.com:IoTS-P/trustzone-m-rs.git
cd trustzone-m-rs
cargo qemu
```

You'll get the following results:

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target\debug\xtask.exe qemu`
xtask: make application and run in QEMU
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
Hello from Secure World!
Hello from Non-Secure World!
secure function called!
secure function with pointers called!
Permission denied
secure function with pointers called!
Permission accepted
Return value: 0
Non-Secure World callback function called!
Program SUCCESS, exit Non-Secure World!
```

The program should finish without any error.
