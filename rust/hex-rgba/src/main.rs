fn hex_to_rgba_float(hex: &str) -> (f32, f32, f32, f32) {
    let r = i32::from_str_radix(&hex[1..3], 16).unwrap() as f32 / 255.0;
    let g = i32::from_str_radix(&hex[3..5], 16).unwrap() as f32 / 255.0;
    let b = i32::from_str_radix(&hex[5..7], 16).unwrap() as f32 / 255.0;
    let a = 1.0;

    (r, g, b, a)
}

fn main() {
    let catppuccin_frappe = vec![
        ("rosewater", hex_to_rgba_float("#f2d5cf")),
        ("flamingo", hex_to_rgba_float("#eebebe")),
        ("pink", hex_to_rgba_float("#f4b8e4")),
        ("mauve", hex_to_rgba_float("#ca9ee6")),
        ("red", hex_to_rgba_float("#e78284")),
        ("maroon", hex_to_rgba_float("#ea999c")),
        ("peach", hex_to_rgba_float("#ef9f76")),
        ("yellow", hex_to_rgba_float("#e5c890")),
        ("green", hex_to_rgba_float("#a6d189")),
        ("teal", hex_to_rgba_float("#81c8be")),
        ("sky", hex_to_rgba_float("#99d1db")),
        ("sapphire", hex_to_rgba_float("#85c1dc")),
        ("blue", hex_to_rgba_float("#8caaee")),
        ("lavender", hex_to_rgba_float("#babbf1")),
        ("text", hex_to_rgba_float("#c6d0f5")),
        ("subtext1", hex_to_rgba_float("#b5bfe2")),
        ("subtext0", hex_to_rgba_float("#a5adce")),
        ("overlay2", hex_to_rgba_float("#949cbb")),
        ("overlay1", hex_to_rgba_float("#838ba7")),
        ("overlay0", hex_to_rgba_float("#737994")),
        ("surface2", hex_to_rgba_float("#626880")),
        ("surface1", hex_to_rgba_float("#51576d")),
        ("surface0", hex_to_rgba_float("#414559")),
        ("base", hex_to_rgba_float("#303446")),
        ("mantle", hex_to_rgba_float("#292c3c")),
        ("crust", hex_to_rgba_float("#232634")),
    ];
    for (name, (r, g, b, a)) in catppuccin_frappe {
        println!(
            "pub const {:<10}: Srgba = Srgba::new({:5.3}, {:5.3}, {:5.3}, {:5.3});",
            name, r, g, b, a
        );
    }
}
