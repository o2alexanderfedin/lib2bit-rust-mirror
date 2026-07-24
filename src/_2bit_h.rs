use super::*;

/// This structure holds the fixed-sized file header (16 bytes, of which 4 are blank). The version should always be 0. In theory, the endianness of the magic number can change (indicating that everything in the file should be swapped). As I've never actually seen this occur in the wild I've not bothered implementing it, though it'd be simple enough to do so.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct TwoBitHeader {
    pub(crate) magic: u32,
    pub(crate) version: u32,
    pub(crate) n_chroms: u32,
}

/// This structure holds the chromosome names and the offset to the on-disk beginning of their sequences
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct TwoBitCL {
    pub(crate) chrom: *mut *mut i8,
    pub(crate) offset: *mut u32,
}

/// This structure holds the number, location and size of the hard (N) and soft (lower case) masked blocks.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct TwoBitMaskedIdx {
    pub(crate) size: *mut u32,
    pub(crate) n_block_count: *mut u32,
    pub(crate) n_block_start: *mut *mut u32,
    pub(crate) n_block_sizes: *mut *mut u32,
    pub(crate) mask_block_count: *mut u32,
    pub(crate) mask_block_start: *mut *mut u32,
    pub(crate) mask_block_sizes: *mut *mut u32,
    pub(crate) offset: *mut u64,
}

/// This is the main structure for holding a 2bit file
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct TwoBit {
    pub(crate) fp: *mut FILE,
    pub(crate) sz: u64,
    pub(crate) offset: u64,
    pub(crate) data: *mut (),
    pub(crate) hdr: *mut TwoBitHeader,
    pub(crate) cl: *mut TwoBitCL,
    pub(crate) idx: *mut TwoBitMaskedIdx,
}
