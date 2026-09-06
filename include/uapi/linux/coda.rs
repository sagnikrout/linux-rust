//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/coda.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


//
// BLURB lgpl
//
// Based on cfs.h from Mach, but revamped for increased simplicity.
// Linux modifications by
// Peter Braam, Aug 1996
//
// Catch new _KERNEL defn for NetBSD and DJGPP/__CYGWIN32__

pub const CODA_MAXSYMLINKS: c_int = 10;

pub type u_long = c_ulong;
pub type u_int = c_uint;
pub type u_short = c_ushort;
pub type ino_t = u_long;
pub type dev_t = u_long;
pub type caddr_t = *mut c_void;

pub type u_quad_t = unsigned __int64;

pub type u_quad_t = c_ulonglong;

// Macro flag: #define inline

pub type u_quad_t = c_ulonglong;

pub const _UQUAD_T_: c_int = 1;
pub type u_quad_t = c_ulonglong;

pub type int8_t = i8;
pub type u_int8_t = c_uchar;
pub type int16_t = c_short;
pub type u_int16_t = c_ushort;
pub type int32_t = c_int;
pub type u_int32_t = c_uint;

//
// Cfs constants
//
pub const CODA_MAXNAMLEN: c_int = 255;
pub const CODA_MAXPATHLEN: c_int = 1024;
pub const CODA_MAXSYMLINK: c_int = 10;
// these are Coda's version of O_RDONLY etc combinations
// to deal with VFS open modes
//
pub const C_O_READ: c_uint = 0x001;
pub const C_O_WRITE: c_uint = 0x002;
pub const C_O_TRUNC: c_uint = 0x010;
pub const C_O_EXCL: c_uint = 0x100;
pub const C_O_CREAT: c_uint = 0x200;
// these are to find mode bits in Venus
pub const C_M_READ: c_int = 00400;
pub const C_M_WRITE: c_int = 00200;
// for access Venus will use

pub const _VENUS_DIRENT_T_: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venus_dirent {
    pub /: *mut *mut u_int32_t d_fileno; / file number of entry,
    pub /: *mut *mut u_int16_t d_reclen; / length of this record,
    pub /: *mut *mut u_int8_t d_type; / file type, see below,
    pub /: *mut *mut u_int8_t d_namlen; / length of string in d_name,
    pub /: *mut *mut char d_name[CODA_MAXNAMLEN + 1];/ name must be no longer than this,
}

//
// File types
//
pub const CDT_UNKNOWN: c_int = 0;
pub const CDT_FIFO: c_int = 1;
pub const CDT_CHR: c_int = 2;
pub const CDT_DIR: c_int = 4;
pub const CDT_BLK: c_int = 6;
pub const CDT_REG: c_int = 8;
pub const CDT_LNK: c_int = 10;
pub const CDT_SOCK: c_int = 12;
pub const CDT_WHT: c_int = 14;
//
// Convert between stat structure types and directory types.
//

pub type vuid_t = u_int32_t;
pub type vgid_t = u_int32_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CodaFid {
    pub opaque: [u_int32_t; 4],
}

