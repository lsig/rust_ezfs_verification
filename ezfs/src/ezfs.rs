// SPDX-License-Identifier: GPL-2.0

//! Log-based filesystem written in Rust

mod defs;
mod dir;
mod inode;
mod sb;
#[cfg(kani)]
mod verification;

use crate::dir::{DirEntryStore, EzfsDirEntry};
use crate::inode::{EzfsInode, InodeStore};
use crate::sb::{EzfsSuperblock, EzfsSuperblockDisk};
use defs::*;
// use kernel::dentry;
use kernel::fs::{File, FileSystem, Offset};
use kernel::inode::{INode, INodeState, Mapper};
// use kernel::prelude::*;
use kernel::sb::{New, SuperBlock, Type as SuperType};
// use kernel::time::UNIX_EPOCH;
// use kernel::transmute::FromBytes;
use kernel::fs;
use kernel::types::{ARef, Error, Locked, Result};

use core::marker::{PhantomData, Send, Sync};
use core::mem::size_of;

struct RustEzFs;

impl RustEzFs {
    fn get_max_blocks(sb: &EzfsSuperblock) -> u64 {
        (sb.disk_blocks - 2).min(EZFS_MAX_DATA_BLKS as u64)
    }

    fn allocate_inode(sb: &EzfsSuperblock) -> Result<usize> {
        let mut sb_data = sb.data.lock().map_err(|_| Error(21))?;

        for idx in 0..EZFS_MAX_INODES {
            if !sb_data.free_inodes.is_set(idx as u64) {
                sb_data.free_inodes.set_bit(idx as u64);
                return Ok(idx + 1); // FS is 1-indexed
            }
        }

        Err(Error(21))
    }
}

impl FileSystem for RustEzFs {
    type Data = Box<EzfsSuperblock>;
    type INodeData = EzfsInode;
    const NAME: &str = "rustezfs";
    const SUPER_TYPE: SuperType = SuperType::BlockDev;

    fn fill_super(
        sb: &mut SuperBlock<Self, New>,
        mapper: Option<Mapper<Self>>,
    ) -> Result<Self::Data> {
        let Some(mapper) = mapper else {
            return Err(Error(22));
        };

        // ASSUME: we succefully read the ezfs superblock from disk
        let disk_sb = EzfsSuperblockDisk::default();

        if disk_sb.magic() != EZFS_MAGIC_NUMBER as u64 {
            return Err(Error(22));
        }

        let ezfs_sb = Box::new(EzfsSuperblock::new(disk_sb, mapper));

        sb.set_magic(EZFS_MAGIC_NUMBER);

        Ok(ezfs_sb)
    }

    // fn init_root(sb: &SuperBlock<Self>) -> Result<dentry::Root<Self>> {
    //
    // }
    // fn init_root(sb: &SuperBlock<Self>) -> Result<dentry::Root<Self>> {
    //     let inode = Self::iget(sb, EZFS_ROOT_INODE_NUMBER)?;
    //     dentry::Root::try_new(inode)
    // }
}

// impl kernel::inode::Operations for RustEzFs {
//     type FileSystem = Self;
//
//     fn lookup(
//         parent: &Locked<&INode<Self::FileSystem>, kernel::inode::ReadSem>,
//         dentry: dentry::Unhashed<'_, Self::FileSystem>,
//     ) -> Result<Option<ARef<dentry::DEntry<Self::FileSystem>>>> {
//         let sb = &*parent.super_block();
//         let h = sb.data();
//
//         let name = dentry.name();
//
//         if name.len() > EZFS_FILENAME_BUF_SIZE {
//             return Err(ENAMETOOLONG);
//         }
//
//         let ezfs_dir_inode = parent.data();
//
//         let offset = ezfs_dir_inode
//             .data_blk_num()
//             .checked_mul(EZFS_BLOCK_SIZE as u64)
//             .ok_or(EIO)?;
//
//         let mapped = h.mapper.mapped_folio(offset.try_into()?)?;
//         let dir_entries =
//             DirEntryStore::from_bytes(&mapped[..size_of::<DirEntryStore>()]).ok_or(EIO)?;
//
//         let dir_entry = dir_entries
//             .iter()
//             .find(|x| x.filename() == name && x.is_active());
//
//         let inode = if let Some(entry) = dir_entry {
//             Some(Self::iget(sb, entry.inode_no().try_into()?)?)
//         } else {
//             None
//         };
//
//         dentry.splice_alias(inode)
//     }
// }

// impl file::Operations for RustEzFs {
//     type FileSystem = Self;
//
//     fn seek(file: &File<Self>, offset: Offset, whence: file::Whence) -> Result<Offset> {
//         file::generic_seek(file, offset, whence)
//     }
//
//     fn read(_: &File<Self>, _: &mut kernel::user::Writer, _: &mut Offset) -> Result<usize> {
//         Err(EISDIR)
//     }
//
//     fn read_dir(
//         file: &File<Self>,
//         inode: &Locked<&INode<Self>, kernel::inode::ReadSem>,
//         emitter: &mut file::DirEmitter,
//     ) -> Result {
//         let pos: usize = emitter.pos().try_into().map_err(|_| ENOENT)?;
//
//         if pos < 2 {
//             if !emitter.emit_dots(file) {
//                 return Ok(());
//             }
//         }
//
//         let sb = &*inode.super_block();
//         let h = sb.data();
//
//         let index = {
//             let disk_pos = pos.checked_sub(2).ok_or(ENOENT)?;
//
//             if disk_pos % size_of::<EzfsDirEntry>() != 0 {
//                 return Err(ENOENT);
//             }
//
//             disk_pos / size_of::<EzfsDirEntry>()
//         };
//
//         if index >= EZFS_MAX_CHILDREN {
//             return Ok(());
//         }
//
//         let ezfs_dir_inode = inode.data();
//
//         let offset = ezfs_dir_inode
//             .data_blk_num()
//             .checked_mul(EZFS_BLOCK_SIZE as u64)
//             .ok_or(EIO)?;
//
//         let mapped = h.mapper.mapped_folio(offset.try_into()?)?;
//         let dir_entries =
//             DirEntryStore::from_bytes(&mapped[..size_of::<DirEntryStore>()]).ok_or(EIO)?;
//
//         let inode_store_offset = EZFS_INODE_STORE_DATABLOCK_NUMBER * EZFS_BLOCK_SIZE;
//         let mapped_inode_store = h.mapper.mapped_folio(inode_store_offset.try_into()?)?;
//         let inode_store =
//             InodeStore::from_bytes(&mapped_inode_store[..size_of::<InodeStore>()]).ok_or(EIO)?;
//
//         let active_entries = dir_entries
//             .iter()
//             .skip(index)
//             .filter(|&entry| entry.is_active());
//
//         for entry in active_entries {
//             let ino: usize = entry.inode_no().try_into()?;
//             let entry_inode = inode_store[ino - EZFS_ROOT_INODE_NUMBER];
//             let etype = file::DirEntryType::from_mode(entry_inode.mode());
//
//             if !emitter.emit(
//                 size_of::<EzfsDirEntry>() as i64,
//                 entry.filename(),
//                 entry.inode_no(),
//                 etype,
//             ) {
//                 return Ok(());
//             }
//         }
//
//         Ok(())
//     }
// }
