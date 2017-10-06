extern crate byteorder;

use std::error;
use std::fs;
use std::io::{Read,SeekFrom,Seek};
use std::path::Path;

use self::byteorder::{LittleEndian,ByteOrder};

#[derive(Clone,Debug)]

/// BIOS parameter block describes the FAT filesystem.
pub struct BIOSParam {
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16,
    pub fat_count: u8,
    pub max_roots: u16,
    pub sectors: u32,
    pub media_id: u8,
    pub sectors_per_fat: u32,
}

#[test]
fn test_bios_param_calculations() {
    let mut params = BIOSParam::new();
    params.sectors = 1024;
    params.bytes_per_sector = 512;
    params.sectors_per_cluster = 4;
    assert_eq!(params.len(), 524288 as usize);
    assert_eq!(params.clusters(), 256 as usize);
}

#[allow(dead_code)]
impl BIOSParam {
    /// Create a new empty BIOS parameter block (BPB)
    pub fn new() -> BIOSParam {
        BIOSParam {
            bytes_per_sector: 0,
            sectors_per_cluster: 0,
            reserved_sectors: 0,
            fat_count: 2,
            max_roots: 0,
            sectors: 0,
            media_id: 0,
            sectors_per_fat: 0,
        }
    }

    /// Extract the BIOS Parameter Block (BPB) from the FAT filesystem image.
    pub fn from_file<P: AsRef<Path>>(p: P, o: usize)
        -> Result<BIOSParam, Box<error::Error>>
    {
        let mut boot_sector: Vec<u8> = vec![0; 512];
        let mut file = fs::File::open(p.as_ref())?;
        file.seek(SeekFrom::Start(o as u64))?;
        file.read_exact(&mut boot_sector)?;

        let mut params = BIOSParam::new();

        params.bytes_per_sector = LittleEndian::read_u16(&boot_sector[11..13]);
        params.sectors_per_cluster = boot_sector[13];
        params.reserved_sectors = LittleEndian::read_u16(&boot_sector[14..16]);
        params.fat_count = boot_sector[16];
        params.max_roots = LittleEndian::read_u16(&boot_sector[17..19]);
        params.sectors = LittleEndian::read_u16(&boot_sector[19..21]) as u32;
        if params.sectors == 0 {
            // 4 byte sector count at 0x020
            params.sectors = LittleEndian::read_u32(&boot_sector[32..37]);
        }
        params.media_id = boot_sector[21];
        params.sectors_per_fat = LittleEndian::read_u16(&boot_sector[22..24]) as u32;
        if params.sectors_per_fat == 0 {
            // 4 byte sectors per fat count at 0x024
            params.sectors_per_fat = LittleEndian::read_u32(&boot_sector[36..41]);
        }
        return Ok(params);
    }

    /// Reported length of FAT filesystem in bytes.
    pub fn len(&self) -> usize {
        return self.sectors as usize * self.bytes_per_sector as usize;
    }

    /// Reported number of clusters within FAT filesystem.
    pub fn clusters(&self) -> usize {
        if self.sectors_per_cluster == 0 {
            return 0;
        }
        return self.sectors as usize / self.sectors_per_cluster as usize;
    }
}
