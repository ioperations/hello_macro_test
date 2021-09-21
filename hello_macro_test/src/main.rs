use hello_macro::HelloMacro;
use hello_macro_derve::HelloMacro;
use serde::{Serialize,Deserialize};

#[derive(Debug,Clone, Copy,PartialEq,PartialOrd)]
#[derive(Serialize,Deserialize)]
enum Color {
    #[allow(unused)]
    Red,
    #[allow(unused)]
    Blue,
    #[allow(unused)]
    Green,
}

#[derive(HelloMacro)]
#[derive(Clone,Debug, PartialEq, PartialOrd)]
#[derive(Serialize,Deserialize)]
struct Packents {
    #[allow(unused)]
    #[serde(skip_serializing)]
    name: u32,
    #[allow(unused)]
    #[serde(skip_serializing)]
    good: &'static str,
    #[allow(unused)]
    #[serde(skip_serializing)]
    test: Color,
}

pub fn main() {
    Packents::hello_macro();
}
