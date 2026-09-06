//! Automatically rewritten from C to Rust
//! Source: drivers/video/nomodeset.c
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

    static bool video_nomodeset;
#[no_mangle]
pub unsafe extern "C" fn video_firmware_drivers_only() -> bool {
    bool video_firmware_drivers_only(void)
    {
    return video_nomodeset;
    }
    EXPORT_SYMBOL(video_firmware_drivers_only);
#[no_mangle]
unsafe extern "C" fn disable_modeset(str: *mut c_char) -> int __init {
    static int __init disable_modeset(char *str)
    {
    video_nomodeset = true;
    pr_warn("Booted with the nomodeset parameter. Only the system framebuffer will be available\n");
    return 1;
    }
// Disable kernel modesetting
    __setup("nomodeset", disable_modeset);
