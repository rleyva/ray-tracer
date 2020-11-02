// Utilities

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Public function used to write a PPM-formatted string to a file.
pub fn write_ppm_to_file(file_path: &String, ppm_content: &String, width: usize, height: usize) {
    // Header given to generated PPMs.
    let header = "P3\n".to_string()
                    + &width.to_string() + " " + &height.to_string() + &"\n".to_string()
                    + &"255".to_string() + &"\n".to_string();

    let file_contents = header + ppm_content;

    // Handle file shenanigans here.
    let path    = Path::new(file_path);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Could not create file at {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(file_contents.as_bytes()) {
        Err(why) => panic!("Could not write to {}: {}", display, why),
        Ok(_) => println!("Successfully wrote to {}.", display),
    };
}

// Public debug function used to create some PPM output.
pub fn create_dummy_ppm_contents(width: usize, height: usize) -> String {
    let mut ppm_contents = String::with_capacity(width * height);

    for h in 0..height {
        for w in 0..width {
            let r = w as f64 / width as f64;
            let g = h as f64 / height as f64;
            let b = 0.25;

            // Rescale values, and store as string.
            let r_str = &((r * 255.0) as u8).to_string();
            let g_str = &((g * 255.0) as u8).to_string();
            let b_str = &((b * 255.0) as u8).to_string();

            // Push to string.
            ppm_contents.push_str(&(r_str.to_owned() + " " + g_str + " " + b_str + "\n"));
        }
    }
    return ppm_contents;
}
