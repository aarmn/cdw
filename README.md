# CDW - Change Directory for Windows paths in WSL 🚀

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

CDW (Change Directory (cd) for Windows path) is a powerful and user-friendly command-line tool that seamlessly bridges the gap between Windows and WSL (Windows Subsystem for Linux) file systems. Say goodbye to the hassle of manually converting Windows paths to WSL paths, just copy your windows path! 🎉

## 🌟 Features

- 🔄 Effortlessly convert Windows paths to WSL paths
- 📂 Change directory using Windows-style paths in WSL
- 🐚 Support for multiple shells (Bash, Zsh, Fish, PowerShell, Nushell, Xonsh, Ksh, and Sh)
- 🚀 Easy installation and setup
- 💡 Intelligent shell detection
- 🔍 Verbose mode for detailed output
- 🛠️ Convert paths without changing directory
- 🎨 Customizable shell functions

## 🚀 Quick Start

### Install from Cargo

1. Install CDW using Cargo:
   ```
   cargo install cdw
   ```

2. Initialize CDW for your shell:
   ```
   cdw --init
   ```

3. Restart your shell or source your shell's configuration file as outputed

4. Start using CDW:
   ```
   cdw C:\Users\YourName\Documents
   ```

### Compile from Source

1. Clone the repository:
   ```
   git clone https://github.com/aarmn/cdw.git
   ```

2. Build the project:
   ```
   cd cdw
   cargo build --release
   ```

3. Initialize CDW for your shell:
   ```
   ./target/release/cdw --init
   ```

4. Restart your shell or source your shell's configuration file.

5. Start using CDW:
   ```
   cdw C:\Users\YourName\Documents
   ```

## 🛠️ Usage

```
cdw [OPTIONS] [PATH]
```

### Options

- `-i`, `--init`: Initialize shell function
- `--init-all`: Initialize shell function for all available shells
- `--init-display <SHELL>`: Display shell function for a specific shell
- `-v`, `--verbose`: Enable verbose mode
- `-c`, `--convert`: Convert path without changing directory
- `--help`: Display help information

### Examples

```bash
# Change directory to a Windows path
cdw C:\Users\YourName\Documents

# Convert a Windows path to WSL path without changing directory
cdw -c D:\Projects\MyProject

# Initialize CDW for your current shell
cdw --init

# Display the shell function for Zsh
cdw --init-display zsh

# Initialize CDW for all shells available (🚧WIP)
cdw --init-all
```

## 🐚 Supported Shells

- Bash
- Zsh
- Fish
- PowerShell
- Nushell
- Xonsh
- Sh (🚧WIP)

## 🎨 Customization

CDW automatically creates shell-specific functions and autocompletion scripts in `~/.config/cdw/`. You can customize these files to fit your needs. Just be careful, by calling `init` and `init-all` flags, it will be overwritten.

## 🚧 Current Limitations

- The `--init-all` feature is currently a work in progress.
- CDW is currently a drop-in replacement for `cd`, but without support for `cd` flags, as they are rarely used.
- CDW may not work correctly if you have `:` or `\` in your file names. This limitation is acknowledged and may be addressed in future versions.

## ✅ TODOs

- [] Add and stablize autocomplete for all supported shells
- [] Implement autocomplete for files, in shells possible
- [] Add a "no auto-complete" mode
- [] Make CDW a drop-in replacement of the system's `cd` command (activate only if using Windows-style path, pass unknown flags downstream, test for no collision with cd flags in common shells)
- [] Address the issue with files containing `:` and `\` in their names in linux, in possible drop-in replacement solutions.
- [] Cleanup code and make enum the passing data structure for shell type, not string or &str
- [] Add bookmarking and jump-like fast cd features (Maybe!)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgements

Special thanks to all the contributors and users who have helped improve CDW!

---

Made with ❤️ by [Aryan L. Horizon (AARMN)](https://github.com/aarmn)

🌟 If you find CDW useful, please consider giving it a star on GitHub! 🌟