mod image;
mod root_entry;

pub use self::image::Image;
pub use self::root_entry::RootEntry;

pub fn cluster_num_is_valid(cluster_num: u16) -> bool {
    2 <= cluster_num && cluster_num < 0xff0
}
