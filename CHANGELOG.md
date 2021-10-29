# Changelog

**biosctl** is a command line tool to manage Dell BIOS/EFI settings on Linux 5.11+.

<!-- next-header -->
## [Unreleased] - TBD

## [0.3.2] - 2021-10-29

### Packaging

* The Minimum Supported Rust Version for biosctl is now 1.52.

### Changes

* `--version` output now shows more build information.

## [0.3.1] - 2021-04-21

### Added

* A `needs-reboot` subcommand now shows if the BIOS/EFI confguration has been changed and a reboot is needed.
* Better logging, with `-v/--verbose` and `-q/--quiet` options to control logging output.

### Fixed

## [0.3.0] - 2021-03-22

### Added

* A `set` subcommand to set the value of a setting.
* The `info` subcommand now shows if a reboot is pending.


## [0.2.0] - 2021-03-17

### Added

* A `get` subcommand to get the value of a setting, with `-d/--default` and `-n/--name` to get the default value or display name instead.
* An `info` subcommand to display global information about the BIOS/EFI device.

### Changed

* Renamed from `firmconfig` to `biosctl`.
* Refactored to only use a single device at a time, with `dell-wmi-sysman` the default.
* Renamed the short option for `--device-name` from `-d` to `-D` and moved it to the top level.

## [0.1.0] - 2021-03-12

### Added

* A `list` subcommand to list all available attributes.
* A `print <ATTRIBUTE>` subcommand to print information about an attribute, or all of them if not specified.
* Both subcommands take a `-d/--device-name` to restrict the search to a single device, otherwise it picks the first attribute with the right name it finds.

<!-- next-url -->
[Unreleased]: https://github.com/gourlaysama/biosctl/compare/v0.3.2...HEAD
[0.3.2]: https://github.com/gourlaysama/biosctl/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/gourlaysama/biosctl/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/gourlaysama/biosctl/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/gourlaysama/biosctl/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/gourlaysama/biosctl/compare/757e73c...v0.1.0