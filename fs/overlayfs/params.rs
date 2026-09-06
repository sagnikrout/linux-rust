//! Automatically rewritten from C Header to Rust Module
//! Source: fs/overlayfs/params.h
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

// The set of options that user requested explicitly via mount options
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovl_opt_set {
    pub metacopy: bool,
    pub redirect: bool,
    pub nfs_export: bool,
    pub index: bool,
}

pub const OVL_MAX_STACK: c_int = 500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovl_fs_context_layer {
    pub name: *mut c_char,
    pub path: path,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovl_fs_context {
    pub upper: path,
    pub work: path,
    pub capacity: usize,
    pub /: *mut *mut size_t nr; / includes nr_data,
    pub nr_data: usize,
    pub set: ovl_opt_set,
    pub lower: *mut ovl_fs_context_layer,
    pub /: *mut *mut *mut char lowerdir_all; / user provided lowerdir string,
    pub casefold_set: bool,
}

extern "C" {
    pub fn ovl_init_fs_context(fc: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn ovl_free_fs(ofs: *mut ovl_fs);
}
extern "C" {
    pub fn ovl_show_options(m: *mut seq_file, dentry: *mut dentry) -> c_int;
}
