# Gel Package Manager

**A Lightweight, Fast, and Reliable Package Manager for Arus**

[![License: GPLv3](https://img.shields.io/badge/license-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

Welcome to Gel, a powerful package manager designed with simplicity, speed, and reliability in mind. Whether you're a developer looking to package your software or a user seeking to effortlessly manage and install packages, Gel has got you covered.

## Features

- **Lightweight:** Gel is designed to be minimalistic, ensuring it doesn't burden your system with unnecessary bloat.

- **Blazing Fast:** Thanks to its efficient Rust implementation, Gel offers lightning-fast package management operations.

- **Package Creation:** Developers can easily create and package their software for distribution using Gel.

- **User-Friendly:** Gel provides a straightforward and user-friendly command-line interface for hassle-free package management.

- **Community Repositories:** Discover a wealth of packages in our community repositories, expanding your software choices.

## Installation

To install Gel, follow these simple steps:

```shell
# Clone the Gel repository
git clone https://github.com/Arus-OS/gel.git

# Change into the Gel directory
cd gel

# Build Gel using Cargo
cargo build

# Install Gel
cargo install --path .
```

## Usage

Once Gel is installed, you can start managing packages with ease. Here are some common commands:

- `gel install <package>`: Install a package.
- `gel remove <package>`: Remove a package.
- `gel search <query>`: Search for packages.
- `gel list`: List installed packages.
- `gel update`: Update installed packages.

For more detailed information, [see](https://github.com/Arus-OS/gel/blob/main/src/text/help.txt).

## Contributing

We welcome contributions from the open-source community. Feel free to fork this repository, make your improvements, and submit a pull request. We appreciate your help in making Gel even better!

## License

Gel is licensed under the GNU General Public License, version 3.0. See the [LICENSE](LICENSE) file for more details.
