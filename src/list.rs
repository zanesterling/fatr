use std::error;
use std::iter::Iterator;

use fat;

pub fn list_files(args: &mut Iterator<Item=String>)
    -> Result<(), Box<error::Error>>
{
    let image_fn = args.next()
        .ok_or("expected image filename")?;
    let image = fat::Image::from(image_fn);

    Ok(())
}
