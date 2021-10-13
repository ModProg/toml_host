use std::{fs::File, io::Read, path::PathBuf};

use rocket::serde::json::Json;
#[macro_use]
extern crate rocket;

#[get("/")]
fn get() -> Result<Json<serde_json::Value>, String> {
    Ok(Json(
        toml::from_str::<serde_json::Value>(&{
            let mut string = String::new();
            File::open(PathBuf::from("/file.toml"))
                .map_err(|e| e.to_string())?
                .read_to_string(&mut string)
                .map_err(|e| e.to_string())?;
            string
        })
        .map_err(|e| e.to_string())?,
    ))
}

#[launch]
fn rocket() -> _ { rocket::build().mount("/", routes![get]) }
