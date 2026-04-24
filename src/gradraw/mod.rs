use color_convert::color::Color;
use color_convert::handles::hex::hex2rgb;

pub fn gradient_print(start_color: String, end_color: String, text: &str) {
    let mut correct_color_start: String = String::new();
    let mut correct_color_end: String = String::new();

    if start_color.starts_with("#") {
        correct_color_start = start_color[1..].to_string();

        if end_color.starts_with("#") {
            correct_color_end = end_color[1..].to_string();
        }
    }

    let start_starts_with_valid_hex = correct_color_start.chars().next().map_or(false, |c| {
        matches!(c, '0'..='9' | 'a'..='f' | 'A'..='F')
    });
    let end_starts_with_valid_hex = correct_color_end.chars().next().map_or(false, |c| {
        matches!(c, '0'..='9' | 'a'..='f' | 'A'..='F')
    });

    validate_hex_colors(start_starts_with_valid_hex, end_starts_with_valid_hex);

    let start_color = Color::new(&correct_color_start);
    let end_color = Color::new(&correct_color_end);
    let rgb_start_color = hex2rgb(&start_color).unwrap();
    let rgb_end_color = hex2rgb(&end_color).unwrap();
    
    println!("{} --- {}", rgb_start_color, rgb_end_color);

    /*let text_len = text.len();
    for (i, c) in text.chars().enumerate() {
        let t = i as f32 / (text_len - 1) as f32;
        
        let r = lerp(rgb_start_color.0, rgb_end_color.0, t);
        let g = lerp(rgb_start_color.1, rgb_end_color.1, t);
        let b = lerp(rgb_start_color.2, rgb_end_color.2, t);
        
        print!("\x1b[38;2;{};{};{}m{}", r, g, b, c);
    }
    print!("\x1b[0m");*/
}
    
pub fn lerp(start: u8, end: u8, t: f32) -> u8 {
    ((start as f32) * (1.0 - t) + (end as f32) * t) as u8
}
fn validate_hex_colors(start_valid: bool, end_valid: bool) {
    match (start_valid, end_valid) {
        (false, false) => panic!("Invalid hex colors: both arguments are invalid"),
        (false, true) => panic!("Invalid hex color: argument 1 is invalid"),
        (true, false) => panic!("Invalid hex color: argument 2 is invalid"),
        (true, true) => (),
    }
}