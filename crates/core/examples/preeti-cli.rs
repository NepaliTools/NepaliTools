//! Development helper: convert stdin one line at a time.
//!
//! `cargo run -q -p npt-core --example preeti-cli -- to-preeti < in.txt`
//!
//! This exists so the corpus can be generated and cross-checked against an
//! independent converter. The application does not use it.

use std::io::{self, BufRead, Write};

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "to-unicode".into());
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    for line in stdin.lock().lines() {
        let line = line.expect("stdin line");
        let result = match mode.as_str() {
            "to-preeti" => npt_core::preeti::unicode_to_preeti(&line),
            _ => npt_core::preeti::preeti_to_unicode(&line),
        };
        writeln!(stdout, "{}", result.text).expect("stdout");
        if !result.unconverted.is_empty() {
            eprintln!("unconverted: {:?}", result.unconverted);
        }
    }
}
