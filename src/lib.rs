#![allow(unused_imports, dead_code)]

mod _2bit;
mod _2bit_h;

pub(crate) type DarwinSizeT = u64;

pub(crate) type Int64T = i64;

pub(crate) type DarwinOffT = Int64T;

pub(crate) type OffT = DarwinOffT;

pub(crate) type Int32T = i32;

pub(crate) type DarwinDevT = Int32T;

pub(crate) type DevT = DarwinDevT;

pub(crate) type Uint16T = u16;

pub(crate) type DarwinModeT = Uint16T;

pub(crate) type ModeT = DarwinModeT;

pub(crate) type NlinkT = Uint16T;

pub(crate) type Uint64T = u64;

pub(crate) type DarwinIno64T = Uint64T;

pub(crate) type Uint32T = u32;

pub(crate) type DarwinUidT = Uint32T;

pub(crate) type UidT = DarwinUidT;

pub(crate) type DarwinGidT = Uint32T;

pub(crate) type GidT = DarwinGidT;

pub(crate) type DarwinBlkcntT = Int64T;

pub(crate) type BlkcntT = DarwinBlkcntT;

pub(crate) type DarwinBlksizeT = Int32T;

pub(crate) type BlksizeT = DarwinBlksizeT;

pub(crate) type DarwinTimeT = i64;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub(crate) struct Timespec {
    pub(crate) tv_sec: i64,
    pub(crate) tv_nsec: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub(crate) struct Stat {
    pub(crate) st_dev: i32,
    pub(crate) st_mode: u16,
    pub(crate) st_nlink: u16,
    pub(crate) st_ino: u64,
    pub(crate) st_uid: u32,
    pub(crate) st_gid: u32,
    pub(crate) st_rdev: i32,
    pub(crate) st_atimespec: Timespec,
    pub(crate) st_mtimespec: Timespec,
    pub(crate) st_ctimespec: Timespec,
    pub(crate) st_birthtimespec: Timespec,
    pub(crate) st_size: i64,
    pub(crate) st_blocks: i64,
    pub(crate) st_blksize: i32,
    pub(crate) st_flags: u32,
    pub(crate) st_gen: u32,
    pub(crate) st_lspare: i32,
    pub(crate) st_qspare: [i64; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub(crate) struct SFILE {
    pub(crate) _opaque: [u8; 0],
}

pub(crate) type FILE = SFILE;

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32) -> bool;
    fn free(_: *mut ()) -> ();
    fn fclose(_: *mut FILE) -> i32;
    fn munmap(_: *mut (), _: u64) -> i32;
    fn __builtin_object_size(_: *const (), _: i32) -> u64;
    fn __builtin___memcpy_chk(_: *mut (), _: *const (), _: u64, _: u64) -> *mut ();
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE) -> u64;
    fn calloc(__count: u64, __size: u64) -> *mut ();
    fn fprintf(_: *mut FILE, _: *const i8, ...) -> i32;
    fn malloc(__size: u64) -> *mut ();
    fn fseek(_: *mut FILE, _: i64, _: i32) -> i32;
    fn ftell(_: *mut FILE) -> i64;
    fn fopen(__filename: *const i8, __mode: *const i8) -> *mut FILE;
    fn fileno(_: *mut FILE) -> i32;
    fn fstat(_: i32, _: *mut Stat) -> i32;
    fn mmap(_: *mut (), _: u64, _: i32, _: i32, _: i32, _: OffT) -> *mut ();
    fn madvise(_: *mut (), _: u64, _: i32) -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn tolower(_c: i32) -> i32;
    fn __builtin_unreachable() -> ();
    static mut __stderrp: *mut FILE;
}
