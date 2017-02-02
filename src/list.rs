use std::error;
use std::fs::File;

pub fn list_files(image_fn: &String)
    -> Result<(), Box<error::Error>>
{
    let img_file = File::open(image_fn);

    Ok(())
}
