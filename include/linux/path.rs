//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/path.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct path {
    pub mnt: *mut vfsmount,
    pub dentry: *mut dentry,
    pub __randomize_layout: },
    pub ): *const extern void path_get(struct path,
    pub ): *const extern void path_put(struct path,
    pub path2->dentry: return path1->mnt == path2->mnt && path1->dentry ==,
//
// Cleanup macro for use with __free(path_put). Avoids dereference and
// copying @path unlike DEFINE_FREE(). path_put() will handle the empty
// path correctly just ensure @path is initialized:
//
// struct path path __free(path_put) = {};
//

