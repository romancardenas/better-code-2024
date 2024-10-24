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

In this talk, we are working with a Sparkfun RED-V evaluation board.
This board contains a SiFive Freedom E310 G002 chip, which complies with the RISCV32 IMC standard.
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
