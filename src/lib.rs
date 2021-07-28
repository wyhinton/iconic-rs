//! This crate contains the auto-generated mapping of icon name for several popular icon fonts.
//! This crate does not provide the fonts or svg's themselves, but provides eponymous constants for  
//! (such as `Icon::NetworkWifi`) to the character codepoint `\u{e1ba}`)
//! in Googles Material Icon Font ([https://material.io/tools/icons/](https://material.io/tools/icons/)) -
//! useful if you want to use the material-icons font in user interfaces created
//! in Rust
//!
//! ## Example
//!
//! ```rust
// ! use material_icons::{Icon, icon_to_char};
// ! let icon_char = icon_to_char(Icon::Rotation3d);
// ! assert_eq!('\u{e84d}', icon_char);
//! ```
//!
//! ## License (please read - regarding embedded font)
/// Octicons Icons

pub mod octicons;


// use fltk::{app, app::*, button::*, enums::*, group::*, frame::Frame, image::SvgImage, prelude::*, window::Window};
// use std::slice::SliceIndex;
// extern crate iconic-rs;

// fn convert(val: u32)->String{
//     String::from(char::from_u32(val).unwrap()) 
// }
// fn main() {
//     let app = App::default().with_scheme(Scheme::Gleam);
//     let mut wind = Window::new(100, 100, 500, 500, "Hello from rust");
//     let rows = 12;
//     let cols = 14;
//     let mut container= Pack::new(50,100,500,400,None);

//     for x in 0..rows{
//         let y_pos = x *15;
//         dbg!(y_pos);
//         let mut container= Pack::new(100,y_pos+100,500,25,None);
//         container.end();
//         container.set_type(PackType::Horizontal);
//         container.set_spacing(5);

//         for y in 0..cols{
// 			let ind = cols*x + y;
//             let x_pos = y*15;
//             let mut frame = Frame::new(x_pos, y_pos,font_size as i32, font_size as i32+20, None);
//             frame.set_label_font(Font::by_name(&octicons));
// 			frame.set_tooltip(&format!("{}", ind));
//             container.add(&frame);
// 			frame.set_label(&format!("{}", ind))
//         }
//     }
//     container.end();
//     wind.end();
//     wind.show();

//     app.run().unwrap();

// }