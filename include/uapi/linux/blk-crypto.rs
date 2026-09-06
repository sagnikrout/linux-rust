//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/blk-crypto.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_crypto_import_key_arg {
// Raw key (input)
    pub raw_key_ptr: __u64,
    pub raw_key_size: __u64,
// Long-term wrapped key blob (output)
    pub lt_key_ptr: __u64,
    pub lt_key_size: __u64,
    pub reserved: [__u64; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_crypto_generate_key_arg {
// Long-term wrapped key blob (output)
    pub lt_key_ptr: __u64,
    pub lt_key_size: __u64,
    pub reserved: [__u64; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_crypto_prepare_key_arg {
// Long-term wrapped key blob (input)
    pub lt_key_ptr: __u64,
    pub lt_key_size: __u64,
// Ephemerally-wrapped key blob (output)
    pub eph_key_ptr: __u64,
    pub eph_key_size: __u64,
    pub reserved: [__u64; 4],
}

//
// These ioctls share the block device ioctl space; see uapi/linux/fs.h.
// 140-141 are reserved for future blk-crypto ioctls; any more than that would
// require an additional allocation from the block device ioctl space.
//

