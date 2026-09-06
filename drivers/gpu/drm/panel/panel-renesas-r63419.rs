//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-renesas-r63419.c
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
// DRM driver for Renesas R63419 based dual-DSI video mode panels
//
// Copyright (c) 2025, Kancy Joe <kancy2333@outlook.com>
// Copyright (C) 2026 Linaro Limited
// Author: Neil Armstrong <neil.armstrong@linaro.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct renesas_r63419_panel {
    pub panel: drm_panel,
    pub dsi: [*mut mipi_dsi_device; 2],
    pub desc: *const panel_desc,
    pub reset_gpio: *mut gpio_desc,
    pub vdd_supplies: *mut regulator_bulk_data,
    pub vcc_supplies: *mut regulator_bulk_data,
    pub orientation: enum drm_panel_orientation,
}

// VDDIO/VDD Supplies
    static const struct regulator_bulk_data renesas_r63419_vdd_supplies[] = {
    { .supply = "vddio" },
    { .supply = "vdd" },
    };
// VSP/VSN/VCI Supplies
    static const struct regulator_bulk_data renesas_r63419_vcc_supplies[] = {
    { .supply = "vsp" },
    { .supply = "vsn" },
    { .supply = "vci" },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panel_desc {
    pub mode: *const drm_display_mode,
    pub lanes: c_uint,
    pub mode_flags: c_ulong,
    pub format: enum mipi_dsi_pixel_format,
    pub dsi_info: mipi_dsi_device_info,
}

    static const struct drm_display_mode wt0600_mode = {
// Dual dsi
    .clock = 2 * (720 + 100 + 8 + 40) * (2560 + 15 + 2 + 8) * 60 / 1000,
    .hdisplay = 2 * 720,
    .hsync_start = 2 * (720 + 100),
    .hsync_end = 2 * (720 + 100 + 8),
    .htotal = 2 * (720 + 100 + 8 + 40),
    .vdisplay = 2560,
    .vsync_start = 2560 + 15,
    .vsync_end = 2560 + 15 + 2,
    .vtotal = 2560 + 15 + 2 + 8,
    .type = DRM_MODE_TYPE_DRIVER,
    .width_mm = 74,
    .height_mm = 131,
    };
    static const struct drm_display_mode wt0630_mode = {
// Dual dsi
    .clock = 2 * (720 + 100 + 8 + 40) * (2560 + 15 + 2 + 8) * 60 / 1000,
    .hdisplay = 2 * 720,
    .hsync_start = 2 * (720 + 100),
    .hsync_end = 2 * (720 + 100 + 8),
    .htotal = 2 * (720 + 100 + 8 + 40),
    .vdisplay = 2560,
    .vsync_start = 2560 + 15,
    .vsync_end = 2560 + 15 + 2,
    .vtotal = 2560 + 15 + 2 + 8,
    .type = DRM_MODE_TYPE_DRIVER,
    .width_mm = 78,
    .height_mm = 140,
    };
    static struct panel_desc wt0600_desc = {
    .lanes = 4,
    .mode = &wt0600_mode,
    .mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_VIDEO_BURST |
    MIPI_DSI_CLOCK_NON_CONTINUOUS | MIPI_DSI_MODE_LPM,
    .format = MIPI_DSI_FMT_RGB888,
    };
    static struct panel_desc wt0630_desc = {
    .lanes = 4,
    .mode = &wt0630_mode,  /* wt0600 only has different screen size */
    .mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_VIDEO_BURST |
    MIPI_DSI_CLOCK_NON_CONTINUOUS | MIPI_DSI_MODE_LPM,
    .format = MIPI_DSI_FMT_RGB888,
    };
    static inline struct renesas_r63419_panel *
    to_renesas_r63419_panel(struct drm_panel *panel)
    {
    return container_of(panel, struct renesas_r63419_panel, panel);
    }
#[no_mangle]
unsafe extern "C" fn renesas_r63419_on(ctx: *mut renesas_r63419_panel) -> c_int {
    static int renesas_r63419_on(struct renesas_r63419_panel *ctx)
    {
    let mut dsi_ctx: mipi_dsi_multi_context = { 0 };
//
// Panel registers are loaded from DDIC Non Volatile Memory
//
// The DDIC expects this sequence to get out of sleep and enable display
//
    mipi_dsi_dual(mipi_dsi_dcs_set_display_on_multi,
    &dsi_ctx, ctx.dsi[0], ctx.dsi[1]);
    mipi_dsi_msleep(&dsi_ctx, 50);
    mipi_dsi_dual(mipi_dsi_dcs_exit_sleep_mode_multi,
    &dsi_ctx, ctx.dsi[0], ctx.dsi[1]);
    mipi_dsi_msleep(&dsi_ctx, 150);
    return dsi_ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn renesas_r63419_disable(panel: *mut drm_panel) -> c_int {
    static int renesas_r63419_disable(struct drm_panel *panel)
    {
    struct renesas_r63419_panel *ctx = to_renesas_r63419_panel(panel);
    let mut dsi_ctx: mipi_dsi_multi_context = { 0 };
    mipi_dsi_dual(mipi_dsi_dcs_set_display_off_multi,
    &dsi_ctx, ctx.dsi[0], ctx.dsi[1]);
    mipi_dsi_msleep(&dsi_ctx, 50);
    mipi_dsi_dual(mipi_dsi_dcs_enter_sleep_mode_multi,
    &dsi_ctx, ctx.dsi[0], ctx.dsi[1]);
    mipi_dsi_msleep(&dsi_ctx, 120);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn renesas_r63419_prepare(panel: *mut drm_panel) -> c_int {
    static int renesas_r63419_prepare(struct drm_panel *panel)
    {
    struct renesas_r63419_panel *ctx = to_renesas_r63419_panel(panel);
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(renesas_r63419_vdd_supplies),
    ctx.vdd_supplies);
    if (ret < 0)
    return ret;
    usleep_range(1000, 2000);
    ret = regulator_bulk_enable(ARRAY_SIZE(renesas_r63419_vcc_supplies),
    ctx.vcc_supplies);
    if (ret < 0) {
    regulator_bulk_disable(ARRAY_SIZE(renesas_r63419_vdd_supplies),
    ctx.vdd_supplies);
    return ret;
    }
    usleep_range(1000, 2000);
    gpiod_set_value_cansleep(ctx.reset_gpio, 0);
    usleep_range(3000, 4000);
    ret = renesas_r63419_on(ctx);
    if (ret < 0) {
    dev_err(panel.dev, "Failed to initialize panel: %d\n", ret);
// Power off sequence from the r63419 datasheet
    regulator_bulk_disable(ARRAY_SIZE(renesas_r63419_vdd_supplies),
    ctx.vdd_supplies);
    gpiod_set_value_cansleep(ctx.reset_gpio, 1);
    regulator_bulk_disable(ARRAY_SIZE(renesas_r63419_vcc_supplies),
    ctx.vcc_supplies);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn renesas_r63419_unprepare(panel: *mut drm_panel) -> c_int {
    static int renesas_r63419_unprepare(struct drm_panel *panel)
    {
    struct renesas_r63419_panel *ctx = to_renesas_r63419_panel(panel);
// Power off sequence from the r63419 datasheet
    regulator_bulk_disable(ARRAY_SIZE(renesas_r63419_vdd_supplies), ctx.vdd_supplies);
    gpiod_set_value_cansleep(ctx.reset_gpio, 1);
    regulator_bulk_disable(ARRAY_SIZE(renesas_r63419_vcc_supplies), ctx.vcc_supplies);
    return 0;
    }
    static int renesas_r63419_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    struct renesas_r63419_panel *ctx = to_renesas_r63419_panel(panel);
    const struct drm_display_mode *mode = ctx.desc.mode;
    return drm_connector_helper_get_modes_fixed(connector, mode);
    }
    static enum drm_panel_orientation
    renesas_r63419_get_orientation(struct drm_panel *panel)
    {
    struct renesas_r63419_panel *ctx = to_renesas_r63419_panel(panel);
    return ctx.orientation;
    }
    static const struct drm_panel_funcs renesas_r63419_panel_funcs = {
    .disable = renesas_r63419_disable,
    .prepare = renesas_r63419_prepare,
    .unprepare = renesas_r63419_unprepare,
    .get_modes = renesas_r63419_get_modes,
    .get_orientation = renesas_r63419_get_orientation,
    };
#[no_mangle]
unsafe extern "C" fn renesas_r63419_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int renesas_r63419_probe(struct mipi_dsi_device *dsi)
    {
    let mut info: mipi_dsi_device_info = { 0 };
    struct device *dev = &dsi.dev;
    struct renesas_r63419_panel *ctx;
    struct device_node *dsi1_node;
    struct mipi_dsi_host *dsi1_host;
    int ret, i;
    ctx = devm_drm_panel_alloc(dev, struct renesas_r63419_panel, panel,
    &renesas_r63419_panel_funcs, DRM_MODE_CONNECTOR_DSI);
    if (IS_ERR(ctx))
    return PTR_ERR(ctx);
    ctx.desc = of_device_get_match_data(dev);
    if (!ctx.desc)
    return dev_err_probe(dev, -ENODEV,
    "Failed to get panel description\n");
    ret = devm_regulator_bulk_get_const(&dsi.dev,
    ARRAY_SIZE(renesas_r63419_vdd_supplies),
    renesas_r63419_vdd_supplies, &ctx.vdd_supplies);
    if (ret < 0)
    return ret;
    ret = devm_regulator_bulk_get_const(&dsi.dev,
    ARRAY_SIZE(renesas_r63419_vcc_supplies),
    renesas_r63419_vcc_supplies, &ctx.vcc_supplies);
    if (ret < 0)
    return ret;
    ctx.reset_gpio = devm_gpiod_get(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(ctx.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(ctx.reset_gpio),
    "Failed to get reset gpio\n");
// Get second DSI host
    dsi1_node = of_graph_get_remote_node(dsi.dev.of_node, 1, -1);
    if (!dsi1_node)
    return dev_err_probe(dev, -ENODEV,
    "Failed to get remote node for second DSI\n");
    dsi1_host = of_find_mipi_dsi_host_by_node(dsi1_node);
    of_node_put(dsi1_node);
    if (!dsi1_host)
    return dev_err_probe(dev, -EPROBE_DEFER,
    "Failed to find second DSI host\n");
// Copy current DSI info, do not provide OF node since no driver needs to be attached
    strscpy(info.type, dsi.name);
    info.channel = dsi.channel;
// Register the second DSI device
    ctx.dsi[1] = devm_mipi_dsi_device_register_full(dev, dsi1_host, &info);
    if (IS_ERR(ctx.dsi[1]))
    return dev_err_probe(dev, PTR_ERR(ctx.dsi[1]),
    "Failed to register second DSI device\n");
    ctx.dsi[0] = dsi;
    mipi_dsi_set_drvdata(dsi, ctx);
// Get panel orientation
    ret = drm_of_get_panel_orientation(dev.of_node, &ctx.orientation);
    if (ret < 0 && ret != -ENODEV)
    return dev_err_probe(dev, ret,
    "Failed to get panel orientation\n");
    ctx.panel.prepare_prev_first = true;
    ret = drm_panel_of_backlight(&ctx.panel);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to get backlight\n");
    ret = devm_drm_panel_add(dev, &ctx.panel);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to add panel\n");
// Configure and attach both DSI devices
    for (i = 0; i < ARRAY_SIZE(ctx.dsi); i++) {
    ctx.dsi[i].lanes = ctx.desc.lanes;
    ctx.dsi[i].format = ctx.desc.format;
    ctx.dsi[i].mode_flags = ctx.desc.mode_flags;
    ret = devm_mipi_dsi_attach(dev, ctx.dsi[i]);
    if (ret < 0)
    return dev_err_probe(dev, ret,
    "Failed to attach DSI device %d\n", i);
    }
    return 0;
    }
    static const struct of_device_id renesas_r63419_of_match[] = {
    {
    .compatible = "ayaneo,wt0600-2k",
    .data = &wt0600_desc,
    },
    {
    .compatible = "ayaneo,wt0630-2k",
    .data = &wt0630_desc,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, renesas_r63419_of_match);
    static struct mipi_dsi_driver renesas_r63419_driver = {
    .probe = renesas_r63419_probe,
    .driver = {
    .name = "panel-renesas-r63419",
    .of_match_table = renesas_r63419_of_match,
    },
    };
    module_mipi_dsi_driver(renesas_r63419_driver);
    MODULE_AUTHOR("Kancy Joe <kancy2333@outlook.com>");
    MODULE_AUTHOR("Neil Armstrong <neil.armstrong@linaro.org>");
    MODULE_DESCRIPTION("DRM driver for Renesas R63419 based dual-DSI video mode panels");
    MODULE_LICENSE("GPL");
