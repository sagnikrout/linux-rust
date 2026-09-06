//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/fuse.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-2-Clause)
//
// This file defines the kernel interface of FUSE
//
// Protocol changelog:
//
// 7.1:
// - add the following messages:
// FUSE_SETATTR, FUSE_SYMLINK, FUSE_MKNOD, FUSE_MKDIR, FUSE_UNLINK,
// FUSE_RMDIR, FUSE_RENAME, FUSE_LINK, FUSE_OPEN, FUSE_READ, FUSE_WRITE,
// FUSE_RELEASE, FUSE_FSYNC, FUSE_FLUSH, FUSE_SETXATTR, FUSE_GETXATTR,
// FUSE_LISTXATTR, FUSE_REMOVEXATTR, FUSE_OPENDIR, FUSE_READDIR,
// FUSE_RELEASEDIR
// - add padding to messages to accommodate 32-bit servers on 64-bit kernels
//
// 7.2:
// - add FOPEN_DIRECT_IO and FOPEN_KEEP_CACHE flags
// - add FUSE_FSYNCDIR message
//
// 7.3:
// - add FUSE_ACCESS message
// - add FUSE_CREATE message
// - add filehandle to fuse_setattr_in
//
// 7.4:
// - add frsize to fuse_kstatfs
// - clean up request size limit checking
//
// 7.5:
// - add flags and max_write to fuse_init_out
//
// 7.6:
// - add max_readahead to fuse_init_in and fuse_init_out
//
// 7.7:
// - add FUSE_INTERRUPT message
// - add POSIX file lock support
//
// 7.8:
// - add lock_owner and flags fields to fuse_release_in
// - add FUSE_BMAP message
// - add FUSE_DESTROY message
//
// 7.9:
// - new fuse_getattr_in input argument of GETATTR
// - add lk_flags in fuse_lk_in
// - add lock_owner field to fuse_setattr_in, fuse_read_in and fuse_write_in
// - add blksize field to fuse_attr
// - add file flags field to fuse_read_in and fuse_write_in
// - Add ATIME_NOW and MTIME_NOW flags to fuse_setattr_in
//
// 7.10
// - add nonseekable open flag
//
// 7.11
// - add IOCTL message
// - add unsolicited notification support
// - add POLL message and NOTIFY_POLL notification
//
// 7.12
// - add umask flag to input argument of create, mknod and mkdir
// - add notification messages for invalidation of inodes and
// directory entries
//
// 7.13
// - make max number of background requests and congestion threshold
// tunables
//
// 7.14
// - add splice support to fuse device
//
// 7.15
// - add store notify
// - add retrieve notify
//
// 7.16
// - add BATCH_FORGET request
// - FUSE_IOCTL_UNRESTRICTED shall now return with array of 'struct
// fuse_ioctl_iovec' instead of ambiguous 'struct iovec'
// - add FUSE_IOCTL_32BIT flag
//
// 7.17
// - add FUSE_FLOCK_LOCKS and FUSE_RELEASE_FLOCK_UNLOCK
//
// 7.18
// - add FUSE_IOCTL_DIR flag
// - add FUSE_NOTIFY_DELETE
//
// 7.19
// - add FUSE_FALLOCATE
//
// 7.20
// - add FUSE_AUTO_INVAL_DATA
//
// 7.21
// - add FUSE_READDIRPLUS
// - send the requested events in POLL request
//
// 7.22
// - add FUSE_ASYNC_DIO
//
// 7.23
// - add FUSE_WRITEBACK_CACHE
// - add time_gran to fuse_init_out
// - add reserved space to fuse_init_out
// - add FATTR_CTIME
// - add ctime and ctimensec to fuse_setattr_in
// - add FUSE_RENAME2 request
// - add FUSE_NO_OPEN_SUPPORT flag
//
// 7.24
// - add FUSE_LSEEK for SEEK_HOLE and SEEK_DATA support
//
// 7.25
// - add FUSE_PARALLEL_DIROPS
//
// 7.26
// - add FUSE_HANDLE_KILLPRIV
// - add FUSE_POSIX_ACL
//
// 7.27
// - add FUSE_ABORT_ERROR
//
// 7.28
// - add FUSE_COPY_FILE_RANGE
// - add FOPEN_CACHE_DIR
// - add FUSE_MAX_PAGES, add max_pages to init_out
// - add FUSE_CACHE_SYMLINKS
//
// 7.29
// - add FUSE_NO_OPENDIR_SUPPORT flag
//
// 7.30
// - add FUSE_EXPLICIT_INVAL_DATA
// - add FUSE_IOCTL_COMPAT_X32
//
// 7.31
// - add FUSE_WRITE_KILL_PRIV flag
// - add FUSE_SETUPMAPPING and FUSE_REMOVEMAPPING
// - add map_alignment to fuse_init_out, add FUSE_MAP_ALIGNMENT flag
//
// 7.32
// - add flags to fuse_attr, add FUSE_ATTR_SUBMOUNT, add FUSE_SUBMOUNTS
//
// 7.33
// - add FUSE_HANDLE_KILLPRIV_V2, FUSE_WRITE_KILL_SUIDGID, FATTR_KILL_SUIDGID
// - add FUSE_OPEN_KILL_SUIDGID
// - extend fuse_setxattr_in, add FUSE_SETXATTR_EXT
// - add FUSE_SETXATTR_ACL_KILL_SGID
//
// 7.34
// - add FUSE_SYNCFS
//
// 7.35
// - add FOPEN_NOFLUSH
//
// 7.36
// - extend fuse_init_in with reserved fields, add FUSE_INIT_EXT init flag
// - add flags2 to fuse_init_in and fuse_init_out
// - add FUSE_SECURITY_CTX init flag
// - add security context to create, mkdir, symlink, and mknod requests
// - add FUSE_HAS_INODE_DAX, FUSE_ATTR_DAX
//
// 7.37
// - add FUSE_TMPFILE
//
// 7.38
// - add FUSE_EXPIRE_ONLY flag to fuse_notify_inval_entry
// - add FOPEN_PARALLEL_DIRECT_WRITES
// - add total_extlen to fuse_in_header
// - add FUSE_MAX_NR_SECCTX
// - add extension header
// - add FUSE_EXT_GROUPS
// - add FUSE_CREATE_SUPP_GROUP
// - add FUSE_HAS_EXPIRE_ONLY
//
// 7.39
// - add FUSE_DIRECT_IO_ALLOW_MMAP
// - add FUSE_STATX and related structures
//
// 7.40
// - add max_stack_depth to fuse_init_out, add FUSE_PASSTHROUGH init flag
// - add backing_id to fuse_open_out, add FOPEN_PASSTHROUGH open flag
// - add FUSE_NO_EXPORT_SUPPORT init flag
// - add FUSE_NOTIFY_RESEND, add FUSE_HAS_RESEND init flag
//
// 7.41
// - add FUSE_ALLOW_IDMAP
// 7.42
// - Add FUSE_OVER_IO_URING and all other io-uring related flags and data
// structures:
// - struct fuse_uring_ent_in_out
// - struct fuse_uring_req_header
// - struct fuse_uring_cmd_req
// - FUSE_URING_IN_OUT_HEADER_SZ
// - FUSE_URING_OP_IN_OUT_SZ
// - enum fuse_uring_cmd
//
// 7.43
// - add FUSE_REQUEST_TIMEOUT
//
// 7.44
// - add FUSE_NOTIFY_INC_EPOCH
//
// 7.45
// - add FUSE_COPY_FILE_RANGE_64
// - add struct fuse_copy_file_range_out
// - add FUSE_NOTIFY_PRUNE
//
// 7.46
// - add FUSE_IO_URING_CMD_ADD_QUEUE
// - add FUSE_HAS_IO_URING_BUFPOOL
// - add fuse_uring_cmd_req bufpool struct
// - add bufpool offset field to fuse_uring_ent_in_out struct
// - add FUSE_URING_ZERO_COPY, FUSE_URING_ENT_ZERO_COPY, and
// FOPEN_IO_URING_ZERO_COPY flag
//

