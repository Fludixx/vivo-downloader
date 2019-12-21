use crate::crypt::rot47;
use regex::Regex;

pub struct Site {
    pub url: String,
}

impl Site {
    pub fn new_from_url(url: String) -> Site {
        Site { url }
    }

    pub fn get_video_size(&self) -> u32 {
        let content = reqwest::get(self.url.clone().as_str())
            .unwrap()
            .text()
            .unwrap();
        let regex = Regex::new(r"\((.*?)MB\)").unwrap();
        let size = regex
            .find(content.as_ref())
            .unwrap()
            .as_str()
            .replace("(", "")
            .replace(")", "")
            .replace(" MB", "");
        (size.parse::<f64>().unwrap() * (10 as f64).powf(6.0)).round() as u32
    }

    pub fn get_video_url(&self) -> String {
        let content = reqwest::get(self.url.clone().as_str())
            .unwrap()
            .text()
            .unwrap();
        let regex = Regex::new(r"(?s)InitializeStream\s*\(\s*(\{.+?})\s*\)\s*;").unwrap();
        let json = regex
            .find(content.as_ref())
            .unwrap()
            .as_str()
            .trim()
            .replace("\t", "");
        let lines: Vec<&str> = json.split("\n").collect();
        let mut link = String::new();
        for line in lines {
            if line.starts_with("source") {
                link = rot47(urldecode::decode(
                    line.replace("source: '", "").replace("',", "").to_string(),
                ));
            }
        }
        link
    }
}
