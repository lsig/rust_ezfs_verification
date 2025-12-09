use crate::inode;
use crate::sb::{self, SuperBlock};
use crate::types::Result;

pub struct File;

pub trait FileSystem {
    type Data: Send + Sync;

    type INodeData: Send + Sync;

    const NAME: &str;
    const SUPER_TYPE: sb::Type = sb::Type::Independent;

    fn fill_super(
        sb: &mut SuperBlock<Self, sb::New>,
        mapper: Option<inode::Mapper<Self>>,
    ) -> Result<Self::Data>;

    // fn init_root(sb: &SuperBlock<Self>) -> Result<dentry::Root<Self>>;
}

pub type Offset = i64;
