use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};

pub fn read_txt_to_json(file_path: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let txt = fs::read_to_string(file_path)?;
    let mut disc_no = 0u32;
    let disc_regex = Regex::new(r"^(\d+)\.$").unwrap(); // unwrap 直接崩
    let movie_regex = Regex::new(r"^(\d{4})(.*?)(（儿童）)?$")?; // ? 错误向上传递
    let mut movies = Vec::new();
    for line in txt.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if let Some(no) = disc_number(line, &disc_regex) {
            disc_no = no;
        } else if let Some(movie) = parse_movie(line, &movie_regex, disc_no) {
            movies.push(movie);
        }
    }
    save_to_json(movies)
}

fn save_to_json(movies: Vec<Movie>) -> Result<PathBuf, Box<dyn Error>> {
    let json_str = serde_json::to_string_pretty(&movies)?;
    let path = FileDialog::new()
        .add_filter("JSON", &["json"])
        .set_title("Save JSON")
        .set_directory("D:\\")
        .save_file()
        .ok_or_else(||"Could not save movies to file")?;
    fs::write(&path, json_str)?;
    Ok(path)
}

fn disc_number(line: &str, re: &Regex) -> Option<u32> {
    re.captures(line)
        .map(|caps| caps.get(1).unwrap().as_str().parse().unwrap())
}

fn parse_movie(line: &str, re: &Regex, no: u32) -> Option<Movie> {
    re.captures(line).map(|caps| Movie {
        disc: no,
        year: caps.get(1).unwrap().as_str().trim().to_string(),
        title: caps.get(2).unwrap().as_str().trim().to_string(),
        remark: caps.get(3).map(|remark| remark.as_str().trim().to_string()),
    })
}

#[derive(Debug,Serialize,Deserialize)]
struct Movie {
    disc: u32,
    year: String,
    title: String,
    remark: Option<String>,
}
