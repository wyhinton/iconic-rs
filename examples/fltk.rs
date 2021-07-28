
use fltk::{app, app::*, enums::*, group::*, frame::Frame, prelude::*, window::Window};
extern crate iconic_rs;
use iconic_rs::{Octicons};

fn convert(val: u32)->String{
    String::from(char::from_u32(val).unwrap()) 
}
fn main() {
    let app = App::default().with_scheme(Scheme::Gleam);
    let font_size = 20; 
    app::set_font_size(font_size);
    let mut wind = Window::new(100, 100, 500, 500, "Hello from rust");
    let octicons = app.load_font("Octicons.ttf").unwrap();
    let rows = 12;
    let cols = 14;
    let mut container= Pack::new(50,100,500,400,None);
    for x in 0..rows{
        let y_pos = x *15;
        dbg!(y_pos);
        let mut container= Pack::new(100,y_pos+100,500,25,None);
        container.end();
        container.set_type(PackType::Horizontal);
        container.set_spacing(5);


        for y in 0..cols{
			let ind = cols*x + y;
            let x_pos = y*15;
            let mut frame = Frame::new(x_pos, y_pos,font_size as i32, font_size as i32+20, None);
            frame.set_label_font(Font::by_name(&octicons));
            container.add(&frame);
            if ind < Octicons::ALL_ICONS.len() as i32{
                frame.set_tooltip(Octicons::ALL_ICONS[ind as usize].0);
                frame.set_label(&convert(Octicons::ALL_ICONS[ind as usize].1));
            }
        }
    }
    container.end();
    wind.end();
    wind.show();
    app.run().unwrap();
}
