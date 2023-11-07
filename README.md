# Rust-Notes
Notes of the Rust programming language.

# Starting with Rust

## Create a new Project
```
cargo new [PROJECT_NAME]
```

## Build the Project
After modified some .rs files in `src` directory, we can build our application
with the following command:
```
cargo build
```
By default, the build in done in debug mode, to compile the code with all optimization use
instead:
```
cargo build --release
```

## Launch the Application
```
cargo run
```
