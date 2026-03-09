use whiskers::prelude::*;

#[sketch_app]
struct MySketch {
    /* add sketch parameters here */
    starting_square_side: f32,
    max_square_side: f32,
    line_spacing: f32
}

impl Default for MySketch {
    fn default() -> Self {
        Self {
            /* initialize sketch parameters to default values here */
            starting_square_side: 10.,
            max_square_side: 570.,
            line_spacing: 13.
        }
    }
}

impl App for MySketch {
    fn update(&mut self, sketch: &mut Sketch, _ctx: &mut Context) -> anyhow::Result<()> {
        // draw code goes here
        sketch.color(Color::DARK_RED);

        let mut side = self.starting_square_side;
        sketch.rect(0., 0., side, side).rotate(90);

        while  side <= self.max_square_side {
            side+= self.line_spacing;
            sketch.rect(0., 0., side, side).rotate(90);
        }

        Ok(())
    }
}

fn main() -> Result {
    MySketch::runner()
        .with_page_size_options(PageSize::A5H)
        /* add other Runner default configuration here */
        .run()
}