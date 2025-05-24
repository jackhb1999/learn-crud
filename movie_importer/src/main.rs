use std::error::Error;
use std::process;
use rfd::FileDialog;
use movie_importer::read_txt_to_json;

fn main() ->Result<(), Box<dyn Error>> {
    match FileDialog::new()
        .add_filter("Text files", &["txt"])
        .set_title("Select the DVD text file")
        .set_directory("D:\\")
        .pick_file()
    {
        Some(file_path) => {
            let save_path = read_txt_to_json(&file_path)?;
            println!("Saved file: {:?}", save_path);
            Ok(())
        }
        None => {
            eprintln!("No such file found.");
            process::exit(-1);
        }
    }
}
