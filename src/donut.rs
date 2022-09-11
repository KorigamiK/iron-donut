use nalgebra::{Rotation3, Vector3};
pub struct Donut {
    pub function_center: Vector3<f64>,
    pub light: Vector3<f64>,

    pub theta: f64, // sweeps from 0 to 2pi on the cross-section of the donut
    pub phi: f64,   // rotates the function about the y_axis which creates the donut

    pub a: f64, // rotates donut about x_axis, when animating
    pub b: f64, // rotaties donut about z_axis

    pub circle_radius: f64, // R1
}

pub trait Drawable {
    fn draw(&self);

    fn function(&self) -> Vector3<f64>;
    fn function_normal(&self) -> Vector3<f64>;
}

impl Drawable for Donut {
    fn function(&self) -> Vector3<f64> {
        Vector3::new(
            self.circle_radius * self.theta.cos(),
            self.circle_radius * self.theta.sin(),
            0.0,
        )
    }
    fn function_normal(&self) -> Vector3<f64> {
        Vector3::new(self.theta.cos(), self.theta.sin(), 0.0)
    }
    fn draw(&self) {}
}

impl Donut {
    fn point_on_function(&self) -> Vector3<f64> {
        self.function() + self.function_center
    }

    fn point_on_donut(&self) -> Vector3<f64> {
        Rotation3::from_axis_angle(&Vector3::y_axis(), self.phi) * self.point_on_function()
    }

    pub fn point_on_donut_rotated(&self) -> Vector3<f64> {
        Rotation3::from_axis_angle(&Vector3::x_axis(), self.a)
            * Rotation3::from_axis_angle(&Vector3::z_axis(), self.b)
            * self.point_on_donut()
    }
}
