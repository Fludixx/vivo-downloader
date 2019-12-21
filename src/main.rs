use indicatif::{ProgressBar, ProgressStyle};
use lust::get_file_size;
use std::fs::File;
use std::process::exit;
use std::time::Duration;
use std::{env, io, thread};

mod crypt;
mod vivo;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <link to vivo> <save as>", args[0]);
        exit(1)
    }
    let url: String = args[1].to_string();
    let save_as: String = args[2].to_string();
    let site = vivo::Site::new_from_url(url);
    let mut file: File = File::create(save_as.clone()).unwrap();
    let total_size = site.get_video_size() as u64;
    thread::spawn(move || {
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.green/grey}] {bytes}/{total_bytes} ({eta})")
            .progress_chars("#>-"));
        let fs = get_file_size(save_as.clone().as_str());
        while fs < total_size {
            let fs = get_file_size(save_as.as_str());
            pb.set_position(fs);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    let mut r = reqwest::get(site.get_video_url().as_str()).unwrap();
    io::copy(&mut r, &mut file);
    println!("done.")
}
