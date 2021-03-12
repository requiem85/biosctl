# firmconfig

**firmconfig** is a command line tool to display firmware configuration attributes exposed by the Linux kernel (since kernel 5.11).

## Usage

List all available config attributes with:

```sh
$ firmconfig list
```

Print information about attributes with the `print` subcommand:

```sh
$ firmconfig print WakeOnAc
$ firmconfig print # print all attributes
```

Example output:

```sh
$ firmconfig print WakeOnAc
Device: dell-wmi-sysman

WakeOnAc
    Name: Wake on AC
    Type: Enumeration
        Possible Values:
            Disabled
            Enabled

    Current value: Disabled
    Default value: Disabled
```

It may be necessary to run with `sudo` to be able to show the current value.

## Background

Since Linux 5.11, the kernel can expose [firmware configuration attributes] and BIOS/EFI authentication options under `/sys/class/firmware-attributes`.

On Dell systems this is used to exposes all the BIOS/EFI configuration to userspace.

## Building from source

firmconfig is written in Rust, so you need a [Rust install] to build it. firmconfig compiles with Rust 1.50 or newer.

Build it from source with:

```sh
$ git clone https://github.com/gourlaysama/firmconfig
$ cd firmconfig
$ cargo build --release
$ ./target/release/firmconfig --version
firmconfig 0.1.0-dev

```

#### License

<sub>
firmconfig is licensed under the <a href="LICENSE-MIT">MIT license</a>.
</sub>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in firmconfig shall be licensed as above,
without any additional terms or conditions.
</sub>

[Rust install]: https://www.rust-lang.org/tools/install
[firmware configuration attributes]: https://www.kernel.org/doc/html/v5.11/admin-guide/abi-testing.html#abi-sys-class-firmware-attributes-attributes