//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/drm_bridge_helper.c
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
// drm_bridge_helper_reset_crtc - Reset the pipeline feeding a bridge
// @bridge: DRM bridge to reset
// @ctx: lock acquisition context
//
// Reset a @bridge pipeline. It will power-cycle all active components
// between the CRTC and connector that bridge is connected to.
//
// As it relies on drm_atomic_helper_reset_crtc(), the same limitations
// apply.
//
// Returns:
//
// 0 on success or a negative error code on failure. If the error
// returned is EDEADLK, the whole atomic sequence must be restarted.
//
    int drm_bridge_helper_reset_crtc(struct drm_bridge *bridge,
    struct drm_modeset_acquire_ctx *ctx)
    {
    struct drm_connector *connector;
    struct drm_encoder *encoder = bridge.encoder;
    struct drm_device *dev = encoder.dev;
    struct drm_crtc *crtc;
    int ret;
    ret = drm_modeset_lock(&dev.mode_config.connection_mutex, ctx);
    if (ret)
    return ret;
    connector = drm_atomic_get_connector_for_encoder(encoder, ctx);
    if (IS_ERR(connector)) {
    ret = PTR_ERR(connector);
    goto out;
    }
    if (!connector.state) {
    ret = -EINVAL;
    goto out;
    }
    crtc = connector.state.crtc;
    ret = drm_atomic_helper_reset_crtc(crtc, ctx);
    if (ret)
    goto out;
    out:
    drm_modeset_unlock(&dev.mode_config.connection_mutex);
    return ret;
    }
    EXPORT_SYMBOL(drm_bridge_helper_reset_crtc);
