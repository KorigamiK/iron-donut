use nalgebra::Vector2;

struct RenderSection {
    start_point: Vector2<usize>,
    size: Vector2<usize>,
}

pub struct Renderer {
    size: Vector2<usize>,
    sections: Vec<RenderSection>,
    console: Vec<Vec<String>>,
}

impl Renderer {
    pub fn new(x: usize, y: usize) -> Renderer {
        let sections = Vec::new();
        let console = {
            let mut console = Vec::new();
            for _y in 0..y {
                let mut row = Vec::new();
                for _x in 0..x {
                    row.push('█'.to_string());
                }
                console.push(row);
            }
            console
        };
        let mut renderer = Renderer {
            size: Vector2::new(0, 0),
            sections,
            console,
        };
        renderer.sections.push(RenderSection {
            start_point: Vector2::new(0, 0),
            size: Vector2::new(x, y),
        });

        renderer
    }

    pub fn insert_section(&mut self, start_point: Vector2<usize>, size: Vector2<usize>) {
        self.sections.push(RenderSection { start_point, size });
    }

    pub fn render(&self) {
        for row in &self.console {
            for e in row {
                print!("{}", e);
            }
            println!();
        }
    }

    pub fn clear_section(&mut self, index: usize) {
        let section = &self.sections[index];
        for y in 0..(section.size.y) {
            for x in 0..(section.size.y) {
                self.console[section.start_point.y + y][section.start_point.x + x] =
                    ' '.to_string();
            }
        }
    }

    pub fn set_background_color(color: String) {
        print!("\x1b[{}", color);
    }

    pub fn clear_screen() {
        print!("\x1b[2J");
    }

    pub fn draw_horizontal_line(
        &mut self,
        section_idx: usize,
        begin: Vector2<usize>,
        length: usize,
        ch: char,
    ) {
        let start = &self.sections[section_idx];
        for x in 0..length {
            self.console[start.start_point.y + begin.y][start.start_point.x + begin.x + x] =
                ch.to_string()
        }
    }
}
