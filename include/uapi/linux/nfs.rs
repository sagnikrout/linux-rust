//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nfs.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// NFS protocol definitions
//
// This file contains constants mostly for Version 2 of the protocol,
// but also has a couple of NFSv3 bits in (notably the error codes).
//

pub const NFS_PROGRAM: c_int = 100003;
pub const NFS_PORT: c_int = 2049;
pub const NFS_RDMA_PORT: c_int = 20049;
pub const NFS_MAXDATA: c_int = 8192;
pub const NFS_MAXPATHLEN: c_int = 1024;
pub const NFS_MAXNAMLEN: c_int = 255;
pub const NFS_MAXGROUPS: c_int = 16;
pub const NFS_FHSIZE: c_int = 32;
pub const NFS_COOKIESIZE: c_int = 4;

pub const NFSMODE_FMT: c_int = 0170000;
pub const NFSMODE_DIR: c_int = 0040000;
pub const NFSMODE_CHR: c_int = 0020000;
pub const NFSMODE_BLK: c_int = 0060000;
pub const NFSMODE_REG: c_int = 0100000;
pub const NFSMODE_LNK: c_int = 0120000;
pub const NFSMODE_SOCK: c_int = 0140000;
pub const NFSMODE_FIFO: c_int = 0010000;
pub const NFS_MNT_PROGRAM: c_int = 100005;
pub const NFS_MNT_VERSION: c_int = 1;
pub const NFS_MNT3_VERSION: c_int = 3;

//
// NFS stats. The good thing with these values is that NFSv3 errors are
// a superset of NFSv2 errors (with the exception of NFSERR_WFLUSH which
// no-one uses anyway), so we can happily mix code as long as we make sure
// no NFSv3 errors are returned to NFSv2 clients.
// Error codes that have a `--' in the v2 column are not part of the
// standard, but seem to be widely used nevertheless.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs_stat {
    NFS_OK = 0,			/* v2 v3 v4 */
    NFSERR_PERM = 1,		/* v2 v3 v4 */
    NFSERR_NOENT = 2,		/* v2 v3 v4 */
    NFSERR_IO = 5,			/* v2 v3 v4 */
    NFSERR_NXIO = 6,		/* v2 v3 v4 */
    NFSERR_ACCES = 13,		/* v2 v3 v4 */
    NFSERR_EXIST = 17,		/* v2 v3 v4 */
    NFSERR_XDEV = 18,		/*    v3 v4 */
    NFSERR_NODEV = 19,		/* v2 v3 v4 */
    NFSERR_NOTDIR = 20,		/* v2 v3 v4 */
    NFSERR_ISDIR = 21,		/* v2 v3 v4 */
    NFSERR_INVAL = 22,		/*    v3 v4 */
    NFSERR_FBIG = 27,		/* v2 v3 v4 */
    NFSERR_NOSPC = 28,		/* v2 v3 v4 */
    NFSERR_ROFS = 30,		/* v2 v3 v4 */
    NFSERR_MLINK = 31,		/*    v3 v4 */
    NFSERR_NAMETOOLONG = 63,	/* v2 v3 v4 */
    NFSERR_NOTEMPTY = 66,		/* v2 v3 v4 */
    NFSERR_DQUOT = 69,		/* v2 v3 v4 */
    NFSERR_STALE = 70,		/* v2 v3 v4 */
    NFSERR_REMOTE = 71,		/* v2 v3 */
    NFSERR_WFLUSH = 99,		/* v2    */
    NFSERR_BADHANDLE = 10001,	/*    v3 v4 */
    NFSERR_NOT_SYNC = 10002,	/*    v3 */
    NFSERR_BAD_COOKIE = 10003,	/*    v3 v4 */
    NFSERR_NOTSUPP = 10004,		/*    v3 v4 */
    NFSERR_TOOSMALL = 10005,	/*    v3 v4 */
    NFSERR_SERVERFAULT = 10006,	/*    v3 v4 */
    NFSERR_BADTYPE = 10007,		/*    v3 v4 */
    NFSERR_JUKEBOX = 10008,		/*    v3 v4 */
    NFSERR_SAME = 10009,		/*       v4 */
    NFSERR_DENIED = 10010,		/*       v4 */
    NFSERR_EXPIRED = 10011,		/*       v4 */
    NFSERR_LOCKED = 10012,		/*       v4 */
    NFSERR_GRACE = 10013,		/*       v4 */
    NFSERR_FHEXPIRED = 10014,	/*       v4 */
    NFSERR_SHARE_DENIED = 10015,	/*       v4 */
    NFSERR_WRONGSEC = 10016,	/*       v4 */
    NFSERR_CLID_INUSE = 10017,	/*       v4 */
    NFSERR_RESOURCE = 10018,	/*       v4 */
    NFSERR_MOVED = 10019,		/*       v4 */
    NFSERR_NOFILEHANDLE = 10020,	/*       v4 */
    NFSERR_MINOR_VERS_MISMATCH = 10021,   /* v4 */
    NFSERR_STALE_CLIENTID = 10022,	/*       v4 */
    NFSERR_STALE_STATEID = 10023,   /*       v4 */
    NFSERR_OLD_STATEID = 10024,     /*       v4 */
    NFSERR_BAD_STATEID = 10025,     /*       v4 */
    NFSERR_BAD_SEQID = 10026,	/*       v4 */
    NFSERR_NOT_SAME = 10027,	/*       v4 */
    NFSERR_LOCK_RANGE = 10028,	/*       v4 */
    NFSERR_SYMLINK = 10029,		/*       v4 */
    NFSERR_RESTOREFH = 10030,	/*       v4 */
    NFSERR_LEASE_MOVED = 10031,	/*       v4 */
    NFSERR_ATTRNOTSUPP = 10032,	/*       v4 */
    NFSERR_NO_GRACE = 10033,	/*       v4 */
    NFSERR_RECLAIM_BAD = 10034,	/*       v4 */
    NFSERR_RECLAIM_CONFLICT = 10035,/*       v4 */
    NFSERR_BAD_XDR = 10036,		/*       v4 */
    NFSERR_LOCKS_HELD = 10037,	/*       v4 */
    NFSERR_OPENMODE = 10038,       /*       v4 */
    NFSERR_BADOWNER = 10039,       /*       v4 */
    NFSERR_BADCHAR = 10040,        /*       v4 */
    NFSERR_BADNAME = 10041,        /*       v4 */
    NFSERR_BAD_RANGE = 10042,      /*       v4 */
    NFSERR_LOCK_NOTSUPP = 10043,   /*       v4 */
    NFSERR_OP_ILLEGAL = 10044,     /*       v4 */
    NFSERR_DEADLOCK = 10045,       /*       v4 */
    NFSERR_FILE_OPEN = 10046,      /*       v4 */
    NFSERR_ADMIN_REVOKED = 10047,  /*       v4 */
    NFSERR_CB_PATH_DOWN = 10048,   /*       v4 */
}

// NFSv2 file types - beware, these are not the same in NFSv3
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs_ftype {
    NFNON = 0,
    NFREG = 1,
    NFDIR = 2,
    NFBLK = 3,
    NFCHR = 4,
    NFLNK = 5,
    NFSOCK = 6,
    NFBAD = 7,
    NFFIFO = 8
}
