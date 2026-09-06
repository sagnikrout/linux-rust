//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/gma500/cdv_intel_hdmi.c
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


//
// Copyright © 2006-2011 Intel Corporation
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//
// Authors:
// jim liu <jim.liu@intel.com>
//

// hdmi control bits

// hdmi-b control bits

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mid_intel_hdmi_priv {
    pub hdmi_reg: u32,
    pub save_HDMIB: u32,
    pub has_hdmi_sink: bool,
    pub has_hdmi_audio: bool,
// Should set this when detect hotplug
    pub hdmi_device_connected: bool,
    pub dev: *mut drm_device,
}

    static void cdv_hdmi_mode_set(struct drm_encoder *encoder,
    struct drm_display_mode *mode,
    struct drm_display_mode *adjusted_mode)
    {
    struct drm_device *dev = encoder.dev;
    struct gma_encoder *gma_encoder = to_gma_encoder(encoder);
    struct mid_intel_hdmi_priv *hdmi_priv = gma_encoder.dev_priv;
    u32 hdmib;
    struct drm_crtc *crtc = encoder.crtc;
    struct gma_crtc *gma_crtc = to_gma_crtc(crtc);
    hdmib = (2 << 10);
    if (adjusted_mode.flags & DRM_MODE_FLAG_PVSYNC)
    hdmib |= HDMI_VSYNC_ACTIVE_HIGH;
    if (adjusted_mode.flags & DRM_MODE_FLAG_PHSYNC)
    hdmib |= HDMI_HSYNC_ACTIVE_HIGH;
    if (gma_crtc.pipe == 1)
    hdmib |= HDMIB_PIPE_B_SELECT;
    if (hdmi_priv.has_hdmi_audio) {
    hdmib |= HDMI_AUDIO_ENABLE;
    hdmib |= HDMI_NULL_PACKETS_DURING_VSYNC;
    }
    REG_WRITE(hdmi_priv.hdmi_reg, hdmib);
    REG_READ(hdmi_priv.hdmi_reg);
    }
