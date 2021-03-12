# Changelog

**firmconfig** is a command line tool to display firmware configuration attributes exposed by the Linux (5.11+) kernel.

<!-- next-header -->
## [Unreleased] - TBD

### Added

* A `list` subcommand to list all available attributes.
* A `print <ATTRIBUTE>` subcommand to print information about an attribute, or all of them if not specified.
* Both subcommands take a `-d/--device-name` to restrict the search to a single device, otherwise it picks the first attribute with the right name it finds.

<!-- next-url -->
[Unreleased]: https://github.com/gourlaysama/girouette/compare/757e73c...HEAD