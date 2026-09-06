//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/vkms/vkms_output.c
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

#[no_mangle]
pub unsafe extern "C" fn vkms_output_init(vkmsdev: *mut vkms_device) -> c_int {
    int vkms_output_init(struct vkms_device *vkmsdev)
    {
    struct drm_device *dev = &vkmsdev.drm;
    struct vkms_config_plane *plane_cfg;
    struct vkms_config_crtc *crtc_cfg;
    struct vkms_config_encoder *encoder_cfg;
    struct vkms_config_connector *connector_cfg;
    int ret;
    int writeback;
    if (!vkms_config_is_valid(vkmsdev.config))
    return -EINVAL;
    vkms_config_for_each_plane(vkmsdev.config, plane_cfg) {
    plane_cfg.plane = vkms_plane_init(vkmsdev, plane_cfg);
    if (IS_ERR(plane_cfg.plane)) {
    DRM_DEV_ERROR(dev.dev, "Failed to init vkms plane\n");
    return PTR_ERR(plane_cfg.plane);
    }
    }
    vkms_config_for_each_crtc(vkmsdev.config, crtc_cfg) {
    struct vkms_config_plane *primary, *cursor;
    primary = vkms_config_crtc_primary_plane(vkmsdev.config, crtc_cfg);
    cursor = vkms_config_crtc_cursor_plane(vkmsdev.config, crtc_cfg);
    crtc_cfg.crtc = vkms_crtc_init(dev, &primary.plane.base,
    cursor ? &cursor.plane.base : core::ptr::null_mut());
    if (IS_ERR(crtc_cfg.crtc)) {
    DRM_ERROR("Failed to allocate CRTC\n");
    return PTR_ERR(crtc_cfg.crtc);
    }
// Initialize the writeback component
    if (vkms_config_crtc_get_writeback(crtc_cfg)) {
    writeback = vkms_enable_writeback_connector(vkmsdev, crtc_cfg.crtc);
    if (writeback)
    DRM_ERROR("Failed to init writeback connector\n");
    }
    }
    vkms_config_for_each_plane(vkmsdev.config, plane_cfg) {
    struct vkms_config_crtc *possible_crtc;
    let mut idx: c_ulong = 0;
    vkms_config_plane_for_each_possible_crtc(plane_cfg, idx, possible_crtc) {
    plane_cfg.plane.base.possible_crtcs |=
    drm_crtc_mask(&possible_crtc.crtc.crtc);
    }
    }
    vkms_config_for_each_encoder(vkmsdev.config, encoder_cfg) {
    struct vkms_config_crtc *possible_crtc;
    let mut idx: c_ulong = 0;
    encoder_cfg.encoder = drmm_kzalloc(dev, sizeof(*encoder_cfg.encoder), GFP_KERNEL);
    if (!encoder_cfg.encoder) {
    DRM_ERROR("Failed to allocate encoder\n");
    return -ENOMEM;
    }
    ret = drmm_encoder_init(dev, encoder_cfg.encoder, core::ptr::null_mut(),
    DRM_MODE_ENCODER_VIRTUAL, core::ptr::null_mut());
    if (ret) {
    DRM_ERROR("Failed to init encoder\n");
    return ret;
    }
    encoder_cfg.encoder.possible_clones |=
    drm_encoder_mask(encoder_cfg.encoder);
    vkms_config_encoder_for_each_possible_crtc(encoder_cfg, idx, possible_crtc) {
    encoder_cfg.encoder.possible_crtcs |=
    drm_crtc_mask(&possible_crtc.crtc.crtc);
    if (vkms_config_crtc_get_writeback(possible_crtc)) {
    struct drm_encoder *wb_encoder =
    &possible_crtc.crtc.wb_encoder;
    encoder_cfg.encoder.possible_clones |=
    drm_encoder_mask(wb_encoder);
    wb_encoder.possible_clones |=
    drm_encoder_mask(encoder_cfg.encoder);
    }
    }
    }
    vkms_config_for_each_connector(vkmsdev.config, connector_cfg) {
    struct vkms_config_encoder *possible_encoder;
    let mut idx: c_ulong = 0;
    connector_cfg.connector = vkms_connector_init(vkmsdev);
    if (IS_ERR(connector_cfg.connector)) {
    DRM_ERROR("Failed to init connector\n");
    return PTR_ERR(connector_cfg.connector);
    }
    vkms_config_connector_for_each_possible_encoder(connector_cfg,
    idx,
    possible_encoder) {
    ret = drm_connector_attach_encoder(&connector_cfg.connector.base,
    possible_encoder.encoder);
    if (ret) {
    DRM_ERROR("Failed to attach connector to encoder\n");
    return ret;
    }
    }
    }
    drm_mode_config_reset(dev);
    return 0;
    }
