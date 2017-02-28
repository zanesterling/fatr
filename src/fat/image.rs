use std::error;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::mem;

use fat::RootEntry;

pub const BYTES_PER_SECTOR: usize = 512;
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
    assert_eq!(mem::size_of::<RootEntry>(), BYTES_PER_ROOT_ENTRY);
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
            .filter(|entry| { entry.filename[0] != 0xe5 })
            .take_while(|entry| { entry.filename[0] != 0 })
            .collect::<Vec<RootEntry>>()
    }

    pub fn get_file_entry(&self, filename: String)
        -> Result<RootEntry, Box<error::Error>>
    {
        let entries = self.root_entries();
        for entry in entries {
            let entry_name: String;
            match entry.filename() {
                Ok(name) => entry_name = name,
                Err(_) => continue,
            }

            if entry_name.to_lowercase() == filename.to_lowercase() {
                return Ok(entry);
            }
        }

        Err(From::from(format!("file {} not found", filename)))
    }

    pub fn create_file_entry(&self, filename: String)
        -> Result<(RootEntry, u16), Box<error::Error>>
    {
        for (index, e) in self.root_entries().iter().enumerate() {
            if !e.is_free() { continue; }

            let mut entry = RootEntry::new();
            try!(entry.set_filename(filename));
            return Ok((entry.clone(), index as u16));
        }

        Err(From::from("no free entries"))
    }

    pub fn save_file_entry(&mut self, entry: RootEntry, index: u16)
        -> Result<(), Box<error::Error>>
    {
        let entry_bytes: [u8; BYTES_PER_ROOT_ENTRY];
        unsafe { entry_bytes = mem::transmute(entry); }

        self.root_dir[
            index as usize * BYTES_PER_ROOT_ENTRY ..
            (index as usize + 1) * BYTES_PER_ROOT_ENTRY
        ].clone_from_slice(&entry_bytes[..]);
        Ok(())
    }

    pub fn fat_entries<'a>(&'a self)
        -> Box<Iterator<Item=(usize, u16)> + 'a>
    {
        Box::new(
            self.fat_1
            .windows(2)
            .enumerate()
            .filter_map(|(i, w)|
                if i % 3 == 2 {
                    None
                } else {
                    let val = if i % 3 == 0 {
                        ((w[1] as u16 & 0xf) << 8) | w[0] as u16
                    } else {
                        (w[1] as u16 & 0xf0) | ((w[1] as u16) << 4)
                    };
                    Some((i, val))
                }
            )
        )
    }

    pub fn get_fat_entry(&self, cluster_num: u16) -> u16 {
        let offset: usize = cluster_num as usize * 3 / 2;
        let byte_1: u16 = self.fat_1[offset] as u16;
        let byte_2: u16 = self.fat_1[offset + 1] as u16;

        if cluster_num % 2 == 0 { byte_1 | ((byte_2 & 0x0f) << 8) }
        else                    { (byte_1 >> 4) | (byte_2 << 4) }
    }

    pub fn get_free_fat_entry(&self) -> Option<usize> {
        self.fat_entries()
            .filter(|&(_, e)| e == 0)
            .map(|(i, _)| i + 2)
            .nth(0)
    }

    pub fn write_data_sector(&mut self, sector: usize, data: &[u8])
        -> Result<(), Box<error::Error>>
    {
        let sector = sector - 2;
        if sector >= SECTORS_PER_DATA_AREA {
            return Err(errorf!("sector {} too high to write to", sector));
        }

        let mut target_slice = &mut self.data_area[
            BYTES_PER_SECTOR * sector ..
            BYTES_PER_SECTOR * (sector + 1)
        ];

        target_slice.copy_from_slice(data);
        Ok(())
    }
}
