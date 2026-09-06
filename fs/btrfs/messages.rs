//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/messages.h
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
// We want to be able to override this in btrfs-progs.
//

extern "C" {
    pub fn _btrfs_printk(fs_info: *const btrfs_fs_info, level: c_uint, fmt: *const c_char, ...);
}

//
// Print a message with filesystem info, enclosed in RCU protection.
//

//
// Wrappers that use a ratelimited printk
//

// When printk() is no_printk(), expand to no-op.

// Stub to verify the assertion format string.
// Take the first token if any.

//
// Skip the first token and return the rest, if it's empty the comma is dropped.
// As ##__VA_ARGS__ cannot be at the beginning of the macro the __VA_OPT__ is needed
// and supported since GCC 8 and Clang 12.
//

//
// Assertion with optional printk() format.
//
// Accepted syntax:
// ASSERT(condition);
// ASSERT(condition, "string");
// ASSERT(condition, "variable=%d", variable);
//
// How it works:
// - if there's no format string, ""[0] evaluates at compile time to 0 and the
// true branch is executed
// - any non-empty format string with the "" prefix evaluates to != 0 at
// compile time and the false branch is executed
// - stringified condition is printed as %s so we don't accidentally mix format
// strings (the % operator)
// - there can be only one printk() call, so the format strings and arguments are
// spliced together:
// DEFAULT_FMT [USER_FMT], DEFAULT_ARGS [, USER_ARGS]
// - comma between DEFAULT_ARGS and USER_ARGS is handled by preprocessor
// (requires __VA_OPT__ support)
// - otherwise we could use __VA_OPT(,) __VA_ARGS__ for the 2nd+ argument of args,
//

// Compile check the @cond expression but don't generate any code.

// Verbose warning only under debug build.

extern "C" {
    pub fn btrfs_decode_error(error: c_int) -> *const char  __attribute_const__;
}

//
// If BTRFS_MOUNT_PANIC_ON_FATAL_ERROR is in mount_opt, __btrfs_panic
// will panic().  Otherwise we BUG() here.
//

//
// The warning threshold is 5/8th of the MAX_LFS_FILESIZE that limits the logical
// addresses of extents.
//
// For 4K page size it's about 10T, for 64K it's 160T.
//

extern "C" {
    pub fn btrfs_warn_32bit_limit(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_err_32bit_limit(fs_info: *mut btrfs_fs_info);
}

