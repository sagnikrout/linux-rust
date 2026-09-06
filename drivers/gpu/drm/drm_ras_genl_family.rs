//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/drm_ras_genl_family.c
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
//
// Copyright © 2026 Intel Corporation
//

// Track family registration so the drm_exit can be called at any time
    static bool registered;
//
// drm_ras_genl_family_register() - Register drm-ras genl family
//
// Only to be called one at drm_drv_init()
//
#[no_mangle]
pub unsafe extern "C" fn drm_ras_genl_family_register() -> c_int {
    int drm_ras_genl_family_register(void)
    {
    int ret;
    registered = false;
    ret = genl_register_family(&drm_ras_nl_family);
    if (ret)
    return ret;
    registered = true;
    return 0;
    }
//
// drm_ras_genl_family_unregister() - Unregister drm-ras genl family
//
// To be called one at drm_drv_exit() at any moment, but only once.
//
#[no_mangle]
pub unsafe extern "C" fn drm_ras_genl_family_unregister() {
    void drm_ras_genl_family_unregister(void)
    {
    if (registered) {
    genl_unregister_family(&drm_ras_nl_family);
    registered = false;
    }
    }
