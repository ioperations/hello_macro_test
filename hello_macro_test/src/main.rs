pub(crate) use hello_macro::HelloMacro;
use hello_macro_derve::HelloMacro;
use serde::{Deserialize, Serialize};
mod double_link_list;
mod find_meidan_from_sorted_arrays;
mod lib;
mod link_list;
mod merge_two_sorted_list;
mod ref_cell_test;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
enum Color {
    #[allow(unused)]
    Red,
    #[allow(unused)]
    Blue,
    #[allow(unused)]
    Green,
}

#[derive(HelloMacro, Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
/// zh_cn: 这是一个要被Debug的结构体
/// eng: the struct to be reflect
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
