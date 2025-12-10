use crate::RustEzFs;
use crate::defs::{EZFS_BLOCK_SIZE, EZFS_MAX_DATA_BLKS, EZFS_MAX_INODES};
use crate::inode::InodeStore;
use core::mem::size_of;
use kernel::fs::FileSystem;
use kernel::inode;
// use kernel::new_mutex;
// use kernel::prelude::*;
// use kernel::sync::Mutex;
// use kernel::transmute::FromBytes;
use std::sync::Mutex;

#[repr(C)]
pub(crate) struct EzfsSuperblockDiskRaw {
    version: u64,
    magic: u64,
    disk_blocks: u64,
    free_inodes: [u32; (EZFS_MAX_INODES / 32) + 1],
    free_data_blocks: [u32; (EZFS_MAX_DATA_BLKS / 32) + 1],
    zero_data_blocks: [u8; (EZFS_MAX_DATA_BLKS / 32) + 1],
}

// TODO: assert size is equal to 4096 bytes
#[repr(C)]
pub(crate) struct EzfsSuperblockDisk {
    data: EzfsSuperblockDiskRaw,
    _padding: [u8; EZFS_BLOCK_SIZE - size_of::<EzfsSuperblockDiskRaw>()],
}

impl EzfsSuperblockDisk {
    pub fn magic(&self) -> u64 {
        self.data.magic
    }
}

impl Default for EzfsSuperblockDiskRaw {
    fn default() -> Self {
        Self {
            version: 0,
            magic: 0,
            disk_blocks: 0,
            free_inodes: [0; (EZFS_MAX_INODES / 32) + 1],
            free_data_blocks: [0; (EZFS_MAX_DATA_BLKS / 32) + 1],
            zero_data_blocks: [0; (EZFS_MAX_DATA_BLKS / 32) + 1],
        }
    }
}

impl Default for EzfsSuperblockDisk {
    fn default() -> Self {
        Self {
            data: EzfsSuperblockDiskRaw::default(),
            _padding: [0; EZFS_BLOCK_SIZE - size_of::<EzfsSuperblockDiskRaw>()],
        }
    }
}

pub(crate) struct EzfsSuperblock {
    pub(crate) version: u64,
    pub(crate) magic: u64,
    pub(crate) disk_blocks: u64,
    pub(crate) data: Mutex<EzfsSuperblockData>,
    pub(crate) mapper: inode::Mapper<RustEzFs>,
}

pub(crate) struct EzfsSuperblockData {
    pub free_inodes: Bitmap<{ (EZFS_MAX_INODES / 32) + 1 }>,
    pub free_data_blocks: Bitmap<{ (EZFS_MAX_DATA_BLKS / 32) + 1 }>,
    pub zero_data_blocks: Bitmap<{ (EZFS_MAX_DATA_BLKS / 32) + 1 }>,
}

impl EzfsSuperblock {
    pub(crate) fn new(disk_sb: EzfsSuperblockDisk, mapper: inode::Mapper<RustEzFs>) -> Self {
        Self {
            version: disk_sb.data.version,
            magic: disk_sb.data.magic,
            disk_blocks: disk_sb.data.disk_blocks,
            data: Mutex::new(EzfsSuperblockData {
                free_inodes: Bitmap::new(disk_sb.data.free_inodes),
                free_data_blocks: Bitmap::new(disk_sb.data.free_data_blocks),
                zero_data_blocks: Bitmap::new(disk_sb.data.zero_data_blocks),
            }),
            mapper,
        }
    }

    pub(crate) fn magic(&self) -> u64 {
        self.magic
    }
}

#[repr(transparent)]
pub(crate) struct Bitmap<const N: usize> {
    inner: [u32; N],
}

impl<const N: usize> Bitmap<N> {
    #[inline]
    pub(crate) fn is_set(&self, block_num: u64) -> bool {
        let idx: usize = (block_num / 32) as usize;
        let mask = 1 << (block_num % 32);
        (self.inner[idx] & mask) != 0
    }

    #[inline]
    pub(crate) fn set_bit(&mut self, block_num: u64) {
        let idx: usize = (block_num / 32) as usize;
        let mask = 1 << (block_num % 32);
        self.inner[idx] |= mask
    }

    #[inline]
    pub(crate) fn clear_bit(&mut self, block_num: u64) {
        let idx: usize = (block_num / 32) as usize;
        let mask = 1 << (block_num % 32);
        self.inner[idx] &= !mask
    }

    const fn new(inner: [u32; N]) -> Self {
        Self { inner }
    }
}

impl<const N: usize> Deref for Bitmap<N> {
    type Target = [u32; N];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<const N: usize> DerefMut for Bitmap<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
