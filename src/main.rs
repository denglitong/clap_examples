#[macro_use]
extern crate clap;

mod flag_args;
mod option_args;
mod quick_example;
mod typed_values;

fn main() {
    // quick_example::a::main();
    // quick_example::b::main();
    // quick_example::c::main();
    // flag_args::main();
    // option_args::main();
    typed_values::a::main();
}
