extern crate nalgebra as na;

mod utils;
mod geometry;
mod camera;
mod visual;

use camera::{Camera};

fn main() {
    let img_width  : usize = 400;
    let img_height : usize = 400;

    // Coordiante system is centered at camera's origin.
    let cam = Camera::new(img_width, img_width, 1.0, na::Point3::<f64>::new(0.0, 0.0, 0.0));
    let points = cam.generate_viewframe_points(); 
    let rays = cam.generate_rays(&points);

    let spheres = vec![geometry::shapes::Sphere{radius: 0.33, origin: na::Point3::<f64>::new(0.0, 0.0, -1.0)}];

    // Trace the ray-olios
    let hits = geometry::tracing::compute_object_intersections(&rays, &spheres); 

    // Visualize stuff.
    let rgb_values = visual::colorize_hits(&hits);
    let ppm_contents = visual::convert_rgb_vec_to_str(&rgb_values); 

    utils::write_ppm_to_file(&"temp.ppm".to_string(), &ppm_contents, img_width, img_height);
}
