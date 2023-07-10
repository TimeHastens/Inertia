use std::fs;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::SystemTime;
use rayon::ThreadPoolBuilder;
use image::{Rgb, RgbImage};

fn main() {
    println!("INPUT:");
    let input_dir = read_user_input();
    println!("OUTPUT:");
    let output_dir = read_user_input();

    let start_time = SystemTime::now();

    let files = read_files_from_directory(&input_dir);
    let thread_pool = ThreadPoolBuilder::new().build().unwrap();
    let (sender, receiver) = mpsc::channel();

    for file in &files {
        let input_file = input_dir.join(&file);
        let output_file = output_dir.join(format!("{}.png", file.file_stem().unwrap().to_string_lossy()));
        let sender_clone = sender.clone();

        thread_pool.spawn(move || {
            let bytes = fs::read(input_file).unwrap();
            let hex_string = hex::encode(bytes);

            let mut image = RgbImage::new(720, (hex_string.len() as f32 / 720.0).ceil() as u32);
            let mut x = 0;
            let mut y = 0;

            for c in hex_string.chars() {
                let (red, green, blue) = match c {
                    '0' => (179, 255, 255),
                    '1' => (30, 132, 73),
                    '2' => (175, 238, 238),
                    '3' => (60, 179, 113),
                    '4' => (225, 165, 0),
                    '5' => (250, 128, 114),
                    '6' => (173, 216, 230),
                    '7' => (225, 103, 129),
                    '8' => (207, 240, 236),
                    '9' => (100, 103, 103),
                    'A' => (121, 94, 255),
                    'B' => (129, 213, 150),
                    'C' => (20, 143, 119),
                    'D' => (52, 152, 219),
                    'E' => (247, 220, 111),
                    'F' => (113, 125, 126),
                    _ => (0, 0, 0),
                };

                image.put_pixel(x, y, Rgb([red as u8, green as u8, blue as u8]));
                x += 1;

                if x >= image.width() {
                    x = 0;
                    y += 1;
                }
            }

            image.save(output_file).unwrap();
            sender_clone.send(()).unwrap();
        });
    }

    drop(sender);

    for _ in receiver.iter().take(files.len()) {}

    let elapsed_time = start_time.elapsed().unwrap();
    println!("Execution time: {:?}", elapsed_time);
}

fn read_user_input() -> PathBuf {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    PathBuf::from(input.trim())
}

fn read_files_from_directory(dir: &PathBuf) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
        .collect()
}