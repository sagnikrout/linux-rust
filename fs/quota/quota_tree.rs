//! Automatically rewritten from C Header to Rust Module
//! Source: fs/quota/quota_tree.h
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
// Definitions of structures for vfsv0 quota format
//

//
// Structure of header of block with quota structures. It is padded to 16 bytes so
// there will be space for exactly 21 quota-entries in a block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qt_disk_dqdbheader {
    pub /: *mut *mut __le32 dqdh_next_free; / Number of next block with free entry,
    pub /: *mut *mut __le32 dqdh_prev_free; / Number of previous block with free entry,
    pub /: *mut *mut __le16 dqdh_entries; / Number of valid entries in block,
    pub dqdh_pad1: __le16,
    pub dqdh_pad2: __le32,
}

