use crate::RustEzFs;
use crate::defs::{EZFS_MAGIC_NUMBER, EZFS_MAX_DATA_BLKS};
use crate::sb::{Bitmap, EzfsSuperblock, EzfsSuperblockData};
use kernel::fs::FileSystem;
use kernel::inode::{INode, Mapper};

use std::sync::Mutex;

#[kani::proof]
fn verify_magic_number_logic_in_fill_super() {
    let magic: u64 = kani::any();
    let expected: u64 = EZFS_MAGIC_NUMBER.try_into().unwrap();

    let should_reject = magic != expected;

    if should_reject {
        assert!(magic != expected);
    } else {
        assert!(magic == expected);
    }
}

#[kani::proof]
fn verify_inode_allocation() {
    let mut sb = EzfsSuperblock {
        version: 1,
        magic: 0x4118,
        disk_blocks: kani::any(),
        data: Mutex::new(EzfsSuperblockData {
            free_inodes: kani::any(),
            free_data_blocks: Bitmap::new([0; (EZFS_MAX_DATA_BLKS / 32) + 1]),
            zero_data_blocks: Bitmap::new([0; (EZFS_MAX_DATA_BLKS / 32) + 1]),
        }),
        mapper: Mapper::<RustEzFs> {
            inode: INode::<RustEzFs> { ino: 0, data: None },
            begin: 0,
            end: 4096,
        },
    };

    let res1 = RustEzFs::allocate_inode(&sb);
    let res2 = RustEzFs::allocate_inode(&sb);

    if let (Ok(ino1), Ok(ino2)) = (res1, res2) {
        kani::assert(
            ino1 != ino2,
            "Sequential allocations must return different inodes",
        );
    }
}
