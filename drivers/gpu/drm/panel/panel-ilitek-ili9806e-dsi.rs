//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-ilitek-ili9806e-dsi.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ili9806e_dsi_panel_desc {
    pub display_mode: *const drm_display_mode,
    pub mode_flags: c_ulong,
    pub format: enum mipi_dsi_pixel_format,
    pub lanes: c_uint,
    pub ctx): *mut *mut void (init_sequence)(struct mipi_dsi_multi_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ili9806e_dsi_panel {
    pub dsi: *mut mipi_dsi_device,
    pub desc: *const ili9806e_dsi_panel_desc,
    pub orientation: enum drm_panel_orientation,
}

#[no_mangle]
unsafe extern "C" fn ili9806e_dsi_on(ili9806e: *mut ili9806e_dsi_panel) -> c_int {
    static int ili9806e_dsi_on(struct ili9806e_dsi_panel *ili9806e)
    {
    let mut ctx: mipi_dsi_multi_context = { .dsi = ili9806e.dsi };
    if (ili9806e.desc.init_sequence)
    ili9806e.desc.init_sequence(&ctx);
    mipi_dsi_dcs_exit_sleep_mode_multi(&ctx);
    mipi_dsi_msleep(&ctx, 120);
    mipi_dsi_dcs_set_display_on_multi(&ctx);
    return ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn ili9806e_dsi_off(panel: *mut ili9806e_dsi_panel) -> c_int {
    static int ili9806e_dsi_off(struct ili9806e_dsi_panel *panel)
    {
    let mut ctx: mipi_dsi_multi_context = { .dsi = panel.dsi };
    mipi_dsi_dcs_set_display_off_multi(&ctx);
    mipi_dsi_dcs_enter_sleep_mode_multi(&ctx);
    mipi_dsi_msleep(&ctx, 120);
    return ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn ili9806e_dsi_prepare(panel: *mut drm_panel) -> c_int {
    static int ili9806e_dsi_prepare(struct drm_panel *panel)
    {
    struct ili9806e_dsi_panel *ctx = ili9806e_get_transport(panel);
    struct device *dev = &ctx.dsi.dev;
    int ret;
    ret = ili9806e_power_on(dev);
    if (ret < 0)
    return ret;
    ret = ili9806e_dsi_on(ctx);
    if (ret < 0) {
    ili9806e_power_off(dev);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ili9806e_dsi_unprepare(panel: *mut drm_panel) -> c_int {
    static int ili9806e_dsi_unprepare(struct drm_panel *panel)
    {
    struct ili9806e_dsi_panel *ctx = ili9806e_get_transport(panel);
    struct device *dev = &ctx.dsi.dev;
    int ret;
    ili9806e_dsi_off(ctx);
    ret = ili9806e_power_off(dev);
    if (ret < 0)
    dev_err(dev, "power off failed: %d\n", ret);
    return ret;
    }
    static int ili9806e_dsi_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    struct ili9806e_dsi_panel *ctx = ili9806e_get_transport(panel);
    const struct drm_display_mode *mode = ctx.desc.display_mode;
    return drm_connector_helper_get_modes_fixed(connector, mode);
    }
#[no_mangle]
unsafe extern "C" fn ili9806e_dsi_get_orientation(panel: *mut drm_panel) -> enum drm_panel_orientation {
    static enum drm_panel_orientation ili9806e_dsi_get_orientation(struct drm_panel *panel)
    {
    struct ili9806e_dsi_panel *ctx = ili9806e_get_transport(panel);
    return ctx.orientation;
    }
    static const struct drm_panel_funcs ili9806e_dsi_funcs = {
    .prepare = ili9806e_dsi_prepare,
    .unprepare = ili9806e_dsi_unprepare,
    .get_modes = ili9806e_dsi_get_modes,
    .get_orientation = ili9806e_dsi_get_orientation,
    };
#[no_mangle]
unsafe extern "C" fn ili9806e_dsi_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int ili9806e_dsi_probe(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    struct ili9806e_dsi_panel *ctx;
    int ret;
    ctx = devm_kzalloc(dev, sizeof(struct ili9806e_dsi_panel), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    ctx.desc = device_get_match_data(dev);
    mipi_dsi_set_drvdata(dsi, ctx);
    ctx.dsi = dsi;
    dsi.mode_flags = ctx.desc.mode_flags;
    dsi.format = ctx.desc.format;
    dsi.lanes = ctx.desc.lanes;
    ret = drm_of_get_panel_orientation(dev.of_node, &ctx.orientation);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to get orientation\n");
    ret = ili9806e_probe(dev, ctx, &ili9806e_dsi_funcs,
    DRM_MODE_CONNECTOR_DSI);
    if (ret)
    return ret;
    ret = mipi_dsi_attach(dsi);
    if (ret < 0) {
    dev_err_probe(dev, ret, "Failed to attach to DSI host\n");
    ili9806e_remove(dev);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ili9806e_dsi_remove(dsi: *mut mipi_dsi_device) {
    static void ili9806e_dsi_remove(struct mipi_dsi_device *dsi)
    {
    mipi_dsi_detach(dsi);
    ili9806e_remove(&dsi.dev);
    }
#[no_mangle]
unsafe extern "C" fn com35h3p70ulc_init(ctx: *mut mipi_dsi_multi_context) {
    static void com35h3p70ulc_init(struct mipi_dsi_multi_context *ctx)
    {
// Switch to page 1
    mipi_dsi_dcs_write_seq_multi(ctx, 0xff, 0xff, 0x98, 0x06, 0x04, 0x01);
// Interface Settings
    mipi_dsi_dcs_write_seq_multi(ctx, 0x08, 0x18);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x21, 0x01);
// Panel Settings
    mipi_dsi_dcs_write_seq_multi(ctx, 0x30, 0x03);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x31, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x60, 0x0d);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x61, 0x08);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x62, 0x08);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x63, 0x09);
// Power Control
    mipi_dsi_dcs_write_seq_multi(ctx, 0x40, 0x30);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x41, 0x44);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x42, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x43, 0x89);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x44, 0x8e);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x45, 0xd9);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x46, 0x33);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x47, 0x33);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x50, 0x90);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x51, 0x90);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x56, 0x00);
// Gamma Settings
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa0, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa1, 0x0c);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa2, 0x13);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa3, 0x0f);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa4, 0x0a);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa5, 0x0d);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa6, 0x0c);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa7, 0x0b);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa8, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa9, 0x06);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xaa, 0x15);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xab, 0x07);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xac, 0x12);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xad, 0x28);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xae, 0x20);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xaf, 0x14);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc0, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc1, 0x0c);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc2, 0x13);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc3, 0x0f);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc4, 0x09);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc5, 0x0d);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc6, 0x0c);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc7, 0x0b);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc8, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc9, 0x06);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xca, 0x14);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xcb, 0x07);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xcc, 0x0f);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xcd, 0x21);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xce, 0x17);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xcf, 0x0a);
// Switch to page 7
    mipi_dsi_dcs_write_seq_multi(ctx, 0xff, 0xff, 0x98, 0x06, 0x04, 0x07);
