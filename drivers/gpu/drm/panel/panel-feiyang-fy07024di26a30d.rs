//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-feiyang-fy07024di26a30d.c
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
//
// Copyright (C) 2018 Amarula Solutions
// Author: Jagan Teki <jagan@amarulasolutions.com>
//

pub const FEIYANG_INIT_CMD_LEN: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct feiyang {
    pub panel: drm_panel,
    pub dsi: *mut mipi_dsi_device,
    pub dvdd: *mut regulator,
    pub avdd: *mut regulator,
    pub reset: *mut gpio_desc,
}

    static inline struct feiyang *panel_to_feiyang(struct drm_panel *panel)
    {
    return container_of(panel, struct feiyang, panel);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct feiyang_init_cmd {
    pub data: [u8; FEIYANG_INIT_CMD_LEN],
}

    static const struct feiyang_init_cmd feiyang_init_cmds[] = {
    { .data = { 0x80, 0x58 } },
    { .data = { 0x81, 0x47 } },
    { .data = { 0x82, 0xD4 } },
    { .data = { 0x83, 0x88 } },
    { .data = { 0x84, 0xA9 } },
    { .data = { 0x85, 0xC3 } },
    { .data = { 0x86, 0x82 } },
    };
#[no_mangle]
unsafe extern "C" fn feiyang_prepare(panel: *mut drm_panel) -> c_int {
    static int feiyang_prepare(struct drm_panel *panel)
    {
    struct feiyang *ctx = panel_to_feiyang(panel);
    struct mipi_dsi_device *dsi = ctx.dsi;
    unsigned int i;
    int ret;
    ret = regulator_enable(ctx.dvdd);
    if (ret)
    return ret;
// T1 (dvdd start + dvdd rise) 0 < T1 <= 10ms
    msleep(10);
    ret = regulator_enable(ctx.avdd);
    if (ret)
    return ret;
// T3 (dvdd rise + avdd start + avdd rise) T3 >= 20ms
    msleep(20);
    gpiod_set_value(ctx.reset, 0);
//
// T5 + T6 (avdd rise + video & logic signal rise)
// T5 >= 10ms, 0 < T6 <= 10ms
//
    msleep(20);
    gpiod_set_value(ctx.reset, 1);
// T12 (video & logic signal rise + backlight rise) T12 >= 200ms
    msleep(200);
    for (i = 0; i < ARRAY_SIZE(feiyang_init_cmds); i++) {
    const struct feiyang_init_cmd *cmd =
    &feiyang_init_cmds[i];
    ret = mipi_dsi_dcs_write_buffer(dsi, cmd.data,
    FEIYANG_INIT_CMD_LEN);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn feiyang_enable(panel: *mut drm_panel) -> c_int {
    static int feiyang_enable(struct drm_panel *panel)
    {
    struct feiyang *ctx = panel_to_feiyang(panel);
// T12 (video & logic signal rise + backlight rise) T12 >= 200ms
    msleep(200);
    return mipi_dsi_dcs_set_display_on(ctx.dsi);
    }
#[no_mangle]
unsafe extern "C" fn feiyang_disable(panel: *mut drm_panel) -> c_int {
    static int feiyang_disable(struct drm_panel *panel)
    {
    struct feiyang *ctx = panel_to_feiyang(panel);
    return mipi_dsi_dcs_set_display_off(ctx.dsi);
    }
#[no_mangle]
unsafe extern "C" fn feiyang_unprepare(panel: *mut drm_panel) -> c_int {
    static int feiyang_unprepare(struct drm_panel *panel)
    {
    struct feiyang *ctx = panel_to_feiyang(panel);
    int ret;
    ret = mipi_dsi_dcs_set_display_off(ctx.dsi);
    if (ret < 0)
    dev_err(panel.dev, "failed to set display off: %d\n", ret);
    ret = mipi_dsi_dcs_enter_sleep_mode(ctx.dsi);
    if (ret < 0)
    dev_err(panel.dev, "failed to enter sleep mode: %d\n", ret);
// T13 (backlight fall + video & logic signal fall) T13 >= 200ms
    msleep(200);
    gpiod_set_value(ctx.reset, 0);
    regulator_disable(ctx.avdd);
// T11 (dvdd rise to fall) 0 < T11 <= 10ms
    msleep(10);
    regulator_disable(ctx.dvdd);
    return 0;
    }
    static const struct drm_display_mode feiyang_default_mode = {
    .clock		= 55000,
    .hdisplay	= 1024,
    .hsync_start	= 1024 + 310,
    .hsync_end	= 1024 + 310 + 20,
    .htotal		= 1024 + 310 + 20 + 90,
    .vdisplay	= 600,
    .vsync_start	= 600 + 12,
    .vsync_end	= 600 + 12 + 2,
    .vtotal		= 600 + 12 + 2 + 21,
    .type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED,
    };
    static int feiyang_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    struct feiyang *ctx = panel_to_feiyang(panel);
    struct drm_display_mode *mode;
    mode = drm_mode_duplicate(connector.dev, &feiyang_default_mode);
    if (!mode) {
    dev_err(&ctx.dsi.dev, "failed to add mode %ux%u@%u\n",
    feiyang_default_mode.hdisplay,
    feiyang_default_mode.vdisplay,
    drm_mode_vrefresh(&feiyang_default_mode));
    return -ENOMEM;
    }
    drm_mode_set_name(mode);
    drm_mode_probed_add(connector, mode);
    return 1;
    }
    static const struct drm_panel_funcs feiyang_funcs = {
    .disable = feiyang_disable,
    .unprepare = feiyang_unprepare,
    .prepare = feiyang_prepare,
    .enable = feiyang_enable,
    .get_modes = feiyang_get_modes,
    };
#[no_mangle]
unsafe extern "C" fn feiyang_dsi_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int feiyang_dsi_probe(struct mipi_dsi_device *dsi)
    {
    struct feiyang *ctx;
    int ret;
    ctx = devm_drm_panel_alloc(&dsi.dev, struct feiyang, panel,
    &feiyang_funcs, DRM_MODE_CONNECTOR_DSI);
    if (IS_ERR(ctx))
    return PTR_ERR(ctx);
    mipi_dsi_set_drvdata(dsi, ctx);
    ctx.dsi = dsi;
    ctx.dvdd = devm_regulator_get(&dsi.dev, "dvdd");
    if (IS_ERR(ctx.dvdd))
    return dev_err_probe(&dsi.dev, PTR_ERR(ctx.dvdd),
    "Couldn't get dvdd regulator\n");
    ctx.avdd = devm_regulator_get(&dsi.dev, "avdd");
    if (IS_ERR(ctx.avdd))
    return dev_err_probe(&dsi.dev, PTR_ERR(ctx.avdd),
    "Couldn't get avdd regulator\n");
    ctx.reset = devm_gpiod_get_optional(&dsi.dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(ctx.reset))
    return dev_err_probe(&dsi.dev, PTR_ERR(ctx.reset),
    "Couldn't get our reset GPIO\n");
    ret = drm_panel_of_backlight(&ctx.panel);
    if (ret)
    return ret;
    drm_panel_add(&ctx.panel);
    dsi.mode_flags = MIPI_DSI_MODE_VIDEO_BURST;
    dsi.format = MIPI_DSI_FMT_RGB888;
    dsi.lanes = 4;
    ret = mipi_dsi_attach(dsi);
    if (ret < 0) {
    drm_panel_remove(&ctx.panel);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn feiyang_dsi_remove(dsi: *mut mipi_dsi_device) {
    static void feiyang_dsi_remove(struct mipi_dsi_device *dsi)
    {
    struct feiyang *ctx = mipi_dsi_get_drvdata(dsi);
    mipi_dsi_detach(dsi);
    drm_panel_remove(&ctx.panel);
    }
    static const struct of_device_id feiyang_of_match[] = {
    { .compatible = "feiyang,fy07024di26a30d", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, feiyang_of_match);
    static struct mipi_dsi_driver feiyang_driver = {
    .probe = feiyang_dsi_probe,
    .remove = feiyang_dsi_remove,
    .driver = {
    .name = "feiyang-fy07024di26a30d",
    .of_match_table = feiyang_of_match,
    },
    };
    module_mipi_dsi_driver(feiyang_driver);
    MODULE_AUTHOR("Jagan Teki <jagan@amarulasolutions.com>");
    MODULE_DESCRIPTION("Feiyang FY07024DI26A30-D MIPI-DSI LCD panel");
    MODULE_LICENSE("GPL");
