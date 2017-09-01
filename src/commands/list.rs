use std::error;

use fat;

pub fn list_files(args: &[String])
    -> Result<(), Box<error::Error>>
{
    expect_args!(args, 1);

    let image_fn = args[0].clone();
    let image = fat::Image::from_file(image_fn)?;

    println!(" Volume {}", image.volume_label()?);
    println!(" Volume has {} bytes per sector\n", image.sector_size());

    let mut file_count = 0;
    let mut size_total = 0;
    for entry in image.root_entries() {
        if entry.rest_are_free() {
            break;
        } else if entry.is_free() || entry.is_volume_label() {
            continue;
        }
        file_count += 1;
        size_total += entry.file_size;

        println!(
            "{}\t{}\t{}\t\t{}",
            entry.last_write_date,
            entry.last_write_time,
            entry.file_size,
            entry.filename().unwrap_or("????????.???".to_string()),
        );
    }
    println!("\t{} File(s)\t\t{} bytes", file_count, size_total);

    Ok(())
}
