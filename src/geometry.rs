use na::Point3;

pub struct Ray {
    // Simple ray class
    
    // Ray in the form P(t) = A + tB
    // P(t): 3D position along a line in 3D.
    // A:    Origin of the ray.
    // B:    Ray direction.
    pub origin : na::Point3<f64>,
    pub direction : na::Vector3<f64>
}

impl Ray {
    // Create new ray
    pub fn new(origin : na::Point3<f64>, direction : na::Vector3<f64>) -> Ray {
        Ray{origin: origin, direction: direction}
    }

    // Ray projection function - handles projecting out ray from origin, along the direction
    // vector up until the value t.
    pub fn at(&self, t : f64) -> Point3<f64> {
        Point3::from(self.origin.coords + t * self.direction)
    }
}

impl std::fmt::Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{Origin: [{}, {}, {}], Direction: [{}, {}, {}]}}", self.origin.x, self.origin.y,
            self.origin.z, self.direction.x, self.direction.y, self.direction.z)
    }
}

// Shapes!
pub mod shapes {
    use crate::geometry;
    use crate::geometry::tracing::{Hit};

    pub struct Sphere {
        pub radius: f64,
        pub origin: na::Point3<f64>,
    }

    impl Sphere {
        // TODO: Make all shapes inherit form a Shape class which has to implement an
        //       `intersects_with` function.
        pub fn intersects_with(&self, ray: &geometry::Ray) -> Option<Hit> {
            let ray_to_sphere = ray.origin.coords - self.origin.coords;

            let a = ray.direction.dot(&ray.direction);
            let b = 2.0 * ray_to_sphere.dot(&ray.direction); 
            let c = ray_to_sphere.dot(&ray_to_sphere) - self.radius * self.radius;
        
            let descriminant = b * b - 4.0 * a * c;

            if descriminant < 0.0 {
                // No hit was registered.
                return None;
            }
            else {
                // Hit was registered.
                
                // Compute t, and use that to compute the surface normal.
                let t = -b - num::Float::sqrt(descriminant) / 2.0 * a;
                let hit_point = ray.at(t);
                let surface_normal = (hit_point.coords - self.origin.coords).normalize(); 
                
                return Some(Hit{t: t, hit_point: hit_point, surface_normal: surface_normal});
            }
        }
    }
}

// Ray tracing fun!
pub mod tracing {
    use crate::geometry::{Ray};
    use crate::geometry::shapes::{Sphere};

    pub struct Hit {
        pub t: f64,
        pub hit_point: na::Point3<f64>, 
        pub surface_normal: na::Vector3<f64>,
    }

    pub fn compute_object_intersections(rays: &Vec<Ray>, objects: &Vec::<Sphere>) -> Vec<Option<Hit>> {
        // The structure of the ppm is linear, why not save the results in the same manner? 
        let mut hits = Vec::with_capacity(rays.len());

        for ray in rays {
            for obj in objects {
                // Check to see if this ray intersects with the given shape.
                let result = obj.intersects_with(ray);
                hits.push(result);
            }
        }

        return hits;
    }
}
