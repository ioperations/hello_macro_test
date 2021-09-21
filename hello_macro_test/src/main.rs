use hello_macro::HelloMacro;
use hello_macro_derve::HelloMacro;

#[derive(HelloMacro)]
struct Packents;

pub fn main() {
    Packents::hello_macro();
}