//
// Version negotiation:
//
// Both the kernel and userspace send the version they support in the
// INIT request and reply respectively.
//
// If the major versions match then both shall use the smallest
// of the two minor versions for communication.
//
// If the kernel supports a larger major version, then userspace shall
// reply with the major version it supports, ignore the rest of the
// INIT message and expect a new INIT message from the kernel with a
// matching major version.
//
// If the library supports a larger major version, then it shall fall
// back to the major protocol version sent by the kernel for
// communication and reply with that major version (and an arbitrary
// supported minor version).
//
// Version number of this interface
pub const FUSE_KERNEL_VERSION: c_int = 7;
// Minor version number of this interface
pub const FUSE_KERNEL_MINOR_VERSION: c_int = 46;
// The node ID of the root inode
pub const FUSE_ROOT_ID: c_int = 1;
// Make sure all structures are padded to 64bit boundary, so 32bit
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_attr {
    pub ino: u64,
    pub size: u64,
    pub blocks: u64,
    pub atime: u64,
    pub mtime: u64,
    pub ctime: u64,
    pub atimensec: u32,
    pub mtimensec: u32,
    pub ctimensec: u32,
    pub mode: u32,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub rdev: u32,
    pub blksize: u32,
    pub flags: u32,
}

//
// The following structures are bit-for-bit compatible with the statx(2) ABI in
// Linux.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_sx_time {
    pub tv_sec: i64,
    pub tv_nsec: u32,
    pub __reserved: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_statx {
    pub mask: u32,
    pub blksize: u32,
    pub attributes: u64,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub mode: u16,
    pub __spare0: [u16; 1],
    pub ino: u64,
    pub size: u64,
    pub blocks: u64,
    pub attributes_mask: u64,
    pub atime: fuse_sx_time,
    pub btime: fuse_sx_time,
    pub ctime: fuse_sx_time,
    pub mtime: fuse_sx_time,
    pub rdev_major: u32,
    pub rdev_minor: u32,
    pub dev_major: u32,
    pub dev_minor: u32,
    pub __spare2: [u64; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_kstatfs {
    pub blocks: u64,
    pub bfree: u64,
    pub bavail: u64,
    pub files: u64,
    pub ffree: u64,
    pub bsize: u32,
    pub namelen: u32,
    pub frsize: u32,
    pub padding: u32,
    pub spare: [u32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_file_lock {
    pub start: u64,
    pub end: u64,
    pub type: u32,
    pub /: *mut *mut uint32_t pid; / tgid,
}

//
// Bitmasks for fuse_setattr_in.valid
//

//
// Flags returned by the OPEN request
//
// FOPEN_DIRECT_IO: bypass page cache for this open file
// FOPEN_KEEP_CACHE: don't invalidate the data cache on open
// FOPEN_NONSEEKABLE: the file is not seekable
// FOPEN_CACHE_DIR: allow caching this directory
// FOPEN_STREAM: the file is stream-like (no file position at all)
// FOPEN_NOFLUSH: don't flush data cache on close (unless FUSE_WRITEBACK_CACHE)
// FOPEN_PARALLEL_DIRECT_WRITES: Allow concurrent direct writes on the same inode
// FOPEN_PASSTHROUGH: passthrough read/write io for this open file
// FOPEN_IO_URING_ZERO_COPY: use io-uring zero-copy for reads/writes on this
// open file. Honored only when the serving io-uring
// queue was set up for zero-copy
// (FUSE_URING_ZERO_COPY) and the request carries page
// payload. Otherwise reads/writes fall back to
// copying.
//

//
// INIT request/reply flags
//
// FUSE_ASYNC_READ: asynchronous read requests
// FUSE_POSIX_LOCKS: remote locking for POSIX file locks
// FUSE_FILE_OPS: kernel sends file handle for fstat, etc... (not yet supported)
// FUSE_ATOMIC_O_TRUNC: handles the O_TRUNC open flag in the filesystem
// FUSE_EXPORT_SUPPORT: filesystem handles lookups of "." and ".."
// FUSE_BIG_WRITES: filesystem can handle write size larger than 4kB
// FUSE_DONT_MASK: don't apply umask to file mode on create operations
// FUSE_SPLICE_WRITE: kernel supports splice write on the device
// FUSE_SPLICE_MOVE: kernel supports splice move on the device
// FUSE_SPLICE_READ: kernel supports splice read on the device
// FUSE_FLOCK_LOCKS: remote locking for BSD style file locks
// FUSE_HAS_IOCTL_DIR: kernel supports ioctl on directories
// FUSE_AUTO_INVAL_DATA: automatically invalidate cached pages
// FUSE_DO_READDIRPLUS: do READDIRPLUS (READDIR+LOOKUP in one)
// FUSE_READDIRPLUS_AUTO: adaptive readdirplus
// FUSE_ASYNC_DIO: asynchronous direct I/O submission
// FUSE_WRITEBACK_CACHE: use writeback cache for buffered writes
// FUSE_NO_OPEN_SUPPORT: kernel supports zero-message opens
// FUSE_PARALLEL_DIROPS: allow parallel lookups and readdir
// FUSE_HANDLE_KILLPRIV: fs handles killing suid/sgid/cap on write/chown/trunc
// FUSE_POSIX_ACL: filesystem supports posix acls
// FUSE_ABORT_ERROR: reading the device after abort returns ECONNABORTED
// FUSE_MAX_PAGES: init_out.max_pages contains the max number of req pages
// FUSE_CACHE_SYMLINKS: cache READLINK responses
// FUSE_NO_OPENDIR_SUPPORT: kernel supports zero-message opendir
// FUSE_EXPLICIT_INVAL_DATA: only invalidate cached pages on explicit request
// FUSE_MAP_ALIGNMENT: init_out.map_alignment contains log2(byte alignment) for
// foffset and moffset fields in struct
// fuse_setupmapping_out and fuse_removemapping_one.
// FUSE_SUBMOUNTS: kernel supports auto-mounting directory submounts
// FUSE_HANDLE_KILLPRIV_V2: fs kills suid/sgid/cap on write/chown/trunc.
// Upon write/truncate suid/sgid is only killed if caller
// does not have CAP_FSETID. Additionally upon
// write/truncate sgid is killed only if file has group
// execute permission. (Same as Linux VFS behavior).
// FUSE_SETXATTR_EXT:	Server supports extended struct fuse_setxattr_in
// FUSE_INIT_EXT: extended fuse_init_in request
// FUSE_INIT_RESERVED: reserved, do not use
// FUSE_SECURITY_CTX:	add security context to create, mkdir, symlink, and
// mknod
// FUSE_HAS_INODE_DAX:  use per inode DAX
// FUSE_CREATE_SUPP_GROUP: add supplementary group info to create, mkdir,
// symlink and mknod (single group that matches parent)
// FUSE_HAS_EXPIRE_ONLY: kernel supports expiry-only entry invalidation
// FUSE_DIRECT_IO_ALLOW_MMAP: allow shared mmap in FOPEN_DIRECT_IO mode.
// FUSE_NO_EXPORT_SUPPORT: explicitly disable export support
// FUSE_HAS_RESEND: kernel supports resending pending requests, and the high bit
// of the request ID indicates resend requests
// FUSE_ALLOW_IDMAP: allow creation of idmapped mounts
// FUSE_OVER_IO_URING: Indicate that client supports io-uring
// FUSE_REQUEST_TIMEOUT: kernel supports timing out requests.
// init_out.request_timeout contains the timeout (in secs)
// FUSE_HAS_IO_URING_BUFPOOL: kernel supports io-uring buffer pools
//

// bits 32..63 get shifted down 32 bits into the flags2 field

// Obsolete alias for FUSE_DIRECT_IO_ALLOW_MMAP

//
// CUSE INIT request/reply flags
//
// CUSE_UNRESTRICTED_IOCTL:  use unrestricted ioctl
//

//
// Release flags
//

//
// Getattr flags
//

//
// Lock flags
//

//
// WRITE flags
//
// FUSE_WRITE_CACHE: delayed write from page cache, file handle is guessed
// FUSE_WRITE_LOCKOWNER: lock_owner field is valid
// FUSE_WRITE_KILL_SUIDGID: kill suid and sgid bits
//

// Obsolete alias; this flag implies killing suid/sgid only.

//
// Read flags
//

//
// Ioctl flags
//
// FUSE_IOCTL_COMPAT: 32bit compat ioctl on 64bit machine
// FUSE_IOCTL_UNRESTRICTED: not restricted to well-formed ioctls, retry allowed
// FUSE_IOCTL_RETRY: retry with new iovecs
// FUSE_IOCTL_32BIT: 32bit ioctl
// FUSE_IOCTL_DIR: is a directory
// FUSE_IOCTL_COMPAT_X32: x32 compat ioctl on 64bit machine (64bit time_t)
//
// FUSE_IOCTL_MAX_IOV: maximum of in_iovecs + out_iovecs
//

pub const FUSE_IOCTL_MAX_IOV: c_int = 256;
//
// Poll flags
//
// FUSE_POLL_SCHEDULE_NOTIFY: request poll notify
//

//
// Fsync flags
//
// FUSE_FSYNC_FDATASYNC: Sync data only, not metadata
//

//
// fuse_attr flags
//
// FUSE_ATTR_SUBMOUNT: Object is a submount root
// FUSE_ATTR_DAX: Enable DAX for this file in per inode DAX mode
//

//
// Open flags
// FUSE_OPEN_KILL_SUIDGID: Kill suid and sgid if executable
//

//
// setxattr flags
// FUSE_SETXATTR_ACL_KILL_SGID: Clear SGID when system.posix_acl_access is set
//

//
// notify_inval_entry flags
// FUSE_EXPIRE_ONLY
//

//
// extension type
// FUSE_MAX_NR_SECCTX: maximum value of &fuse_secctx_header.nr_secctx
// FUSE_EXT_GROUPS: &fuse_supp_groups extension
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fuse_ext_type {
// Types 0..31 are reserved for fuse_secctx_header
    FUSE_MAX_NR_SECCTX	= 31,
    FUSE_EXT_GROUPS		= 32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fuse_opcode {
    FUSE_LOOKUP		= 1,
    FUSE_FORGET		= 2,  /* no reply */
    FUSE_GETATTR		= 3,
    FUSE_SETATTR		= 4,
    FUSE_READLINK		= 5,
    FUSE_SYMLINK		= 6,
    FUSE_MKNOD		= 8,
    FUSE_MKDIR		= 9,
    FUSE_UNLINK		= 10,
    FUSE_RMDIR		= 11,
    FUSE_RENAME		= 12,
    FUSE_LINK		= 13,
    FUSE_OPEN		= 14,
    FUSE_READ		= 15,
    FUSE_WRITE		= 16,
    FUSE_STATFS		= 17,
    FUSE_RELEASE		= 18,
    FUSE_FSYNC		= 20,
    FUSE_SETXATTR		= 21,
    FUSE_GETXATTR		= 22,
    FUSE_LISTXATTR		= 23,
    FUSE_REMOVEXATTR	= 24,
    FUSE_FLUSH		= 25,
    FUSE_INIT		= 26,
    FUSE_OPENDIR		= 27,
    FUSE_READDIR		= 28,
    FUSE_RELEASEDIR		= 29,
    FUSE_FSYNCDIR		= 30,
    FUSE_GETLK		= 31,
    FUSE_SETLK		= 32,
    FUSE_SETLKW		= 33,
    FUSE_ACCESS		= 34,
    FUSE_CREATE		= 35,
    FUSE_INTERRUPT		= 36,
    FUSE_BMAP		= 37,
    FUSE_DESTROY		= 38,
    FUSE_IOCTL		= 39,
    FUSE_POLL		= 40,
    FUSE_NOTIFY_REPLY	= 41,
    FUSE_BATCH_FORGET	= 42,
    FUSE_FALLOCATE		= 43,
    FUSE_READDIRPLUS	= 44,
    FUSE_RENAME2		= 45,
    FUSE_LSEEK		= 46,
    FUSE_COPY_FILE_RANGE	= 47,
    FUSE_SETUPMAPPING	= 48,
    FUSE_REMOVEMAPPING	= 49,
    FUSE_SYNCFS		= 50,
    FUSE_TMPFILE		= 51,
    FUSE_STATX		= 52,
    FUSE_COPY_FILE_RANGE_64	= 53,

// CUSE specific operations
    CUSE_INIT		= 4096,

// Reserved opcodes: helpful to detect structure endian-ness
    CUSE_INIT_BSWAP_RESERVED	= 1048576,	/* CUSE_INIT << 8 */
    FUSE_INIT_BSWAP_RESERVED	= 436207616,	/* FUSE_INIT << 24 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fuse_notify_code {
    FUSE_NOTIFY_POLL   = 1,
    FUSE_NOTIFY_INVAL_INODE = 2,
    FUSE_NOTIFY_INVAL_ENTRY = 3,
    FUSE_NOTIFY_STORE = 4,
    FUSE_NOTIFY_RETRIEVE = 5,
    FUSE_NOTIFY_DELETE = 6,
    FUSE_NOTIFY_RESEND = 7,
    FUSE_NOTIFY_INC_EPOCH = 8,
    FUSE_NOTIFY_PRUNE = 9,
}

// The read buffer is required to be at least 8k, but may be much larger
pub const FUSE_MIN_READ_BUFFER: c_int = 8192;
pub const FUSE_COMPAT_ENTRY_OUT_SIZE: c_int = 120;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_entry_out {
    pub /: *mut *mut uint64_t nodeid; / Inode ID,
    pub must: *mut *mut uint64_t generation; / Inode generation: nodeid:gen,
    pub /: *mut *mut uint64_t entry_valid; / Cache timeout for the name,
    pub /: *mut *mut uint64_t attr_valid; / Cache timeout for the attributes,
    pub entry_valid_nsec: u32,
    pub attr_valid_nsec: u32,
    pub attr: fuse_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_forget_in {
    pub nlookup: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_forget_one {
    pub nodeid: u64,
    pub nlookup: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_batch_forget_in {
    pub count: u32,
    pub dummy: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_getattr_in {
    pub getattr_flags: u32,
    pub dummy: u32,
    pub fh: u64,
}

pub const FUSE_COMPAT_ATTR_OUT_SIZE: c_int = 96;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_attr_out {
    pub /: *mut *mut uint64_t attr_valid; / Cache timeout for the attributes,
    pub attr_valid_nsec: u32,
    pub dummy: u32,
    pub attr: fuse_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_statx_in {
    pub getattr_flags: u32,
    pub reserved: u32,
    pub fh: u64,
    pub sx_flags: u32,
    pub sx_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_statx_out {
    pub /: *mut *mut uint64_t attr_valid; / Cache timeout for the attributes,
    pub attr_valid_nsec: u32,
    pub flags: u32,
    pub spare: [u64; 2],
    pub stat: fuse_statx,
}

pub const FUSE_COMPAT_MKNOD_IN_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_mknod_in {
    pub mode: u32,
    pub rdev: u32,
    pub umask: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_mkdir_in {
    pub mode: u32,
    pub umask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_rename_in {
    pub newdir: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_rename2_in {
    pub newdir: u64,
    pub flags: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_link_in {
    pub oldnodeid: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_setattr_in {
    pub valid: u32,
    pub padding: u32,
    pub fh: u64,
    pub size: u64,
    pub lock_owner: u64,
    pub atime: u64,
    pub mtime: u64,
    pub ctime: u64,
    pub atimensec: u32,
    pub mtimensec: u32,
    pub ctimensec: u32,
    pub mode: u32,
    pub unused4: u32,
    pub uid: u32,
    pub gid: u32,
    pub unused5: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_open_in {
    pub flags: u32,
    pub /: *mut *mut uint32_t open_flags; / FUSE_OPEN_...,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_create_in {
    pub flags: u32,
    pub mode: u32,
    pub umask: u32,
    pub /: *mut *mut uint32_t open_flags; / FUSE_OPEN_...,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_open_out {
    pub fh: u64,
    pub open_flags: u32,
    pub backing_id: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_release_in {
    pub fh: u64,
    pub flags: u32,
    pub release_flags: u32,
    pub lock_owner: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_flush_in {
    pub fh: u64,
    pub unused: u32,
    pub padding: u32,
    pub lock_owner: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_read_in {
    pub fh: u64,
    pub offset: u64,
    pub size: u32,
    pub read_flags: u32,
    pub lock_owner: u64,
    pub flags: u32,
    pub padding: u32,
}

pub const FUSE_COMPAT_WRITE_IN_SIZE: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_write_in {
    pub fh: u64,
    pub offset: u64,
    pub size: u32,
    pub write_flags: u32,
    pub lock_owner: u64,
    pub flags: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_write_out {
    pub size: u32,
    pub padding: u32,
}

pub const FUSE_COMPAT_STATFS_SIZE: c_int = 48;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_statfs_out {
    pub st: fuse_kstatfs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_fsync_in {
    pub fh: u64,
    pub fsync_flags: u32,
    pub padding: u32,
}

pub const FUSE_COMPAT_SETXATTR_IN_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_setxattr_in {
    pub size: u32,
    pub flags: u32,
    pub setxattr_flags: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_getxattr_in {
    pub size: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_getxattr_out {
    pub size: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_lk_in {
    pub fh: u64,
    pub owner: u64,
    pub lk: fuse_file_lock,
    pub lk_flags: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_lk_out {
    pub lk: fuse_file_lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_access_in {
    pub mask: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_init_in {
    pub major: u32,
    pub minor: u32,
    pub max_readahead: u32,
    pub flags: u32,
    pub flags2: u32,
    pub unused: [u32; 11],
}

pub const FUSE_COMPAT_INIT_OUT_SIZE: c_int = 8;
pub const FUSE_COMPAT_22_INIT_OUT_SIZE: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_init_out {
    pub major: u32,
    pub minor: u32,
    pub max_readahead: u32,
    pub flags: u32,
    pub max_background: u16,
    pub congestion_threshold: u16,
    pub max_write: u32,
    pub time_gran: u32,
    pub max_pages: u16,
    pub map_alignment: u16,
    pub flags2: u32,
    pub max_stack_depth: u32,
    pub request_timeout: u16,
    pub unused: [u16; 11],
}

pub const CUSE_INIT_INFO_MAX: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cuse_init_in {
    pub major: u32,
    pub minor: u32,
    pub unused: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cuse_init_out {
    pub major: u32,
    pub minor: u32,
    pub unused: u32,
    pub flags: u32,
    pub max_read: u32,
    pub max_write: u32,
    pub /: *mut *mut uint32_t dev_major; / chardev major,
    pub /: *mut *mut uint32_t dev_minor; / chardev minor,
    pub spare: [u32; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_interrupt_in {
    pub unique: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_bmap_in {
    pub block: u64,
    pub blocksize: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_bmap_out {
    pub block: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_ioctl_in {
    pub fh: u64,
    pub flags: u32,
    pub cmd: u32,
    pub arg: u64,
    pub in_size: u32,
    pub out_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_ioctl_iovec {
    pub base: u64,
    pub len: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_ioctl_out {
    pub result: i32,
    pub flags: u32,
    pub in_iovs: u32,
    pub out_iovs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_poll_in {
    pub fh: u64,
    pub kh: u64,
    pub flags: u32,
    pub events: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_poll_out {
    pub revents: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_notify_poll_wakeup_out {
    pub kh: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_fallocate_in {
    pub fh: u64,
    pub offset: u64,
    pub length: u64,
    pub mode: u32,
    pub padding: u32,
}

//
// FUSE request unique ID flag
//
// Indicates whether this is a resend request. The receiver should handle this
// request accordingly.
//

//
// This value will be set by the kernel to
// (struct fuse_in_header).{uid,gid} fields in
// case when:
// - fuse daemon enabled FUSE_ALLOW_IDMAP
// - idmapping information is not available and uid/gid
// can not be mapped in accordance with an idmapping.
//
// Note: an idmapping information always available
// for inode creation operations like:
// FUSE_MKNOD, FUSE_SYMLINK, FUSE_MKDIR, FUSE_TMPFILE,
// FUSE_CREATE and FUSE_RENAME2 (with RENAME_WHITEOUT).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_in_header {
    pub len: u32,
    pub opcode: u32,
    pub unique: u64,
    pub nodeid: u64,
    pub uid: u32,
    pub gid: u32,
    pub pid: u32,
    pub /: *mut *mut uint16_t total_extlen; / length of extensions in 8byte units,
    pub padding: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_out_header {
    pub len: u32,
    pub error: i32,
    pub unique: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_dirent {
    pub ino: u64,
    pub off: u64,
    pub namelen: u32,
    pub type: u32,
    pub name: [c_char; ],
}

// Align variable length records to 64bit boundary

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_direntplus {
    pub entry_out: fuse_entry_out,
    pub dirent: fuse_dirent,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_notify_inval_inode_out {
    pub ino: u64,
    pub off: i64,
    pub len: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_notify_inval_entry_out {
    pub parent: u64,
    pub namelen: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_notify_delete_out {
    pub parent: u64,
    pub child: u64,
    pub namelen: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_notify_store_out {
    pub nodeid: u64,
    pub offset: u64,
    pub size: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_notify_retrieve_out {
    pub notify_unique: u64,
    pub nodeid: u64,
    pub offset: u64,
    pub size: u32,
    pub padding: u32,
}

// Matches the size of fuse_write_in
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_notify_retrieve_in {
    pub dummy1: u64,
    pub offset: u64,
    pub size: u32,
    pub dummy2: u32,
    pub dummy3: u64,
    pub dummy4: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_notify_prune_out {
    pub count: u32,
    pub padding: u32,
    pub spare: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_backing_map {
    pub fd: i32,
    pub flags: u32,
    pub padding: u64,
}

// Device ioctls:
pub const FUSE_DEV_IOC_MAGIC: c_int = 229;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_lseek_in {
    pub fh: u64,
    pub offset: u64,
    pub whence: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_lseek_out {
    pub offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_copy_file_range_in {
    pub fh_in: u64,
    pub off_in: u64,
    pub nodeid_out: u64,
    pub fh_out: u64,
    pub off_out: u64,
    pub len: u64,
    pub flags: u64,
}

// For FUSE_COPY_FILE_RANGE_64
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_copy_file_range_out {
    pub bytes_copied: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_setupmapping_in {
// An already open handle
    pub fh: u64,
// Offset into the file to start the mapping
    pub foffset: u64,
// Length of mapping required
    pub len: u64,
// Flags, FUSE_SETUPMAPPING_FLAG_*
    pub flags: u64,
// Offset in Memory Window
    pub moffset: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_removemapping_in {
// number of fuse_removemapping_one follows
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_removemapping_one {
// Offset into the dax window start the unmapping
    pub moffset: u64,
// Length of mapping required
    pub len: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_syncfs_in {
    pub padding: u64,
}

//
// For each security context, send fuse_secctx with size of security context
// fuse_secctx will be followed by security context name and this in turn
// will be followed by actual context label.
// fuse_secctx, name, context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_secctx {
    pub size: u32,
    pub padding: u32,
}

//
// Contains the information about how many fuse_secctx structures are being
// sent and what's the total size of all security contexts (including
// size of fuse_secctx_header).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_secctx_header {
    pub size: u32,
    pub nr_secctx: u32,
}

//
// struct fuse_ext_header - extension header
// @size: total size of this extension including this header
// @type: type of extension
//
// This is made compatible with fuse_secctx_header by using type values >
// FUSE_MAX_NR_SECCTX
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_ext_header {
    pub size: u32,
    pub type: u32,
}

//
// struct fuse_supp_groups - Supplementary group extension
// @nr_groups: number of supplementary groups
// @groups: flexible array of group IDs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_supp_groups {
    pub nr_groups: u32,
    pub groups: [u32; ],
}

//
// Size of the ring buffer header
//
pub const FUSE_URING_IN_OUT_HEADER_SZ: c_int = 128;
pub const FUSE_URING_OP_IN_OUT_SZ: c_int = 128;
//
// fuse_uring_ent_in_out flags
//
// FUSE_URING_ENT_ZERO_COPY: Set if the ent's payload is zero-copied
//

// Used as part of the fuse_uring_req_header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_uring_ent_in_out {
    pub flags: u64,
//
// commit ID to be used in a reply to a ring request (see also
// struct fuse_uring_cmd_req)
//
    pub commit_id: u64,
// size of user payload buffer
    pub payload_sz: u32,
// Offset into the bufpool, if bufpools are used
    pub offset: u32,
    pub reserved: u64,
}

//
// Header for all fuse-io-uring requests
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_uring_req_header {
// struct fuse_in_header / struct fuse_out_header
    pub in_out: [c_char; FUSE_URING_IN_OUT_HEADER_SZ],
// per op code header
    pub op_in: [c_char; FUSE_URING_OP_IN_OUT_SZ],
    pub ring_ent_in_out: fuse_uring_ent_in_out,
}

//
// sqe commands to the kernel
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fuse_uring_cmd {
    FUSE_IO_URING_CMD_INVALID = 0,

// register the request buffer and fetch a fuse request
    FUSE_IO_URING_CMD_REGISTER = 1,

// commit fuse request result and fetch next request
    FUSE_IO_URING_CMD_COMMIT_AND_FETCH = 2,

// add a queue
    FUSE_IO_URING_CMD_ADD_QUEUE = 3,

// add a bufpool to a queue
    FUSE_IO_URING_CMD_ADD_BUFPOOL = 4,
}

//
// fuse_uring_cmd_req flags for FUSE_IO_URING_CMD_ADD_QUEUE
//
// FUSE_URING_ZERO_COPY is only supported for queues with bufpools on privileged
// servers
//

//
// In the 80B command area of the SQE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_uring_cmd_req {
    pub flags: u64,
// entry identifier for commits
    pub commit_id: u64,
// queue the command is for (queue index)
    pub qid: u16,
    pub padding: [u8; 6],
// base address of bufpool
    pub uaddr: u64,
    pub len: u32,
    pub reserved: u32,
    pub bufpool: },
//
// Index of this entry's slot in the server's io_uring
// registered buffer table, where the kernel registers the
// request's pages for zero-copy. Set for
// FUSE_IO_URING_CMD_REGISTER cmds only, and only on queues
// created with FUSE_URING_ZERO_COPY. On a non-zero-copy queue
// this must be 0
//
    pub ent_zero_copy_buf_index: u16,
}