// Macro flag: #define coda_f2i(fid)\
//
// Vnode types.  VNON means no type.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coda_vtype {

    struct coda_timespec {
    int64_t		tv_sec;		/* seconds */
    long		tv_nsec;	/* nanoseconds */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_vattr {
    pub /: *mut *mut long va_type; / vnode type (for create),
    pub /: *mut *mut u_short va_mode; / files access mode and type,
    pub /: *mut *mut short va_nlink; / number of references to file,
    pub /: *mut *mut vuid_t va_uid; / owner user id,
    pub /: *mut *mut vgid_t va_gid; / owner group id,
    pub /: *mut *mut long va_fileid; / file id,
    pub /: *mut *mut u_quad_t va_size; / file size in bytes,
    pub /: *mut *mut long va_blocksize; / blocksize preferred for i/o,
    pub /: *mut *mut coda_timespec va_atime; / time of last access,
    pub /: *mut *mut coda_timespec va_mtime; / time of last modification,
    pub /: *mut *mut coda_timespec va_ctime; / time file changed,
    pub /: *mut *mut u_long va_gen; / generation number of file,
    pub /: *mut *mut u_long va_flags; / flags defined for file,
    pub /: *mut *mut cdev_t va_rdev; / device special file represents,
    pub /: *mut *mut u_quad_t va_bytes; / bytes of disk space held by file,
    pub /: *mut *mut u_quad_t va_filerev; / file modification number,
}

// structure used by CODA_STATFS for getting cache information from venus
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_statfs {
    pub f_blocks: i32,
    pub f_bfree: i32,
    pub f_bavail: i32,
    pub f_files: i32,
    pub f_ffree: i32,
}

//
// Kernel <--> Venus communications.
//
pub const CODA_ROOT: c_int = 2;
pub const CODA_OPEN_BY_FD: c_int = 3;
pub const CODA_OPEN: c_int = 4;
pub const CODA_CLOSE: c_int = 5;
pub const CODA_IOCTL: c_int = 6;
pub const CODA_GETATTR: c_int = 7;
pub const CODA_SETATTR: c_int = 8;
pub const CODA_ACCESS: c_int = 9;
pub const CODA_LOOKUP: c_int = 10;
pub const CODA_CREATE: c_int = 11;
pub const CODA_REMOVE: c_int = 12;
pub const CODA_LINK: c_int = 13;
pub const CODA_RENAME: c_int = 14;
pub const CODA_MKDIR: c_int = 15;
pub const CODA_RMDIR: c_int = 16;
pub const CODA_SYMLINK: c_int = 18;
pub const CODA_READLINK: c_int = 19;
pub const CODA_FSYNC: c_int = 20;
pub const CODA_VGET: c_int = 22;
pub const CODA_SIGNAL: c_int = 23;

pub const CODA_OPEN_BY_PATH: c_int = 31;
pub const CODA_RESOLVE: c_int = 32;
pub const CODA_REINTEGRATE: c_int = 33;
pub const CODA_STATFS: c_int = 34;
pub const CODA_STORE: c_int = 35;
pub const CODA_RELEASE: c_int = 36;
pub const CODA_ACCESS_INTENT: c_int = 37;
pub const CODA_NCALLS: c_int = 38;

pub const VC_MAXDATASIZE: c_int = 8192;

// CODA_KERNEL_VERSION 0 /* don't care about kernel version number
// CODA_KERNEL_VERSION 1 /* The old venus 4.6 compatible interface
// CODA_KERNEL_VERSION 2 /* venus_lookup gets an extra parameter
// CODA_KERNEL_VERSION 3 /* 128-bit file identifiers
// CODA_KERNEL_VERSION 4 /* 64-bit timespec

//
// Venus <-> Coda  RPC arguments
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_in_hdr {
    pub opcode: u_int32_t,
    pub /: *mut *mut u_int32_t unique; / Keep multiple outstanding msgs distinct,
    pub pid: __kernel_pid_t,
    pub pgid: __kernel_pid_t,
    pub uid: vuid_t,
}

// Really important that opcode and unique are 1st two fields!
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_out_hdr {
    pub opcode: u_int32_t,
    pub unique: u_int32_t,
    pub result: u_int32_t,
}

// coda_root: NO_IN
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_root_out {
    pub oh: coda_out_hdr,
    pub VFid: CodaFid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_root_in {
    pub in: coda_in_hdr,
}

// coda_open:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_open_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_open_out {
    pub oh: coda_out_hdr,
    pub dev: cdev_t,
    pub inode: ino_t,
}

// coda_store:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_store_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_store_out {
    pub out: coda_out_hdr,
}

// coda_release:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_release_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_release_out {
    pub out: coda_out_hdr,
}

// coda_close:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_close_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_close_out {
    pub out: coda_out_hdr,
}

// coda_ioctl:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_ioctl_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
    pub cmd: c_int,
    pub len: c_int,
    pub rwflag: c_int,
    pub /: *mut *mut *mut char data; / Place holder for data.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_ioctl_out {
    pub oh: coda_out_hdr,
    pub len: c_int,
    pub /: *mut *mut caddr_t data; / Place holder for data.,
}

// coda_getattr:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_getattr_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_getattr_out {
    pub oh: coda_out_hdr,
    pub attr: coda_vattr,
}

// coda_setattr: NO_OUT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_setattr_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
    pub attr: coda_vattr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_setattr_out {
    pub out: coda_out_hdr,
}

// coda_access: NO_OUT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_access_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_access_out {
    pub out: coda_out_hdr,
}

