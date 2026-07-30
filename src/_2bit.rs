use super::*;
use crate::_2bit_h::{TwoBit, TwoBitCL, TwoBitHeader, TwoBitMaskedIdx};

pub(crate) extern "C" fn twobit_chrom_list_destroy(tb: &TwoBit) -> () {
    let mut i: u32 = 0 as u32;
    if !((*tb).cl).is_null() {
        if !(unsafe { (*(*tb).cl).offset }).is_null() {
            unsafe { free(unsafe { (*(*tb).cl).offset } as *mut ()) };
        }
        if !(unsafe { (*(*tb).cl).chrom }).is_null() {
            {
                i = 0 as u32;
                '__b0: loop {
                    if !(i < unsafe { (*(*tb).hdr).n_chroms }) {
                        break '__b0;
                    }
                    '__c0: loop {
                        if !(unsafe { *unsafe { (*(*tb).cl).chrom.add(i as usize) } }).is_null() {
                            unsafe {
                                free(unsafe { *unsafe { (*(*tb).cl).chrom.add(i as usize) } }
                                    as *mut ())
                            };
                        }
                        break '__c0;
                    }
                    i = i.wrapping_add(1);
                }
            }
            unsafe { free(unsafe { (*(*tb).cl).chrom } as *mut ()) };
        }
        unsafe { free((*tb).cl as *mut ()) };
    }
}

pub(crate) extern "C" fn twobit_index_destroy(tb: &TwoBit) -> () {
    let mut i: u32 = 0 as u32;
    if !((*tb).idx).is_null() {
        if !(unsafe { (*(*tb).idx).size }).is_null() {
            unsafe { free(unsafe { (*(*tb).idx).size } as *mut ()) };
        }
        if !(unsafe { (*(*tb).idx).n_block_count }).is_null() {
            unsafe { free(unsafe { (*(*tb).idx).n_block_count } as *mut ()) };
        }
        if !(unsafe { (*(*tb).idx).n_block_start }).is_null() {
            {
                i = 0 as u32;
                '__b1: loop {
                    if !(i < unsafe { (*(*tb).hdr).n_chroms }) {
                        break '__b1;
                    }
                    '__c1: loop {
                        if !(unsafe { *unsafe { (*(*tb).idx).n_block_start.add(i as usize) } })
                            .is_null()
                        {
                            unsafe {
                                free(unsafe {
                                    *unsafe { (*(*tb).idx).n_block_start.add(i as usize) }
                                } as *mut ())
                            };
                        }
                        break '__c1;
                    }
                    i = i.wrapping_add(1);
                }
            }
            unsafe { free(unsafe { (*(*tb).idx).n_block_start } as *mut ()) };
        }
        if !(unsafe { (*(*tb).idx).n_block_sizes }).is_null() {
            {
                i = 0 as u32;
                '__b2: loop {
                    if !(i < unsafe { (*(*tb).hdr).n_chroms }) {
                        break '__b2;
                    }
                    '__c2: loop {
                        if !(unsafe { *unsafe { (*(*tb).idx).n_block_sizes.add(i as usize) } })
                            .is_null()
                        {
                            unsafe {
                                free(unsafe {
                                    *unsafe { (*(*tb).idx).n_block_sizes.add(i as usize) }
                                } as *mut ())
                            };
                        }
                        break '__c2;
                    }
                    i = i.wrapping_add(1);
                }
            }
            unsafe { free(unsafe { (*(*tb).idx).n_block_sizes } as *mut ()) };
        }
        if !(unsafe { (*(*tb).idx).mask_block_count }).is_null() {
            unsafe { free(unsafe { (*(*tb).idx).mask_block_count } as *mut ()) };
        }
        if !(unsafe { (*(*tb).idx).mask_block_start }).is_null() {
            {
                i = 0 as u32;
                '__b3: loop {
                    if !(i < unsafe { (*(*tb).hdr).n_chroms }) {
                        break '__b3;
                    }
                    '__c3: loop {
                        if !(unsafe { *unsafe { (*(*tb).idx).mask_block_start.add(i as usize) } })
                            .is_null()
                        {
                            unsafe {
                                free(unsafe {
                                    *unsafe { (*(*tb).idx).mask_block_start.add(i as usize) }
                                } as *mut ())
                            };
                        }
                        break '__c3;
                    }
                    i = i.wrapping_add(1);
                }
            }
            unsafe { free(unsafe { (*(*tb).idx).mask_block_start } as *mut ()) };
        }
        if !(unsafe { (*(*tb).idx).mask_block_sizes }).is_null() {
            {
                i = 0 as u32;
                '__b4: loop {
                    if !(i < unsafe { (*(*tb).hdr).n_chroms }) {
                        break '__b4;
                    }
                    '__c4: loop {
                        if !(unsafe { *unsafe { (*(*tb).idx).mask_block_sizes.add(i as usize) } })
                            .is_null()
                        {
                            unsafe {
                                free(unsafe {
                                    *unsafe { (*(*tb).idx).mask_block_sizes.add(i as usize) }
                                } as *mut ())
                            };
                        }
                        break '__c4;
                    }
                    i = i.wrapping_add(1);
                }
            }
            unsafe { free(unsafe { (*(*tb).idx).mask_block_sizes } as *mut ()) };
        }
        if !(unsafe { (*(*tb).idx).offset }).is_null() {
            unsafe { free(unsafe { (*(*tb).idx).offset } as *mut ()) };
        }
        unsafe { free((*tb).idx as *mut ()) };
    }
}

pub(crate) extern "C" fn twobit_hdr_destroy(tb: &TwoBit) -> () {
    if !((*tb).hdr).is_null() {
        unsafe { free((*tb).hdr as *mut ()) };
    }
}

/// Closes a 2bit file and free memory.
#[allow(unused_doc_comments)]
pub(crate) extern "C" fn twobit_close(tb: *mut TwoBit) -> () {
    if !(tb).is_null() {
        if !(unsafe { (*tb).fp }).is_null() {
            unsafe { fclose(unsafe { (*tb).fp }) };
        }
        if !(unsafe { (*tb).data }).is_null() {
            unsafe { munmap(unsafe { (*tb).data }, unsafe { (*tb).sz }) };
        }
        twobit_chrom_list_destroy(unsafe { &*tb });
        twobit_index_destroy(unsafe { &*tb });

        ///N.B., this needs to be called last
        twobit_hdr_destroy(unsafe { &*tb });
        unsafe { free(tb as *mut ()) };
    }
}

///    Read nmemb elements, each of size sz from the current file offset
///    into data. Return the number of elements read. On error, the return
///    value is either 0 or less than nmemb
pub(crate) extern "C" fn twobit_read(data: &mut [u8], nmemb: u64, tb: &mut TwoBit) -> u64 {
    if !((*tb).data).is_null() {
        if unsafe {
            __builtin___memcpy_chk(
                data.as_ptr() as *mut (),
                unsafe { (*tb).data.add((*tb).offset as usize) } as *const (),
                nmemb.wrapping_mul(data.len() as u64),
                unsafe { __builtin_object_size(data.as_ptr() as *const (), 0) },
            )
        } == 0 as *mut ()
        {
            return 0 as u64;
        }
        (*tb).offset = (*tb)
            .offset
            .wrapping_add(nmemb.wrapping_mul(data.len() as u64) as u64);
        return nmemb;
    } else {
        return unsafe { fread(data.as_ptr() as *mut (), data.len() as u64, nmemb, (*tb).fp) };
    }
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn twobit_hdr_read(tb: *mut TwoBit) -> () {
    unsafe {
        ///Read the first 16 bytes
        let hdr: *mut TwoBitHeader =
            unsafe { calloc(1 as u64, core::mem::size_of::<TwoBitHeader>() as u64) }
                as *mut TwoBitHeader;
        '__b5: loop {
            '__c5: loop {
                ///Read the first 16 bytes
                let mut data: [u32; 4] = [0; 4];
                if (hdr).is_null() as i32 != 0 {
                    return;
                }
                if twobit_read(
                    unsafe {
                        let __p = &raw mut data[0 as usize] as *mut u8 as *mut u8;
                        if __p.is_null() {
                            &mut []
                        } else {
                            core::slice::from_raw_parts_mut(__p, 4 as usize)
                        }
                    },
                    4 as u64,
                    unsafe { &mut *tb },
                ) != 4 as u64
                {
                    break '__b5;
                }

                ///Magic
                unsafe {
                    (*hdr).magic = data[0 as usize]
                };
                if unsafe { (*hdr).magic } != 440477507 as u32 {
                    unsafe {
                        fprintf(
                            __stderrp,
                            c"[twobitHdrRead] Received an invalid file magic number (0x%x)!\n"
                                .as_ptr() as *mut i8 as *const i8,
                            unsafe { (*hdr).magic },
                        )
                    };
                    break '__b5;
                }

                ///Version
                unsafe {
                    (*hdr).version = data[1 as usize]
                };
                if unsafe { (*hdr).version } != 0 as u32 {
                    unsafe {
                        fprintf(__stderrp,
                            c"[twobitHdrRead] The file version is %u while only version 0 is defined!\n".as_ptr()
                                    as *mut i8 as *const i8, unsafe { (*hdr).version })
                    };
                    break '__b5;
                }

                ///Sequence Count
                unsafe {
                    (*hdr).n_chroms = data[2 as usize]
                };
                if unsafe { (*hdr).n_chroms } == 0 as u32 {
                    eprintln!(
                        "[twobitHdrRead] There are apparently no chromosomes/contigs in this file!"
                    );
                    break '__b5;
                }
                unsafe { (*tb).hdr = hdr };
                return;
                break '__c5;
            }
            if !(false) {
                break '__b5;
            }
        }
        if !(hdr).is_null() {
            unsafe { free(hdr as *mut ()) };
        }
    }
}

