//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-himax-hx83112a.c
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
// Generated with linux-mdss-dsi-panel-driver-generator from vendor device tree.
// Copyright (c) 2024 Luca Weiss <luca.weiss@fairphone.com>
//

// Manufacturer specific DSI commands
pub const HX83112A_SETPOWER1: c_uint = 0xb1;
pub const HX83112A_SETDISP: c_uint = 0xb2;
pub const HX83112A_SETDRV: c_uint = 0xb4;
pub const HX83112A_SETEXTC: c_uint = 0xb9;
pub const HX83112A_SETBANK: c_uint = 0xbd;
pub const HX83112A_SETPTBA: c_uint = 0xbf;
pub const HX83112A_SETDGCLUT: c_uint = 0xc1;
pub const HX83112A_SETTCON: c_uint = 0xc7;
pub const HX83112A_SETCLOCK: c_uint = 0xcb;
pub const HX83112A_SETPANEL: c_uint = 0xcc;
pub const HX83112A_SETPOWER2: c_uint = 0xd2;
pub const HX83112A_SETGIP0: c_uint = 0xd3;
pub const HX83112A_SETGIP1: c_uint = 0xd5;
pub const HX83112A_SETGIP2: c_uint = 0xd6;
pub const HX83112A_SETGIP3: c_uint = 0xd8;
pub const HX83112A_SETTP1: c_uint = 0xe7;
pub const HX83112A_UNKNOWN1: c_uint = 0xe9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hx83112a_panel {
    pub panel: drm_panel,
    pub dsi: *mut mipi_dsi_device,
    pub supplies: [regulator_bulk_data; 3],
    pub reset_gpio: *mut gpio_desc,
}

    static inline struct hx83112a_panel *to_hx83112a_panel(struct drm_panel *panel)
    {
    return container_of(panel, struct hx83112a_panel, panel);
    }
