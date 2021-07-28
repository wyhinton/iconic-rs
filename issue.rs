use fltk::{app, app::*, button::*, enums::*, group::*, frame::Frame, image::SvgImage, prelude::*, window::Window};
use std::slice::SliceIndex;
fn convert(val: u32)->String{
    String::from(char::from_u32(val).unwrap()) 
}
fn main() {
    let app = App::default().with_scheme(Scheme::Gleam);
    let mut wind = Window::new(100, 100, 500, 500, "Hello from rust");
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
			frame.set_tooltip(&format!("{}", ind));
            container.add(&frame);
			frame.set_label(&format!("{}", ind))
        }
    }
    container.end();
    wind.end();
    wind.show();

    app.run().unwrap();

}