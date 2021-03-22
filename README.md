# biosctl

**biosctl** is a command line tool to manage Dell BIOS/EFI settings on Linux 5.11+.

## Installation

Precompiled binaries are available on the [Release Page] for x86_64 Linux (statically linked).

Otherwise you will need to [build from source](#building-from-source).

## Usage

List all available config attributes with:

```sh
$ biosctl list
```

Print information about attributes with the `print` subcommand:

```sh
$ biosctl print WakeOnAc
$ biosctl print # print all attributes
```

Get values directly with the `get` subcommand:
```sh
$ sudo biosctl get VtForDirectIo
```

It may be necessary to run with `sudo` to be able to show the current value.

Example output:

```sh
$ biosctl print WakeOnAc
Device: dell-wmi-sysman

WakeOnAc
    Name: Wake on AC
    Type: Enumeration
        Possible Values:
            Disabled
            Enabled

    Current value: Disabled
    Default value: Disabled
$ biosctl get --name VtForDirectIo
Enable Intel VT for Direct I/O
$ biosctl get --default VtForDirectIo
Enabled
$ sudo biosctl get VtForDirectIo
Enabled
```

Set values with the `set` subcommand:

```sh
$ sudo biosctl get WakeOnDock
Enabled
$ sudo biosctl set WakeOnDock Disabled
$ sudo biosctl get WakeOnDock
Disabled
```

## Background

Since Linux 5.11, the kernel can expose [firmware configuration attributes] under `/sys/class/firmware-attributes`.

On Dell systems this is used to exposes all the BIOS/EFI configuration to userspace.

## Building from source

biosctl is written in Rust, so you need a [Rust install] to build it. biosctl compiles with Rust 1.50 or newer.

Build the latest release (0.3.0) from source with:

```sh
$ git clone https://github.com/gourlaysama/biosctl -b v0.3.0
$ cd biosctl
$ cargo build --release
$ ./target/release/biosctl --version
biosctl 0.3.0

```

#### License

<sub>
biosctl is licensed under the <a href="LICENSE-MIT">MIT license</a>.
</sub>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in biosctl shall be licensed as above,
without any additional terms or conditions.
</sub>

[Rust install]: https://www.rust-lang.org/tools/install
[firmware configuration attributes]: https://www.kernel.org/doc/html/v5.11/admin-guide/abi-testing.html#abi-sys-class-firmware-attributes-attributes
[Release Page]: https://github.com/gourlaysama/biosctl/releases/latest