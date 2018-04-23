use std::error;
use std::fs;
use std::io;
use std::io::{Read,Write,SeekFrom,Seek};
use std::mem;
use std::str;
use std::path::Path;

use fat::RootEntry;
use fat::BIOSParam;

// Always the same
const SECTORS_PER_ROOT: usize = 14;
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
    bpb_data: BIOSParam,
}

#[allow(dead_code)]
impl Image {
    /// Create a new blank FAT Image from a defined BPB
    fn new(bpb: BIOSParam, length: usize) -> Image {
        let boot_sector_size = bpb.bytes_per_sector as usize
            * bpb.reserved_sectors as usize;
        let bytes_per_fat = (bpb.sectors_per_fat
            * bpb.bytes_per_sector as u32) as usize;
        let bytes_per_root = SECTORS_PER_ROOT * bpb.bytes_per_sector as usize;
        let data_offset = boot_sector_size as usize
            + (bytes_per_fat * 2) as usize + bytes_per_root as usize;
        let bytes_per_data_area = length - data_offset;
        Image {
            boot_sector: vec![0; boot_sector_size],
            fat_1: vec![0; bytes_per_fat],
            fat_2: vec![0; bytes_per_fat],
            root_dir: vec![0; bytes_per_root],
            data_area: vec![0; bytes_per_data_area],
            bpb_data: bpb,
        }
    }

    /// Create a new FAT Image from the specified file.
    pub fn from_file<P: AsRef<Path>>(image_fn: P)
        -> Result<Image, Box<error::Error>>
    {
        let metadata = fs::metadata(image_fn.as_ref())?;
        let bpb = BIOSParam::from_file(image_fn.as_ref(), 0)?;

        let mut file = fs::File::open(image_fn.as_ref())?;
        let mut image = Image::new(bpb, metadata.len() as usize);

        try!(file.read_exact(&mut image.boot_sector));
        try!(file.read_exact(&mut image.fat_1));
        try!(file.read_exact(&mut image.fat_2));
        try!(file.read_exact(&mut image.root_dir));
        try!(file.read_exact(&mut image.data_area));

        Ok(image)
    }

    /// Create a new FAT Image from the specified file and offset.
    pub fn from_file_offset<P: AsRef<Path>>(image_fn: P, start: usize, length: usize)
        -> Result<Image, Box<error::Error>>
    {
        let metadata = fs::metadata(image_fn.as_ref())?;
        let bpb = BIOSParam::from_file(image_fn.as_ref(), start)?;

        if metadata.is_file() && (start + length > (metadata.len() as usize)) {
            return Err(From::from(format!("start + offset outside image bounds")));
        }

        let mut file = fs::File::open(image_fn.as_ref())?;
        file.seek(SeekFrom::Start(start as u64))?;

        let mut image = Image::new(bpb, length);

        try!(file.read_exact(&mut image.boot_sector));
        try!(file.read_exact(&mut image.fat_1));
        try!(file.read_exact(&mut image.fat_2));
        try!(file.read_exact(&mut image.root_dir));
        try!(file.read_exact(&mut image.data_area));

        Ok(image)
    }

    /// Save the FAT filesystem image to the specified file.
    pub fn save<P: AsRef<Path>>(&self, image_fn: P)
        -> Result<(), io::Error>
    {
        let mut file = fs::File::create(image_fn.as_ref())?;

        try!(file.write_all(&self.boot_sector));
        try!(file.write_all(&self.fat_1));
        try!(file.write_all(&self.fat_2));
        try!(file.write_all(&self.root_dir));
        try!(file.write_all(&self.data_area));

        Ok(())
    }

    /// Extract the BIOS Parameter Block (BPB) from the FAT filesystem.
    pub fn bios_parameter(&self) -> BIOSParam {
        self.bpb_data.clone()
    }

    /// FAT sector size in bytes.
    pub fn sector_size(&self) -> usize {
        self.bpb_data.bytes_per_sector as usize
    }

    /// FAT volume label
    pub fn volume_label(&self) -> Result<String, Box<error::Error>> {
        let entries = self.root_entries();
        for entry in entries {
            if !entry.is_volume_label() {
                continue;
            }
            let label = format!("{}{}",
                str::from_utf8(&entry.filename)?,
                str::from_utf8(&entry.extension)?);
            return Ok(label);
        }
        return Ok("has no label".to_string());
    }

    // TODO: Make iterator once "impl Trait" is stable.
    /// Return all FAT root entries (including unused)
    pub fn root_entries_all(&self) -> Vec<RootEntry> {
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

    // TODO: Make iterator once "impl Trait" is stable.
    /// Return used FAT root entries
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

    /// Get the RootEntry for the specified file within the Image.
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

    /// Create a new RootEntry within the Image with the specified filename.
    pub fn create_file_entry(&self, filename: String, bytes: u32)
        -> Result<(RootEntry, u16), Box<error::Error>>
    {
        match self.get_file_entry(filename.clone()) {
            Ok(_) => return Err(From::from("entry already exists")),
            Err(_) => {}
        }

        for (index, e) in self.root_entries_all().iter().enumerate() {
            if !e.is_free() { continue; }

            let mut entry = RootEntry::new();
            entry.set_filename(filename)?;
            entry.set_size(bytes)?;
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
        let bytes_per_sector = self.bpb_data.bytes_per_sector as usize;
        let start_byte = bytes_per_sector * sector;

        if start_byte >= self.data_area.len() {
            return Err(From::from(format!("sector {} too high to write to", sector)))
        }

        let mut target_slice = &mut self.data_area[start_byte..start_byte + data.len()];
        target_slice.copy_from_slice(data);
        Ok(())
    }
}
