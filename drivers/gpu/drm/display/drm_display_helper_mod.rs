//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/display/drm_display_helper_mod.c
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


// SPDX-License-Identifier: MIT

    MODULE_DESCRIPTION("DRM display adapter helper");
    MODULE_LICENSE("GPL and additional rights");
#[no_mangle]
unsafe extern "C" fn drm_display_helper_module_init() -> int __init {
    static int __init drm_display_helper_module_init(void)
    {
    return drm_dp_aux_dev_init();
    }
#[no_mangle]
unsafe extern "C" fn drm_display_helper_module_exit() -> void __exit {
    static void __exit drm_display_helper_module_exit(void)
    {
// Call exit functions from specific dp helpers here
    drm_dp_aux_dev_exit();
    }
    module_init(drm_display_helper_module_init);
    module_exit(drm_display_helper_module_exit);