// lookup flags
pub const CLU_CASE_SENSITIVE: c_uint = 0x01;
pub const CLU_CASE_INSENSITIVE: c_uint = 0x02;
// coda_lookup:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_lookup_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
    pub /: *mut *mut int name; / Place holder for data.,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_lookup_out {
    pub oh: coda_out_hdr,
    pub VFid: CodaFid,
    pub vtype: c_int,
}

// coda_create:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_create_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
    pub attr: coda_vattr,
    pub excl: c_int,
    pub mode: c_int,
    pub /: *mut *mut int name; / Place holder for data.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_create_out {
    pub oh: coda_out_hdr,
    pub VFid: CodaFid,
    pub attr: coda_vattr,
}

// coda_remove: NO_OUT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_remove_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
    pub /: *mut *mut int name; / Place holder for data.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_remove_out {
    pub out: coda_out_hdr,
}

// coda_link: NO_OUT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_link_in {
    pub ih: coda_in_hdr,
    pub /: *mut *mut *mut *mut CodaFid sourceFid; / cnode to link to,
    pub /: *mut *mut CodaFid destFid; / Directory in which to place link,
    pub /: *mut *mut int tname; / Place holder for data.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_link_out {
    pub out: coda_out_hdr,
}

// coda_rename: NO_OUT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_rename_in {
    pub ih: coda_in_hdr,
    pub sourceFid: CodaFid,
    pub srcname: c_int,
    pub destFid: CodaFid,
    pub destname: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_rename_out {
    pub out: coda_out_hdr,
}

// coda_mkdir:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_mkdir_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
    pub attr: coda_vattr,
    pub /: *mut *mut int name; / Place holder for data.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_mkdir_out {
    pub oh: coda_out_hdr,
    pub VFid: CodaFid,
    pub attr: coda_vattr,
}

// coda_rmdir: NO_OUT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_rmdir_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
    pub /: *mut *mut int name; / Place holder for data.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_rmdir_out {
    pub out: coda_out_hdr,
}

// coda_symlink: NO_OUT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_symlink_in {
    pub ih: coda_in_hdr,
    pub /: *mut *mut CodaFid VFid; / Directory to put symlink in,
    pub srcname: c_int,
    pub attr: coda_vattr,
    pub tname: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_symlink_out {
    pub out: coda_out_hdr,
}

// coda_readlink:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_readlink_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_readlink_out {
    pub oh: coda_out_hdr,
    pub count: c_int,
    pub /: *mut *mut caddr_t data; / Place holder for data.,
}

// coda_fsync: NO_OUT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_fsync_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_fsync_out {
    pub out: coda_out_hdr,
}

// coda_vget:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_vget_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_vget_out {
    pub oh: coda_out_hdr,
    pub VFid: CodaFid,
    pub vtype: c_int,
}

// CODA_SIGNAL is out-of-band, doesn't need data.
// CODA_INVALIDATE is a venus->kernel call
// CODA_FLUSH is a venus->kernel call
// coda_purgeuser:
// CODA_PURGEUSER is a venus->kernel call
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_purgeuser_out {
    pub oh: coda_out_hdr,
    pub uid: vuid_t,
}

// coda_zapfile:
// CODA_ZAPFILE is a venus->kernel call
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_zapfile_out {
    pub oh: coda_out_hdr,
    pub CodaFid: CodaFid,
}

// coda_zapdir:
// CODA_ZAPDIR is a venus->kernel call
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_zapdir_out {
    pub oh: coda_out_hdr,
    pub CodaFid: CodaFid,
}

// coda_purgefid:
// CODA_PURGEFID is a venus->kernel call
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_purgefid_out {
    pub oh: coda_out_hdr,
    pub CodaFid: CodaFid,
}

// coda_replace:
// CODA_REPLACE is a venus->kernel call
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_replace_out {
    pub oh: coda_out_hdr,
    pub NewFid: CodaFid,
    pub OldFid: CodaFid,
}

// coda_open_by_fd:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_open_by_fd_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_open_by_fd_out {
    pub oh: coda_out_hdr,
    pub fd: c_int,

    pub /: *mut *mut *mut file fh; / not passed from userspace but used in-kernel only,

}

// coda_open_by_path:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_open_by_path_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_open_by_path_out {
    pub oh: coda_out_hdr,
    pub path: c_int,
}

