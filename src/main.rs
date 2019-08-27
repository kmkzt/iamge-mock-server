#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::request::Form;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// image_mock 
#[derive(FromForm)]
struct ImageInfo {
    width: u8,
    height: u8
}

#[get("/image?<format>&<info..>")]
fn image_mock(format: Option<String>, info: Option<Form<ImageInfo>>) -> String {
    let image_format = match format {
        Some(ext) => format!("image is {}!", ext),
        None => "default is png.".to_string()
    };

    let image_size = match info {
        Some(_form) => "size is changed!".to_string(),
        None => "100x100 is default size".to_string()
    };

    format!("{} {}", image_format, image_size)
}

fn main() {
    rocket::ignite().mount("/", routes![index, image_mock]).launch();
}