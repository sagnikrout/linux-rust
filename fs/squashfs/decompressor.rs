//! Automatically rewritten from C Header to Rust Module
//! Source: fs/squashfs/decompressor.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Squashfs - a compressed read only filesystem for Linux
//
// Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009
// Phillip Lougher <phillip@squashfs.org.uk>
//
// decompressor.h
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_decompressor {
    pub ): *mut *mut *mut *mut void (init)(struct squashfs_sb_info , void,
    pub int): *mut *mut *mut *mut *mut void (comp_opts)(struct squashfs_sb_info , void ,,
    pub ): *mut *mut void (free)(void,
    pub ): *mut *mut bio , int, int, squashfs_page_actor,
    pub id: c_int,
    pub name: *mut c_char,
    pub alloc_buffer: c_int,
    pub supported: c_int,
}

