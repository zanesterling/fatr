use std::error;

use fat;

pub fn list_files(args: &[String])
    -> Result<(), Box<error::Error>>
{
    expect_args!(args, 1);

    let image_fn = args[0].clone();
    let image = fat::Image::from(image_fn)?;

    let params = image.bios_parameter();
    // Just dump for testing. Remove when tried of me :-)
    println!("{:?}", params);

    for entry in image.root_entries() {
        if entry.rest_are_free() {
            break;
        } else if entry.is_free() {
            continue;
        }

        println!(
            "{} {}",
            entry.filename_full(),
            entry.file_size,
        );
    }

    Ok(())
}
