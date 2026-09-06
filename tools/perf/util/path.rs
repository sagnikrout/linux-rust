//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/path.h
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

extern "C" {
    pub fn path__join(bf: *mut c_char, size: usize, path1: *const c_char, path2: *const c_char) -> c_int;
}
extern "C" {
    pub fn path__join3(bf: *mut c_char, size: usize, path1: *const c_char, path2: *const c_char, path3: *const c_char) -> c_int;
}
extern "C" {
    pub fn is_regular_file(file: *const c_char) -> bool;
}
extern "C" {
    pub fn is_directory(base_path: *const c_char, dent: *const dirent) -> bool;
}
extern "C" {
    pub fn is_directory_at(dir_fd: c_int, path: *const c_char) -> bool;
}