#[no_mangle]
unsafe extern "C" fn hx83112a_reset(ctx: *mut hx83112a_panel) {
    static void hx83112a_reset(struct hx83112a_panel *ctx)
    {
    gpiod_set_value_cansleep(ctx.reset_gpio, 0);
    msleep(20);
    gpiod_set_value_cansleep(ctx.reset_gpio, 1);
    msleep(20);
    gpiod_set_value_cansleep(ctx.reset_gpio, 0);
    msleep(50);
    }
#[no_mangle]
unsafe extern "C" fn hx83112a_on(dsi: *mut mipi_dsi_device) -> c_int {
    static int hx83112a_on(struct mipi_dsi_device *dsi)
    {
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = dsi };
    dsi.mode_flags |= MIPI_DSI_MODE_LPM;
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETEXTC, 0x83, 0x11, 0x2a);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETPOWER1,
    0x08, 0x28, 0x28, 0x83, 0x83, 0x4c, 0x4f, 0x33);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETDISP,
    0x00, 0x02, 0x00, 0x90, 0x24, 0x00, 0x08, 0x19,
    0xea, 0x11, 0x11, 0x00, 0x11, 0xa3);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETDRV,
    0x58, 0x68, 0x58, 0x68, 0x0f, 0xef, 0x0b, 0xc0,
    0x0b, 0xc0, 0x0b, 0xc0, 0x00, 0xff, 0x00, 0xff,
    0x00, 0x00, 0x14, 0x15, 0x00, 0x29, 0x11, 0x07,
    0x12, 0x00, 0x29);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETBANK, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETDRV,
    0x00, 0x12, 0x12, 0x11, 0x88, 0x12, 0x12, 0x00,
    0x53);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETBANK, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETBANK, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETDGCLUT,
    0xff, 0xfe, 0xfb, 0xf8, 0xf4, 0xf1, 0xed, 0xe6,
    0xe2, 0xde, 0xdb, 0xd6, 0xd3, 0xcf, 0xca, 0xc6,
    0xc2, 0xbe, 0xb9, 0xb0, 0xa7, 0x9e, 0x96, 0x8d,
    0x84, 0x7c, 0x74, 0x6b, 0x62, 0x5a, 0x51, 0x49,
    0x41, 0x39, 0x31, 0x29, 0x21, 0x19, 0x12, 0x0a,
    0x06, 0x05, 0x02, 0x01, 0x00, 0x00, 0xc9, 0xb3,
    0x08, 0x0e, 0xf2, 0xe1, 0x59, 0xf4, 0x22, 0xad,
    0x40);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETBANK, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETDGCLUT,
    0xff, 0xfe, 0xfb, 0xf8, 0xf4, 0xf1, 0xed, 0xe6,
    0xe2, 0xde, 0xdb, 0xd6, 0xd3, 0xcf, 0xca, 0xc6,
    0xc2, 0xbe, 0xb9, 0xb0, 0xa7, 0x9e, 0x96, 0x8d,
    0x84, 0x7c, 0x74, 0x6b, 0x62, 0x5a, 0x51, 0x49,
    0x41, 0x39, 0x31, 0x29, 0x21, 0x19, 0x12, 0x0a,
    0x06, 0x05, 0x02, 0x01, 0x00, 0x00, 0xc9, 0xb3,
    0x08, 0x0e, 0xf2, 0xe1, 0x59, 0xf4, 0x22, 0xad,
    0x40);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETDGCLUT,
    0xff, 0xfe, 0xfb, 0xf8, 0xf4, 0xf1, 0xed, 0xe6,
    0xe2, 0xde, 0xdb, 0xd6, 0xd3, 0xcf, 0xca, 0xc6,
    0xc2, 0xbe, 0xb9, 0xb0, 0xa7, 0x9e, 0x96, 0x8d,
    0x84, 0x7c, 0x74, 0x6b, 0x62, 0x5a, 0x51, 0x49,
    0x41, 0x39, 0x31, 0x29, 0x21, 0x19, 0x12, 0x0a,
    0x06, 0x05, 0x02, 0x01, 0x00, 0x00, 0xc9, 0xb3,
    0x08, 0x0e, 0xf2, 0xe1, 0x59, 0xf4, 0x22, 0xad,
    0x40);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETBANK, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETDGCLUT, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETTCON,
    0x70, 0x00, 0x04, 0xe0, 0x33, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETPANEL, 0x08);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETPOWER2, 0x2b, 0x2b);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETGIP0,
    0x80, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x08,
    0x08, 0x03, 0x03, 0x22, 0x18, 0x07, 0x07, 0x07,
    0x07, 0x32, 0x10, 0x06, 0x00, 0x06, 0x32, 0x10,
    0x07, 0x00, 0x07, 0x32, 0x19, 0x31, 0x09, 0x31,
    0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x08,
    0x09, 0x30, 0x00, 0x00, 0x00, 0x06, 0x0d, 0x00,
    0x0f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETGIP0,
    0x00, 0x00, 0x19, 0x10, 0x00, 0x0a, 0x00, 0x81);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETBANK, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETGIP1,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0xc0, 0xc0, 0x18, 0x18, 0x19, 0x19, 0x18, 0x18,
    0x40, 0x40, 0x18, 0x18, 0x18, 0x18, 0x3f, 0x3f,
    0x28, 0x28, 0x24, 0x24, 0x02, 0x03, 0x02, 0x03,
    0x00, 0x01, 0x00, 0x01, 0x31, 0x31, 0x31, 0x31,
    0x30, 0x30, 0x30, 0x30, 0x2f, 0x2f, 0x2f, 0x2f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETGIP2,
    0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
    0x40, 0x40, 0x18, 0x18, 0x18, 0x18, 0x19, 0x19,
    0x40, 0x40, 0x18, 0x18, 0x18, 0x18, 0x3f, 0x3f,
    0x24, 0x24, 0x28, 0x28, 0x01, 0x00, 0x01, 0x00,
    0x03, 0x02, 0x03, 0x02, 0x31, 0x31, 0x31, 0x31,
    0x30, 0x30, 0x30, 0x30, 0x2f, 0x2f, 0x2f, 0x2f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETGIP3,
    0xaa, 0xea, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xea,
    0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xea, 0xab, 0xaa,
    0xaa, 0xaa, 0xaa, 0xea, 0xab, 0xaa, 0xaa, 0xaa);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETGIP3,
    0xaa, 0x2e, 0x28, 0x00, 0x00, 0x00, 0xaa, 0x2e,
    0x28, 0x00, 0x00, 0x00, 0xaa, 0xee, 0xaa, 0xaa,
    0xaa, 0xaa, 0xaa, 0xee, 0xaa, 0xaa, 0xaa, 0xaa);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETBANK, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETGIP3,
    0xaa, 0xff, 0xff, 0xff, 0xff, 0xff, 0xaa, 0xff,
    0xff, 0xff, 0xff, 0xff);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETBANK, 0x03);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETGIP3,
    0xaa, 0xaa, 0xea, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
    0xea, 0xaa, 0xaa, 0xaa, 0xaa, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xaa, 0xff, 0xff, 0xff, 0xff, 0xff);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETBANK, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETTP1,
    0x0e, 0x0e, 0x1e, 0x65, 0x1c, 0x65, 0x00, 0x50,
    0x20, 0x20, 0x00, 0x00, 0x02, 0x02, 0x02, 0x05,
    0x14, 0x14, 0x32, 0xb9, 0x23, 0xb9, 0x08);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETBANK, 0x01);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETTP1,
    0x02, 0x00, 0xa8, 0x01, 0xa8, 0x0d, 0xa4, 0x0e);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETBANK, 0x02);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETTP1,
    0x00, 0x00, 0x08, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00,
    0x00, 0x00, 0x00, 0x02, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETBANK, 0x00);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_UNKNOWN1, 0xc3);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETCLOCK, 0xd1, 0xd6);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_UNKNOWN1, 0x3f);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_UNKNOWN1, 0xc6);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_SETPTBA, 0x37);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, HX83112A_UNKNOWN1, 0x3f);
    mipi_dsi_dcs_exit_sleep_mode_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 150);
    mipi_dsi_dcs_set_display_on_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 50);
    return dsi_ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn hx83112a_disable(panel: *mut drm_panel) -> c_int {
    static int hx83112a_disable(struct drm_panel *panel)
    {
    struct hx83112a_panel *ctx = to_hx83112a_panel(panel);
    struct mipi_dsi_device *dsi = ctx.dsi;
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = dsi };
    dsi.mode_flags &= ~MIPI_DSI_MODE_LPM;
    mipi_dsi_dcs_set_display_off_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 20);
    mipi_dsi_dcs_enter_sleep_mode_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 120);
    return dsi_ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn hx83112a_prepare(panel: *mut drm_panel) -> c_int {
    static int hx83112a_prepare(struct drm_panel *panel)
    {
    struct hx83112a_panel *ctx = to_hx83112a_panel(panel);
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(ctx.supplies), ctx.supplies);
    if (ret < 0)
    return ret;
    hx83112a_reset(ctx);
    ret = hx83112a_on(ctx.dsi);
    if (ret < 0) {
    gpiod_set_value_cansleep(ctx.reset_gpio, 1);
    regulator_bulk_disable(ARRAY_SIZE(ctx.supplies), ctx.supplies);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hx83112a_unprepare(panel: *mut drm_panel) -> c_int {
    static int hx83112a_unprepare(struct drm_panel *panel)
    {
    struct hx83112a_panel *ctx = to_hx83112a_panel(panel);
    gpiod_set_value_cansleep(ctx.reset_gpio, 1);
    regulator_bulk_disable(ARRAY_SIZE(ctx.supplies), ctx.supplies);
    return 0;
    }
    static const struct drm_display_mode hx83112a_mode = {
    .clock = (1080 + 28 + 8 + 8) * (2340 + 27 + 5 + 5) * 60 / 1000,
    .hdisplay = 1080,
    .hsync_start = 1080 + 28,
    .hsync_end = 1080 + 28 + 8,
    .htotal = 1080 + 28 + 8 + 8,
    .vdisplay = 2340,
    .vsync_start = 2340 + 27,
    .vsync_end = 2340 + 27 + 5,
    .vtotal = 2340 + 27 + 5 + 5,
    .width_mm = 67,
    .height_mm = 145,
    .type = DRM_MODE_TYPE_DRIVER,
    };
    static int hx83112a_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    return drm_connector_helper_get_modes_fixed(connector, &hx83112a_mode);
    }
    static const struct drm_panel_funcs hx83112a_panel_funcs = {
    .prepare = hx83112a_prepare,
    .unprepare = hx83112a_unprepare,
    .disable = hx83112a_disable,
    .get_modes = hx83112a_get_modes,
    };
#[no_mangle]
unsafe extern "C" fn hx83112a_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int hx83112a_probe(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    struct hx83112a_panel *ctx;
    int ret;
    ctx = devm_drm_panel_alloc(dev, struct hx83112a_panel, panel,
    &hx83112a_panel_funcs,
    DRM_MODE_CONNECTOR_DSI);
    if (IS_ERR(ctx))
    return PTR_ERR(ctx);
    ctx.supplies[0].supply = "vdd1";
    ctx.supplies[1].supply = "vsn";
    ctx.supplies[2].supply = "vsp";
    ret = devm_regulator_bulk_get(dev, ARRAY_SIZE(ctx.supplies),
    ctx.supplies);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to get regulators\n");
    ctx.reset_gpio = devm_gpiod_get(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(ctx.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(ctx.reset_gpio),
    "Failed to get reset-gpios\n");
    ctx.dsi = dsi;
    mipi_dsi_set_drvdata(dsi, ctx);
    dsi.lanes = 4;
    dsi.format = MIPI_DSI_FMT_RGB888;
    dsi.mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_VIDEO_BURST |
    MIPI_DSI_MODE_VIDEO_HSE |
    MIPI_DSI_CLOCK_NON_CONTINUOUS;
    ctx.panel.prepare_prev_first = true;
    ret = drm_panel_of_backlight(&ctx.panel);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to get backlight\n");
    drm_panel_add(&ctx.panel);
    ret = mipi_dsi_attach(dsi);
    if (ret < 0) {
    dev_err_probe(dev, ret, "Failed to attach to DSI host\n");
    drm_panel_remove(&ctx.panel);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hx83112a_remove(dsi: *mut mipi_dsi_device) {
    static void hx83112a_remove(struct mipi_dsi_device *dsi)
    {
    struct hx83112a_panel *ctx = mipi_dsi_get_drvdata(dsi);
    int ret;
    ret = mipi_dsi_detach(dsi);
    if (ret < 0)
    dev_err(&dsi.dev, "Failed to detach from DSI host: %d\n", ret);
    drm_panel_remove(&ctx.panel);
    }
    static const struct of_device_id hx83112a_of_match[] = {
    { .compatible = "djn,9a-3r063-1102b" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, hx83112a_of_match);
    static struct mipi_dsi_driver hx83112a_driver = {
    .probe = hx83112a_probe,
    .remove = hx83112a_remove,
    .driver = {
    .name = "panel-himax-hx83112a",
    .of_match_table = hx83112a_of_match,
    },
    };
    module_mipi_dsi_driver(hx83112a_driver);
    MODULE_DESCRIPTION("DRM driver for hx83112a-equipped DSI panels");
    MODULE_LICENSE("GPL");
