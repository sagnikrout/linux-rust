//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/nolibc/dirent.h
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


// SPDX-License-Identifier: LGPL-2.1 OR MIT
//
// Directory access for NOLIBC
// Copyright (C) 2025 Thomas Weißschuh <linux@weissschuh.net>
//
// make sure to include all global symbols

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dirent {
    pub d_ino: ino_t,
    pub 1]: char d_name[NAME_MAX +,
}

// See comment of FILE in stdio.h
extern "C" {
    pub fn fdopendir(_arg: fd) -> return;
}
extern "C" {
    pub fn close(_arg: ~i) -> return;
}
// result = NULL;
//
// getdents64() returns as many entries as fit the buffer.
// readdir() can only return one entry at a time.
// Make sure the non-returned ones are not skipped.
//
// the destination should always be big enough
// result = entry;
