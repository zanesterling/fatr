use std::error;

use fat;

pub fn list_files(args: &[String])
    -> Result<(), Box<error::Error>>
{
    if args.len() < 1 {
        return Err(From::from("expected image filename"));
    }

    let image_fn = args[0].clone();
    let image = fat::Image::from(image_fn)?;

    println!("{:?}", image);

    for entry in image.root_entries() {
        if entry.rest_are_free() {
            break;
        } else if entry.is_free() {
            continue;
        }

        println!("{}", entry.filename_full());
    }

    Ok(())
}
