#[macro_use]
extern crate conrod;

use conrod::backend::glium::glium::{self, Surface};
use conrod::{color, widget, Colorable, Positionable, Widget};

pub fn main() {
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Hello Conrod!")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Generate the widget identifiers.
    widget_ids!(struct Ids { text });
    let ids = Ids::new(ui.widget_id_generator());

    // // Add a `Font` to the `Ui`'s `font::Map` from file.
    const FONT_PATH: &'static str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/fonts/NotoSans/NotoSans-Regular.ttf"
    );
    ui.fonts.insert_from_file(FONT_PATH).unwrap();

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    events_loop.run_forever(|event| {
        match event.clone() {
            glium::glutin::Event::WindowEvent { event, .. } => match event {
                // Break from the loop upon `Escape` or closed window.
                glium::glutin::WindowEvent::CloseRequested
                | glium::glutin::WindowEvent::KeyboardInput {
                    input:
                        glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => return glium::glutin::ControlFlow::Break,

                _ => (),
            },
            _ => (),
        }

        // Use the `winit` backend feature to convert the winit event to a conrod one.
        let input = match conrod::backend::winit::convert_event(event, &display) {
            None => return glium::glutin::ControlFlow::Continue,
            Some(input) => input,
        };

        // Handle the input with the `Ui`.
        ui.handle_event(input);

        // Set the triangle widget.
        {
            let ui: &mut conrod::UiCell = &mut ui.set_widgets();
            // let rect = ui.rect_of(ui.window).unwrap();
            widget::Text::new("sadfa")
                // .font_id(fonts.regular)
                .color(color::LIGHT_RED)
                // .padded_w_of(ids.left_col, PAD)
                // .mid_top_with_margin_on(ids.left_col, PAD)
                .middle_of(ui.window)
                .left_justify()
                .line_spacing(10.0)
                .set(ids.text, ui);
        }

        // Draw the `Ui` if it has changed.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }

        glium::glutin::ControlFlow::Continue
    });
}
