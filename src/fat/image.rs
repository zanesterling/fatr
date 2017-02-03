use std::error;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::mem;

use fat::RootEntry;

const BYTES_PER_SECTOR: usize = 512;
const SECTORS_PER_FAT: usize = 9;
const SECTORS_PER_ROOT: usize = 14;
const SECTORS_PER_DATA_AREA: usize = 2847;

const BYTES_PER_FAT: usize = BYTES_PER_SECTOR * SECTORS_PER_FAT;
const BYTES_PER_ROOT: usize
    = BYTES_PER_SECTOR * SECTORS_PER_ROOT;
const BYTES_PER_DATA_AREA: usize
    = BYTES_PER_SECTOR * SECTORS_PER_DATA_AREA;

const BYTES_PER_ROOT_ENTRY: usize = 32;
#[test]
fn test_root_entry_size() {
    assert_eq!(
        mem::size_of::<RootEntry>(),
        BYTES_PER_ROOT_ENTRY
    );
}

#[derive(Debug)]
pub struct Image {
    boot_sector: Vec<u8>,
    fat_1: Vec<u8>,
    fat_2: Vec<u8>,
    root_dir: Vec<u8>,
    data_area: Vec<u8>,
}

impl Image {
    pub fn new() -> Image {
        Image {
            boot_sector: vec![0; 1 * BYTES_PER_SECTOR],
            fat_1: vec![0; BYTES_PER_FAT],
            fat_2: vec![0; BYTES_PER_FAT],
            root_dir: vec![0; BYTES_PER_ROOT],
            data_area: vec![0; BYTES_PER_DATA_AREA],
        }
    }

    fn blank_image() -> Image {
        Image {
            boot_sector: vec![0; BYTES_PER_SECTOR],
            fat_1: vec![0; BYTES_PER_FAT],
            fat_2: vec![0; BYTES_PER_FAT],
            root_dir: vec![0; BYTES_PER_ROOT],
            data_area: vec![0; BYTES_PER_DATA_AREA],
        }
    }

    pub fn from(image_fn: String)
        -> Result<Image, Box<error::Error>>
    {
        let mut file = fs::File::open(image_fn)?;
        let mut image = Image::blank_image();

        try!(file.read_exact(&mut image.boot_sector));
        try!(file.read_exact(&mut image.fat_1));
        try!(file.read_exact(&mut image.fat_2));
        try!(file.read_exact(&mut image.root_dir));
        try!(file.read_exact(&mut image.data_area));

        Ok(image)
    }

    pub fn save(&self, image_fn: String)
        -> Result<(), io::Error>
    {
        let mut file = fs::File::create(image_fn)?;

        try!(file.write_all(&self.boot_sector));
        try!(file.write_all(&self.fat_1));
        try!(file.write_all(&self.fat_2));
        try!(file.write_all(&self.root_dir));
        try!(file.write_all(&self.data_area));

        Ok(())
    }

    // TODO: Make this an iterator
    pub fn root_entries(&self) -> Vec<RootEntry> {
        self.root_dir
            .chunks(BYTES_PER_ROOT_ENTRY)
            .map(|chunk| {
                let mut entry_bytes = [0; BYTES_PER_ROOT_ENTRY];
                entry_bytes.clone_from_slice(chunk);

                let entry: RootEntry;
                unsafe { entry = mem::transmute(entry_bytes); }
                entry
            })
            .collect::<Vec<RootEntry>>()
    }

    pub fn get_file_entry(&self, filename: String)
        -> Result<RootEntry, Box<error::Error>>
    {
        for entry in self.root_entries() {
            let entry_name = entry.filename();
            if entry_name.is_ok() && entry_name.unwrap() == filename {
                return Ok(entry);
            }
        }

        Err(From::from(format!("file {} not found", filename)))
    }
}
