use std::cmp::min;
use std::collections::HashMap;

fn encode_string_to_base(
    s: &str,
    base: u32,
    color_map: &std::collections::HashMap<char, u32>,
) -> u32 {
    s.chars()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, c)| acc + color_map[&c] * base.pow(i as u32))
}

fn _i(x: u32, base: u32, i: usize) -> u32 {
    x % base.pow(i as u32)
}

fn i_(x: u32, base: u32, i: usize) -> u32 {
    x / base.pow((l(x, base) - i) as u32)
}

fn l(x: u32, base: u32) -> usize {
    (x as f64).log(base as f64) as usize + 1
}

fn clc(p: u32, s: u32, base: u32) -> u32 {
    for i in 1..=(min(l(p, base), l(s, base))) {
        if _i(p, base, i) == i_(s, base, i) {
            return i as u32;
        }
    }
    0
}

fn main() {
    // Define the colors and their corresponding base values
    let colors = vec!['A', 'B', 'C', 'D'];
    let base = colors.len() as u32 + 1;

    // Create a color map
    let mut color_map = HashMap::new();
    for (i, &color) in colors.iter().enumerate() {
        color_map.insert(color, (i + 1) as u32);
    }

    // Example strings
    let p_str = "ABC";
    let s_str = "BCD";

    // Encode the strings
    let p_encoded = encode_string_to_base(p_str, base, &color_map);
    let s_encoded = encode_string_to_base(s_str, base, &color_map);

    // Print the encoded values
    println!("Encoded P: {}", p_encoded);
    println!("Encoded S: {}", s_encoded);

    // Check compatibility
    let clc_ps = clc(p_encoded, s_encoded, base);

    // Print the result
    if clc_ps > 0 {
        println!("The strings are compatible. Clc: {}", clc_ps)
    } else {
        println!("The strings are not compatible.");
    }
}
