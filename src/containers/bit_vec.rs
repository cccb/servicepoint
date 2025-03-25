/// A byte-packed vector of booleans.
///
/// The implementation is provided by [bitvec].
/// This is an alias for the specific type of [`bitvec::BitVec`] used in this crate.
pub type BitVec = bitvec::BitVec<u8, bitvec::Msb0>;

pub mod bitvec {
    //! Re-export of the used library [mod@bitvec].
    pub use bitvec::prelude::*;
}