// Power Control
    mipi_dsi_dcs_write_seq_multi(ctx, 0x06, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x18, 0x1d);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x17, 0x32);
// Switch to page 6
    mipi_dsi_dcs_write_seq_multi(ctx, 0xff, 0xff, 0x98, 0x06, 0x04, 0x06);
// GIP settings
    mipi_dsi_dcs_write_seq_multi(ctx, 0x00, 0x20);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x01, 0x02);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x02, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x03, 0x02);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x04, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x05, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x06, 0x88);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x07, 0x04);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x08, 0x03);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x09, 0x80);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x0a, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x0b, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x0c, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x0d, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x0e, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x0f, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x10, 0x55);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x11, 0x50);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x12, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x13, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x14, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x15, 0x43);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x16, 0x0b);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x17, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x18, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x19, 0x10);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x1a, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x1b, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x1c, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x1d, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x20, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x21, 0x23);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x22, 0x45);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x23, 0x67);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x24, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x25, 0x23);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x26, 0x45);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x27, 0x67);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x30, 0x02);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x31, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x32, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x33, 0x88);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x34, 0xaa);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x35, 0xbb);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x36, 0x66);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x37, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x38, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x39, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x3a, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x3b, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x3c, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x3d, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x3e, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x3f, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x40, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x53, 0x12);
// Switch to page 0
    mipi_dsi_dcs_write_seq_multi(ctx, 0xff, 0xff, 0x98, 0x06, 0x04, 0x00);
// Interface Pixel format
    mipi_dsi_dcs_write_seq_multi(ctx, 0x3a, 0x60);
    };
    static const struct drm_display_mode com35h3p70ulc_default_mode = {
    .clock = 22400,
    .hdisplay = 480,
    .hsync_start = 480 + 16,
    .hsync_end = 480 + 16 + 16,
    .htotal = 480 + 16 + 16 + 16,
    .vdisplay = 640,
    .vsync_start = 640 + 52,
    .vsync_end = 640 + 52 + 4,
    .vtotal = 640 + 52 + 4 + 16,
    .width_mm = 53,
    .height_mm = 71,
    };
    static const struct ili9806e_dsi_panel_desc com35h3p70ulc_desc = {
    .init_sequence = com35h3p70ulc_init,
    .display_mode = &com35h3p70ulc_default_mode,
    .mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_VIDEO_BURST |
    MIPI_DSI_MODE_LPM,
    .format = MIPI_DSI_FMT_RGB888,
    .lanes = 2,
    };
#[no_mangle]
unsafe extern "C" fn dmt028vghmcmi_1d_init(ctx: *mut mipi_dsi_multi_context) {
    static void dmt028vghmcmi_1d_init(struct mipi_dsi_multi_context *ctx)
    {
    mipi_dsi_dcs_write_seq_multi(ctx, 0xff, 0xff, 0x98, 0x06, 0x04, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x08, 0x10);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x21, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x30, 0x03);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x31, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x60, 0x06);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x61, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x62, 0x07);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x63, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x40, 0x16);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x41, 0x44);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x42, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x43, 0x83);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x44, 0x89);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x45, 0x8a);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x46, 0x44);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x47, 0x44);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x50, 0x78);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x51, 0x78);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x52, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x53, 0x6c);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x54, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x55, 0x6c);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x56, 0x00);
