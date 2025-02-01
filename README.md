# Cargo Project Finder

Cargo Project Finder is a CLI tool designed to recursively find Cargo projects in directories. This tool is particularly useful for developers working with multiple Rust projects, allowing them to quickly locate and manage their Cargo projects.

## Features

- Recursively search directories for Cargo projects.
- Option to skip hidden files and directories.
- Output results to a JSON file.
- Display results in a table format.
- Works on Unix and Windows operating systems.

## Installation

You can install the cli tool globally using Cargo:

```sh
cargo install cargo-project-finder
```

## Usage

You can use Cargo Project Finder by running the following command:

```sh
cargo-project-finder [OPTIONS]
```

### Options

- `-o, --output <FILE>`: Write output to a JSON file (defaults to `cargo_projects`).
- `-p, --path <DIR>`: Directory to search (defaults to home directory).
- `-n, --noskip`: Don't skip hidden files and directories (will take some time).

### Examples

Search the home directory for Cargo projects and output the results to a JSON file:

```sh
cargo-project-finder --output projects
```

Search a specific directory for Cargo projects without skipping hidden files (This will take time):

```sh
cargo-project-finder --path /path/to/directory --noskip
```

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## Author

Ahmet Özcan - [ahmetozcan21@yahoo.com](mailto:ahmetozcan21@yahoo.com)
