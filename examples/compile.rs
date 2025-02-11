// usage:  cargo run --release --example compile

use fasteval2::Compiler;
use fasteval2::{EmptyNamespace, Evaler}; // use this trait so we can call eval().
use std::collections::BTreeMap; // use this trait so we can call compile().
fn main() -> Result<(), fasteval2::Error> {
    let parser = fasteval2::Parser::new();
    let mut slab = fasteval2::Slab::new();
    let mut map = BTreeMap::new();

    let expr_str = "sin(deg/360 * 2*pi())";
    let compiled = parser
        .parse(expr_str, &mut slab.ps)?
        .from(&slab.ps)
        .compile(&slab.ps, &mut slab.cs, &mut EmptyNamespace);
    for deg in 0..360 {
        map.insert("deg".to_string(), deg as f64);
        // When working with compiled constant expressions, you can use the
        // eval_compiled*!() macros to save a function call:
        let val = fasteval2::eval_compiled!(compiled, &slab, &mut map);
        eprintln!("sin({}°) = {}", deg, val);
    }

    Ok(())
}
