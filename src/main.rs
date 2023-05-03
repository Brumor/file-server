#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use log::info;
use rocket::{response::Debug, Data};
use std::fs;
use std::io;
use std::path::Path;
mod paste_id;
use paste_id::PasteId;
use std::io::prelude::*;

#[get("/")]
fn hello_world() -> &'static str {
    "Hello, World!"
}

#[post("/upload", data = "<data>")]
fn upload(data: Data) -> Result<String, Debug<io::Error>> {
    if !Path::new("./upload").is_dir() {
        fs::create_dir("./upload")?;
        info!("Upload dir created");
    }

    let id = PasteId::new(8);
    let filename = format!("./upload/{id}.txt", id = id);
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);

    let path = Path::new(&filename);
    let display = path.display();
    info!("{}", display);

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match fs::File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // data.stream_to_file(Path::new(&filename))?;
    match file.write_all(data.peek()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
    Ok(url)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello_world, upload])
        .launch();
}
