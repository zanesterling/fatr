use std::error;

use fat;

pub fn detail_file(args: &[String])
    -> Result<(), Box<error::Error>>
{
    if args.len() < 2 {
        return Err(From::from("expected <file> <image> filename"));
    }

    let image_fn = args[0].clone();
    let image = fat::Image::from(image_fn)?;

    let file_metadata = image.get_file_entry(args[1].clone())?;
    println!("{:?}", file_metadata);

    Ok(())
}
