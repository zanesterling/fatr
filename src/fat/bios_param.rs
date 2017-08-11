
#[derive(Clone,Debug)]
#[repr(C)]
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

/*
impl BIOSParam {
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
}
*/
