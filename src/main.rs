use orbtk::prelude::*;

fn main() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("Slider resize example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .resizable(true)
                .child(Slider::new().val(80).build(ctx))
                .build(ctx)
        })
        .run();
}
