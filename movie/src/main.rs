use clap::{Parser, Subcommand};
use movie::handle::{handle_add, handle_delete, handle_edit, handle_list, handle_login, handle_logout};
use std::error::Error;

#[derive(Parser)]
#[command(version, about = "Movie app", long_about = "Movie information app")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// User login to the system
    Login {
        /// Username for login
        #[arg(short, long)]
        username: String,
    },
    /// User logout from the system
    Logout,
    /// Show all movies
    List,
    /// Add a new movie
    Add {
        /// Disc number
        #[arg(short, long)]
        disc: usize,
        /// Year of the movie
        #[arg(short, long)]
        year: String,
        /// Title of the movie
        #[arg(short, long)]
        title: String,
        /// Remark of the movie
        #[arg(short, long)]
        remark: Option<String>,
    },
    /// Delete a movie
    Delete {
        /// Disc number
        #[arg(short, long)]
        disc: usize, 
        /// Index of the movie
        #[arg(short, long)]
        index: usize,
        
    },
    /// Update a movie
    Edit {
        /// Disc number
        #[arg(short, long)]
        disc: usize,
        /// Index of the movie
        #[arg(short, long)]
        index: usize,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Login { username }) => {
            handle_login(username)?;
        }
        Some(Commands::Logout) => {
            handle_logout()?;
        }
        Some(Commands::List) => {
            handle_list()?;
        }
        Some(Commands::Add { disc, year, title, remark }) => {
            handle_add(*disc, year, title, remark)?;  
        }
        Some(Commands::Delete { disc, index }) => {
            handle_delete(disc, index)?;
        }
        Some(Commands::Edit { disc, index }) => {
            handle_edit(disc, index)?;
        }
        None => {
            println!("No command specified");
        }
    }
    Ok(())
}
