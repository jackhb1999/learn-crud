use crate::models::{Movie, Role};
use crate::services::{
    get_logged_in_users, get_users, list_movies, login_success, logout, read_from_json,
    write_to_json,
};
use std::error::Error;
use std::io;
use std::io::Write;

pub fn handle_login(username: &str) -> Result<(), Box<dyn Error>> {
    println!("Login called with username {}", username);
    if let Some(user) = get_users()
        .iter()
        .find(|user| user.username.eq_ignore_ascii_case(username))
    {
        println!("Please enter password");
        match rpassword::read_password() {
            Ok(pw) => {
                if pw == user.password {
                    login_success(&user.role)?;
                    println!("Login successful");
                } else {
                    println!("Invalid password");
                }
            }
            _ => {
                println!("Failed to get password");
            }
        }
    } else {
        println!("User not found");
    }
    Ok(())
}

pub fn handle_logout() -> Result<(), Box<dyn Error>> {
    logout();
    Ok(())
}

pub fn handle_list() -> Result<(), Box<dyn Error>> {
    match get_logged_in_users()? {
        Some(_) => {
            let movies = read_from_json()?;
            list_movies(&movies);
        }
        None => {
            println!("No users found");
        }
    }
    Ok(())
}

pub fn handle_add(
    disc: usize,
    year: &str,
    title: &str,
    remark: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    match get_logged_in_users()? {
        Some(Role::Admin) => {
            let mut movies = read_from_json()?;
            let new_movie = Movie {
                disc,
                year: year.to_string(),
                title: title.to_string(),
                remark: remark.clone(),
            };
            movies.push(new_movie);
            write_to_json(&movies)?;
            println!("Movie added successfully");
        }
        _ => {
            println!("Not authorized to add movie");
        }
    }
    Ok(())
}

pub fn handle_delete(disc: &usize, index: &usize) -> Result<(), Box<dyn Error>> {
    match get_logged_in_users()? {
        Some(Role::Admin) => {
            let mut movies = read_from_json()?;
            if let Some(movie) = movies
                .iter()
                .filter(|movie| movie.disc == *disc)
                .enumerate()
                .find(|(i, _)| i == index)
                .map(|(_, movie)| movie.clone())
            {
                let left_movies = movies
                    .into_iter()
                    .filter(|m| *m != movie)
                    .collect::<Vec<Movie>>();
                write_to_json(&left_movies)?;
                println!("Movie deleted successfully");
            }
        }
        _ => {
            println!("Not authorized to delete movie");
        }
    }
    Ok(())
}

pub fn handle_edit(disc: &usize, index: &usize) -> Result<(), Box<dyn Error>> {
    match get_logged_in_users()? {
        Some(Role::Admin) => {
            let mut movies = read_from_json()?;
            if let Some(movie) = movies
                .iter_mut()
                .filter(|movie| movie.disc == *disc)
                .enumerate()
                .find(|(i, _)| i == index)
                .map(|(_, movie)| movie)
            {
                println!("Current movie: {:?}", movie);

                println!("Enter new disc : ");
                io::stdout().flush()?;
                let mut disc = String::new();
                io::stdin().read_line(&mut disc)?;
                let disc = disc.trim();
                if let Ok(disc) = disc.parse::<usize>() {
                    movie.disc = disc;
                } else {
                    println!("Invalid disc");
                    return Ok(());
                }

                println!("Enter new year : ");
                io::stdout().flush()?;
                let mut year = String::new();
                io::stdin().read_line(&mut year)?;
                let year = year.trim();
                if !year.is_empty()  {
                    movie.year = year.to_string();
                } else {
                    println!("Invalid year");
                    return Ok(());
                }

                println!("Enter new title : ");
                io::stdout().flush()?;
                let mut title = String::new();
                io::stdin().read_line(&mut title)?;
                let title = title.trim();
                if !title.is_empty()  {
                    movie.title = title.to_string();
                } else {
                    println!("Invalid title");
                    return Ok(());
                }

                println!("Enter new remark : ");
                io::stdout().flush()?;
                let mut remark = String::new();
                io::stdin().read_line(&mut remark)?;
                let remark = remark.trim();
                if remark.is_empty()  {
                    movie.remark = None;
                } else {
                    movie.remark = remark.to_string().into();
                }
                
                write_to_json(&movies)?;
                println!("Movie edited successfully");
            }
        }
        _ => {
            println!("Not authorized to edit movie");
        }
    }
    Ok(())
}
