# Rust Learning Journey

This repository documents my journey of learning the Rust programming language. It contains a collection of small projects and experiments, each focusing on a different concept or feature of Rust.

## About This Repository

This repository is structured as a Cargo workspace, which allows managing multiple related packages in one place. Each sub-directory in the `apps` folder is a separate Rust project (crate) that can be run independently.

## Workspace Structure

The repository is organized as follows:

```
/
├── apps/
│   ├── guessing_game/
│   └── variables/
├── .gitignore
├── Cargo.lock
├── Cargo.toml
└── README.md
```

### Projects

*   **guessing_game**: A simple game where the user has to guess a number between 1 and 100. This project demonstrates basic concepts like variables, loops, conditional statements, and handling user input.
*   **variables**: A small project that explores the concepts of variables and constants in Rust.

## Getting Started

To get started with this project, you'll need to have Rust and Cargo installed on your machine. You can find the installation instructions on the official [Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can clone this repository:

```bash
git clone https://github.com/your-username/rust_learning.git
cd rust_learning
```

## Usage

You can run each project individually using Cargo.

### Guessing Game

To run the guessing game, navigate to its directory and use `cargo run`:

```bash
cargo run -p guessing_game
```

### Variables

To run the variables example, use the following command:

```bash
cargo run -p variables
```

## Contributing

Since this is a personal learning project, I'm not actively seeking contributions at the moment. However, if you have any suggestions or find any issues, feel free to open an issue.

## License

This project is licensed under the terms of the MIT license. You can find the full license text in the [LICENSE](LICENSE) file.
