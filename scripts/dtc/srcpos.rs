//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/dtc/srcpos.h
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
// Copyright 2007 Jon Loeliger, Freescale Semiconductor, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srcfile_state {
    pub f: *mut FILE,
    pub name: *mut c_char,
    pub dir: *mut c_char,
    pub colno: int lineno,,
    pub prev: *mut srcfile_state,
}

//
// Open a source file.
//
// If the source file is a relative pathname, then it is searched for in the
// current directory (the directory of the last source file read) and after
// that in the search path.
//
// We work through the search path in order from the first path specified to
// the last.
//
// If the file is not found, then this function does not return, but calls
// die().
//
// @param fname		Filename to search
// @param fullnamep	If non-NULL, it is set to the allocated filename of the
// file that was opened. The caller is then responsible
// for freeing the pointer.
// @return pointer to opened FILE
//
extern "C" {
    pub fn srcfile_push(fname: *const c_char);
}
extern "C" {
    pub fn srcfile_pop() -> bool;
}
//
// Add a new directory to the search path for input files
//
// The new path is added at the end of the list.
//
// @param dirname	Directory to add
//
extern "C" {
    pub fn srcfile_add_search_path(dirname: *const c_char);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srcpos {
    pub first_line: c_int,
    pub first_column: c_int,
    pub last_line: c_int,
    pub last_column: c_int,
    pub file: *mut srcfile_state,
    pub next: *mut srcpos,
}

extern "C" {
    pub fn srcpos_update(pos: *mut srcpos, text: *const c_char, len: c_int);
}
extern "C" {
    pub fn srcpos_free(pos: *mut srcpos);
}
extern "C" {
    pub fn srcpos_set_line(f: *mut c_char, l: c_int);
}