pub(crate) extern "C" fn twobit_chrom_list_read(tb: *mut TwoBit) -> () {
    let mut i: u32 = 0 as u32;
    let mut byte: u8 = 0 as u8;
    let mut str: *mut i8 = core::ptr::null_mut();
    let mut cl: *mut TwoBitCL = core::ptr::null_mut();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 {
            break;
        }
        '__s7: {
            match __state {
                0 => {
                    __state = 3;
                }
                2 => {
                    if !(str).is_null() {
                        __state = 32;
                    } else {
                        __state = 31;
                    }
                }
                3 => {
                    __state = 4;
                }
                4 => {
                    str = 0 as *mut () as *mut i8;
                    __state = 5;
                }
                5 => {
                    cl = unsafe { calloc(1 as u64, core::mem::size_of::<TwoBitCL>() as u64) }
                        as *mut TwoBitCL;
                    __state = 6;
                }
                6 => {
                    if (cl).is_null() as i32 != 0 {
                        __state = 8;
                    } else {
                        __state = 7;
                    }
                }
                7 => {
                    unsafe {
                        (*cl).chrom = unsafe {
                            calloc(
                                unsafe { (*unsafe { (*tb).hdr }).n_chroms } as u64,
                                core::mem::size_of::<*mut i8>() as u64,
                            )
                        } as *mut *mut i8
                    };
                    __state = 9;
                }
                8 => {
                    __state = 2;
                }
                9 => {
                    unsafe {
                        (*cl).offset = unsafe {
                            malloc(
                                (core::mem::size_of::<u32>() as u64)
                                    .wrapping_mul(
                                        unsafe { (*unsafe { (*tb).hdr }).n_chroms } as u64
                                    ),
                            )
                        } as *mut u32
                    };
                    __state = 10;
                }
                10 => {
                    if (unsafe { (*cl).chrom }).is_null() as i32 != 0 {
                        __state = 12;
                    } else {
                        __state = 11;
                    }
                }
                11 => {
                    if (unsafe { (*cl).offset }).is_null() as i32 != 0 {
                        __state = 14;
                    } else {
                        __state = 13;
                    }
                }
                12 => {
                    __state = 2;
                }
                13 => {
                    i = 0 as u32;
                    __state = 16;
                }
                14 => {
                    __state = 2;
                }
                15 => {
                    unsafe { (*tb).cl = cl };
                    __state = 29;
                }
                16 => {
                    if i < unsafe { (*unsafe { (*tb).hdr }).n_chroms } {
                        __state = 17;
                    } else {
                        __state = 15;
                    }
                }
                17 => {
                    if twobit_read(
                        unsafe {
                            let __p = &raw mut byte as *mut u8 as *mut u8;
                            if __p.is_null() {
                                &mut []
                            } else {
                                core::slice::from_raw_parts_mut(__p, 1 as usize)
                            }
                        },
                        1 as u64,
                        unsafe { &mut *tb },
                    ) != 1 as u64
                    {
                        __state = 20;
                    } else {
                        __state = 19;
                    }
                }
                18 => {
                    {
                        let __old = i;
                        i = i.wrapping_add(1);
                        __old
                    };
                    __state = 16;
                }
                19 => {
                    str = unsafe {
                        calloc((1 + byte as i32) as u64, core::mem::size_of::<i8>() as u64)
                    } as *mut i8;
                    __state = 21;
                }
                20 => {
                    __state = 2;
                }
                21 => {
                    if (str).is_null() as i32 != 0 {
                        __state = 23;
                    } else {
                        __state = 22;
                    }
                }
                22 => {
                    if twobit_read(
                        unsafe {
                            let __p = str as *mut u8 as *mut u8;
                            if __p.is_null() {
                                &mut []
                            } else {
                                core::slice::from_raw_parts_mut(__p, 1 as usize)
                            }
                        },
                        byte as u64,
                        unsafe { &mut *tb },
                    ) != byte as u64
                    {
                        __state = 25;
                    } else {
                        __state = 24;
                    }
                }
                23 => {
                    __state = 2;
                }
                24 => {
                    unsafe { *unsafe { (*cl).chrom.add(i as usize) } = str };
                    __state = 26;
                }
                25 => {
                    __state = 2;
                }
                26 => {
                    str = 0 as *mut () as *mut i8;
                    __state = 27;
                }
                27 => {
                    if twobit_read(
                        unsafe {
                            let __p = unsafe { unsafe { (*cl).offset.add(i as usize) } } as *mut u8
                                as *mut u8;
                            if __p.is_null() {
                                &mut []
                            } else {
                                core::slice::from_raw_parts_mut(
                                    __p,
                                    core::mem::size_of::<u32>() as usize,
                                )
                            }
                        },
                        1 as u64,
                        unsafe { &mut *tb },
                    ) != 1 as u64
                    {
                        __state = 28;
                    } else {
                        __state = 18;
                    }
                }
                28 => {
                    __state = 2;
                }
                29 => {
                    return;
                }
                30 => {
                    __state = 2;
                }
                31 => {
                    if !(cl).is_null() {
                        __state = 33;
                    } else {
                        __state = 1;
                    }
                }
                32 => {
                    unsafe { free(str as *mut ()) };
                    __state = 31;
                }
                33 => {
                    if !(unsafe { (*cl).offset }).is_null() {
                        __state = 35;
                    } else {
                        __state = 34;
                    }
                }
                34 => {
                    if !(unsafe { (*cl).chrom }).is_null() {
                        __state = 37;
                    } else {
                        __state = 36;
                    }
                }
                35 => {
                    unsafe { free(unsafe { (*cl).offset } as *mut ()) };
                    __state = 34;
                }
                36 => {
                    unsafe { free(cl as *mut ()) };
                    __state = 1;
                }
                37 => {
                    i = 0 as u32;
                    __state = 39;
                }
                38 => {
                    unsafe { free(unsafe { (*cl).chrom } as *mut ()) };
                    __state = 36;
                }
                39 => {
                    if i < unsafe { (*unsafe { (*tb).hdr }).n_chroms } {
                        __state = 40;
                    } else {
                        __state = 38;
                    }
                }
                40 => {
                    if !(unsafe { *unsafe { (*cl).chrom.add(i as usize) } }).is_null() {
                        __state = 42;
                    } else {
                        __state = 41;
                    }
                }
                41 => {
                    {
                        let __old = i;
                        i = i.wrapping_add(1);
                        __old
                    };
                    __state = 39;
                }
                42 => {
                    unsafe { free(unsafe { *unsafe { (*cl).chrom.add(i as usize) } } as *mut ()) };
                    __state = 41;
                }
                _ => {}
            }
        }
    }
}

///    Seek to a specific position, which is essentially trivial for memmaped stuff
///
///    Returns: 0 on success, -1 on error
pub(crate) extern "C" fn twobit_seek(tb: &mut TwoBit, offset: u64) -> i32 {
    if offset >= (*tb).sz {
        return -1;
    }
    if !((*tb).data).is_null() {
        (*tb).offset = offset;
        return 0;
    } else {
        return unsafe { fseek((*tb).fp, offset as i64, 0) };
    }
}

///    Like ftell, but generalized to handle memmaped files
///
///    Returns the offset
pub(crate) extern "C" fn twobit_tell(tb: &TwoBit) -> u64 {
    if !((*tb).data).is_null() {
        return (*tb).offset;
    }
    return unsafe { ftell((*tb).fp) } as u64;
}

