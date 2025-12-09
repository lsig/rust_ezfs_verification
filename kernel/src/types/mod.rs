use core::num::NonZeroI32;
use std::marker::PhantomData;

pub type ARef<T> = Box<T>;

pub struct Locked<T, L> {
    inner: T,
    _lock: PhantomData<L>,
}

impl<T, L> Locked<T, L> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            _lock: PhantomData,
        }
    }
}

impl<T, L> core::ops::Deref for Locked<T, L> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

pub struct ReadSem;
pub struct WriteSem;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Error(pub i32);

pub type Result<T = (), E = Error> = core::result::Result<T, E>;
