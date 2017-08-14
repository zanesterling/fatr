
#[derive(Clone,Debug)]
#[repr(C)]
/// BIOS parameter block describes the FAT filesystem.
pub struct BIOSParam {
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16,
    pub fat_count: u8,
    pub max_roots: u16,
    pub sectors: u16,
    pub media_id: u8,
    pub sectors_per_fat: u16,
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
