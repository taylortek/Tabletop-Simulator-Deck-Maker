use dialog::DialogBox;
use rfd::FileDialog;

use image::{GenericImage, ImageBuffer, RgbImage};

fn main() {
    let mut image_paths = if let Some(files) = FileDialog::new()
        .add_filter("image", &["jpg", "jpeg", "png"])
        .pick_files() {
            files
        } else {
            eprintln!("Could not find image");
            std::process::exit(0);
        };

    let output_path = if let Some(files) = FileDialog::new()
        .pick_folder() {
            files
        } else {
            eprintln!("Could not open folder");
            std::process::exit(0);
        };

    let cols: u32 = dialog::Input::new("How many columns?")
        .title("Columns")
        .show()
        .expect("Could not display dialog box").unwrap().trim().parse().unwrap();
    let rows: u32 = dialog::Input::new("How many rows?")
        .title("Rows")
        .show()
        .expect("Could not display dialog box").unwrap().trim().parse().unwrap();
    let name: String = dialog::Input::new("What should the sheets be named?")
        .title("Name")
        .show()
        .expect("Could not display dialog box").unwrap().trim().to_string();

    let refrence_image = image::open(image_paths[0].clone()).unwrap();
    let height = refrence_image.height();
    let width = refrence_image.width();
    let total_height = rows * height;
    let total_width = cols * width;
    
    let mut sheets = 0;
    
    while cols*rows*sheets < image_paths.len() as u32 {
        sheets += 1;
    }
    
    for sheet in 0..sheets {
        let mut canvas: RgbImage = ImageBuffer::new(total_width, total_height);

        for pixel in canvas.pixels_mut() {
            *pixel = image::Rgb([255, 255, 255]);
        }

        for (index, path) in image_paths.iter().enumerate() {
            if index as u32 == cols*rows {
                break;
            }
            let current_index = index as u32;
            let x_pos = current_index % cols;
            let y_pos = current_index / cols;

            let x = x_pos * width;
            let y = y_pos * height;

            if let Ok(img) = image::open(path) {
                canvas.copy_from(&img.to_rgb8(), x, y).unwrap();
            }
        }
        
        for _x in 0..(cols*rows) {

            if image_paths.len() > 0 {
                image_paths.remove(0);
            }
        }
        canvas.save(format!("{}/{}{:03}.jpg", output_path.display(), name, sheet+1)).unwrap();
    }
}
