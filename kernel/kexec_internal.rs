//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/kexec_internal.h
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
    pub fn sanity_check_segment_list(image: *mut kimage) -> c_int;
}
extern "C" {
    pub fn kimage_free_page_list(list: *mut list_head);
}
extern "C" {
    pub fn kimage_free(image: *mut kimage);
}
extern "C" {
    pub fn kimage_load_segment(image: *mut kimage, idx: c_int) -> c_int;
}
extern "C" {
    pub fn kimage_terminate(image: *mut kimage);
}
//
// Whatever is used to serialize accesses to the kexec_crash_image needs to be
// NMI safe, as __crash_kexec() can happen during nmi_panic(), so here we use a
// "simple" atomic variable that is acquired with a cmpxchg().
//
extern "C" {
    pub fn atomic_try_cmpxchg_acquire(_arg: &__kexec_lock, _arg: &old, _arg: 1) -> return;
}

extern "C" {
    pub fn kimage_file_post_load_cleanup(image: *mut kimage);
}

extern "C" {
    pub fn kho_fill_kimage(image: *mut kimage) -> c_int;
}

