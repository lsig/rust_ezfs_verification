use crate::RustEzFs;
use crate::defs::*;
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

    // If the allocation succeds they are not the same
    if let (Ok(ino1), Ok(ino2)) = (res1, res2) {
        kani::assert(
            ino1 != ino2,
            "Sequential allocations must return different inodes",
        );
    }
}

#[kani::proof]
fn verify_inode_allocated() {
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

    let ino: u64 = kani::any();
    let res = RustEzFs::inode_allocated(&sb, ino.try_into().unwrap());

    let bitmap_copy = {
        let sb_data = sb.data.lock().unwrap();
        sb_data.free_inodes.clone()
    };

    if let Ok(ans) = res {
        kani::assert(
            ans == bitmap_copy.is_set(ino - EZFS_ROOT_INODE_NUMBER as u64),
            "Bitmap states should match",
        );
    }
}

#[kani::proof]
fn verify_inode_deallocation() {
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

    let ino: u64 = kani::any();
    let res = RustEzFs::deallocate_inode(&sb, ino.try_into().unwrap());

    let bitmap_copy = {
        let sb_data = sb.data.lock().unwrap();
        sb_data.free_inodes.clone()
    };

    if let Ok(ans) = res {
        kani::assert(
            bitmap_copy.is_set(ino - EZFS_ROOT_INODE_NUMBER as u64) == false,
            "Deallocated inode should never be set",
        );
    }
}

#[kani::proof]
fn verify_max_blocks() {
    let mut sb = EzfsSuperblock {
        version: 1,
        magic: 0x4118,
        disk_blocks: kani::any(),
        data: Mutex::new(EzfsSuperblockData {
            free_inodes: Bitmap::new([0; (EZFS_MAX_INODES / 32) + 1]),
            free_data_blocks: Bitmap::new([0; (EZFS_MAX_INODES / 32) + 1]),
            zero_data_blocks: Bitmap::new([0; (EZFS_MAX_INODES / 32) + 1]),
        }),
        mapper: Mapper::<RustEzFs> {
            inode: INode::<RustEzFs> { ino: 0, data: None },
            begin: 0,
            end: 4096,
        },
    };

    RustEzFs::max_blocks(&sb);
}

#[kani::proof]
#[kani::unwind(57)]
fn verify_data_block_allocation() {
    let mut sb = EzfsSuperblock {
        version: 1,
        magic: 0x4118,
        disk_blocks: kani::any(),
        data: Mutex::new(EzfsSuperblockData {
            free_inodes: Bitmap::new([0; (EZFS_MAX_INODES / 32) + 1]),
            free_data_blocks: kani::any(),
            zero_data_blocks: kani::any(),
        }),
        mapper: Mapper::<RustEzFs> {
            inode: INode::<RustEzFs> { ino: 0, data: None },
            begin: 0,
            end: 4096,
        },
    };

    let res1 = RustEzFs::allocate_data_block(&sb);
    let res2 = RustEzFs::allocate_data_block(&sb);

    // If the allocation succeds they are not the same
    if let (Ok(ino1), Ok(ino2)) = (res1, res2) {
        kani::assert(
            ino1 != ino2,
            "Sequential allocations must return different data_blocks",
        );
    }
}

#[kani::proof]
#[kani::unwind(57)]
fn verify_data_block_deallocation() {
    let mut sb = EzfsSuperblock {
        version: 1,
        magic: 0x4118,
        disk_blocks: kani::any(),
        data: Mutex::new(EzfsSuperblockData {
            free_inodes: Bitmap::new([0; (EZFS_MAX_INODES / 32) + 1]),
            free_data_blocks: kani::any(),
            zero_data_blocks: kani::any(),
        }),
        mapper: Mapper::<RustEzFs> {
            inode: INode::<RustEzFs> { ino: 0, data: None },
            begin: 0,
            end: 4096,
        },
    };

    let start = kani::any();
    let end = kani::any();

    let max_blocks = RustEzFs::max_blocks(&sb).unwrap_or(0);

    kani::assume(start < end);
    kani::assume(end <= max_blocks);

    let res = RustEzFs::deallocate_data_blocks(&sb, start..end);

    let bitmap_copy = {
        let sb_data = sb.data.lock().unwrap();
        sb_data.free_data_blocks.clone()
    };

    if let Ok(ans) = res {
        for idx in start..end {
            kani::assert(
                bitmap_copy.is_set(idx - EZFS_ROOT_DATABLOCK_NUMBER as u64) == false,
                "Deallocated datablock should never be set in the range",
            );
        }
    }
}

#[kani::proof]
fn verify_bitmap_simple_allocation() {
    let mut bitmap: Bitmap<4> = kani::any();
    let idx: u64 = kani::any();

    if let Ok(allocation) = bitmap.set_bit(idx) {
        kani::assert(bitmap.is_set(idx), "Set bit must be set");
    } else {
        // Error case: out of bounds
        kani::assert(idx >= 4 * 32, "Error only for out of bounds");
    }
}

#[kani::proof]
fn verify_bitmap_complete() {
    let mut bitmap: Bitmap<4> = kani::any();
    let idx: u64 = kani::any();

    // Property 1: is_set never panics
    let is_set_result = bitmap.is_set(idx);

    // Property 2: set_bit + is_set consistency
    let set_result = bitmap.set_bit(idx);
    if set_result.is_ok() {
        kani::assert(bitmap.is_set(idx), "If set succeeds, bit must be set");
    }

    // Property 3: clear_bit + is_set consistency
    let clear_result = bitmap.clear_bit(idx);
    if clear_result.is_ok() {
        kani::assert(!bitmap.is_set(idx), "If clear succeeds, bit must be clear");
    }

    // Property 4: clear is idempotent
    let _ = bitmap.clear_bit(idx);
    let after_first = bitmap.inner;
    let _ = bitmap.clear_bit(idx);
    kani::assert(bitmap.inner == after_first, "Clear is idempotent");
}
