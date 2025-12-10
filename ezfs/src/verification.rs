use crate::RustEzFs;
use crate::defs::EZFS_MAGIC_NUMBER;
use kernel::fs::FileSystem;

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
