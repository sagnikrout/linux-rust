//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/indexer/chapter-index.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2023 Red Hat
//

//
// A chapter index for an open chapter is a mutable structure that tracks all the records that have
// been added to the chapter. A chapter index for a closed chapter is similar except that it is
// immutable because the contents of a closed chapter can never change, and the immutable structure
// is more efficient. Both types of chapter index are implemented with a delta index.
//
// The value returned when no entry is found in the chapter index.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct open_chapter_index {
    pub geometry: *const index_geometry,
    pub delta_index: delta_index,
    pub virtual_chapter_number: u64,
    pub volume_nonce: u64,
    pub memory_size: usize,
}

extern "C" {
    pub fn uds_free_open_chapter_index(chapter_index: *mut open_chapter_index);
}
