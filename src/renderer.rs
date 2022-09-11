use std::f64::consts::PI;

use nalgebra::{Rotation3, Vector3};

use crate::{
    donut::{Donut, Drawable},
    range_iterator::StepRange,
};

const PIXELS: &str = ".,-~:;=!*#$@";

pub struct Renderer<'a> {
    cols: usize,
    rows: usize,
    pub output_buffer: &'a mut Vec<Vec<char>>,
    pub z_buffer: &'a mut Vec<Vec<f64>>,
    donut: Donut,
    angle_step: f64,
    pub k2: f64,
}

impl Renderer<'_> {
    pub fn new<'a>(
        output_buffer: &'a mut Vec<Vec<char>>,
        z_buffer: &'a mut Vec<Vec<f64>>,
    ) -> Renderer<'a> {
        Renderer {
            cols: output_buffer.len() - 1,
            rows: output_buffer[0].len() - 1,
            output_buffer,
            z_buffer,
            angle_step: 0.1,
            k2: 5.0,
            donut: Donut {
                theta: 0.0,
                phi: 0.0,
                a: 0.0,
                b: 0.0,
                function_center: Vector3::new(2.0, 0.0, 0.0),
                circle_radius: 1.0,
                light: Vector3::new(0.0, 1.0, -1.0),
            },
        }
    }

    pub fn render(&self) {
        // bring cursor to "home" location, in just about any currently-used
        // terminal emulation mode
        print!("\x1B[2J\x1B[1;1H");
        for i in 0..self.cols {
            for j in 0..self.rows {
                print!("{}", self.output_buffer[i][j]);
            }
            println!();
        }
    }

    pub fn luminance(&self) -> f64 {
        let normal = Rotation3::from_axis_angle(&Vector3::x_axis(), self.donut.a)
            * Rotation3::from_axis_angle(&Vector3::z_axis(), self.donut.b)
            * Rotation3::from_axis_angle(&Vector3::y_axis(), self.donut.phi)
            * self.donut.function_normal();

        normal.dot(&self.donut.light)
    }

    fn k1(&self) -> f64 {
        self.cols as f64 * self.k2 * 3.0
            / (8.0 * (self.donut.circle_radius + self.donut.function_center.x))
    }

    pub fn draw(&mut self) {
        for theta in StepRange(0.0, 2.0 * PI, self.angle_step) {
            for phi in StepRange(0.0, 2.0 * PI, self.angle_step) {
                self.donut.theta = theta;
                self.donut.phi = phi;

                let point = self.donut.point_on_donut_rotated();
                let luminance = self.luminance();
                let one_over_z = 1.0 / point.z;
                let k1 = self.k1();
                let (xp, yp) = (
                    ((self.cols as f64 / 2.0) + k1 * point.x * one_over_z).round() as usize,
                    ((self.rows as f64 / 2.0) - k1 * point.y * one_over_z).round() as usize,
                );
                if luminance > 0.0 {
                    // test against the z-buffer.  larger 1/z means the pixel is
                    // closer to the viewer than what's already plotted.
                    if xp > self.rows || yp > self.cols {
                        continue;
                    }
                    if one_over_z > self.z_buffer[yp][xp] {
                        self.z_buffer[yp][xp] = one_over_z;
                        let luminance_index = luminance.round() as usize * 8;
                        // luminance_index is now in the range 0..11 (8*sqrt(2) = 11.3)
                        // now we lookup the character corresponding to the
                        // luminance and plot it in our output:
                        self.output_buffer[yp][xp] = PIXELS.chars().nth(luminance_index).unwrap();
                    }
                }
            }
        }
        self.render();
    }

    pub fn set_angle(&mut self, a: f64, b: f64) {
        self.donut.a = a;
        self.donut.b = b;
    }

    pub fn animate(&mut self) {
        for a in StepRange(0.0, 2.0 * PI, 0.1) {
            for b in StepRange(0.0, 2.0 * PI, 0.1) {
                self.set_angle(a, b);
                self.draw();
            }
        }
    }
}
