pub struct Renderer<'a> {
    cols: usize,
    rows: usize,
    pub output_buffer: &'a mut Vec<Vec<char>>,
    // donut: Donut,
}

impl Renderer<'_> {
    pub fn new<'a>(output_buffer: &'a mut Vec<Vec<char>>) -> Renderer<'a> {
        Renderer {
            cols: output_buffer.len(),
            rows: output_buffer[0].len(),
            output_buffer,
        }
    }

    pub fn render(&self) {
        for i in 0..self.cols {
            for j in 0..self.rows {
                print!("{}", self.output_buffer[i][j]);
            }
            println!();
        }
    }
}
