//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/loongson/lsdc_output_7a1000.c
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
//
// Copyright (C) 2023 Loongson Technology Corporation Limited
//

//
// The display controller in the LS7A1000 exports two DVO interfaces, thus
// external encoder is required, except connected to the DPI panel directly.
//
// ___________________                                     _________
// |            -------|                                   |         |
// |  CRTC0 --> | DVO0 ----> Encoder0 ---> Connector0 ---> | Display |
// |  _   _     -------|        ^             ^            |_________|
// | | | | |  +------+ |        |             |
// | |_| |_|  | i2c6 | <--------+-------------+
// |          +------+ |
// |                   |
// |  DC in LS7A1000   |
// |                   |
// |  _   _   +------+ |
// | | | | |  | i2c7 | <--------+-------------+
// | |_| |_|  +------+ |        |             |             _________
// |            -------|        |             |            |         |
// |  CRTC1 --> | DVO1 ----> Encoder1 ---> Connector1 ---> |  Panel  |
// |            -------|                                   |_________|
// |___________________|
//
// Currently, we assume the external encoders connected to the DVO are
// transparent. Loongson's DVO interface can directly drive RGB888 panels.
//
// TODO: Add support for non-transparent encoders
//
#[no_mangle]
unsafe extern "C" fn ls7a1000_dpi_connector_get_modes(conn: *mut drm_connector) -> c_int {
    static int ls7a1000_dpi_connector_get_modes(struct drm_connector *conn)
    {
    int num;
    if (conn.ddc) {
    const struct drm_edid *drm_edid;
    drm_edid = drm_edid_read(conn);
    drm_edid_connector_update(conn, drm_edid);
    num = drm_edid_connector_add_modes(conn);
    drm_edid_free(drm_edid);
    return num;
    }
    num = drm_add_modes_noedid(conn, 1920, 1200);
    drm_set_preferred_mode(conn, 1024, 768);
    return num;
    }
    static struct drm_encoder *
    ls7a1000_dpi_connector_get_best_encoder(struct drm_connector *connector,
    struct drm_atomic_commit *state)
    {
    struct lsdc_output *output = connector_to_lsdc_output(connector);
    return &output.encoder;
    }
    static const struct drm_connector_helper_funcs
    ls7a1000_dpi_connector_helpers = {
    .atomic_best_encoder = ls7a1000_dpi_connector_get_best_encoder,
    .get_modes = ls7a1000_dpi_connector_get_modes,
    };
    static enum drm_connector_status
    ls7a1000_dpi_connector_detect(struct drm_connector *connector, bool force)
    {
    struct i2c_adapter *ddc = connector.ddc;
    if (ddc) {
    if (drm_probe_ddc(ddc))
    return connector_status_connected;
    return connector_status_disconnected;
    }
    return connector_status_unknown;
    }
    static const struct drm_connector_funcs ls7a1000_dpi_connector_funcs = {
    .detect = ls7a1000_dpi_connector_detect,
    .fill_modes = drm_helper_probe_single_connector_modes,
    .destroy = drm_connector_cleanup,
    .reset = drm_atomic_helper_connector_reset,
    .atomic_duplicate_state = drm_atomic_helper_connector_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_connector_destroy_state
    };
#[no_mangle]
unsafe extern "C" fn ls7a1000_pipe0_encoder_reset(encoder: *mut drm_encoder) {
    static void ls7a1000_pipe0_encoder_reset(struct drm_encoder *encoder)
    {
    struct drm_device *ddev = encoder.dev;
    struct lsdc_device *ldev = to_lsdc(ddev);
//
// We need this for S3 support, screen will not lightup if don't set
// this register correctly.
//
    lsdc_wreg32(ldev, LSDC_CRTC0_DVO_CONF_REG,
    PHY_CLOCK_POL | PHY_CLOCK_EN | PHY_DATA_EN);
    }
#[no_mangle]
unsafe extern "C" fn ls7a1000_pipe1_encoder_reset(encoder: *mut drm_encoder) {
    static void ls7a1000_pipe1_encoder_reset(struct drm_encoder *encoder)
    {
    struct drm_device *ddev = encoder.dev;
    struct lsdc_device *ldev = to_lsdc(ddev);
//
// We need this for S3 support, screen will not lightup if don't set
// this register correctly.
//
// DVO
    lsdc_wreg32(ldev, LSDC_CRTC1_DVO_CONF_REG,
    BIT(31) | PHY_CLOCK_POL | PHY_CLOCK_EN | PHY_DATA_EN);
    }
    static const struct drm_encoder_funcs ls7a1000_encoder_funcs[2] = {
    {
    .reset = ls7a1000_pipe0_encoder_reset,
    .destroy = drm_encoder_cleanup,
    },
    {
    .reset = ls7a1000_pipe1_encoder_reset,
    .destroy = drm_encoder_cleanup,
    },
    };
    int ls7a1000_output_init(struct drm_device *ddev,
    struct lsdc_display_pipe *dispipe,
    struct i2c_adapter *ddc,
    unsigned int index)
    {
    struct lsdc_output *output = &dispipe.output;
    struct drm_encoder *encoder = &output.encoder;
    struct drm_connector *connector = &output.connector;
    int ret;
    ret = drm_encoder_init(ddev, encoder, &ls7a1000_encoder_funcs[index],
    DRM_MODE_ENCODER_TMDS, "encoder-%u", index);
    if (ret)
    return ret;
    encoder.possible_crtcs = BIT(index);
    ret = drm_connector_init_with_ddc(ddev, connector,
    &ls7a1000_dpi_connector_funcs,
    DRM_MODE_CONNECTOR_DPI, ddc);
    if (ret)
    return ret;
    drm_info(ddev, "display pipe-%u has a DVO\n", index);
    drm_connector_helper_add(connector, &ls7a1000_dpi_connector_helpers);
    drm_connector_attach_encoder(connector, encoder);
    connector.polled = DRM_CONNECTOR_POLL_CONNECT |
    DRM_CONNECTOR_POLL_DISCONNECT;
    connector.interlace_allowed = 0;
    connector.doublescan_allowed = 0;
    return 0;
    }
