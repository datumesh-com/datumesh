//! The `datumesh` command: the Datumesh local resolver. Everything is in the library,
//! [`datumesh_local::cli`]; see `datumesh --help`.

#![forbid(unsafe_code)]

fn main() {
    std::process::exit(datumesh_local::cli::main());
}
