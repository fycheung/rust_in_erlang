#[macro_use]
extern crate rustler;

use rustler::{Encoder, Env, NifResult, Term};
rustler_export_nifs!(
    "test_inf", 
    [("add", 2, add)],
    Some(on_load)
);

fn on_load<'a>(_env: Env<'a>, _load_info: Term<'a>) -> bool {
    println!("Runs on library load");
    true
}

fn add<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let num1: i32 = args[0].decode()?;
    let num2: i32 = args[1].decode()?;
    Ok((num1 + num2).encode(env))
}
