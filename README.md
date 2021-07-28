# Iconic_rs

## Icon Utility Crate for Rust

Iconic_rs is a rust crate for working with icon fonts. It's purpose is to improve developer experience while working with icon fonts. Iconic_rs helps you find, preview, and document symbol glyphs in your app.

This crate does not provide the fonts or SVG's themselves. It provides eponymous u32 constants for each glyph in the included font list. Each font is represented by a struct, and each glyph in the font has a u32 value corresponding to a glyph in that font.

```rust
//for github's icon font, octicons
pub struct Octicons;
impl Octicons {
/// ## [heart](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/heart.svg)

	pub const HEART: u32 = 9829;
/// ## [zap](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/zap.svg)

	pub const ZAP: u32 = 9889;
/// ## [light-bulb](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/light-bulb.svg)

	pub const LIGHTBULB: u32 = 61440;
/// ## [repo](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/repo.svg)

	pub const REPO: u32 = 61441;
/// ## [repo-forked](https://raw.githubusercontent.com/wyhinton/iconic-rs/master/icons/octicons/repo-forked.svg)

	pub const REPOFORKED: u32 = 61442;
//...
```

As a convenience, each struct also includes a slice containing all the characters in the font, should you need to iterate over the icons:

```rust
impl Octicons {
//..
pub const ALL_ICONS: [(u32, &'static str); 167] = [("HEART", 9829),("ZAP", 9889),...]
}
```

```rust
import iconic_rs::Octicons;

fn main(){
    let heart = char::from_u32(Octicons::HEART);
}

```

See the demos folder for examples using fltk, egui, and gtk.
