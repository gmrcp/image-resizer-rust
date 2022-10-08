use std::thread;
use std::sync::Arc;
mod lib;
use lib::resolution::TargetResolution;
use lib::image_proc::rescale;

fn main() {
    let target_resolutions: [TargetResolution; 4] = [
        TargetResolution::new(30, 1101023),
        TargetResolution::new(1430, 30),
        TargetResolution::new(20, 530),
        TargetResolution::new(1300, 700),
    ];

    let file_name = "another".to_string();
    let file_path = format!(".test/input/{}.jpg", file_name);
    let img = image::open(file_path).unwrap();

    let file_name_arc = Arc::new(file_name);
    let img_arc = Arc::new(img);

    let mut handles = vec![];

    for resolution in target_resolutions {
        let img_thread = Arc::clone(&img_arc);
        let file_name_thread = Arc::clone(&file_name_arc);

        handles.push(thread::spawn(move || {
            rescale(&img_thread, resolution, &file_name_thread);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