// Gamma settings
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa0, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa1, 0x09);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa2, 0x14);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa3, 0x09);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa4, 0x05);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa5, 0x0a);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa6, 0x07);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa7, 0x07);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa8, 0x08);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xa9, 0x0b);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xaa, 0x0c);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xab, 0x05);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xac, 0x0a);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xad, 0x19);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xae, 0x0b);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xaf, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc0, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc1, 0x0c);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc2, 0x14);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc3, 0x11);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc4, 0x05);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc5, 0x0c);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc6, 0x08);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc7, 0x03);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc8, 0x06);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xc9, 0x0a);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xca, 0x10);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xcb, 0x05);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xcc, 0x0d);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xcd, 0x15);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xce, 0x13);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xcf, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xff, 0xff, 0x98, 0x06, 0x04, 0x07);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x17, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x18, 0x1d);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x02, 0x77);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xe1, 0x79);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x06, 0x13);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xff, 0xff, 0x98, 0x06, 0x04, 0x06);
// GIP 0
    mipi_dsi_dcs_write_seq_multi(ctx, 0x00, 0x21);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x01, 0x0a);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x02, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x03, 0x05);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x04, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x05, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x06, 0x98);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x07, 0x06);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x08, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x09, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x0a, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x0b, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x0c, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x0d, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x0e, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x0f, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x10, 0xf7);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x11, 0xf0);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x12, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x13, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x14, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x15, 0xc0);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x16, 0x08);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x17, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x18, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x19, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x1a, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x1b, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x1c, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x1d, 0x00);
// GIP 1
    mipi_dsi_dcs_write_seq_multi(ctx, 0x20, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x21, 0x23);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x22, 0x44);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x23, 0x67);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x24, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x25, 0x23);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x26, 0x45);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x27, 0x67);
// GIP 2
    mipi_dsi_dcs_write_seq_multi(ctx, 0x30, 0x01);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x31, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x32, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x33, 0xbc);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x34, 0xad);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x35, 0xda);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x36, 0xcb);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x37, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x38, 0x55);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x39, 0x76);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x3a, 0x67);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x3b, 0x88);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x3c, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x3d, 0x11);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x3e, 0x00);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x3f, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x40, 0x22);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x52, 0x10);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x53, 0x10);
    mipi_dsi_dcs_write_seq_multi(ctx, 0x54, 0x13);
    mipi_dsi_dcs_write_seq_multi(ctx, 0xff, 0xff, 0x98, 0x06, 0x04, 0x00);
    };
    static const struct drm_display_mode dmt028vghmcmi_1d_default_mode = {
    .clock		= 22000,
    .hdisplay	= 480,
    .hsync_start	= 480 + 20,
    .hsync_end	= 480 + 20 + 4,
    .htotal		= 480 + 20 + 4 + 10,
    .vdisplay	= 640,
    .vsync_start	= 640 + 40,
    .vsync_end	= 640 + 40 + 4,
    .vtotal		= 640 + 40 + 4 + 20,
    .width_mm	= 53,
    .height_mm	= 79,
    .flags		= DRM_MODE_FLAG_NHSYNC | DRM_MODE_FLAG_NVSYNC,
    .type		= DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED,
    };
    static const struct ili9806e_dsi_panel_desc dmt028vghmcmi_1d_desc = {
    .init_sequence = dmt028vghmcmi_1d_init,
    .display_mode = &dmt028vghmcmi_1d_default_mode,
    .mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_VIDEO_BURST |
    MIPI_DSI_MODE_LPM | MIPI_DSI_CLOCK_NON_CONTINUOUS,
    .format = MIPI_DSI_FMT_RGB888,
    .lanes = 2,
    };
    static const struct of_device_id ili9806e_dsi_of_match[] = {
    { .compatible = "densitron,dmt028vghmcmi-1d", .data = &dmt028vghmcmi_1d_desc },
    { .compatible = "ortustech,com35h3p70ulc", .data = &com35h3p70ulc_desc },
    { }
    };
    MODULE_DEVICE_TABLE(of, ili9806e_dsi_of_match);
    static struct mipi_dsi_driver ili9806e_dsi_driver = {
    .driver = {
    .name = "ili9806e-dsi",
    .of_match_table = ili9806e_dsi_of_match,
    },
    .probe = ili9806e_dsi_probe,
    .remove = ili9806e_dsi_remove,
    };
    module_mipi_dsi_driver(ili9806e_dsi_driver);
    MODULE_AUTHOR("Gunnar Dibbern <gunnar.dibbern@lht.dlh.de>");
    MODULE_AUTHOR("Michael Walle <mwalle@kernel.org>");
    MODULE_DESCRIPTION("Ilitek ILI9806E Controller DSI Driver");
    MODULE_LICENSE("GPL");
