struct Donut {
    function_center: Vector3<f64>,
    theta: f64, // sweeps from 0 to 2pi on the cross-section of the donut
    phi: f64,   // rotates the function about the y_axis which creates the donut

    a: f64, // rotates donut about x_axis, when animating
    b: f64, // rotaties donut about z_axis
}

trait Drawable {
    fn draw(&self);
}

impl Drawable for Donut {
    fn draw(&self) {}
}
