use crate::models::{Movie, Role, User};
use std::error::Error;
use std::{fs, io};
use unicode_width::UnicodeWidthStr;

pub fn get_users() -> Vec<User> {
    vec![
        User {
            username: "Admin".to_string(),
            password: "admin".to_string(),
            role: Role::Admin,
        },
        User {
            username: "User".to_string(),
            password: "user".to_string(),
            role: Role::User,
        },
        User {
            username: "Van".to_string(),
            password: "van".to_string(),
            role: Role::User,
        },
    ]
}

pub fn login_success(role: &Role) -> Result<(), Box<dyn Error>> {
    fs::write(".session", role.to_string())?;
    Ok(())
}

pub fn get_logged_in_users() -> Result<Option<Role>, Box<dyn Error>> {
    let role = fs::read_to_string(".session")?;
    println!("Role: {}", role);
    match role.as_str() {
        "Admin" => Ok(Some(Role::Admin)),
        "User" => Ok(Some(Role::User)),
        _ => Ok(None),
    }
}

pub fn logout() {
    fs::remove_file(".session").unwrap_or_else(|_| {
        println!("No user is logged in.");
    })
}

pub fn read_from_json() -> Result<Vec<Movie>, Box<dyn Error>> {
    let file = fs::File::open("movie.json")?;
    let reader = io::BufReader::new(file);
    let movies: Vec<Movie> = serde_json::from_reader(reader)?;
    Ok(movies)
}

pub fn list_movies(movies: &[Movie]) {
    println!("{:<5}{:<7}{:<80}{:<15}", "Disc", "Year", "Title", "Remark");
    println!("{:-<110}", "");
    movies.iter().for_each(|movie| {
        let remark = movie.remark.as_ref().unwrap_or(&"".to_string());
        let remark = movie.remark.as_deref().unwrap_or("");
        let title = pad_display_width(&movie.title, 80);
        let remark = pad_display_width(&remark, 15);
        println!(
            "{:<5}{:<7}{:<80}{:<15}",
            movie.disc, movie.year, title, remark
        )
    })
}

fn pad_display_width(s: &str, target_width: usize) -> String {
    let width = UnicodeWidthStr::width(s);
    format!("{}{}", s, " ".repeat(target_width.saturating_sub(width)))
}

pub fn write_to_json(movies: &[Movie]) -> Result<(), Box<dyn Error>> {
    let file = fs::File::create("movie.json")?;
    let mut writer = io::BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, movies)?;
    Ok(())
}