///    Fill in tb->idx.
///
///    Note that the masked stuff will only be stored if storeMasked == 1, since it uses gobs of memory otherwise.
///    On error, tb->idx is left as NULL.
pub(crate) extern "C" fn twobit_index_read(tb: *mut TwoBit, store_masked_1: i32) -> () {
    let mut i: u32 = 0 as u32;
    let mut data: [u32; 2] = [0; 2];
    let mut idx: *mut TwoBitMaskedIdx = core::ptr::null_mut();
    let mut __state: i32 = 0;
    loop {
        if __state == 1 {
            break;
        }
        '__s9: {
            match __state {
                0 => {
                    __state = 3;
                }
                2 => {
                    if !(idx).is_null() {
                        __state = 71;
                    } else {
                        __state = 1;
                    }
                }
                3 => {
                    idx =
                        unsafe { calloc(1 as u64, core::mem::size_of::<TwoBitMaskedIdx>() as u64) }
                            as *mut TwoBitMaskedIdx;
                    __state = 4;
                }
                4 => {
                    if (idx).is_null() as i32 != 0 {
                        __state = 6;
                    } else {
                        __state = 5;
                    }
                }
                5 => {
                    unsafe {
                        (*idx).size = unsafe {
                            malloc(
                                (unsafe { (*unsafe { (*tb).hdr }).n_chroms } as u64)
                                    .wrapping_mul(core::mem::size_of::<u32>() as u64),
                            )
                        } as *mut u32
                    };
                    __state = 7;
                }
                6 => {
                    return;
                }
                7 => {
                    unsafe {
                        (*idx).n_block_count = unsafe {
                            calloc(
                                unsafe { (*unsafe { (*tb).hdr }).n_chroms } as u64,
                                core::mem::size_of::<u32>() as u64,
                            )
                        } as *mut u32
                    };
                    __state = 8;
                }
                8 => {
                    unsafe {
                        (*idx).n_block_start = unsafe {
                            calloc(
                                unsafe { (*unsafe { (*tb).hdr }).n_chroms } as u64,
                                core::mem::size_of::<*mut u32>() as u64,
                            )
                        } as *mut *mut u32
                    };
                    __state = 9;
                }
                9 => {
                    unsafe {
                        (*idx).n_block_sizes = unsafe {
                            calloc(
                                unsafe { (*unsafe { (*tb).hdr }).n_chroms } as u64,
                                core::mem::size_of::<*mut u32>() as u64,
                            )
                        } as *mut *mut u32
                    };
                    __state = 10;
                }
                10 => {
                    if (unsafe { (*idx).size }).is_null() as i32 != 0 {
                        __state = 12;
                    } else {
                        __state = 11;
                    }
                }
                11 => {
                    if (unsafe { (*idx).n_block_count }).is_null() as i32 != 0 {
                        __state = 14;
                    } else {
                        __state = 13;
                    }
                }
                12 => {
                    __state = 2;
                }
                13 => {
                    if (unsafe { (*idx).n_block_start }).is_null() as i32 != 0 {
                        __state = 16;
                    } else {
                        __state = 15;
                    }
                }
                14 => {
                    __state = 2;
                }
                15 => {
                    if (unsafe { (*idx).n_block_sizes }).is_null() as i32 != 0 {
                        __state = 18;
                    } else {
                        __state = 17;
                    }
                }
                16 => {
                    __state = 2;
                }
                17 => {
                    unsafe {
                        (*idx).mask_block_count = unsafe {
                            calloc(
                                unsafe { (*unsafe { (*tb).hdr }).n_chroms } as u64,
                                core::mem::size_of::<u32>() as u64,
                            )
                        } as *mut u32
                    };
                    __state = 19;
                }
                18 => {
                    __state = 2;
                }
                19 => {
                    if (unsafe { (*idx).mask_block_count }).is_null() as i32 != 0 {
                        __state = 21;
                    } else {
                        __state = 20;
                    }
                }
                20 => {
                    if store_masked_1 != 0 {
                        __state = 23;
                    } else {
                        __state = 22;
                    }
                }
                21 => {
                    __state = 2;
                }
                22 => {
                    unsafe {
                        (*idx).offset = unsafe {
                            malloc(
                                (unsafe { (*unsafe { (*tb).hdr }).n_chroms } as u64)
                                    .wrapping_mul(core::mem::size_of::<u64>() as u64),
                            )
                        } as *mut u64
                    };
                    __state = 29;
                }
                23 => {
                    unsafe {
                        (*idx).mask_block_start = unsafe {
                            calloc(
                                unsafe { (*unsafe { (*tb).hdr }).n_chroms } as u64,
                                core::mem::size_of::<*mut u32>() as u64,
                            )
                        } as *mut *mut u32
                    };
                    __state = 24;
                }
                24 => {
                    unsafe {
                        (*idx).mask_block_sizes = unsafe {
                            calloc(
                                unsafe { (*unsafe { (*tb).hdr }).n_chroms } as u64,
                                core::mem::size_of::<*mut u32>() as u64,
                            )
                        } as *mut *mut u32
                    };
                    __state = 25;
                }
                25 => {
                    if (unsafe { (*idx).mask_block_start }).is_null() as i32 != 0 {
                        __state = 27;
                    } else {
                        __state = 26;
                    }
                }
                26 => {
                    if (unsafe { (*idx).mask_block_sizes }).is_null() as i32 != 0 {
                        __state = 28;
                    } else {
                        __state = 22;
                    }
                }
                27 => {
                    __state = 2;
                }
                28 => {
                    __state = 2;
                }
                29 => {
                    if (unsafe { (*idx).offset }).is_null() as i32 != 0 {
                        __state = 31;
                    } else {
                        __state = 30;
                    }
                }
                30 => {
                    i = 0 as u32;
                    __state = 33;
                }
                31 => {
                    __state = 2;
                }
                32 => {
                    unsafe { (*tb).idx = idx };
                    __state = 69;
                }
                33 => {
                    if i < unsafe { (*unsafe { (*tb).hdr }).n_chroms } {
                        __state = 34;
                    } else {
                        __state = 32;
                    }
                }
                34 => {
                    if twobit_seek(unsafe { &mut *tb }, unsafe {
                        *unsafe { (*unsafe { (*tb).cl }).offset.add(i as usize) }
                    } as u64)
                        != 0
                    {
                        __state = 37;
                    } else {
                        __state = 36;
                    }
                }
                35 => {
                    {
                        let __old = i;
                        i = i.wrapping_add(1);
                        __old
                    };
                    __state = 33;
                }
                36 => {
                    if twobit_read(
                        unsafe {
                            let __p = &raw mut data[0 as usize] as *mut u8 as *mut u8;
                            if __p.is_null() {
                                &mut []
                            } else {
                                core::slice::from_raw_parts_mut(
                                    __p,
                                    core::mem::size_of::<u32>() as usize,
                                )
                            }
                        },
                        2 as u64,
                        unsafe { &mut *tb },
                    ) != 2 as u64
                    {
                        __state = 39;
                    } else {
                        __state = 38;
                    }
                }
                37 => {
                    __state = 2;
                }
                38 => {
                    unsafe { *unsafe { (*idx).size.add(i as usize) } = data[0 as usize] };
                    __state = 40;
                }
                39 => {
                    __state = 2;
                }
                40 => {
                    unsafe { *unsafe { (*idx).n_block_count.add(i as usize) } = data[1 as usize] };
                    __state = 41;
                }
                41 => {
                    unsafe {
                        *unsafe { (*idx).n_block_start.add(i as usize) } = unsafe {
                            malloc(
                                (unsafe { *unsafe { (*idx).n_block_count.add(i as usize) } }
                                    as u64)
                                    .wrapping_mul(core::mem::size_of::<u32>() as u64),
                            )
                        }
                            as *mut u32
                    };
                    __state = 42;
                }
                42 => {
                    unsafe {
                        *unsafe { (*idx).n_block_sizes.add(i as usize) } = unsafe {
                            malloc(
                                (unsafe { *unsafe { (*idx).n_block_count.add(i as usize) } }
                                    as u64)
                                    .wrapping_mul(core::mem::size_of::<u32>() as u64),
                            )
                        }
                            as *mut u32
                    };
                    __state = 43;
                }
                43 => {
                    if (unsafe { *unsafe { (*idx).n_block_start.add(i as usize) } }).is_null()
                        as i32
                        != 0
                    {
                        __state = 45;
                    } else {
                        __state = 44;
                    }
                }
                44 => {
                    if (unsafe { *unsafe { (*idx).n_block_sizes.add(i as usize) } }).is_null()
                        as i32
                        != 0
                    {
                        __state = 47;
                    } else {
                        __state = 46;
                    }
                }
                45 => {
                    __state = 2;
                }
                46 => {
                    if twobit_read(
                        unsafe {
                            let __p = unsafe { *unsafe { (*idx).n_block_start.add(i as usize) } }
                                as *mut u8 as *mut u8;
                            if __p.is_null() {
                                &mut []
                            } else {
                                core::slice::from_raw_parts_mut(
                                    __p,
                                    core::mem::size_of::<u32>() as usize,
                                )
                            }
                        },
                        unsafe { *unsafe { (*idx).n_block_count.add(i as usize) } } as u64,
                        unsafe { &mut *tb },
                    ) != unsafe { *unsafe { (*idx).n_block_count.add(i as usize) } } as u64
                    {
                        __state = 49;
                    } else {
                        __state = 48;
                    }
                }
                47 => {
                    __state = 2;
                }
                48 => {
                    if twobit_read(
                        unsafe {
                            let __p = unsafe { *unsafe { (*idx).n_block_sizes.add(i as usize) } }
                                as *mut u8 as *mut u8;
                            if __p.is_null() {
                                &mut []
                            } else {
                                core::slice::from_raw_parts_mut(
                                    __p,
                                    core::mem::size_of::<u32>() as usize,
                                )
                            }
                        },
                        unsafe { *unsafe { (*idx).n_block_count.add(i as usize) } } as u64,
                        unsafe { &mut *tb },
                    ) != unsafe { *unsafe { (*idx).n_block_count.add(i as usize) } } as u64
                    {
                        __state = 51;
                    } else {
                        __state = 50;
                    }
                }
                49 => {
                    __state = 2;
                }
                50 => {
                    if twobit_read(
                        unsafe {
                            let __p = unsafe { unsafe { (*idx).mask_block_count.add(i as usize) } }
                                as *mut u8 as *mut u8;
                            if __p.is_null() {
                                &mut []
                            } else {
                                core::slice::from_raw_parts_mut(
                                    __p,
                                    core::mem::size_of::<u32>() as usize,
                                )
                            }
                        },
                        1 as u64,
                        unsafe { &mut *tb },
                    ) != 1 as u64
                    {
                        __state = 53;
                    } else {
                        __state = 52;
                    }
                }
                51 => {
                    __state = 2;
                }
                52 => {
                    if store_masked_1 != 0 {
                        __state = 55;
                    } else {
                        __state = 56;
                    }
                }
                53 => {
                    __state = 2;
                }
                54 => {
                    if twobit_read(
                        unsafe {
                            let __p = &raw mut data[0 as usize] as *mut u8 as *mut u8;
                            if __p.is_null() {
                                &mut []
                            } else {
                                core::slice::from_raw_parts_mut(
                                    __p,
                                    core::mem::size_of::<u32>() as usize,
                                )
                            }
                        },
                        1 as u64,
                        unsafe { &mut *tb },
                    ) != 1 as u64
                    {
                        __state = 68;
                    } else {
                        __state = 67;
                    }
                }
                55 => {
                    unsafe {
                        *unsafe { (*idx).mask_block_start.add(i as usize) } = unsafe {
                            malloc(
                                (unsafe { *unsafe { (*idx).mask_block_count.add(i as usize) } }
                                    as u64)
                                    .wrapping_mul(core::mem::size_of::<u32>() as u64),
                            )
                        }
                            as *mut u32
                    };
                    __state = 57;
                }
                56 => {
                    if twobit_seek(
                        unsafe { &mut *tb },
                        twobit_tell(unsafe { &*tb })
                            .wrapping_add((8 as u32).wrapping_mul(unsafe {
                                *unsafe { (*idx).mask_block_count.add(i as usize) }
                            }) as u64),
                    ) != 0
                    {
                        __state = 66;
                    } else {
                        __state = 54;
                    }
                }
                57 => {
                    unsafe {
                        *unsafe { (*idx).mask_block_sizes.add(i as usize) } = unsafe {
                            malloc(
                                (unsafe { *unsafe { (*idx).mask_block_count.add(i as usize) } }
                                    as u64)
                                    .wrapping_mul(core::mem::size_of::<u32>() as u64),
                            )
                        }
                            as *mut u32
                    };
                    __state = 58;
                }
                58 => {
                    if (unsafe { *unsafe { (*idx).mask_block_start.add(i as usize) } }).is_null()
                        as i32
                        != 0
                    {
                        __state = 60;
                    } else {
                        __state = 59;
                    }
                }
                59 => {
                    if (unsafe { *unsafe { (*idx).mask_block_sizes.add(i as usize) } }).is_null()
                        as i32
                        != 0
                    {
                        __state = 62;
                    } else {
                        __state = 61;
                    }
                }
                60 => {
                    __state = 2;
                }
                61 => {
                    if twobit_read(
                        unsafe {
                            let __p = unsafe { *unsafe { (*idx).mask_block_start.add(i as usize) } }
                                as *mut u8 as *mut u8;
                            if __p.is_null() {
                                &mut []
                            } else {
                                core::slice::from_raw_parts_mut(
                                    __p,
                                    core::mem::size_of::<u32>() as usize,
                                )
                            }
                        },
                        unsafe { *unsafe { (*idx).mask_block_count.add(i as usize) } } as u64,
                        unsafe { &mut *tb },
                    ) != unsafe { *unsafe { (*idx).mask_block_count.add(i as usize) } } as u64
                    {
                        __state = 64;
                    } else {
                        __state = 63;
                    }
                }
                62 => {
                    __state = 2;
                }
                63 => {
                    if twobit_read(
                        unsafe {
                            let __p = unsafe { *unsafe { (*idx).mask_block_sizes.add(i as usize) } }
                                as *mut u8 as *mut u8;
                            if __p.is_null() {
                                &mut []
                            } else {
                                core::slice::from_raw_parts_mut(
                                    __p,
                                    core::mem::size_of::<u32>() as usize,
                                )
                            }
                        },
                        unsafe { *unsafe { (*idx).mask_block_count.add(i as usize) } } as u64,
                        unsafe { &mut *tb },
                    ) != unsafe { *unsafe { (*idx).mask_block_count.add(i as usize) } } as u64
                    {
                        __state = 65;
                    } else {
                        __state = 54;
                    }
                }
                64 => {
                    __state = 2;
                }
                65 => {
                    __state = 2;
                }
                66 => {
                    __state = 2;
                }
                67 => {
                    unsafe {
                        *unsafe { (*idx).offset.add(i as usize) } = twobit_tell(unsafe { &*tb })
                    };
                    __state = 35;
                }
                68 => {
                    __state = 2;
                }
                69 => {
                    return;
                }
                70 => {
                    __state = 2;
                }
                71 => {
                    if !(unsafe { (*idx).size }).is_null() {
                        __state = 73;
                    } else {
                        __state = 72;
                    }
                }
                72 => {
                    if !(unsafe { (*idx).n_block_count }).is_null() {
                        __state = 75;
                    } else {
                        __state = 74;
                    }
                }
                73 => {
                    unsafe { free(unsafe { (*idx).size } as *mut ()) };
                    __state = 72;
                }
                74 => {
                    if !(unsafe { (*idx).n_block_start }).is_null() {
                        __state = 77;
                    } else {
                        __state = 76;
                    }
                }
                75 => {
                    unsafe { free(unsafe { (*idx).n_block_count } as *mut ()) };
                    __state = 74;
                }
                76 => {
                    if !(unsafe { (*idx).n_block_sizes }).is_null() {
                        __state = 84;
                    } else {
                        __state = 83;
                    }
                }
                77 => {
                    i = 0 as u32;
                    __state = 79;
                }
                78 => {
                    unsafe {
                        free(unsafe { *unsafe { (*idx).n_block_start.add(i as usize) } } as *mut ())
                    };
                    __state = 76;
                }
                79 => {
                    if i < unsafe { (*unsafe { (*tb).hdr }).n_chroms } {
                        __state = 80;
                    } else {
                        __state = 78;
                    }
                }
                80 => {
                    if !(unsafe { *unsafe { (*idx).n_block_start.add(i as usize) } }).is_null() {
                        __state = 82;
                    } else {
                        __state = 81;
                    }
                }
                81 => {
                    {
                        let __old = i;
                        i = i.wrapping_add(1);
                        __old
                    };
                    __state = 79;
                }
                82 => {
                    unsafe {
                        free(unsafe { *unsafe { (*idx).n_block_start.add(i as usize) } } as *mut ())
                    };
                    __state = 81;
                }
                83 => {
                    if !(unsafe { (*idx).mask_block_count }).is_null() {
                        __state = 91;
                    } else {
                        __state = 90;
                    }
                }
                84 => {
                    i = 0 as u32;
                    __state = 86;
                }
                85 => {
                    unsafe {
                        free(unsafe { *unsafe { (*idx).n_block_sizes.add(i as usize) } } as *mut ())
                    };
                    __state = 83;
                }
                86 => {
                    if i < unsafe { (*unsafe { (*tb).hdr }).n_chroms } {
                        __state = 87;
                    } else {
                        __state = 85;
                    }
                }
                87 => {
                    if !(unsafe { *unsafe { (*idx).n_block_sizes.add(i as usize) } }).is_null() {
                        __state = 89;
                    } else {
                        __state = 88;
                    }
                }
                88 => {
                    {
                        let __old = i;
                        i = i.wrapping_add(1);
                        __old
                    };
                    __state = 86;
                }
                89 => {
                    unsafe {
                        free(unsafe { *unsafe { (*idx).n_block_sizes.add(i as usize) } } as *mut ())
                    };
                    __state = 88;
                }
                90 => {
                    if !(unsafe { (*idx).mask_block_start }).is_null() {
                        __state = 93;
                    } else {
                        __state = 92;
                    }
                }
                91 => {
                    unsafe { free(unsafe { (*idx).mask_block_count } as *mut ()) };
                    __state = 90;
                }
                92 => {
                    if !(unsafe { (*idx).mask_block_sizes }).is_null() {
                        __state = 100;
                    } else {
                        __state = 99;
                    }
                }
                93 => {
                    i = 0 as u32;
                    __state = 95;
                }
                94 => {
                    unsafe {
                        free(
                            unsafe { *unsafe { (*idx).mask_block_start.add(i as usize) } }
                                as *mut (),
                        )
                    };
                    __state = 92;
                }
                95 => {
                    if i < unsafe { (*unsafe { (*tb).hdr }).n_chroms } {
                        __state = 96;
                    } else {
                        __state = 94;
                    }
                }
                96 => {
                    if !(unsafe { *unsafe { (*idx).mask_block_start.add(i as usize) } }).is_null() {
                        __state = 98;
                    } else {
                        __state = 97;
                    }
                }
                97 => {
                    {
                        let __old = i;
                        i = i.wrapping_add(1);
                        __old
                    };
                    __state = 95;
                }
                98 => {
                    unsafe {
                        free(
                            unsafe { *unsafe { (*idx).mask_block_start.add(i as usize) } }
                                as *mut (),
                        )
                    };
                    __state = 97;
                }
                99 => {
                    if !(unsafe { (*idx).offset }).is_null() {
                        __state = 107;
                    } else {
                        __state = 106;
                    }
                }
                100 => {
                    i = 0 as u32;
                    __state = 102;
                }
                101 => {
                    unsafe {
                        free(
                            unsafe { *unsafe { (*idx).mask_block_sizes.add(i as usize) } }
                                as *mut (),
                        )
                    };
                    __state = 99;
                }
                102 => {
                    if i < unsafe { (*unsafe { (*tb).hdr }).n_chroms } {
                        __state = 103;
                    } else {
                        __state = 101;
                    }
                }
                103 => {
                    if !(unsafe { *unsafe { (*idx).mask_block_sizes.add(i as usize) } }).is_null() {
                        __state = 105;
                    } else {
                        __state = 104;
                    }
                }
                104 => {
                    {
                        let __old = i;
                        i = i.wrapping_add(1);
                        __old
                    };
                    __state = 102;
                }
                105 => {
                    unsafe {
                        free(
                            unsafe { *unsafe { (*idx).mask_block_sizes.add(i as usize) } }
                                as *mut (),
                        )
                    };
                    __state = 104;
                }
                106 => {
                    unsafe { free(idx as *mut ()) };
                    __state = 1;
                }
                107 => {
                    unsafe { free(unsafe { (*idx).offset } as *mut ()) };
                    __state = 106;
                }
                _ => {}
            }
        }
    }
}

