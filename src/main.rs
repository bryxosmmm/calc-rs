use std::{env::args, io::Result};

use build::build;
use parser::parse_n_write;

mod build;
mod parser;

const OUT: &str = "test";

fn main() -> Result<()> {
    let expr_raw = args()
        .nth(1)
        .unwrap_or_else(|| "(* (* 10 2) (+ 1 1))".to_string());
    let fmt = args().nth(2).unwrap_or_else(|| "RESULT: %d".to_string());
    parse_n_write(&expr_raw, fmt, OUT)?;
    build(OUT);
    Ok(())
}
