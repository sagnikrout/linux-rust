//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nilfs2/page.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Buffer/page management specific to NILFS
//
// Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
//
// Written by Ryusuke Konishi and Seiji Kihara.
//

//
// Extended buffer state bits
//
extern "C" {
    pub fn __nilfs_clear_folio_dirty(: *mut folio);
}
extern "C" {
    pub fn nilfs_forget_buffer(: *mut buffer_head);
}
extern "C" {
    pub fn nilfs_copy_buffer(: *mut buffer_head, : *mut buffer_head);
}
extern "C" {
    pub fn nilfs_folio_buffers_clean(: *mut folio) -> bool;
}
extern "C" {
    pub fn nilfs_folio_bug(: *mut folio);
}
extern "C" {
    pub fn nilfs_copy_dirty_pages(: *mut address_space, : *mut address_space) -> c_int;
}
extern "C" {
    pub fn nilfs_copy_back_pages(: *mut address_space, : *mut address_space);
}
extern "C" {
    pub fn nilfs_clear_folio_dirty(folio: *mut folio);
}
extern "C" {
    pub fn nilfs_clear_dirty_pages(mapping: *mut address_space);
}