/// Opens a local 2bit file
///
/// # Arguments
///
/// * `fname` - The name of the 2bit file.
/// * `storeMasked` - Whether soft-masking information should be stored. If this is 1 then soft-masking information will be stored and the `twobitSequence()` function will return lower case letters in soft-masked regions. Note that this has a considerable performance and memory impact.
///
/// # Returns
///
/// A pointer to a TwoBit object.
#[allow(unused_doc_comments)]
pub(crate) extern "C" fn twobit_open(fname: *const i8, store_masked_1: i32) -> *mut TwoBit {
    let tb: *mut TwoBit =
        unsafe { calloc(1 as u64, core::mem::size_of::<TwoBit>() as u64) } as *mut TwoBit;
    '__b10: loop {
        '__c10: loop {
            let mut fd: i32 = 0;
            let mut fs: Stat = Stat::default();
            if (tb).is_null() as i32 != 0 {
                return 0 as *mut () as *mut TwoBit;
            }
            unsafe {
                (*tb).fp =
                    unsafe { fopen(fname as *const i8, c"rb".as_ptr() as *mut i8 as *const i8) }
            };
            if (unsafe { (*tb).fp }).is_null() as i32 != 0 {
                break '__b10;
            }

            ///Try to memory map the whole thing, since these aren't terribly large
            ///Since we might be multithreading this in python, use shared memory
            (fd = unsafe { fileno(unsafe { (*tb).fp }) });
            if unsafe { fstat(fd, &mut fs) } == 0 {
                unsafe { (*tb).sz = fs.st_size as u64 };
                unsafe {
                    (*tb).data =
                        unsafe { mmap(0 as *mut (), fs.st_size as u64, 1, 1, fd, 0 as OffT) }
                };
                if !(unsafe { (*tb).data }).is_null() {
                    if unsafe { madvise(unsafe { (*tb).data }, fs.st_size as u64, 1) } != 0 {
                        unsafe { munmap(unsafe { (*tb).data }, fs.st_size as u64) };
                        unsafe { (*tb).data = 0 as *mut () };
                    }
                }
            }

            ///Attempt to read in the fixed header
            twobit_hdr_read(tb);
            if (unsafe { (*tb).hdr }).is_null() as i32 != 0 {
                break '__b10;
            }

            ///Read in the chromosome list
            twobit_chrom_list_read(tb);
            if (unsafe { (*tb).cl }).is_null() as i32 != 0 {
                break '__b10;
            }

            ///Read in the mask index
            twobit_index_read(tb, store_masked_1);
            if (unsafe { (*tb).idx }).is_null() as i32 != 0 {
                break '__b10;
            }
            return tb;
            break '__c10;
        }
        if !(false) {
            break '__b10;
        }
    }

    ///Try to memory map the whole thing, since these aren't terribly large
    ///Since we might be multithreading this in python, use shared memory
    ///Attempt to read in the fixed header
    ///Read in the chromosome list
    ///Read in the mask index
    twobit_close(tb);
    return 0 as *mut () as *mut TwoBit;
}

