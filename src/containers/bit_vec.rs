/// A byte-packed vector of booleans.
///
/// The implementation is provided by [bitvec].
/// This is an alias for the specific type of [`bitvec::BitVec`] used in this crate.
pub type BitVecU8Msb0 = bitvec::BitVec<u8, bitvec::Msb0>;

pub mod bitvec {
    //! Re-export of the used library [`::bitvec`].
    pub use ::bitvec::prelude::*;
}
