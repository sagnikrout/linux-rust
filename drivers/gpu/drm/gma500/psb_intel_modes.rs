//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/gma500/psb_intel_modes.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2007 Intel Corporation
//
// Authers: Jesse Barnes <jesse.barnes@intel.com>
//

//
// psb_intel_ddc_get_modes - get modelist from monitor
// @connector: DRM connector device to use
// @adapter:   Associated I2C adaptor
//
// Fetch the EDID information from @connector using the DDC bus.
//
    int psb_intel_ddc_get_modes(struct drm_connector *connector,
    struct i2c_adapter *adapter)
    {
    struct edid *edid;
    let mut ret: c_int = 0;
    edid = drm_get_edid(connector, adapter);
    if (edid) {
    drm_connector_update_edid_property(connector, edid);
    ret = drm_add_edid_modes(connector, edid);
    kfree(edid);
    }
    return ret;
    }
