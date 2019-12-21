use indicatif::{ProgressBar, ProgressStyle};
use lust::get_file_size;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::SeekFrom::Start;
use std::io::{BufWriter, Read, Seek, Write};
use std::process::exit;

mod crypt;
mod vivo;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: {} <link to vivo> <save as> <continue>", args[0]);
        exit(1)
    }

    let url: String = args[1].to_string();
    let save_as: String = args[2].to_string();
    let should_continue: String = args[3].to_string();
    let site = vivo::Site::new_from_url(url);
    let file;
    if let Ok(opened_file) = OpenOptions::new().write(true).read(true).open(&save_as) {
        println!("⚠ | File already exists");
        file = opened_file;
    } else {
        file = File::create(save_as.clone())
            .expect(&format!("Could not create file '{}'", save_as.clone()));
    }
    let total_size = reqwest::get(site.get_video_url().as_str())
        .expect("Could not make request to get size")
        .content_length()
        .expect("Could not get content length");
    let mut cursor = 0;
    let mut chunk_size = 2048;
    if should_continue.eq("true") {
        let file_size = get_file_size(&save_as);
        if &file_size + &chunk_size >= total_size {
            println!("⚠ | File is already fully downloaded");
            return;
        }
        if &file_size > &0 {
            println!("⚠ | Continuing download at {} bytes", file_size);
            cursor = file_size;
        }
    }
    let progress_bar = ProgressBar::new(total_size);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.green/grey}] {bytes}/{total_bytes}({bytes_per_sec}) ({eta})")
        .progress_chars("#>-"));
    let mut r = reqwest::get(site.get_video_url().as_str()).unwrap();

    let mut writer = BufWriter::new(file.try_clone().expect("Could not clone file"));
    if cursor > 0 {
        let mut tempbuf = vec![0u8; cursor as usize];
        r.read_exact(&mut tempbuf).unwrap();
    }
    writer
        .seek(Start(cursor))
        .expect("Could not seek to continue download");
    while cursor < total_size {
        if cursor + chunk_size > total_size {
            chunk_size = cursor + chunk_size - total_size;
            cursor += chunk_size;
            println!("⚠ | Adjusted chunk size to {} bytes", &chunk_size)
        } else {
            cursor += chunk_size;
        }
        let mut tempbuf = vec![0u8; chunk_size as usize];
        if let Err(_) = r.read_exact(&mut tempbuf) {
            break;
        }
        writer.write(&tempbuf).expect("Could not write to file");
        progress_bar.set_position(cursor);
    }
    println!("done.")
}
