use std::{marker::PhantomData, num::NonZeroI32};

use crate::{
    fs::FileSystem,
    inode::{self, INode, INodeState},
    types::{ARef, Error, Result},
};

pub trait DataInited {}

pub enum Type {
    Independent,
    BlockDev,
}

pub enum New {}

pub enum Ready {}

impl DataInited for Ready {}

pub struct Inner {}

pub struct SuperBlock<T: FileSystem + ?Sized, S> {
    magic: usize,
    data: T::Data,
    _p: PhantomData<(S, T)>,
}

impl<T: FileSystem> SuperBlock<T, New> {
    pub fn new(data: T::Data) -> Self {
        SuperBlock {
            magic: 0,
            data,
            _p: PhantomData,
        }
    }

    pub fn magic(&self) -> usize {
        self.magic
    }

    pub fn set_magic(&mut self, magic: usize) -> &mut Self {
        self.magic = magic;

        self
    }

    pub fn ready(self) -> SuperBlock<T, Ready> {
        SuperBlock {
            magic: self.magic,
            data: self.data,
            _p: PhantomData,
        }
    }
}

impl<T: FileSystem, S: DataInited> SuperBlock<T, S> {
    pub fn get_or_create_inode(ino: usize) -> Result<INodeState<T>> {
        // TODO: Add kani
        // let exists: bool = kani::any();
        let exists = false;

        if exists {
            // TODO: Add true case
            // let inode = INode::new(ino, Some(kani::any()));
            // FIXME:
            Err(Error(1))
        } else {
            // Return uninitialized inode
            let inode = inode::New::new(ino, None);
            Ok(INodeState::Uninitilized(inode))
        }
    }
}
