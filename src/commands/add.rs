use std::error;
use std::fs;
use std::io::Read;

use fat;

pub fn add_file(args: &[String])
    -> Result<(), Box<error::Error>>
{
    expect_args!(args, 2);

    let file_name  = args[0].clone();
    let image_name = args[1].clone();
    let fat_file_name = if args.len() > 2 {
        args[2].clone()
    } else {
        file_name.clone()
    };

    let mut image = fat::Image::from(image_name.clone())?;

    // Don't overwrite a preexisting file.
    if let Ok(_) = image.get_file_entry(file_name.clone()) {
        return Err(errorf!("file {} already exists", file_name));
    }

    // Ensure input file exists.
    let file = fs::File::open(file_name)?;

    // Create a root dir entry.
    let (entry, index) = image.create_file_entry(fat_file_name)?;

    // Get free FAT entries, fill sectors with file data.
    let file_bytes = file.bytes();
    loop {
        let chunk: Vec<_> = file_bytes
            .take(fat::BYTES_PER_SECTOR)
            .collect();
        if chunk.len() == 0 { break; }

        // TODO
        // Get free sector, write data
    }

    image.save_file_entry(entry, index)?;
    image.save(image_name)?;
    Ok(())
}
