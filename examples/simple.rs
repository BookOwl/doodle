extern crate doodle;
use doodle::*;

#[derive(Default)]
struct State {
    change: i16,
    color: i16
}

fn main() {
    let mut doodle = DoodleBuilder::new()
            .name("Test")
            .width(600)
            .height(400)
            .fps(60)
            .state(State {
                change: 1,
                color: 0,
            })
            .setup(Box::new(|_, _| {
            }))
            .draw(Box::new(|s, r| {
                let c = s.color as u8;
                s.color += s.change;
                if s.color > 255 {
                    s.color = 255;
                    s.change = -1;
                } else if s.color < 0 {
                    s.color = 0;
                    s.change = 1;
                }
                r.set_draw_color(Color::RGB(c, c, c));
                r.clear();
            }))
            .build()
            .unwrap();
        doodle.run();
}