/// Returns the length of a given chromosome.
///
/// # Arguments
///
/// * `tb` - A pointer to a TwoBit object.
/// * `chrom` - The chromosome name.
///
/// # Returns
///
/// The chromosome length as a uint32_t. Note that if the chromosome/contig isn't present in the file that 0 is returned.
pub(crate) extern "C" fn twobit_chrom_len(tb: &TwoBit, chrom: *const i8) -> u32 {
    let mut i: u32 = 0 as u32;
    {
        i = 0 as u32;
        '__b11: loop {
            if !(i < unsafe { (*(*tb).hdr).n_chroms }) {
                break '__b11;
            }
            '__c11: loop {
                if unsafe {
                    strcmp(
                        unsafe { *unsafe { (*(*tb).cl).chrom.add(i as usize) } } as *const i8,
                        chrom as *const i8,
                    )
                } == 0
                {
                    return unsafe { *unsafe { (*(*tb).idx).size.add(i as usize) } };
                }
                break '__c11;
            }
            i = i.wrapping_add(1);
        }
    }
    return 0 as u32;
}

///    Given a byte containing 4 bases, return the character representation of the offset'th base
pub(crate) extern "C" fn byte2base(byte: u8, offset: i32) -> i8 {
    let rev: i32 = 3 - offset;
    let mask: u8 = (3 << 2 * rev) as u8;
    let foo: i32 = (mask as i32 & byte as i32) >> 2 * rev;
    let bases: [i8; 4] = [84 as i8, 67 as i8, 65 as i8, 71 as i8];
    return bases[foo as usize];
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn bytes2bases(
    seq: *mut i8,
    byte: *const u8,
    sz: u32,
    mut offset: i32,
) -> () {
    let mut pos: u32 = 0 as u32;
    let mut remainder: u32 = 0 as u32;
    let mut i: u32 = 0 as u32;
    let bases: [i8; 4] = [84 as i8, 67 as i8, 65 as i8, 71 as i8];
    let mut foo: u8 = unsafe { *byte.offset(0 as isize) };
    if offset != 0 {
        while offset < 4 && pos < sz {
            unsafe {
                *seq.add({
                    let __old = pos;
                    pos = pos.wrapping_add(1);
                    __old
                } as usize) = byte2base(foo, {
                    let __old = offset;
                    offset += 1;
                    __old
                })
            };
        }
        if pos >= sz {
            return;
        }
        foo = unsafe {
            *byte.add({
                i = i.wrapping_add(1);
                i
            } as usize)
        };
    }

    /// Deal with everything else, with the possible exception of the last fractional byte
    (remainder = sz.wrapping_sub(pos) % 4 as u32);
    while pos < sz.wrapping_sub(remainder) {
        foo = unsafe {
            *byte.add({
                let __old = i;
                i = i.wrapping_add(1);
                __old
            } as usize)
        };
        unsafe { *seq.add(pos.wrapping_add(3 as u32) as usize) = bases[(foo as i32 & 3) as usize] };
        foo >>= 2;
        unsafe { *seq.add(pos.wrapping_add(2 as u32) as usize) = bases[(foo as i32 & 3) as usize] };
        foo >>= 2;
        unsafe { *seq.add(pos.wrapping_add(1 as u32) as usize) = bases[(foo as i32 & 3) as usize] };
        foo >>= 2;
        unsafe { *seq.add(pos as usize) = bases[(foo as i32 & 3) as usize] };
        foo >>= 2;
        pos = pos.wrapping_add(4 as u32);
    }
    if remainder > 0 as u32 {
        foo = unsafe { *byte.add(i as usize) };
    }
    {
        offset = 0;
        '__b14: loop {
            if !((offset as u32) < remainder) {
                break '__b14;
            }
            '__c14: loop {
                unsafe {
                    *seq.add({
                        let __old = pos;
                        pos = pos.wrapping_add(1);
                        __old
                    } as usize) = byte2base(foo, offset)
                };
                break '__c14;
            }
            offset += 1;
        }
    }
}

///    Replace Ts (or whatever else is being used) with N as appropriate
pub(crate) extern "C" fn n_mask(seq: *mut i8, tb: &TwoBit, tid: u32, start: u32, end: u32) -> () {
    let mut i: u32 = 0 as u32;
    let mut width: u32 = 0 as u32;
    let mut pos: u32 = 0 as u32;
    let mut block_start: u32 = 0 as u32;
    let mut block_end: u32 = 0 as u32;
    {
        i = 0 as u32;
        '__b15: loop {
            if !(i < unsafe { *unsafe { (*(*tb).idx).n_block_count.add(tid as usize) } }) {
                break '__b15;
            }
            '__c15: loop {
                block_start = unsafe {
                    *unsafe {
                        (*unsafe { (*(*tb).idx).n_block_start.add(tid as usize) }).add(i as usize)
                    }
                };
                block_end = block_start.wrapping_add(unsafe {
                    *unsafe {
                        (*unsafe { (*(*tb).idx).n_block_sizes.add(tid as usize) }).add(i as usize)
                    }
                });
                if block_end <= start {
                    break '__c15;
                }
                if block_start >= end {
                    break '__b15;
                }
                if block_start < start {
                    block_end = if block_end < end { block_end } else { end };
                    pos = 0 as u32;
                    width = block_end.wrapping_sub(start);
                } else {
                    block_end = if block_end < end { block_end } else { end };
                    pos = block_start.wrapping_sub(start);
                    width = block_end.wrapping_sub(block_start);
                }
                width = width.wrapping_add(pos);
                {
                    '__b16: loop {
                        if !(pos < width) {
                            break '__b16;
                        }
                        '__c16: loop {
                            unsafe { *seq.add(pos as usize) = 'N' as i32 as i8 };
                            break '__c16;
                        }
                        pos = pos.wrapping_add(1);
                    }
                }
                break '__c15;
            }
            i = i.wrapping_add(1);
        }
    }
}

