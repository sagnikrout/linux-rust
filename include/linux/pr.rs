//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pr.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pr_keys {
    pub generation: u32,
    pub num_keys: u32,
    pub keys: [u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pr_held_reservation {
    pub key: u64,
    pub generation: u32,
    pub type: pr_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pr_ops {
    pub flags): u32,
    pub flags): pr_type type, u32,
    pub type): pr_type,
    pub abort): pr_type type, bool,
    pub key): *mut *mut *mut int (pr_clear)(struct block_device bdev, u64,
//
// pr_read_keys - Read the registered keys and return them in the
// pr_keys->keys array. The keys array will have been allocated at the
// end of the pr_keys struct, and pr_keys->num_keys must be set to the
// number of keys the array can hold. If there are more than can fit
// in the array, success will still be returned and pr_keys->num_keys
// will reflect the total number of keys the device contains, so the
// caller can retry with a larger array.
//
    pub keys_info): *mut pr_keys,
    pub rsv): *mut pr_held_reservation,
}
