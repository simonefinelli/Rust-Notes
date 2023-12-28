/*
    In Rust we can use three modes to organize our code:
     - Packages:
        Packages are managed through Cargo commands (cargo build, cargo test). A single Package can contain one or more
        Crates. Inside a Package we find a .toml that is used as central configuration file (managing metadata, dependencies,
        hot to build the crates, info on optional features).
        They are the highest level of code organization.
        *** They must have at least one Crate and at most 1 Library Crate, but any number of binary Crates. ***

        To create a new Package we can use the following command:
            ~$ cargo new package_name
        TIP: in this case the package_name is the name of the project!

        Using
            ~$ cargo run
        we can create the executable (placed in target folder).

        To run a binary we can use:
            ~$ cargo run --bin package_name  // package_name in this cas is _01_basic_equipment
            ~$ cargo run --bin bin  // using specific bin in bin folder

     - Crates:
        A Crate is a compilation unit that houses a set of Modules and their associated items (functions and structures).
        From to a Project prospective, a Crate is essentially a tree of Modules (hierarchical structure).
        We can have a binary Crate (produce an executable) or a Library Crate (for sharing purposes).
     - Modules:
        Allow us to group items at a finer level (controlling visibility and privacy).

     A typical Structure of a project is:

     +--- Package --------------------------------------------------------------------------------------------+
     |                                                                                                        |
     |  +--- Binary Crate (main.rs) -------------------+    +--- Library Crate (lib.rs) -------------------+  |
     |  |                                              |    |                                              |  |
     |  | +--- Root Module -------------------------+  |    | +--- Root Module -------------------------+  |  |
     |  | |                                         |  |    | |                                         |  |  |
     |  | |  +--- Module 1 ---+ +--- Module 2 ---+  |  |    | |  +--- Module 1 ---+ +--- Module 2 ---+  |  |  |
     |  | |  |                | |                |  |  |    | |  |                | |                |  |  |  |
     |  | |  +----------------+ +----------------+  |  |    | |  +----------------+ +----------------+  |  |  |
     |  | |  +--- Module 3 ---+ +--- Module N ---+  |  |    | |  +--- Module 3 ---+ +--- Module N ---+  |  |  |
     |  | |  |                | |                |  |  |    | |  |                | |                |  |  |  |
     |  | |  +----------------+ +----------------+  |  |    | |  +----------------+ +----------------+  |  |  |
     |  | +-----------------------------------------+  |    | +-----------------------------------------+  |  |
     |  +----------------------------------------------+    +----------------------------------------------+  |
     |                                                                                                        |
     +--------------------------------------------------------------------------------------------------------+
 */
fn main() {
    println!("Hello, world!");
}
