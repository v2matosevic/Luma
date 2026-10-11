//! README captures on real photographs: `cargo run --release --example showcase -- <photo folder> <output folder>`
//! Renders the current Luma UI on the CPU, creates no window and writes PNGs only into the output folder.
use std::path::Path;

fn main() {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 2 {
        eprintln!("Usage: showcase <photo folder> <output folder>");
        std::process::exit(2)
    }
    luma::visual::showcase(Path::new(&args[1]), Path::new(&args[0])).expect("showcase captures");
}
