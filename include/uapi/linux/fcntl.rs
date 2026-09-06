//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/fcntl.h
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
// Request nofications on a directory.
// See below for events that may be notified.
//

// Was the file just created?

//
// Cancel a blocking posix lock; internal use only until we expose an
// asynchronous lock api to userspace:
//

// Create a file descriptor with FD_CLOEXEC set.

//
// Set and get of pipe page size array
//

//
// Set/Get seals
//

//
// Types of seals
//
pub const F_SEAL_SEAL: c_uint = 0x0001	/* prevent further seals from being set */;
pub const F_SEAL_SHRINK: c_uint = 0x0002	/* prevent file from shrinking */;
pub const F_SEAL_GROW: c_uint = 0x0004	/* prevent file from growing */;
pub const F_SEAL_WRITE: c_uint = 0x0008	/* prevent writes */;
pub const F_SEAL_FUTURE_WRITE: c_uint = 0x0010  /* prevent future writes while mapped */;
pub const F_SEAL_EXEC: c_uint = 0x0020  /* prevent chmod modifying exec bits */;
// (1U << 31) is reserved for signed error codes
//
// Set/Get write life time hints. {GET,SET}_RW_HINT operate on the
// underlying inode, while {GET,SET}_FILE_RW_HINT operate only on
// the specific file.
//

//
// Valid hint values for F_{GET,SET}_RW_HINT. 0 is "not set", or can be
// used to clear any hints previously set.
//
pub const RWH_WRITE_LIFE_NOT_SET: c_int = 0;
pub const RWH_WRITE_LIFE_NONE: c_int = 1;
pub const RWH_WRITE_LIFE_SHORT: c_int = 2;
pub const RWH_WRITE_LIFE_MEDIUM: c_int = 3;
pub const RWH_WRITE_LIFE_LONG: c_int = 4;
pub const RWH_WRITE_LIFE_EXTREME: c_int = 5;
//
// The originally introduced spelling is remained from the first
// versions of the patch set that introduced the feature, see commit
// v4.13-rc1~212^2~51.
//

// Set/Get delegations

// Argument structure for F_GETDELEG and F_SETDELEG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delegation {
    pub /: *mut *mut __u32 d_flags; / Must be 0,
    pub /: *mut *mut __u16 d_type; / F_RDLCK, F_WRLCK, F_UNLCK,
    pub /: *mut *mut __u16 __pad; / Must be 0,
}

//
// Types of directory notifications that may be requested.
//
pub const DN_ACCESS: c_uint = 0x00000001	/* File accessed */;
pub const DN_MODIFY: c_uint = 0x00000002	/* File modified */;
pub const DN_CREATE: c_uint = 0x00000004	/* File created */;
pub const DN_DELETE: c_uint = 0x00000008	/* File removed */;
pub const DN_RENAME: c_uint = 0x00000010	/* File renamed */;
pub const DN_ATTRIB: c_uint = 0x00000020	/* File changed attibutes */;
pub const DN_MULTISHOT: c_uint = 0x80000000	/* Don't remove notifier */;
// Reserved kernel ranges [-100], [-10000, -40000].

//
// The concept of process and threads in userland and the kernel is a confusing
// one - within the kernel every thread is a 'task' with its own individual PID,
// however from userland's point of view threads are grouped by a single PID,
// which is that of the 'thread group leader', typically the first thread
// spawned.
//
// To cut the Gideon knot, for internal kernel usage, we refer to
// PIDFD_SELF_THREAD to refer to the current thread (or task from a kernel
// perspective), and PIDFD_SELF_THREAD_GROUP to refer to the current thread
// group leader...
//

// Generic flags for the *at(2) family of syscalls.
// Reserved for per-syscall flags	0xff.
pub const AT_SYMLINK_NOFOLLOW: c_uint = 0x100   /* Do not follow symbolic;
// Reserved for per-syscall flags	0x200
pub const AT_SYMLINK_FOLLOW: c_uint = 0x400   /* Follow symbolic links. */;
pub const AT_NO_AUTOMOUNT: c_uint = 0x800	/* Suppress terminal automount;
pub const AT_EMPTY_PATH: c_uint = 0x1000	/* Allow empty relative;
//
// These flags are currently statx(2)-specific, but they could be made generic
// in the future and so they should not be used for other per-syscall flags.
//
pub const AT_STATX_SYNC_TYPE: c_uint = 0x6000	/* Type of synchronisation required from statx() */;
pub const AT_STATX_SYNC_AS_STAT: c_uint = 0x0000	/* - Do whatever stat() does */;
pub const AT_STATX_FORCE_SYNC: c_uint = 0x2000	/* - Force the attributes to be sync'd with the server */;
pub const AT_STATX_DONT_SYNC: c_uint = 0x4000	/* - Don't sync attributes with the server */;
pub const AT_RECURSIVE: c_uint = 0x8000	/* Apply to the entire subtree */;
//
// Per-syscall flags for the *at(2) family of syscalls.
//
// These are flags that are so syscall-specific that a user passing these flags
// to the wrong syscall is so "clearly wrong" that we can safely call such
// usage "undefined behaviour".
//
// For example, the constants AT_REMOVEDIR and AT_EACCESS have the same value.
// AT_EACCESS is meaningful only to faccessat, while AT_REMOVEDIR is meaningful
// only to unlinkat. The two functions do completely different things and
// therefore, the flags can be allowed to overlap. For example, passing
// AT_REMOVEDIR to faccessat would be undefined behavior and thus treating it
// equivalent to AT_EACCESS is valid undefined behavior.
//
// Note for implementers: When picking a new per-syscall AT_* flag, try to
// reuse already existing flags first. This leaves us with as many unused bits
// as possible, so we can use them for generic bits in the future if necessary.
//
// Flags for renameat2(2) (must match legacy RENAME_* flags).
pub const AT_RENAME_NOREPLACE: c_uint = 0x0001;
pub const AT_RENAME_EXCHANGE: c_uint = 0x0002;
pub const AT_RENAME_WHITEOUT: c_uint = 0x0004;
// Flag for faccessat(2).
pub const AT_EACCESS: c_uint = 0x200	/* Test access permitted for;
// Flag for unlinkat(2).
pub const AT_REMOVEDIR: c_uint = 0x200   /* Remove directory instead of;
// Flags for name_to_handle_at(2).
pub const AT_HANDLE_FID: c_uint = 0x200	/* File handle is needed to compare;
pub const AT_HANDLE_MNT_ID_UNIQUE: c_uint = 0x001	/* Return the u64 unique mount ID. */;
pub const AT_HANDLE_CONNECTABLE: c_uint = 0x002	/* Request a connectable file handle */;
// Flags for execveat2(2).
pub const AT_EXECVE_CHECK: c_uint = 0x10000	/* Only perform a check if execution;
