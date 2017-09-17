extern crate doodle;
use doodle::*;

#[derive(Default)]
struct State {}

fn main() {
    let mut doodle = DoodleBuilder::new()
            .name("Test")
            .width(600)
            .height(400)
            .fps(30)
            .state(State {})
            .setup(Box::new(|_, r| {
                r.set_draw_color(Color::RGB(100, 100, 100));
                r.clear();
            }))
            .draw(Box::new(|_, r| {
                r.clear();
            }))
            .build()
            .unwrap();
        doodle.run();
}