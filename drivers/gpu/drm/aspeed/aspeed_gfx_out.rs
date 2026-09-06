//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/aspeed/aspeed_gfx_out.c
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright 2018 IBM Corporation

#[no_mangle]
unsafe extern "C" fn aspeed_gfx_get_modes(connector: *mut drm_connector) -> c_int {
    static int aspeed_gfx_get_modes(struct drm_connector *connector)
    {
    return drm_add_modes_noedid(connector, 800, 600);
    }
    static const struct
    drm_connector_helper_funcs aspeed_gfx_connector_helper_funcs = {
    .get_modes = aspeed_gfx_get_modes,
    };
    static const struct drm_connector_funcs aspeed_gfx_connector_funcs = {
    .fill_modes		= drm_helper_probe_single_connector_modes,
    .destroy		= drm_connector_cleanup,
    .reset			= drm_atomic_helper_connector_reset,
    .atomic_duplicate_state	= drm_atomic_helper_connector_duplicate_state,
    .atomic_destroy_state	= drm_atomic_helper_connector_destroy_state,
    };
#[no_mangle]
pub unsafe extern "C" fn aspeed_gfx_create_output(drm: *mut drm_device) -> c_int {
    int aspeed_gfx_create_output(struct drm_device *drm)
    {
    struct aspeed_gfx *priv = to_aspeed_gfx(drm);
    int ret;
    priv.connector.dpms = DRM_MODE_DPMS_OFF;
    priv.connector.polled = 0;
    drm_connector_helper_add(&priv.connector,
    &aspeed_gfx_connector_helper_funcs);
    ret = drm_connector_init(drm, &priv.connector,
    &aspeed_gfx_connector_funcs,
    DRM_MODE_CONNECTOR_Unknown);
    return ret;
    }