#[no_mangle]
unsafe extern "C" fn cdv_hdmi_dpms(encoder: *mut drm_encoder, mode: c_int) {
    static void cdv_hdmi_dpms(struct drm_encoder *encoder, int mode)
    {
    struct drm_device *dev = encoder.dev;
    struct gma_encoder *gma_encoder = to_gma_encoder(encoder);
    struct mid_intel_hdmi_priv *hdmi_priv = gma_encoder.dev_priv;
    u32 hdmib;
    hdmib = REG_READ(hdmi_priv.hdmi_reg);
    if (mode != DRM_MODE_DPMS_ON)
    REG_WRITE(hdmi_priv.hdmi_reg, hdmib & ~HDMIB_PORT_EN);
    else
    REG_WRITE(hdmi_priv.hdmi_reg, hdmib | HDMIB_PORT_EN);
    REG_READ(hdmi_priv.hdmi_reg);
    }
#[no_mangle]
unsafe extern "C" fn cdv_hdmi_save(connector: *mut drm_connector) {
    static void cdv_hdmi_save(struct drm_connector *connector)
    {
    struct drm_device *dev = connector.dev;
    struct gma_encoder *gma_encoder = gma_attached_encoder(connector);
    struct mid_intel_hdmi_priv *hdmi_priv = gma_encoder.dev_priv;
    hdmi_priv.save_HDMIB = REG_READ(hdmi_priv.hdmi_reg);
    }
#[no_mangle]
unsafe extern "C" fn cdv_hdmi_restore(connector: *mut drm_connector) {
    static void cdv_hdmi_restore(struct drm_connector *connector)
    {
    struct drm_device *dev = connector.dev;
    struct gma_encoder *gma_encoder = gma_attached_encoder(connector);
    struct mid_intel_hdmi_priv *hdmi_priv = gma_encoder.dev_priv;
    REG_WRITE(hdmi_priv.hdmi_reg, hdmi_priv.save_HDMIB);
    REG_READ(hdmi_priv.hdmi_reg);
    }
    static enum drm_connector_status cdv_hdmi_detect(
    struct drm_connector *connector, bool force)
    {
    struct gma_encoder *gma_encoder = gma_attached_encoder(connector);
    struct mid_intel_hdmi_priv *hdmi_priv = gma_encoder.dev_priv;
    struct edid *edid = core::ptr::null_mut();
    let mut status: enum drm_connector_status = connector_status_disconnected;
    edid = drm_get_edid(connector, connector.ddc);
    hdmi_priv.has_hdmi_sink = false;
    hdmi_priv.has_hdmi_audio = false;
    if (edid) {
    if (edid.input & DRM_EDID_INPUT_DIGITAL) {
    status = connector_status_connected;
    hdmi_priv.has_hdmi_sink =
    drm_detect_hdmi_monitor(edid);
    hdmi_priv.has_hdmi_audio =
    drm_detect_monitor_audio(edid);
    }
    kfree(edid);
    }
    return status;
    }
    static int cdv_hdmi_set_property(struct drm_connector *connector,
    struct drm_property *property,
    uint64_t value)
    {
    struct drm_encoder *encoder = connector.encoder;
    if (!strcmp(property.name, "scaling mode") && encoder) {
    struct gma_crtc *crtc = to_gma_crtc(encoder.crtc);
    bool centre;
    uint64_t curValue;
    if (!crtc)
    return -1;
    switch (value) {
    case DRM_MODE_SCALE_FULLSCREEN:
    break;
    case DRM_MODE_SCALE_NO_SCALE:
    break;
    case DRM_MODE_SCALE_ASPECT:
    break;
    default:
    return -1;
    }
    if (drm_object_property_get_value(&connector.base,
    property, &curValue))
    return -1;
    if (curValue == value)
    return 0;
    if (drm_object_property_set_value(&connector.base,
    property, value))
    return -1;
    centre = (curValue == DRM_MODE_SCALE_NO_SCALE) ||
    (value == DRM_MODE_SCALE_NO_SCALE);
    if (crtc.saved_mode.hdisplay != 0 &&
    crtc.saved_mode.vdisplay != 0) {
    if (centre) {
    if (!drm_crtc_helper_set_mode(encoder.crtc, &crtc.saved_mode,
    encoder.crtc.x, encoder.crtc.y, encoder.crtc.primary.fb))
    return -1;
    } else {
    const struct drm_encoder_helper_funcs *helpers
    = encoder.helper_private;
    helpers.mode_set(encoder, &crtc.saved_mode,
    &crtc.saved_adjusted_mode);
    }
    }
    }
    return 0;
    }
//
// Return the list of HDMI DDC modes if available.
//
#[no_mangle]
unsafe extern "C" fn cdv_hdmi_get_modes(connector: *mut drm_connector) -> c_int {
    static int cdv_hdmi_get_modes(struct drm_connector *connector)
    {
    struct edid *edid = core::ptr::null_mut();
    let mut ret: c_int = 0;
    edid = drm_get_edid(connector, connector.ddc);
    if (edid) {
    drm_connector_update_edid_property(connector, edid);
    ret = drm_add_edid_modes(connector, edid);
    kfree(edid);
    }
    return ret;
    }
    static enum drm_mode_status cdv_hdmi_mode_valid(struct drm_connector *connector,
    const struct drm_display_mode *mode)
    {
    if (mode.clock > 165000)
    return MODE_CLOCK_HIGH;
    if (mode.clock < 20000)
    return MODE_CLOCK_HIGH;
// just in case
    if (mode.flags & DRM_MODE_FLAG_DBLSCAN)
    return MODE_NO_DBLESCAN;
// just in case
    if (mode.flags & DRM_MODE_FLAG_INTERLACE)
    return MODE_NO_INTERLACE;
    return MODE_OK;
    }
#[no_mangle]
unsafe extern "C" fn cdv_hdmi_destroy(connector: *mut drm_connector) {
    static void cdv_hdmi_destroy(struct drm_connector *connector)
    {
    struct gma_connector *gma_connector = to_gma_connector(connector);
    struct gma_i2c_chan *ddc_bus = to_gma_i2c_chan(connector.ddc);
    gma_i2c_destroy(ddc_bus);
    drm_connector_cleanup(connector);
    kfree(gma_connector);
    }
    static const struct drm_encoder_funcs cdv_hdmi_funcs = {
    .destroy = drm_encoder_cleanup,
    };
    static const struct drm_encoder_helper_funcs cdv_hdmi_helper_funcs = {
    .dpms = cdv_hdmi_dpms,
    .prepare = gma_encoder_prepare,
    .mode_set = cdv_hdmi_mode_set,
    .commit = gma_encoder_commit,
    };
    static const struct drm_connector_helper_funcs
    cdv_hdmi_connector_helper_funcs = {
    .get_modes = cdv_hdmi_get_modes,
    .mode_valid = cdv_hdmi_mode_valid,
    .best_encoder = gma_best_encoder,
    };
    static const struct drm_connector_funcs cdv_hdmi_connector_funcs = {
    .dpms = drm_helper_connector_dpms,
    .detect = cdv_hdmi_detect,
    .fill_modes = drm_helper_probe_single_connector_modes,
    .set_property = cdv_hdmi_set_property,
    .destroy = cdv_hdmi_destroy,
    };
    void cdv_hdmi_init(struct drm_device *dev,
    struct psb_intel_mode_device *mode_dev, int reg)
    {
    struct gma_encoder *gma_encoder;
    struct gma_connector *gma_connector;
    struct drm_connector *connector;
    struct mid_intel_hdmi_priv *hdmi_priv;
    struct gma_i2c_chan *ddc_bus;
    int ddc_reg;
    int ret;
    gma_encoder = kzalloc_obj(struct gma_encoder);
    if (!gma_encoder)
    return;
    gma_connector = kzalloc_obj(struct gma_connector);
    if (!gma_connector)
    goto err_free_encoder;
    hdmi_priv = kzalloc_obj(struct mid_intel_hdmi_priv);
    if (!hdmi_priv)
    goto err_free_connector;
    connector = &gma_connector.base;
    connector.polled = DRM_CONNECTOR_POLL_HPD;
    gma_connector.save = cdv_hdmi_save;
    gma_connector.restore = cdv_hdmi_restore;
    switch (reg) {
    case SDVOB:
    ddc_reg = GPIOE;
    gma_encoder.ddi_select = DDI0_SELECT;
    break;
    case SDVOC:
    ddc_reg = GPIOD;
    gma_encoder.ddi_select = DDI1_SELECT;
    break;
    default:
    DRM_ERROR("unknown reg 0x%x for HDMI\n", reg);
    goto err_free_hdmi_priv;
    }
    ddc_bus = gma_i2c_create(dev, ddc_reg,
    (reg == SDVOB) ? "HDMIB" : "HDMIC");
    if (!ddc_bus) {
    dev_err(dev.dev, "No ddc adapter available!\n");
    goto err_free_hdmi_priv;
    }
    ret = drm_connector_init_with_ddc(dev, connector,
    &cdv_hdmi_connector_funcs,
    DRM_MODE_CONNECTOR_DVID,
    &ddc_bus.base);
    if (ret)
    goto err_ddc_destroy;
    ret = drm_encoder_init(dev, &gma_encoder.base, &cdv_hdmi_funcs,
    DRM_MODE_ENCODER_TMDS, core::ptr::null_mut());
    if (ret)
    goto err_connector_cleanup;
    gma_connector_attach_encoder(gma_connector, gma_encoder);
    gma_encoder.type = INTEL_OUTPUT_HDMI;
    hdmi_priv.hdmi_reg = reg;
    hdmi_priv.has_hdmi_sink = false;
    gma_encoder.dev_priv = hdmi_priv;
    drm_encoder_helper_add(&gma_encoder.base, &cdv_hdmi_helper_funcs);
    drm_connector_helper_add(connector,
    &cdv_hdmi_connector_helper_funcs);
    connector.display_info.subpixel_order = SubPixelHorizontalRGB;
    connector.interlace_allowed = false;
    connector.doublescan_allowed = false;
    drm_object_attach_property(&connector.base,
    dev.mode_config.scaling_mode_property,
    DRM_MODE_SCALE_FULLSCREEN);
    hdmi_priv.dev = dev;
    return;
    err_connector_cleanup:
    drm_connector_cleanup(connector);
    err_ddc_destroy:
    gma_i2c_destroy(ddc_bus);
    err_free_hdmi_priv:
    kfree(hdmi_priv);
    err_free_connector:
    kfree(gma_connector);
    err_free_encoder:
    kfree(gma_encoder);
    }
