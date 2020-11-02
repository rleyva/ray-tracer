extern crate nalgebra as na;
use crate::geometry;

use crate::geometry::tracing::{Hit};

pub struct RGB {
    r: u8,
    g: u8,
    b: u8
}

pub mod Colors {
    use crate::visual::{RGB};

    // Wtf is this...?
    pub const BLACK: RGB = RGB{r: 0, g: 0, b: 0};
    pub const WHITE: RGB = RGB{r: 255, g: 255, b: 255};
    pub const RED: RGB = RGB{r: 255, g: 0, b: 0};
    pub const GREEN: RGB = RGB{r: 0, g: 255, b: 0};
    pub const BLUE: RGB = RGB{r: 0, g: 0, b: 255};
}

impl std::fmt::Display for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "){{[R: {}, G: {}, B: {}]}}", self.r, self.g, self.b)
    }
}

impl RGB {
    fn to_string(&self) -> String {
        return self.r.to_string() + " " + &self.g.to_string().to_owned() + " " + &self.b.to_string().to_owned() + "\n"; 
    }
}

pub fn convert_rgb_vec_to_str(rgb_vec: &Vec<RGB>) -> String {
    let mut mega_string: String = "".to_owned();
    for rgb in rgb_vec {
        let rgb_str = rgb.to_string();
        mega_string.push_str(&rgb_str); 
    }
    return mega_string;
}

pub fn colorize_surface_normals(vec: &na::Vector3<f64>) -> RGB {
    // Assume that rays are normalized.
    return RGB{r: (num::abs(vec.x) * 255.0) as u8,
               g: (num::abs(vec.y) * 255.0) as u8,
               b: (num::abs(vec.z) * 255.0) as u8};
}

pub fn colorize_rays(rays: &Vec::<geometry::Ray>) -> Vec::<RGB> {
    // Simple function that will take in a ray, and remap that to RGB.
    // This will involve making the following mapping: x -> r, y -> g, z -> b.

    let mut rgb_values = Vec::with_capacity(rays.len());
    for _ray in rays {
        
        let mut normalized_ray = na::Point3::<f64>::new(0.0, 0.0, 0.0);

        //normalized_ray.x = num::abs(normalized_ray.x) * 255.0;
        //normalized_ray.y = num::abs(normalized_ray.y) * 255.0;
        //normalized_ray.z = num::abs(normalized_ray.z) * 255.0;

        rgb_values.push(RGB{r: normalized_ray.x as u8, g: normalized_ray.y as u8, b: normalized_ray.z as u8});
    }
    
    return rgb_values;
}

pub fn colorize_hits(hits: &Vec<Option<Hit>>) -> Vec<RGB> {
    let mut colors = Vec::<RGB>::with_capacity(hits.len());

    let mut hit_count: i32 = 0;
    let mut not_hit_count: i32 = 0;

    for hit in hits {
        match hit {
            None => {
                colors.push(Colors::BLACK);
                not_hit_count = not_hit_count + 1;
            },
            Some(h) => {
                let mut rescaled_normal = h.surface_normal;
                rescaled_normal.z = rescaled_normal.z;
                rescaled_normal = rescaled_normal.normalize();

                colors.push(colorize_surface_normals(&rescaled_normal));
                hit_count = hit_count + 1;
            }
        }
    }

    return colors;
}