///    Replace uppercase with lower-case letters, if required
pub(crate) extern "C" fn soft_mask(
    seq: *mut i8,
    tb: &TwoBit,
    tid: u32,
    start: u32,
    end: u32,
) -> () {
    let mut i: u32 = 0 as u32;
    let mut width: u32 = 0 as u32;
    let mut pos: u32 = 0 as u32;
    let mut block_start: u32 = 0 as u32;
    let mut block_end: u32 = 0 as u32;
    if (unsafe { (*(*tb).idx).mask_block_start }).is_null() as i32 != 0 {
        return;
    }
    {
        i = 0 as u32;
        '__b17: loop {
            if !(i < unsafe { *unsafe { (*(*tb).idx).mask_block_count.add(tid as usize) } }) {
                break '__b17;
            }
            '__c17: loop {
                block_start = unsafe {
                    *unsafe {
                        (*unsafe { (*(*tb).idx).mask_block_start.add(tid as usize) })
                            .add(i as usize)
                    }
                };
                block_end = block_start.wrapping_add(unsafe {
                    *unsafe {
                        (*unsafe { (*(*tb).idx).mask_block_sizes.add(tid as usize) })
                            .add(i as usize)
                    }
                });
                if block_end <= start {
                    break '__c17;
                }
                if block_start >= end {
                    break '__b17;
                }
                if block_start < start {
                    block_end = if block_end < end { block_end } else { end };
                    pos = 0 as u32;
                    width = block_end.wrapping_sub(start);
                } else {
                    block_end = if block_end < end { block_end } else { end };
                    pos = block_start.wrapping_sub(start);
                    width = block_end.wrapping_sub(block_start);
                }
                width = width.wrapping_add(pos);
                {
                    '__b18: loop {
                        if !(pos < width) {
                            break '__b18;
                        }
                        '__c18: loop {
                            if unsafe { *seq.add(pos as usize) } as i32 != 'N' as i32 {
                                unsafe {
                                    *seq.add(pos as usize) =
                                        unsafe { tolower(unsafe { *seq.add(pos as usize) } as i32) }
                                            as i8
                                };
                            }
                            break '__c18;
                        }
                        pos = pos.wrapping_add(1);
                    }
                }
                break '__c17;
            }
            i = i.wrapping_add(1);
        }
    }
}

///    This is the worker function for twobitSequence, which mostly does error checking
#[allow(unused_doc_comments)]
pub(crate) extern "C" fn construct_sequence(
    tb: *mut TwoBit,
    tid: u32,
    start: u32,
    end: u32,
) -> *mut i8 {
    let sz: u32 = end.wrapping_sub(start).wrapping_add(1 as u32);
    let seq: *mut i8 =
        unsafe { malloc((sz as u64).wrapping_mul(core::mem::size_of::<i8>() as u64)) } as *mut i8;
    let mut bytes: *mut u8 = 0 as *mut () as *mut u8;
    '__b19: loop {
        '__c19: loop {
            let mut block_start: u32 = 0 as u32;
            let mut block_end: u32 = 0 as u32;
            let mut offset: i32 = 0;
            if (seq).is_null() as i32 != 0 {
                return 0 as *mut () as *mut i8;
            }

            ///There are 4 bases/byte
            (block_start = start / 4 as u32);
            offset = (start % 4 as u32) as i32;
            block_end =
                (end / 4 as u32).wrapping_add(if end % 4 as u32 != 0 { 1 } else { 0 } as u32);
            bytes = unsafe { malloc(block_end.wrapping_sub(block_start) as u64) } as *mut u8;
            if (bytes).is_null() as i32 != 0 {
                break '__b19;
            }
            if twobit_seek(unsafe { &mut *tb }, unsafe {
                (*unsafe { (*unsafe { (*tb).idx }).offset.add(tid as usize) })
                    .wrapping_add(block_start as u64)
            }) != 0
            {
                break '__b19;
            }
            if twobit_read(
                unsafe {
                    let __p = bytes as *mut u8 as *mut u8;
                    if __p.is_null() {
                        &mut []
                    } else {
                        core::slice::from_raw_parts_mut(
                            __p,
                            block_end.wrapping_sub(block_start) as usize,
                        )
                    }
                },
                1 as u64,
                unsafe { &mut *tb },
            ) != 1 as u64
            {
                break '__b19;
            }
            bytes2bases(seq, bytes as *const u8, sz.wrapping_sub(1 as u32), offset);
            unsafe { free(bytes as *mut ()) };

            ///Null terminate the output
            unsafe {
                *seq.add(sz.wrapping_sub(1 as u32) as usize) = '\u{0}' as i32 as i8
            };

            ///N-mask everything
            n_mask(seq, unsafe { &*tb }, tid, start, end);

            ///Soft-mask if requested
            soft_mask(seq, unsafe { &*tb }, tid, start, end);
            return seq;
            break '__c19;
        }
        if !(false) {
            break '__b19;
        }
    }
    if !(seq).is_null() {
        unsafe { free(seq as *mut ()) };
    }
    if !(bytes).is_null() {
        unsafe { free(bytes as *mut ()) };
    }
    return 0 as *mut () as *mut i8;
}

/// Returns the sequence of a chromosome/contig or range of it.
///
/// # Arguments
///
/// * `tb` - A pointer to a TwoBit object.
/// * `chrom` - The chromosome name.
/// * `start` - The starting position in 0-based coordinates.
/// * `end` - The end position in 1-based coordinates.
///
/// # Returns
///
/// The sequence or NULL on error. If both start and end are 0 then the sequence for the entire chromosome/contig is returned.
pub(crate) extern "C" fn twobit_sequence(
    tb: *mut TwoBit,
    chrom: *const i8,
    start: u32,
    mut end: u32,
) -> *mut i8 {
    let mut i: u32 = 0 as u32;
    let mut tid: u32 = 0 as u32;
    {
        i = 0 as u32;
        '__b20: loop {
            if !(i < unsafe { (*unsafe { (*tb).hdr }).n_chroms }) {
                break '__b20;
            }
            '__c20: loop {
                if unsafe {
                    strcmp(
                        unsafe { *unsafe { (*unsafe { (*tb).cl }).chrom.add(i as usize) } }
                            as *const i8,
                        chrom as *const i8,
                    )
                } == 0
                {
                    tid = i;
                    break '__b20;
                }
                break '__c20;
            }
            i = i.wrapping_add(1);
        }
    }
    if tid == 0 as u32
        && unsafe {
            strcmp(
                unsafe { *unsafe { (*unsafe { (*tb).cl }).chrom.add(i as usize) } } as *const i8,
                chrom as *const i8,
            )
        } != 0
    {
        return 0 as *mut () as *mut i8;
    }
    if start == end && end == 0 as u32 {
        end = unsafe { *unsafe { (*unsafe { (*tb).idx }).size.add(tid as usize) } };
    }
    if end > unsafe { *unsafe { (*unsafe { (*tb).idx }).size.add(tid as usize) } } {
        return 0 as *mut () as *mut i8;
    }
    if start >= end {
        return 0 as *mut () as *mut i8;
    }
    return construct_sequence(tb, tid, start, end);
}

pub(crate) extern "C" fn get_byte_mask_from_offset(offset: i32) -> u8 {
    '__s21: {
        match offset {
            0 => {
                return 15 as u8;
            }
            1 => {
                return 7 as u8;
            }
            2 => {
                return 3 as u8;
            }
            _ => {}
        }
    }
    return 1 as u8;
}

