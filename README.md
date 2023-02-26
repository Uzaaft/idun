# Idun

## NOTE: This project is a WIP, and the following docs serves as a placeholder. For a tracking issue for the first release, please see #1

Idun is a tiling window manager for macOS, written in Rust with a focus on performance and user experience.

## Features

- Automatic tiling of windows for a clean and organized desktop layout
- Customizable key bindings for easy navigation and window management
- Dynamic resizing and repositioning of windows for a flexible and intuitive workspace
- Support for multiple displays for improved productivity and workflow

## Installation

Idun is designed to be lightweight and easy to install. To get started, simply clone the repository and build the project using Rust:

    $ git clone https://github.com/uzaaft/idun.git
    $ cd idun
    $ cargo build --release

After building the project, you can run Idun using the following command:

    $ ./target/release/idun

## Getting Started

To get started with Idun, follow these steps:

1. Start Idun by running the `idun` command from the terminal.
2. Use the default key bindings to navigate and manage your windows. For example, use `mod + j` and `mod + k` to move between windows, and use `mod + h` and `mod + l` to adjust the size of your windows.
3. To create a new window, simply open an application and it will be tiled automatically.
4. To switch between tiling mode and floating mode, use the `mod + return` key binding.
5. To quit Idun, use the `mod + q` key binding.

Customizing the appearance and behavior of Idun is easy! Simply edit the `config.toml` file in the project directory to customize key bindings, colors, and other settings.

## Roadmap

Here are some planned features and improvements for future versions of Idun:

- Improved support for external monitors and projectors
- Integration with macOS window snapping and split-screen features
- A plugin system for customizing the behavior and appearance of windows
- More intuitive and user-friendly configuration options

## Known Issues

- Currently, Idun does not support multiple desktop spaces on macOS. This is a known issue and will be addressed in a future release.

## The Name "Idun"

Idun is named after the Norse goddess of youth and renewal. The name was chosen to reflect the idea of managing windows and visual elements in a dynamic and refreshing way, while also emphasizing the clean and organized layout of a tiling window manager. Additionally, the name has a unique and distinctive sound that sets it apart from other window managers and reflects the innovative and forward-thinking design of the project.

## Contributions

Idun is an open-source project and contributions are always welcome! To get started, simply fork the repository and submit a pull request with your changes.

## License

Idun is released under the MIT License. See the LICENSE file for more details.
