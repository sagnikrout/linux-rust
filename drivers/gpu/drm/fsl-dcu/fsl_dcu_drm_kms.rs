//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/fsl-dcu/fsl_dcu_drm_kms.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2015 Freescale Semiconductor, Inc.
//
// Freescale DCU drm device driver
//

    static const struct drm_mode_config_funcs fsl_dcu_drm_mode_config_funcs = {
    .atomic_check = drm_atomic_helper_check,
    .atomic_commit = drm_atomic_helper_commit,
    .fb_create = drm_gem_fb_create,
    };
#[no_mangle]
pub unsafe extern "C" fn fsl_dcu_drm_modeset_init(fsl_dev: *mut fsl_dcu_drm_device) -> c_int {
    int fsl_dcu_drm_modeset_init(struct fsl_dcu_drm_device *fsl_dev)
    {
    int ret;
    drm_mode_config_init(fsl_dev.drm);
    fsl_dev.drm.mode_config.min_width = 0;
    fsl_dev.drm.mode_config.min_height = 0;
    fsl_dev.drm.mode_config.max_width = 2031;
    fsl_dev.drm.mode_config.max_height = 2047;
    fsl_dev.drm.mode_config.funcs = &fsl_dcu_drm_mode_config_funcs;
    ret = fsl_dcu_drm_crtc_create(fsl_dev);
    if (ret)
    goto err;
    ret = fsl_dcu_drm_encoder_create(fsl_dev, &fsl_dev.crtc);
    if (ret)
    goto err;
    ret = fsl_dcu_create_outputs(fsl_dev);
    if (ret)
    goto err;
    drm_mode_config_reset(fsl_dev.drm);
    drm_kms_helper_poll_init(fsl_dev.drm);
    return 0;
    err:
    drm_mode_config_cleanup(fsl_dev.drm);
    return ret;
    }
