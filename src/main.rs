use whiskers::prelude::*;

#[sketch_app]
struct MySketch {
    /* add sketch parameters here */
    rectangle_width: f32,
    rectangle_height: f32,
}

impl Default for MySketch {
    fn default() -> Self {
        Self {
            /* initialize sketch parameters to default values here */
            rectangle_width: 150.,
            rectangle_height: 50.,
        }
    }
}

impl App for MySketch {
    fn update(&mut self, sketch: &mut Sketch, _ctx: &mut Context) -> anyhow::Result<()> {
        // draw code goes here
        sketch
            .color(Color::DARK_RED)
            .rect(200., 200., self.rectangle_width, self.rectangle_height);

        Ok(())
    }
}

fn main() -> Result {
    MySketch::runner()
        .with_page_size_options(PageSize::A5H)
        /* add other Runner default configuration here */
        .run()
}