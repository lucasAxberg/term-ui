use term_ui::{self, Canvas, TerminalContext, WindowSetup};
fn main() {
    let mut ctx = TerminalContext::new(WindowSetup::default()).unwrap();
    let mut canvas = Canvas::new(&ctx);

    canvas.put_pixel(0, 0);
    canvas.put_pixel(0, 2);
    let _ = canvas.draw(&mut ctx);
    loop {}
}
