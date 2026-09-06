//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/api/io_dir.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// Lightweight directory reading library.
//

pub const SYS_getdents64: c_int = 217;

pub const SYS_getdents64: c_int = 220;

pub const SYS_getdents64: c_int = 377;

pub const SYS_getdents64: c_int = 308;

pub const SYS_getdents64: c_int = 202;

pub const SYS_getdents64: c_int = 154;

pub const SYS_getdents64: c_int = 60;

pub const SYS_getdents64: c_int = 61;

extern "C" {
    pub fn syscall(_arg: SYS_getdents64, _arg: fd, _arg: dirp, _arg: count) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_dirent64 {
    pub /: *mut *mut ino64_t d_ino; / 64-bit inode number,
    pub /: *mut *mut off64_t d_off; / 64-bit offset to next structure,
    pub /: *mut *mut unsigned short d_reclen; / Size of this dirent,
    pub /: *mut *mut unsigned char d_type; / File type,
    pub /: *mut *mut char d_name[NAME_MAX + 1]; / Filename (null-terminated),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_dir {
    pub dirfd: c_int,
    pub available_bytes: isize,
    pub next: *mut io_dirent64,
    pub buff: [io_dirent64; 4],
}
