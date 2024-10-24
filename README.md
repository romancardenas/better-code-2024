# Examples for `betterCode()` Rust 2024

This repository contains all the examples shown in my talk for [`betterCode()` Rust 2024](https://rust.bettercode.eu/veranstaltung-22605-se-0-safe-and-open-rust-and-risc-v-for-embedded-developers.html).

## Pre-requisites

If you want to run the provided examples, you first need to install the following dependencies:

### Rust 1.76 or higher

To install Rust on your machine, refer to [the official Rust page](https://www.rust-lang.org/tools/install).
You can check the RUst version on your machine by running the following command in your terminal:

```bash
rustc --version
```

If your current version is older than `rustc 1.76.0`, you can upgrade the toolchain with:

```bash
rustup update
```

### `riscv32imc-unknown-none-elf` target

In this talk, we are working with a [Sparkfun RED-V](https://www.sparkfun.com/products/retired/15594) evaluation board.
This board contains a [SiFive Freedom E310 G002 chip](https://cdn.sparkfun.com/assets/7/f/0/2/7/fe310-g002-manual-v19p05.pdf).
This chip complies with the RISCV32 IMC standard.
In a few words, this means that our board is:

- A RISC-V target that implements the Base Integer Instruction Set for 32 bits (RV32I).
- It includes the Standard Extension for Integer Multiplication and Division (M).
- It includes the Standard Extension for Compressed Instructions (C).

This board also implements the Zaamo extension, a subset of the A extension.
This allows us to execute Atomic Memory Operations (AMOs) such as `AMOSWAP.W`, `AMOADD.w`, `AMOAND.w`, or `AMOOR.w`.

We need to install the Rust target to build code for this board.
You can check if it is already installed by running the following command:

```bash
rustup target list 
```

Within a long list of available targets, you should see something like:

<pre>
<b>riscv32imc-unknown-none-elf (installed)</b>
</pre>

If you don't have it installed yet, you can run the following command:

```bash
rustup target add riscv32imc-unknown-none-elf
```

### QEMU system emulation

All the examples are ready to be emulated using QEMU system emulation without requiring any hardware.
Check [the official QEMU page](https://www.qemu.org/download/#linux) to learn how to install it on your machine.

### `netcat` (recommended)

We use the [QTest Device Emulation Testing Framework](https://www.qemu.org/docs/master/devel/testing/qtest.html) to emulate input stimuli from external sources (e.g., buttons connected to a pin configured as GPIO input).
To achieve this, we must stablish a TCP connection with QEMU.
The project is configured to use the `netcat` command to achieve this.
You can check if `netcat` is installed on your machine by running the following command:

```bash
nc --version
```

MacOS comes with `netcat` installed.
If you are on Linux, you can follow [this tutorial](https://www.ucartz.com/clients/knowledgebase/658/How-to-Install-and-Use-netcat-Command-on-Linux.html).
If you are a Windows user, follow [this tutorial](https://medium.com/@bonguides25/how-to-install-netcat-on-windows-10-11-f5be1a185611).

> [!NOTE]
> 
> We just need to open TCP connections with QEMU, so you can use any tool that allows this (e.g., `telnet` or `socat`).
> Use the one you are more comfortable with and tweak those parts of this project that rely on `netcat` so everything works with your tool.

### Visual Studio Code (recommended)

Visual Studio Code (VSCode) is one of the most popular IDEs for any programming language and use case.
In fact, it is the IDE I usually use when developing Embedded Rust applications.
This repository has been configured to make working with Rust Embedded nearly as easy as developing native applications.
If you haven't installed it yet, follow the instructions from [the official VSCode webpage](https://code.visualstudio.com).

> [!NOTE]
> 
> While I think VSCode is a great tool for embedded Rust projects, and this repository has been specially configured to work with VSCode, you can opt it out and use the terminal and your favorite tools.
> Note, however, that some things will probably not work, and you will need to spend some time configuring your environment.

### VSCode extensions (for those working with VSCode)

If you want to use VSCode (which is the recommended way), then you will need to install the following VSCode extensions to make it work:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer): This official extension provides support for the Rust programming language.
- [Cortex-debug](https://marketplace.visualstudio.com/items?itemName=marus25.cortex-Debug): conceived for debugging on Arm Cortex-M microcontrollers, it can also be used with RISC-V targets. This plugin is a must-have for embedded developers, as it supports J-Link, OpenOCD, GDB, QEMU, semihosting...
- [Command Variable](https://marketplace.visualstudio.com/items?itemName=rioj7.command-variable): this is a very convenient extension when you want to share variables between `launch.json` and `tasks.json`.
We use it to ease the process of running different examples.

## Available examples

Check the `examples/` directory.
You will find 6 usable examples that illustrate how to program a RISC-V embedded devices:

- `1_hello_world.rs`: Prints "Hello, World!" using semihosting.
- `2_blinky.rs`: Makes an LED blink.
- `3_mtimer.rs`: Illustrates how to work with core interrupts.
- `4_button.rs`: Illustrates how to work with external interrupts provoked by an external button.
- `5_slic.rs`: It uses the [SLIC](https://github.com/romancardenas/riscv-slic/tree/mecall) crate to generate software interrupts with programmable priority levels.
- `6_rtic.rs`: It shows a simple application with asynchronous tasks built on top of the [RTIC](https://github.com/romancardenas/rtic) Real-Time Operating System (RTOS).


## Configuring Cargo for your target

As mentioned above, this project is based on a RISCV32IMC + Zaamo target.
Thus, we need to configure Cargo to build the examples for this kind of targets.
We can provide all the required configuration through the terminal every time we run `cargo`.
Alternatively, we can set the default configuration in `.cargo/config.toml`.
Take a look to this file in this repo:

```toml
[target.'cfg(all(target_arch = "riscv32", target_os = "none"))']
runner = "qemu-system-riscv32 -machine sifive_e,revb=true -nographic -semihosting-config enable=on,target=native -kernel" # QEMU
# runner = "qemu-system-riscv32 -machine sifive_e,revb=true -nographic -semihosting-config enable=on,target=native -qtest tcp:localhost:3333 -kernel" # QTest
# runner = "riscv64-unknown-elf-gdb -q -x gdb_init" # OpenOCD
rustflags = [
    "-C", "link-arg=-Thifive1-link.x",
    "--cfg", "portable_atomic_target_feature=\"zaamo\"",
]

[build]
target = "riscv32imc-unknown-none-elf"
```

As you can see, we are telling `cargo` to use the `riscv32imc-unknown-none-elf` whenever something needs to be built.
Additionally, we provide a few more configuration parameters whenever the target is a RISCV32 chip with no OS.
Namely, we are passing a bunch of `RUSTFLAGS` and the runner we want to use when running an example.
Now, every time we run the following command:

```bash
cargo build --example <EXAMPLE>
```

We will be effectively executing this command:

```bash
RUSTFLAGS="-C link-arg=-Thifive1-link.x --cfg portable_atomic_target_feature=\"zaamo\"" cargo build --target riscv32imc-unknown-none-elf --example <EXAMPLE>
```

Additionally, when running the following command:

```bash
cargo run --example <EXAMPLE>
```

Cargo will build the example if there are any changes in the code and then emulate the example in QEMU running this command:

```bash
qemu-system-riscv32 -machine sifive_e,revb=true -nographic -semihosting-config enable=on,target=native -kernel target/riscv32imc-unknown-none-elf/debug/examples/<EXAMPLE>
```

As you can see, this configuration will reduce significantly the length of the commands we need to prompt when using the terminal.
Not bad, right?

> [!TIP]
> 
> Take a closer look to `.cargo/config.toml`.
> Note that we provide three different `runner`s.
> The first one executes the example in QEMU.
> The second one executes the example in QEMU too, but it connects to a TCP socket at port 3333 to allow us to send QTest commands.
> The last one is connects to OpenOCD using GDB to debug the example on a physical board.
> Choose the `runner` that better suits your project and leave the rest commented

## Running examples in VSCode

VSCode is able to run all the provided examples in a user-friendly manner.
It allows you to execute each of these examples step by step and read all the generated logs.
To run an example, go to the `Run and Debug` tab in the left bar.
You will see that there are three Debug configurations:

- `Debug (OpenOCD)`: use this configuration if you want to run and debug the example on a physical board.
- `Debug (QEMU)`: use this configuration if you want to run and debug the example using the QEMU emulator.
- `Debug (QTest)`: this configuration is similar to `Debug (QEMU)`, however, it connects to a TCP socket at port 3333 to receive QTest commands.

In this talk, we will mainly use the `Debug (QEMU)` configuration to emulate the examples.
However, the the example `4_button.rs`, we will open a TCP connection at port 3333 and use the `Debug (QTest)` configuration.
This configuration will allow us to emulate external stimuli from the button.
Whenever we want to emulate that the button has been pressed, we will send the following QTest command through the TCP socket:

```bash
set_irq_in /machine/soc unnamed-gpio-in 9 0
```

Alternatively, if we want to emulate that the button has been release, we will send the following command:

```bash
set_irq_in /machine/soc unnamed-gpio-in 9 1
```
