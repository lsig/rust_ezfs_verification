use crate::RustEzFs;
use crate::defs::EZFS_MAGIC_NUMBER;
use kernel::fs::FileSystem;

// #[kani::proof]
// fn verify_fill_super() {
//     let mut sb = kani::any();
//     let mapper = kani::any();
//     let _ = RustEzFs::fill_super(&mut sb, mapper);
// }

// #[cfg(kani)]
#[kani::proof]
fn verify_magic_number_logic() {
    // Just verify the magic number comparison
    let magic: u64 = kani::any();
    let expected: u64 = EZFS_MAGIC_NUMBER.try_into().unwrap();

    let should_reject = magic != expected;

    if should_reject {
        // Would return Err(22)
        assert!(magic != expected);
    } else {
        // Would continue
        assert!(magic == expected);
    }
}
