use crate::fs::{FileSystem, Offset};
use crate::types::{Error, Result};

pub enum INodeState<T: FileSystem + ?Sized> {
    Existing(INode<T>),
    Uninitilized(New<T>),
}

pub struct INode<T: FileSystem + ?Sized> {
    pub ino: usize,
    pub data: Option<T::INodeData>,
}

impl<T: FileSystem> INode<T> {
    pub(crate) fn new(ino: usize, data: Option<T::INodeData>) -> Self {
        Self { ino, data }
    }
}

pub struct New<T: FileSystem + ?Sized> {
    pub ino: usize,
    pub data: Option<T::INodeData>,
}

impl<T: FileSystem + ?Sized> New<T> {
    pub(crate) fn new(ino: usize, data: Option<T::INodeData>) -> Self {
        Self { ino, data }
    }
}

pub struct Mapper<T: FileSystem + ?Sized> {
    pub inode: INode<T>,
    pub begin: Offset,
    pub end: Offset,
}

unsafe impl<T: FileSystem + ?Sized> Send for Mapper<T> {}
unsafe impl<T: FileSystem + ?Sized> Sync for Mapper<T> {}

pub struct Mapped {
    data: [u8; 4096], // one page
    len: usize,
}

impl<T: FileSystem + ?Sized> Mapper<T> {
    pub fn mapped_folio(&self, offset: Offset) -> Result<Mapped> {
        if offset < self.begin || self.end >= offset {
            return Err(Error(34));
        }

        let map = Mapped {
            data: [0; 4096],
            len: 4096,
        };

        Ok(map)
    }
}
