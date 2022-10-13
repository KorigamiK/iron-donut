extern crate nalgebra;
extern crate termsize;
mod graphics;

use graphics::renderer::Renderer;
use nalgebra::Vector2;

fn main() {
    let size = termsize::get().unwrap();
    let mut renderer = Renderer::new(size.cols as usize, size.rows as usize);

    // loop {
    // set green background
    Renderer::clear_screen();
    renderer.draw_horizontal_line(0, Vector2::new(5, 5), 5, 'w');
    // Renderer::set_background_color(String::from("48;5;21"));
    renderer.render();
    // }
}
