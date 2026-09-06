//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/highuid.h
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


// SPDX-License-Identifier: GPL-2.0

//
// general notes:
//
// CONFIG_UID16 is defined if the given architecture needs to
// support backwards compatibility for old system calls.
//
// kernel code should use uid_t and gid_t at all times when dealing with
// kernel-private data.
//
// old_uid_t and old_gid_t should only be different if CONFIG_UID16 is
// defined, else the platform should provide dummy typedefs for them
// such that they are equivalent to __kernel_{u,g}id_t.
//
// uid16_t and gid16_t are used on all architectures. (when dealing
// with structures hard coded to 16 bits, such as in filesystems)
//
// This is the "overflow" UID and GID. They are used to signify uid/gid
// overflow to old programs when they request uid/gid information but are
// using the old 16 bit interfaces.
// When you run a libc5 program, it will think that all highuid files or
// processes are owned by this uid/gid.
// The idea is that it's better to do so than possibly return 0 in lieu of
// 65536, etc.
//
extern "C" {
    pub fn __bad_uid();
}
extern "C" {
    pub fn __bad_gid();
}
pub const DEFAULT_OVERFLOWUID: c_int = 65534;
pub const DEFAULT_OVERFLOWGID: c_int = 65534;

// prevent uid mod 65536 effect by returning a default value for high UIDs

//
// -1 is different in 16 bits than it is in 32 bits
// these macros are used by chown(), setreuid(), ...,
//

// uid/gid input should be always 32bit uid_t

//
// Everything below this line is needed on all architectures, to deal with
// filesystems that only store 16 bits of the UID/GID, etc.
//
// This is the UID and GID that will get written to disk if a filesystem
// only supports 16-bit UIDs and the kernel has a high UID/GID to write
//
pub const DEFAULT_FS_OVERFLOWUID: c_int = 65534;
pub const DEFAULT_FS_OVERFLOWGID: c_int = 65534;
//
// Since these macros are used in architectures that only need limited
// 16-bit UID back compatibility, we won't use old_uid_t and old_gid_t
//

