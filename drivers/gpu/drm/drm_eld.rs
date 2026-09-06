//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/drm_eld.c
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
// Copyright © 2023 Intel Corporation
//

//
// drm_eld_sad_get - get SAD from ELD to struct cea_sad
// @eld: ELD buffer
// @sad_index: SAD index
// @cta_sad: destination struct cea_sad
//
// @return: 0 on success, or negative on errors
//
#[no_mangle]
pub unsafe extern "C" fn drm_eld_sad_get(eld: *const u8, sad_index: c_int, cta_sad: *mut cea_sad) -> c_int {
    int drm_eld_sad_get(const u8 *eld, int sad_index, struct cea_sad *cta_sad)
    {
    const u8 *sad;
    if (sad_index >= drm_eld_sad_count(eld))
    return -EINVAL;
    sad = eld + DRM_ELD_CEA_SAD(drm_eld_mnl(eld), sad_index);
    drm_edid_cta_sad_set(cta_sad, sad);
    return 0;
    }
    EXPORT_SYMBOL(drm_eld_sad_get);
//
// drm_eld_sad_set - set SAD to ELD from struct cea_sad
// @eld: ELD buffer
// @sad_index: SAD index
// @cta_sad: source struct cea_sad
//
// @return: 0 on success, or negative on errors
//
#[no_mangle]
pub unsafe extern "C" fn drm_eld_sad_set(eld: *mut u8, sad_index: c_int, cta_sad: *const cea_sad) -> c_int {
    int drm_eld_sad_set(u8 *eld, int sad_index, const struct cea_sad *cta_sad)
    {
    u8 *sad;
    if (sad_index >= drm_eld_sad_count(eld))
    return -EINVAL;
    sad = eld + DRM_ELD_CEA_SAD(drm_eld_mnl(eld), sad_index);
    drm_edid_cta_sad_get(cta_sad, sad);
    return 0;
    }
    EXPORT_SYMBOL(drm_eld_sad_set);
