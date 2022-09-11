extern crate nalgebra as na;

use std::f64::consts::PI;

use na::{Point2, Point3, Rotation3, Vector3};
use termsize;
mod renderer;
use renderer::Renderer;

fn test() {
    let cols;
    // let rows;
    // let renderer = Renderer::new();
    match termsize::get() {
        Some(size) => {
            cols = size.cols as usize;
            // rows = size.rows as usize;
        }
        None => {
            cols = 80;
            // rows = 24;
        }
    }

    // let mut zbuffer = vec![vec![0.0; rows + 1]; cols + 1];
    // let mut output = vec![vec![' '; rows + 1]; cols + 1];
    // let pixels = ".,-~:;=!*#$@";

    let mut theta: f64; // sweeps from 0 to 2pi on the cross-section of the donut
    let mut phi: f64; // sweeps from 0 to 2pi, rotates the circle about the y_axis which creates the donut

    let circle_radius = 1.0; // R1
    let circle_center = Point3::new(2.0, 0.0, 0.0); // R2
    let k2 = 5.0;
    let k1 = cols as f64 * k2 * 3.0 / (8.0 * (circle_radius + circle_center.x));

    let mut a = 0.0; // rotates donut about x_axis
    let mut b = 0.0; // rotaties donut about z_axis
    a = a + 0.0;
    b = b + 0.0;

    for i in 0..360 {
        theta = i as f64 * PI / 180.0;

        for j in 0..360 {
            phi = j as f64 * PI / 180.0;

            let point_on_circle = circle_center
                + Vector3::new(
                    circle_radius * theta.cos(),
                    circle_radius * theta.sin(),
                    0.0,
                );

            let point_on_donut =
                Rotation3::from_axis_angle(&Vector3::y_axis(), phi) * point_on_circle;

            let point_on_donut_rotated_rotated = Rotation3::from_axis_angle(&Vector3::x_axis(), a)
                * Rotation3::from_axis_angle(&Vector3::z_axis(), b)
                * point_on_donut;

            let one_over_z = 1.0 / point_on_donut_rotated_rotated.z;

            let point_on_plane = Point2::new(
                k1 * point_on_donut_rotated_rotated.x * one_over_z,
                k1 * point_on_donut_rotated_rotated.y * one_over_z,
            );
            println!("{:?}", point_on_plane);
        }
    }

    // for row in output {
    //     println!("{}", row.iter().collect::<String>());
    // }
    // println!("{:?}", zbuffer)
}

fn print_frame() {
    let cols;
    let rows;
    match termsize::get() {
        Some(size) => {
            cols = size.cols as usize;
            rows = size.rows as usize;
        }
        None => {
            cols = 80;
            rows = 24;
        }
    }
    let mut output: Vec<Vec<char>> = vec![vec![' '; cols]; rows];
    let renderer = Renderer::new(&mut output);

    for idy in 0..renderer.output_buffer.len() {
        for idx in 0..renderer.output_buffer[0].len() {
            renderer.output_buffer[idy][idx] = (65 + idy) as u8 as char;
        }
    }

    renderer.render();
}

fn main() {
    // test();
    print_frame();
}
