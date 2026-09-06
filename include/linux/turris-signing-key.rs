//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/turris-signing-key.h
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
// 2025 by Marek Behún <kabel@kernel.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct turris_signing_key_subtype {
    pub key_size: u16,
    pub data_size: u8,
    pub sig_size: u8,
    pub public_key_size: u8,
    pub hash_algo: *const c_char,
    pub key): *const *const *const void (get_public_key)(struct key,
    pub signature): *const *const *const *const int (sign)(struct key key, void msg, void,
}

