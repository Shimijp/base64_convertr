mod base64_converter;
mod utils;
mod errors;

fn main() {
    println!("Hello, world!");
    let str = "aHR0cHM6Ly8xMzM3eC50by90b3JyZW50LzU2NDI5MjIvVGhlLUxlZ2VuZC1vZi1aZWxkYS1UZWFycy1vZi10aGUtS2luZ2RvbS1TV0lUQ0gv";
    let base64 = base64_converter::Base64Converter::new(str, crate::utils::Bases::Ascii).expect("Failed to create Base64Converter");

    let str_result = base64.convert(str);
    println!("Decoded string: {}", str_result);

}