// coda_statfs: NO_IN
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_statfs_in {
    pub in: coda_in_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_statfs_out {
    pub oh: coda_out_hdr,
    pub stat: coda_statfs,
}

pub const CODA_ACCESS_TYPE_READ: c_int = 1;
pub const CODA_ACCESS_TYPE_WRITE: c_int = 2;
pub const CODA_ACCESS_TYPE_MMAP: c_int = 3;
pub const CODA_ACCESS_TYPE_READ_FINISH: c_int = 4;
pub const CODA_ACCESS_TYPE_WRITE_FINISH: c_int = 5;
// coda_access_intent: NO_OUT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_access_intent_in {
    pub ih: coda_in_hdr,
    pub VFid: CodaFid,
    pub count: c_int,
    pub pos: c_int,
    pub type: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_access_intent_out {
    pub out: coda_out_hdr,
}

//
// Occasionally, we don't cache the fid returned by CODA_LOOKUP.
// For instance, if the fid is inconsistent.
// This case is handled by setting the top bit of the type result parameter.
//
pub const CODA_NOCACHE: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub union inputArgs {
    pub /: *mut *mut coda_in_hdr ih; / NB: every below begins with an ih,
    pub coda_open: coda_open_in,
    pub coda_store: coda_store_in,
    pub coda_release: coda_release_in,
    pub coda_close: coda_close_in,
    pub coda_ioctl: coda_ioctl_in,
    pub coda_getattr: coda_getattr_in,
    pub coda_setattr: coda_setattr_in,
    pub coda_access: coda_access_in,
    pub coda_lookup: coda_lookup_in,
    pub coda_create: coda_create_in,
    pub coda_remove: coda_remove_in,
    pub coda_link: coda_link_in,
    pub coda_rename: coda_rename_in,
    pub coda_mkdir: coda_mkdir_in,
    pub coda_rmdir: coda_rmdir_in,
    pub coda_symlink: coda_symlink_in,
    pub coda_readlink: coda_readlink_in,
    pub coda_fsync: coda_fsync_in,
    pub coda_vget: coda_vget_in,
    pub coda_open_by_fd: coda_open_by_fd_in,
    pub coda_open_by_path: coda_open_by_path_in,
    pub coda_statfs: coda_statfs_in,
    pub coda_access_intent: coda_access_intent_in,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union outputArgs {
    pub /: *mut *mut coda_out_hdr oh; / NB: every below begins with an oh,
    pub coda_root: coda_root_out,
    pub coda_open: coda_open_out,
    pub coda_ioctl: coda_ioctl_out,
    pub coda_getattr: coda_getattr_out,
    pub coda_lookup: coda_lookup_out,
    pub coda_create: coda_create_out,
    pub coda_mkdir: coda_mkdir_out,
    pub coda_readlink: coda_readlink_out,
    pub coda_vget: coda_vget_out,
    pub coda_purgeuser: coda_purgeuser_out,
    pub coda_zapfile: coda_zapfile_out,
    pub coda_zapdir: coda_zapdir_out,
    pub coda_purgefid: coda_purgefid_out,
    pub coda_replace: coda_replace_out,
    pub coda_open_by_fd: coda_open_by_fd_out,
    pub coda_open_by_path: coda_open_by_path_out,
    pub coda_statfs: coda_statfs_out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union coda_downcalls {
// CODA_INVALIDATE is a venus->kernel call
// CODA_FLUSH is a venus->kernel call
    pub purgeuser: coda_purgeuser_out,
    pub zapfile: coda_zapfile_out,
    pub zapdir: coda_zapdir_out,
    pub purgefid: coda_purgefid_out,
    pub replace: coda_replace_out,
}

//
// Used for identifying usage of "Control" and pioctls
//
pub const PIOCPARM_MASK: c_uint = 0x0000ffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ViceIoctl {
    pub /: *mut *mut *mut void __user in; / Data to be transferred in,
    pub /: *mut *mut *mut void __user out; / Data to be transferred out,
    pub /: *mut *mut u_short in_size; / Size of input buffer <= 2K,
    pub /: *mut *mut u_short out_size; / Maximum size of output buffer, <= 2K,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PioctlData {
    pub path: *const char __user,
    pub follow: c_int,
    pub vi: ViceIoctl,
}

pub const CODA_CONTROLLEN: c_int = 8;

// Data passed to mount
pub const CODA_MOUNT_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coda_mount_data {
    pub version: c_int,
    pub /: *mut *mut int fd; / Opened device,
}