///    Given a tid and a position, set the various mask variables to an appropriate block of Ns.
///
///If maskIdx is not -1, these are set to the first overlapping block (or maskIdx is set to the number of N blocks).
///If maskIdx is not -1 then it's incremented and maskStart/maskEnd set appropriately.
///
///    If the returned interval doesn't overlap the start/end range, then both values will be -1.
pub(crate) extern "C" fn get_mask(
    tb: &TwoBit,
    tid: u32,
    start: u32,
    end: u32,
    mask_idx_1: &mut u32,
    mask_start_1: &mut u32,
    mask_end_1: &mut u32,
) -> () {
    if *mask_idx_1 == -1i32 as u32 {
        {
            *mask_idx_1 = 0 as u32;
            '__b22: loop {
                if !(*mask_idx_1
                    < unsafe { *unsafe { (*(*tb).idx).n_block_count.add(tid as usize) } })
                {
                    break '__b22;
                }
                '__c22: loop {
                    *mask_start_1 = unsafe {
                        *unsafe {
                            (*unsafe { (*(*tb).idx).n_block_start.add(tid as usize) })
                                .add(*mask_idx_1 as usize)
                        }
                    };
                    *mask_end_1 = (*mask_start_1).wrapping_add(unsafe {
                        *unsafe {
                            (*unsafe { (*(*tb).idx).n_block_sizes.add(tid as usize) })
                                .add(*mask_idx_1 as usize)
                        }
                    });
                    if *mask_end_1 < start {
                        break '__c22;
                    }
                    if *mask_end_1 >= start {
                        break '__b22;
                    }
                    break '__c22;
                }
                *mask_idx_1 = (*mask_idx_1).wrapping_add(1);
            }
        }
    } else if *mask_idx_1 >= unsafe { *unsafe { (*(*tb).idx).n_block_count.add(tid as usize) } } {
        *mask_start_1 = -1i32 as u32;
        *mask_end_1 = -1i32 as u32;
    } else {
        *mask_idx_1 = (*mask_idx_1).wrapping_add(1 as u32);
        if *mask_idx_1 >= unsafe { *unsafe { (*(*tb).idx).n_block_count.add(tid as usize) } } {
            *mask_start_1 = -1i32 as u32;
            *mask_end_1 = -1i32 as u32;
        } else {
            *mask_start_1 = unsafe {
                *unsafe {
                    (*unsafe { (*(*tb).idx).n_block_start.add(tid as usize) })
                        .add(*mask_idx_1 as usize)
                }
            };
            *mask_end_1 = (*mask_start_1).wrapping_add(unsafe {
                *unsafe {
                    (*unsafe { (*(*tb).idx).n_block_sizes.add(tid as usize) })
                        .add(*mask_idx_1 as usize)
                }
            });
        }
    }
    if *mask_idx_1 >= unsafe { *unsafe { (*(*tb).idx).n_block_count.add(tid as usize) } }
        || *mask_start_1 >= end
    {
        *mask_start_1 = -1i32 as u32;
        *mask_end_1 = -1i32 as u32;
    }
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn twobit_bases_worker(
    tb: *mut TwoBit,
    tid: u32,
    mut start: u32,
    end: u32,
    fraction: i32,
) -> *mut () {
    let mut out: *mut () = core::ptr::null_mut();
    let mut tmp: [u32; 4] = [0 as u32, 0 as u32, 0 as u32, 0 as u32];
    let mut len: u32 = 0 as u32;
    let mut i: u32 = 0 as u32;
    let mut j: u32 = 0 as u32;
    let mut seq_len: u32 = 0 as u32;
    let mut block_start: u32 = 0 as u32;
    let mut block_end: u32 = 0 as u32;
    let mut mask_idx: u32 = 0 as u32;
    let mut mask_start: u32 = 0 as u32;
    let mut mask_end: u32 = 0 as u32;
    let mut foo: u32 = 0 as u32;
    let mut bytes: *mut u8 = core::ptr::null_mut();
    let mut mask: u8 = 0 as u8;
    let mut offset: u8 = 0 as u8;
    let mut __state: i32 = 0;
    loop {
        if __state == 1 {
            break;
        }
        '__s24: {
            match __state {
                0 => {
                    __state = 3;
                }
                2 => {
                    if !(out).is_null() {
                        __state = 89;
                    } else {
                        __state = 88;
                    }
                }
                3 => {
                    len = end.wrapping_sub(start).wrapping_add(start % 4 as u32);
                    i = 0 as u32;
                    j = 0 as u32;
                    __state = 4;
                }
                4 => {
                    seq_len = end.wrapping_sub(start);
                    __state = 5;
                }
                5 => {
                    mask_idx = -1i32 as u32;
                    __state = 6;
                }
                6 => {
                    bytes = 0 as *mut () as *mut u8;
                    mask = 0 as u8;
                    __state = 7;
                }
                7 => {
                    if fraction != 0 {
                        __state = 9;
                    } else {
                        __state = 10;
                    }
                }
                8 => {
                    if (out).is_null() as i32 != 0 {
                        __state = 12;
                    } else {
                        __state = 11;
                    }
                }
                9 => {
                    out = unsafe {
                        malloc((4 as u64).wrapping_mul(core::mem::size_of::<f64>() as u64))
                    };
                    __state = 8;
                }
                10 => {
                    out = unsafe {
                        malloc((4 as u64).wrapping_mul(core::mem::size_of::<u32>() as u64))
                    };
                    __state = 8;
                }
                11 => {
                    block_start = start / 4 as u32;
                    __state = 13;
                }
                12 => {
                    return 0 as *mut ();
                }
                13 => {
                    offset = (start % 4 as u32) as u8;
                    __state = 14;
                }
                14 => {
                    block_end =
                        (end / 4 as u32)
                            .wrapping_add(if end % 4 as u32 != 0 { 1 } else { 0 } as u32);
                    __state = 15;
                }
                15 => {
                    bytes =
                        unsafe { malloc(block_end.wrapping_sub(block_start) as u64) } as *mut u8;
                    __state = 16;
                }
                16 => {
                    if (bytes).is_null() as i32 != 0 {
                        __state = 18;
                    } else {
                        __state = 17;
                    }
                }
                17 => {
                    mask = get_byte_mask_from_offset(offset as i32);
                    __state = 19;
                }
                18 => {
                    __state = 2;
                }
                19 => {
                    start = (4 as u32).wrapping_mul(block_start);
                    __state = 20;
                }
                20 => {
                    offset = 0 as u8;
                    __state = 21;
                }
                21 => {
                    if twobit_seek(unsafe { &mut *tb }, unsafe {
                        (*unsafe { (*unsafe { (*tb).idx }).offset.add(tid as usize) })
                            .wrapping_add(block_start as u64)
                    }) != 0
                    {
                        __state = 23;
                    } else {
                        __state = 22;
                    }
                }
                22 => {
                    if twobit_read(
                        unsafe {
                            let __p = bytes as *mut u8 as *mut u8;
                            if __p.is_null() {
                                &mut []
                            } else {
                                core::slice::from_raw_parts_mut(
                                    __p,
                                    block_end.wrapping_sub(block_start) as usize,
                                )
                            }
                        },
                        1 as u64,
                        unsafe { &mut *tb },
                    ) != 1 as u64
                    {
                        __state = 25;
                    } else {
                        __state = 24;
                    }
                }
                23 => {
                    __state = 2;
                }
                24 => {
                    get_mask(
                        unsafe { &*tb },
                        tid,
                        start,
                        end,
                        &mut mask_idx,
                        &mut mask_start,
                        &mut mask_end,
                    );
                    __state = 26;
                }
                25 => {
                    __state = 2;
                }
                26 => {
                    if i < len {
                        __state = 28;
                    } else {
                        __state = 27;
                    }
                }
                27 => {
                    unsafe { free(bytes as *mut ()) };
                    __state = 77;
                }
                28 => {
                    if mask_idx != -1i32 as u32
                        && start.wrapping_add(i).wrapping_add(4 as u32) >= mask_start
                    {
                        __state = 30;
                    } else {
                        __state = 29;
                    }
                }
                29 => {
                    if i.wrapping_add(4 as u32) >= len {
                        __state = 53;
                    } else {
                        __state = 52;
                    }
                }
                30 => {
                    if start.wrapping_add(i) >= mask_start
                        || start
                            .wrapping_add(i)
                            .wrapping_add(4 as u32)
                            .wrapping_sub(offset as u32)
                            > mask_start
                    {
                        __state = 31;
                    } else {
                        __state = 29;
                    }
                }
                31 => {
                    if start.wrapping_add(i) >= mask_start
                        && start
                            .wrapping_add(i)
                            .wrapping_add(4 as u32)
                            .wrapping_sub(offset as u32)
                            < mask_end
                    {
                        __state = 33;
                    } else {
                        __state = 32;
                    }
                }
                32 => {
                    foo = (4 as u32)
                        .wrapping_mul(j)
                        .wrapping_add((4 as u32).wrapping_mul(block_start));
                    __state = 41;
                }
                33 => {
                    i = mask_end.wrapping_sub(start);
                    __state = 34;
                }
                34 => {
                    get_mask(
                        unsafe { &*tb },
                        tid,
                        i,
                        end,
                        &mut mask_idx,
                        &mut mask_start,
                        &mut mask_end,
                    );
                    __state = 35;
                }
                35 => {
                    offset = (start.wrapping_add(i) % 4 as u32) as u8;
                    __state = 36;
                }
                36 => {
                    j = i / 4 as u32;
                    __state = 37;
                }
                37 => {
                    mask = get_byte_mask_from_offset(offset as i32);
                    __state = 38;
                }
                38 => {
                    i = (4 as u32).wrapping_mul(j);
                    __state = 39;
                }
                39 => {
                    offset = 0 as u8;
                    __state = 40;
                }
                40 => {
                    __state = 26;
                }
                41 => {
                    if mask as i32 & 1 != 0
                        && (foo.wrapping_add(3 as u32) >= mask_start
                            && foo.wrapping_add(3 as u32) < mask_end)
                    {
                        __state = 43;
                    } else {
                        __state = 42;
                    }
                }
                42 => {
                    if mask as i32 & 2 != 0
                        && (foo.wrapping_add(2 as u32) >= mask_start
                            && foo.wrapping_add(2 as u32) < mask_end)
                    {
                        __state = 45;
                    } else {
                        __state = 44;
                    }
                }
                43 => {
                    mask = mask.wrapping_sub(1 as u8);
                    __state = 42;
                }
                44 => {
                    if mask as i32 & 4 != 0
                        && (foo.wrapping_add(1 as u32) >= mask_start
                            && foo.wrapping_add(1 as u32) < mask_end)
                    {
                        __state = 47;
                    } else {
                        __state = 46;
                    }
                }
                45 => {
                    mask = mask.wrapping_sub(2 as u8);
                    __state = 44;
                }
                46 => {
                    if mask as i32 & 8 != 0 && (foo >= mask_start && foo < mask_end) {
                        __state = 49;
                    } else {
                        __state = 48;
                    }
                }
                47 => {
                    mask = mask.wrapping_sub(4 as u8);
                    __state = 46;
                }
                48 => {
                    if foo.wrapping_add(4 as u32) > mask_end {
                        __state = 50;
                    } else {
                        __state = 29;
                    }
                }
                49 => {
                    mask = mask.wrapping_sub(8 as u8);
                    __state = 48;
                }
                50 => {
                    get_mask(
                        unsafe { &*tb },
                        tid,
                        i,
                        end,
                        &mut mask_idx,
                        &mut mask_start,
                        &mut mask_end,
                    );
                    __state = 51;
                }
                51 => {
                    __state = 26;
                }
                52 => {
                    foo = unsafe {
                        *bytes.add({
                            let __old = j;
                            j = j.wrapping_add(1);
                            __old
                        } as usize)
                    } as u32;
                    __state = 61;
                }
                53 => {
                    if mask as i32 & 1 != 0 && i.wrapping_add(3 as u32) >= len {
                        __state = 55;
                    } else {
                        __state = 54;
                    }
                }
                54 => {
                    if mask as i32 & 2 != 0 && i.wrapping_add(2 as u32) >= len {
                        __state = 57;
                    } else {
                        __state = 56;
                    }
                }
                55 => {
                    mask = mask.wrapping_sub(1 as u8);
                    __state = 54;
                }
                56 => {
                    if mask as i32 & 4 != 0 && i.wrapping_add(1 as u32) >= len {
                        __state = 59;
                    } else {
                        __state = 58;
                    }
                }
                57 => {
                    mask = mask.wrapping_sub(2 as u8);
                    __state = 56;
                }
                58 => {
                    if mask as i32 & 8 != 0 && i >= len {
                        __state = 60;
                    } else {
                        __state = 52;
                    }
                }
                59 => {
                    mask = mask.wrapping_sub(4 as u8);
                    __state = 58;
                }
                60 => {
                    mask = mask.wrapping_sub(8 as u8);
                    __state = 52;
                }
                61 => {
                    if mask as i32 & 1 != 0 {
                        __state = 63;
                    } else {
                        __state = 62;
                    }
                }
                62 => {
                    foo >>= 2 as u32;
                    __state = 64;
                }
                63 => {
                    {
                        let __p = &mut tmp[(foo & 3 as u32) as usize];
                        let __t = *__p;
                        *__p = (*__p).wrapping_add(1);
                        __t
                    };
                    __state = 62;
                }
                64 => {
                    mask >>= 1;
                    __state = 65;
                }
                65 => {
                    if mask as i32 & 1 != 0 {
                        __state = 67;
                    } else {
                        __state = 66;
                    }
                }
                66 => {
                    foo >>= 2 as u32;
                    __state = 68;
                }
                67 => {
                    {
                        let __p = &mut tmp[(foo & 3 as u32) as usize];
                        let __t = *__p;
                        *__p = (*__p).wrapping_add(1);
                        __t
                    };
                    __state = 66;
                }
                68 => {
                    mask >>= 1;
                    __state = 69;
                }
                69 => {
                    if mask as i32 & 1 != 0 {
                        __state = 71;
                    } else {
                        __state = 70;
                    }
                }
                70 => {
                    foo >>= 2 as u32;
                    __state = 72;
                }
                71 => {
                    {
                        let __p = &mut tmp[(foo & 3 as u32) as usize];
                        let __t = *__p;
                        *__p = (*__p).wrapping_add(1);
                        __t
                    };
                    __state = 70;
                }
                72 => {
                    mask >>= 1;
                    __state = 73;
                }
                73 => {
                    if mask as i32 & 1 != 0 {
                        __state = 75;
                    } else {
                        __state = 74;
                    }
                }
                74 => {
                    i = i.wrapping_add(4 as u32);
                    __state = 76;
                }
                75 => {
                    {
                        let __p = &mut tmp[(foo & 3 as u32) as usize];
                        let __t = *__p;
                        *__p = (*__p).wrapping_add(1);
                        __t
                    };
                    __state = 74;
                }
                76 => {
                    mask = 15 as u8;
                    __state = 26;
                }
                77 => {
                    if fraction != 0 {
                        __state = 79;
                    } else {
                        __state = 80;
                    }
                }
                78 => {
                    return out;
                }
                79 => {
                    unsafe {
                        *(out as *mut f64).offset(0 as isize) =
                            tmp[2 as usize] as f64 / seq_len as f64
                    };
                    __state = 81;
                }
                80 => {
                    unsafe { *(out as *mut u32).offset(0 as isize) = tmp[2 as usize] };
                    __state = 84;
                }
                81 => {
                    unsafe {
                        *(out as *mut f64).offset(1 as isize) =
                            tmp[1 as usize] as f64 / seq_len as f64
                    };
                    __state = 82;
                }
                82 => {
                    unsafe {
                        *(out as *mut f64).offset(2 as isize) =
                            tmp[0 as usize] as f64 / seq_len as f64
                    };
                    __state = 83;
                }
                83 => {
                    unsafe {
                        *(out as *mut f64).offset(3 as isize) =
                            tmp[3 as usize] as f64 / seq_len as f64
                    };
                    __state = 78;
                }
                84 => {
                    unsafe { *(out as *mut u32).offset(1 as isize) = tmp[1 as usize] };
                    __state = 85;
                }
                85 => {
                    unsafe { *(out as *mut u32).offset(2 as isize) = tmp[0 as usize] };
                    __state = 86;
                }
                86 => {
                    unsafe { *(out as *mut u32).offset(3 as isize) = tmp[3 as usize] };
                    __state = 78;
                }
                87 => {
                    __state = 2;
                }
                88 => {
                    if !(bytes).is_null() {
                        __state = 91;
                    } else {
                        __state = 90;
                    }
                }
                89 => {
                    unsafe { free(out) };
                    __state = 88;
                }
                90 => {
                    return 0 as *mut ();
                }
                91 => {
                    unsafe { free(bytes as *mut ()) };
                    __state = 90;
                }
                _ => {}
            }
        }
    }

    ///There are 4 bases/byte
    ///Set the initial mask, reset start/offset so we always deal with full bytes
    ///Get the index/start/end of the next N-mask block
    /// Check if we need to jump
    ///Jump iff the whole byte is inside an N block
    ///iff we're fully in an N block then jump
    ///Now that the mask has been set, reset i to byte offsets
    ///Set the mask, if appropriate
    /// The smallest position in the byte
    ///Ensure that anything after then end is masked
    ///Offset 3
    ///Offset 2
    ///Offset 1
    ///Offset 0
    /// offset 0
    ///out is in TCAG order, since that's how 2bit is stored.
    ///However, for whatever reason I went with ACTG in the first release...
    unreachable!();
}

