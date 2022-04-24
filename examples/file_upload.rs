use infer;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;
use uuid::Uuid;

use lab01_2022_input_validation::*;
use read_input::prelude::*;

type Filepath = String;
type UuidStr = String;

// Use the hashmap as follows:
// ```
// let map = HASHMAP.lock().unwrap();
// ```
lazy_static! {
    static ref HASHMAP: Mutex<HashMap<UuidStr, Filepath>> = Mutex::new(HashMap::new());
}

// TODO: IMPLEMENT UPLOAD LOGIC
fn file_upload_handler() {
    let usr_input = input::<String>()
        .repeat_msg("Please enter the path to an image or a video file : ")
        .add_err_test(
            |s: &String| validate_file(s, false),
            "Invalid file contents",
        )
        .get();

    let file_uuid = Uuid::new_v5(
        &Uuid::NAMESPACE_OID,
        fs::read(&usr_input).unwrap().as_slice(),
    )
    .to_string();

    let mut map = HASHMAP.lock().unwrap();
    match map.get(&file_uuid) {
        Some(filepath) => {
            println!(
                "File already uploaded, cannot overwrite : {} => {}\n\n",
                file_uuid, filepath
            );
            return;
        }
        None => (),
    }

    println!("File uploaded successfully, UUID : {}\n\n", &file_uuid);
    map.insert(file_uuid, usr_input);
}

// TODO: IMPLEMENT VERIFY LOGIC
fn file_verify_handler() {
    let usr_input = input::<String>()
        .repeat_msg("Please enter the UUID to check : ")
        .add_err_test(|s: &String| validate_uuid(s), "Invalid UUID")
        .get();

    let map = HASHMAP.lock().unwrap();

    match map.get(&usr_input) {
        Some(filepath) => {
            let filetype = infer::get_from_path(filepath).unwrap().unwrap().mime_type();
            let filetype = filetype.split('/').collect::<Vec<_>>()[0];
            let filetype = match filetype {
                "image" => format!("an {}", filetype),
                _ => format!("a {}", filetype),
            };
            println!("File {} exists, it is {} file.\n\n", usr_input, filetype);
        }
        None => println!("File not found\n\n"),
    };
}

// TODO: IMPLEMENT GET URL LOGIC
fn get_url_handler() {
    let usr_input = input::<String>()
        .repeat_msg("Please enter the UUID to get : ")
        .add_err_test(|s: &String| validate_uuid(s), "Invalid UUID")
        .get();

    let map = HASHMAP.lock().unwrap();

    match map.get(&usr_input) {
        Some(filepath) => {
            let filetype = infer::get_from_path(filepath).unwrap().unwrap().mime_type();
            let filetype = filetype.split('/').collect::<Vec<_>>()[0];
            println!("sec.upload/{}s/{}\n\n", filetype, filepath);
        }
        None => println!("File not found\n\n"),
    }
}

fn main() {
    println!("Welcome to the super secure file upload tool !");
    loop {
        match input::<i32>().repeat_msg("Please select one of the following options to continue :\n1 - Upload a file\n2 - Verify file exists\n3 - Get file URL\n0 - Exit\nYour input ? [0-3] ")
            .min_max(0, 3).get() {
            0 => {
                println!("Goodbye!");
                break
            },
            1 => file_upload_handler(),
            2 => file_verify_handler(),
            3 => get_url_handler(),
            _ => panic!("Invalid input"),
        }
    }
}
