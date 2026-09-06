//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/debug.h
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


//
// Copyright (c) 2012 Broadcom Corporation
// Copyright (c) 2012 Canonical Ltd.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

extern "C" {
    pub fn __brcms_info(dev: *mut device, fmt: *const c_char, ...);
}
extern "C" {
    pub fn __brcms_warn(dev: *mut device, fmt: *const c_char, ...);
}
extern "C" {
    pub fn __brcms_err(dev: *mut device, fmt: *const c_char, ...);
}
extern "C" {
    pub fn __brcms_crit(dev: *mut device, fmt: *const c_char, ...);
}

//
// Debug macros cannot be used when wlc is uninitialized. Generally
// this means any code that could run before brcms_c_attach() has
// returned successfully probably shouldn't use the following macros.
//

extern "C" {
    pub fn brcms_debugfs_init();
}
extern "C" {
    pub fn brcms_debugfs_exit();
}
extern "C" {
    pub fn brcms_debugfs_attach(drvr: *mut brcms_pub);
}
extern "C" {
    pub fn brcms_debugfs_detach(drvr: *mut brcms_pub);
}
extern "C" {
    pub fn brcms_debugfs_create_files(drvr: *mut brcms_pub);
}
