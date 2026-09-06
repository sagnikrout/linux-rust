//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/ast/ast_vga.c
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
// Encoder
//
    static const struct drm_encoder_funcs ast_vga_encoder_funcs = {
    .destroy = drm_encoder_cleanup,
    };
//
// Connector
//
#[no_mangle]
unsafe extern "C" fn ast_vga_connector_helper_get_modes(connector: *mut drm_connector) -> c_int {
    static int ast_vga_connector_helper_get_modes(struct drm_connector *connector)
    {
    struct ast_connector *ast_connector = to_ast_connector(connector);
    int count;
    if (ast_connector.physical_status == connector_status_connected) {
    count = drm_connector_helper_get_modes(connector);
    } else {
    drm_edid_connector_update(connector, core::ptr::null_mut());
//
// There's no EDID data without a connected monitor. Set BMC-
// compatible modes in this case. The XGA default resolution
// should work well for all BMCs.
//
    count = drm_add_modes_noedid(connector, 4096, 4096);
    if (count)
    drm_set_preferred_mode(connector, 1024, 768);
    }
    return count;
    }
    static int ast_vga_connector_helper_detect_ctx(struct drm_connector *connector,
    struct drm_modeset_acquire_ctx *ctx,
    bool force)
    {
    struct ast_connector *ast_connector = to_ast_connector(connector);
    enum drm_connector_status status;
    status = drm_connector_helper_detect_from_ddc(connector, ctx, force);
    if (status != ast_connector.physical_status)
    ++connector.epoch_counter;
    ast_connector.physical_status = status;
    return connector_status_connected;
    }
    static const struct drm_connector_helper_funcs ast_vga_connector_helper_funcs = {
    .get_modes = ast_vga_connector_helper_get_modes,
    .detect_ctx = ast_vga_connector_helper_detect_ctx,
    };
    static const struct drm_connector_funcs ast_vga_connector_funcs = {
    .reset = drm_atomic_helper_connector_reset,
    .fill_modes = drm_helper_probe_single_connector_modes,
    .destroy = drm_connector_cleanup,
    .atomic_duplicate_state = drm_atomic_helper_connector_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_connector_destroy_state,
    };
//
// Output
//
#[no_mangle]
pub unsafe extern "C" fn ast_vga_output_init(ast: *mut ast_device) -> c_int {
    int ast_vga_output_init(struct ast_device *ast)
    {
    struct drm_device *dev = &ast.base;
    struct drm_crtc *crtc = &ast.crtc;
    struct i2c_adapter *ddc;
    struct drm_encoder *encoder;
    struct ast_connector *ast_connector;
    struct drm_connector *connector;
    int ret;
// DDC
    ddc = ast_ddc_create(ast);
    if (IS_ERR(ddc))
    return PTR_ERR(ddc);
// encoder
    encoder = &ast.output.vga.encoder;
    ret = drm_encoder_init(dev, encoder, &ast_vga_encoder_funcs,
    DRM_MODE_ENCODER_DAC, core::ptr::null_mut());
    if (ret)
    return ret;
    encoder.possible_crtcs = drm_crtc_mask(crtc);
// connector
    ast_connector = &ast.output.vga.connector;
    connector = &ast_connector.base;
    ret = drm_connector_init_with_ddc(dev, connector, &ast_vga_connector_funcs,
    DRM_MODE_CONNECTOR_VGA, ddc);
    if (ret)
    return ret;
    drm_connector_helper_add(connector, &ast_vga_connector_helper_funcs);
    connector.interlace_allowed = 0;
    connector.doublescan_allowed = 0;
    connector.polled = DRM_CONNECTOR_POLL_CONNECT | DRM_CONNECTOR_POLL_DISCONNECT;
    ast_connector.physical_status = connector.status;
    ret = drm_connector_attach_encoder(connector, encoder);
    if (ret)
    return ret;
    return 0;
    }
