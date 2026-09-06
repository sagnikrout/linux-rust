//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/core/fb_internal.h
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

// fb_devfs.c

extern "C" {
    pub fn fb_register_chrdev() -> c_int;
}
extern "C" {
    pub fn fb_unregister_chrdev();
}

// fb_logo.c

extern "C" {
    pub fn fb_prepare_logo(fb_info: *mut fb_info, rotate: c_int) -> c_int;
}
extern "C" {
    pub fn fb_show_logo(fb_info: *mut fb_info, rotate: c_int) -> c_int;
}

// fbmem.c
extern "C" {
    pub fn put_fb_info(fb_info: *mut fb_info);
}
extern "C" {
    pub fn fb_blank_from_user(info: *mut fb_info, blank: c_int) -> c_int;
}
// fb_procfs.c

extern "C" {
    pub fn fb_init_procfs() -> c_int;
}
extern "C" {
    pub fn fb_cleanup_procfs();
}

// fbsysfs.c

extern "C" {
    pub fn fb_device_create(fb_info: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn fb_device_destroy(fb_info: *mut fb_info);
}

//
// Acquire a reference on the parent device to avoid
// unplug operations behind our back. With the fbdev
// device enabled, this is performed within register_device().
//
// Undo the get_device() from fb_device_create()

