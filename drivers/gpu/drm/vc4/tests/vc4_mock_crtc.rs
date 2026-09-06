//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/vc4/tests/vc4_mock_crtc.c
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

    static const struct drm_crtc_helper_funcs vc4_dummy_crtc_helper_funcs = {
    .atomic_check	= vc4_crtc_atomic_check,
    };
    static const struct drm_crtc_funcs vc4_dummy_crtc_funcs = {
    .atomic_destroy_state	= vc4_crtc_destroy_state,
    .atomic_duplicate_state	= vc4_crtc_duplicate_state,
    .reset			= vc4_crtc_reset,
    };
    struct vc4_dummy_crtc *vc4_mock_pv(struct kunit *test,
    struct drm_device *drm,
    struct drm_plane *plane,
    const struct vc4_crtc_data *data)
    {
    struct vc4_dummy_crtc *dummy_crtc;
    struct vc4_crtc *vc4_crtc;
    int ret;
    dummy_crtc = drmm_kzalloc(drm, sizeof(*dummy_crtc), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, dummy_crtc);
    vc4_crtc = &dummy_crtc.crtc;
    ret = __vc4_crtc_init(drm, core::ptr::null_mut(),
    vc4_crtc, data, plane,
    &vc4_dummy_crtc_funcs,
    &vc4_dummy_crtc_helper_funcs,
    false);
    KUNIT_ASSERT_EQ(test, ret, 0);
    return dummy_crtc;
    }
