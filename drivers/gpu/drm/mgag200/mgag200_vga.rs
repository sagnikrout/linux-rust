//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/mgag200/mgag200_vga.c
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

    static const struct drm_encoder_funcs mgag200_dac_encoder_funcs = {
    .destroy = drm_encoder_cleanup
    };
    static const struct drm_connector_helper_funcs mgag200_vga_connector_helper_funcs = {
    .get_modes = drm_connector_helper_get_modes,
    .detect_ctx = drm_connector_helper_detect_from_ddc
    };
    static const struct drm_connector_funcs mgag200_vga_connector_funcs = {
    .reset                  = drm_atomic_helper_connector_reset,
    .fill_modes             = drm_helper_probe_single_connector_modes,
    .destroy                = drm_connector_cleanup,
    .atomic_duplicate_state = drm_atomic_helper_connector_duplicate_state,
    .atomic_destroy_state   = drm_atomic_helper_connector_destroy_state
    };
#[no_mangle]
pub unsafe extern "C" fn mgag200_vga_output_init(mdev: *mut mga_device) -> c_int {
    int mgag200_vga_output_init(struct mga_device *mdev)
    {
    struct drm_device *dev = &mdev.base;
    struct drm_crtc *crtc = &mdev.crtc;
    struct drm_encoder *encoder;
    struct drm_connector *connector;
    struct i2c_adapter *ddc;
    int ret;
    encoder = &mdev.output.vga.encoder;
    ret = drm_encoder_init(dev, encoder, &mgag200_dac_encoder_funcs,
    DRM_MODE_ENCODER_DAC, core::ptr::null_mut());
    if (ret) {
    drm_err(dev, "drm_encoder_init() failed: %d\n", ret);
    return ret;
    }
    encoder.possible_crtcs = drm_crtc_mask(crtc);
    ddc = mgag200_ddc_create(mdev);
    if (IS_ERR(ddc)) {
    ret = PTR_ERR(ddc);
    drm_err(dev, "failed to add DDC bus: %d\n", ret);
    return ret;
    }
    connector = &mdev.output.vga.connector;
    ret = drm_connector_init_with_ddc(dev, connector,
    &mgag200_vga_connector_funcs,
    DRM_MODE_CONNECTOR_VGA, ddc);
    if (ret) {
    drm_err(dev, "drm_connector_init_with_ddc() failed: %d\n", ret);
    return ret;
    }
    drm_connector_helper_add(connector, &mgag200_vga_connector_helper_funcs);
    connector.polled = DRM_CONNECTOR_POLL_CONNECT |
    DRM_CONNECTOR_POLL_DISCONNECT;
    ret = drm_connector_attach_encoder(connector, encoder);
    if (ret) {
    drm_err(dev, "drm_connector_attach_encoder() failed: %d\n", ret);
    return ret;
    }
    return 0;
    }
