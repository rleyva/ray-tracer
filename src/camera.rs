extern crate num;

use crate::geometry;

// 
// Viewport
//

// TODO: Figure out where to put this.
fn generate_viewport_corners(width: usize, height: usize) -> Vec<na::Point2<f64>> {
    // Compute aspect ratio.
    let gcd = num::integer::gcd(width as i32, height as i32); 
    let width_ratio = width as i32 / gcd;
    let height_ratio = height as i32 / gcd;

    let midpoint = na::Point2::<f64>::new(width_ratio as f64 / 2.0, height_ratio as f64 / 2.0);
    
    // Translation matrix
    // NOTE: The array indexing is pretty weird...
    let mut translation = na::Matrix3::<f64>::identity();
    translation[(0, 2)] = -midpoint.x;
    translation[(1, 2)] = midpoint.y;

    // Generate points for the viewport.
    let mut corners = vec!(na::Point2::<f64>::new(width_ratio as f64, -(height_ratio as f64)),
                           na::Point2::<f64>::new(width_ratio as f64, height_ratio as f64 - height_ratio as f64), 
                           na::Point2::<f64>::new(width_ratio as f64 - width_ratio as f64, -(height_ratio as f64)),
                           na::Point2::<f64>::new(width_ratio as f64 - width_ratio as f64, height_ratio as f64 - height_ratio as f64));

    // Translate points.
    for point in &mut corners {
        *point = na::Point2::from_homogeneous(translation * point.to_homogeneous()).unwrap();
    }

    return corners;
}

pub struct ViewFrame {
    origin: na::Point3::<f64>,
    upper_left: na::Point3::<f64>, 
    upper_right: na::Point3::<f64>,
    lower_left: na::Point3::<f64>,
    lower_right: na::Point3::<f64>
}

impl ViewFrame {
    fn new(width: usize, height: usize, distance_m: f64) -> ViewFrame {
        assert!(distance_m > 0.0, " Very small or negative distance provided!");

        let planar_corners = generate_viewport_corners(width, height);

        // Convert them over to 3D.
        // NOTE: In order to maintain right-hand coordinate sys, -z is in front of the camera
        //       origin.
        // NOTE: There's some implicit bruhaha happening here were we are taking the pixel-heights,
        //       and combining them with a physical distance in meters. 
        let mut viewport_corners = Vec::with_capacity(4);
        for corner in planar_corners {
            viewport_corners.push(na::Point3::<f64>::new(corner.x, corner.y, -distance_m));
        }  

        let viewframe = ViewFrame {
            origin: na::Point3::<f64>::new(0.0, 0.0, -distance_m),
            upper_left: viewport_corners[3],
            lower_left: viewport_corners[2],
            upper_right: viewport_corners[1], 
            lower_right: viewport_corners[0]
        };

        println!("{}", viewframe);

        return viewframe;
    }
}

impl std::fmt::Display for ViewFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Origin: {}\nUpper Left: {}\nUpper Right: {}\nLower Left: {}\nLower Right: {}\n",
            self.origin, self.upper_left, self.upper_right, self.lower_left, self.lower_right)
    }
}

pub struct Camera {
    width: usize,
    height: usize,
    focal_length_m: f64,
    origin: na::Point3<f64>,
    viewframe: ViewFrame, 
} 

impl Camera {
   pub fn new(width: usize, height: usize, focal_length_m: f64, origin: na::Point3::<f64>) -> Self {
        return Self {
            width,
            height,
            focal_length_m,
            origin,
            viewframe: ViewFrame::new(width, height, focal_length_m),
        } 
    }

    pub fn generate_viewframe_points(&self) -> Vec::<na::Point3::<f64>>{
        // Function will generate a vector of coordinates in the viewport which correspond
        // to a pixel in the given frame.
        
        let vertical_span = num::abs(self.viewframe.upper_left.x - self.viewframe.upper_right.x) as f64;  
        let horizontal_span = num::abs(self.viewframe.upper_left.y - self.viewframe.lower_left.y) as f64;

        let width_inc = 1.0 / self.width as f64;
        let height_inc = 1.0 / self.height as f64;

        let mut viewframe_points = Vec::with_capacity((vertical_span * horizontal_span) as usize);

        for h in 0..self.height {
            for w in 0..self.width {
                let x = self.viewframe.upper_left.x + w as f64 * width_inc * horizontal_span;
                let y = self.viewframe.upper_left.y - h as f64 * height_inc * vertical_span;
            
                viewframe_points.push(na::Point3::<f64>::new(x, y, -1.0 * self.focal_length_m));
            }
        }

        return viewframe_points;
    }       

    pub fn generate_rays(&self, points: &Vec<na::Point3::<f64>>) -> Vec::<geometry::Ray>{
        let mut rays = Vec::with_capacity(points.len());
        for point in points {
            rays.push(geometry::Ray::new(self.origin, point.coords));
        }
        
        return rays;
    }
}