/// Return the number/fraction of A, C, T, and G in a chromosome/region
///
/// # Arguments
///
/// * `tb` - A pointer to a TwoBit object.
/// * `chrom` - The chromosome name.
/// * `start` - The starting position in 0-based coordinates.
/// * `end` - The end position in 1-based coordinates.
/// * `fraction` - Whether to return the values as fractions (1) or integers (0).
///
/// # Returns
///
/// If fraction is not 0, then 4 `double`s with the fraction of bases as A, C, T and G, respectively. If fraction is 1, integer counts are returned as 4 `uint32_t`s in the aforementioned order.
pub(crate) extern "C" fn twobit_bases(
    tb: *mut TwoBit,
    chrom: *const i8,
    start: u32,
    mut end: u32,
    fraction: i32,
) -> *mut () {
    let mut tid: u32 = 0 as u32;
    let mut i: u32 = 0 as u32;
    {
        i = 0 as u32;
        '__b25: loop {
            if !(i < unsafe { (*unsafe { (*tb).hdr }).n_chroms }) {
                break '__b25;
            }
            '__c25: loop {
                if unsafe {
                    strcmp(
                        unsafe { *unsafe { (*unsafe { (*tb).cl }).chrom.add(i as usize) } }
                            as *const i8,
                        chrom as *const i8,
                    )
                } == 0
                {
                    tid = i;
                    break '__b25;
                }
                break '__c25;
            }
            i = i.wrapping_add(1);
        }
    }
    if tid == 0 as u32
        && unsafe {
            strcmp(
                unsafe { *unsafe { (*unsafe { (*tb).cl }).chrom.add(i as usize) } } as *const i8,
                chrom as *const i8,
            )
        } != 0
    {
        return 0 as *mut ();
    }
    if start == end && end == 0 as u32 {
        end = unsafe { *unsafe { (*unsafe { (*tb).idx }).size.add(tid as usize) } };
    }
    if end > unsafe { *unsafe { (*unsafe { (*tb).idx }).size.add(tid as usize) } } {
        return 0 as *mut ();
    }
    if start >= end {
        return 0 as *mut ();
    }
    return twobit_bases_worker(tb, tid, start, end, fraction);
}
