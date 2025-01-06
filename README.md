<h1 align="center">Playground 🦀</h1>

<div align="center">

[Why?](#why) - [How?](#how)

</div>

# Why

I'm constantly experimenting with crates and specific topics in Rust, but I don't feel that a new repo suits the broad majority of these "experiments." However, I don't want to lose the code. The idea of this repo is to have small projects.

The examples may have comments that doesn't make sense. And even the code may be flaky, some calculations may be off and i could've interpreted some concepts wrong, but the overall idea is what matters.

# How

To compile and run a specific example, follow these steps:

1. **Navigate to the Example Directory**: Replace `{example_dir}` with the name of the directory containing the example you want to run.

2. **Use Cargo Command**: Execute the following command in your terminal:

```sh
cargo run --manifest-path projects/{example_dir}/Cargo.toml
```

## Example

For instance, if you want to run an example located in the `ratatui_example` directory, you would use:

```sh
cargo run --manifest-path projects/ratatui_test/Cargo.toml
```

If the following error is thrown:
```sh
error: a bin target must be available for `cargo run`
```

The project may be a library (`lib.rs`), if there are tests in the project, they can be run with `cargo test` instead of `cargo run`.

> [!NOTE]  
> Additional information and instructions can be found in each example's `src/main.rs` or each `README.md` file.