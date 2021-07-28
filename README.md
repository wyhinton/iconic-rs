# Iconic_rs

## Icon Utility Crate for Rust

This crate contains the auto-generated mapping of icon name for several popular icon fonts. It's purpose is to make designing GUI's in Rust with icons easier.
This crate does not provide the fonts or SVG's themselves, but a set eponymous constants for glyphs in an icon set.
Each font is represented by a struct, and each glyph in the font has a u32 value corresponding to the glyph in the font.

As a convenience, each struct also includes a vec containing all the characters in the font.

```rust
import iconic_rs::Octicons;

fn main(){
    let heart = char::from_u32(Octicons::HEART);
}

```

See the demos for more advanced usage.
