//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-anbernic-td4310.c
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
// Driver for Anbernic panels with TD4310 panel controller.
//
// Copyright (C) 2026 Chris Morgan <macromorgan@hotmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct anbernic_panel_td4310_info {
    pub display_mode: *const drm_display_mode,
    pub width_mm: u16,
    pub height_mm: u16,
    pub bus_flags: u32,
    pub mode_flags: c_ulong,
    pub format: u32,
    pub lanes: u32,
    pub prepare_delay: u16,
    pub reset_delay: u16,
    pub init_delay: u16,
    pub enable_delay: u16,
    pub disable_delay: u16,
    pub unprepare_delay: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct anbernic_panel_td4310 {
    pub dev: *mut device,
    pub dsi: *mut mipi_dsi_device,
    pub panel: drm_panel,
    pub panel_info: *const anbernic_panel_td4310_info,
    pub reset_gpio: *mut gpio_desc,
    pub enable_gpio: *mut gpio_desc,
    pub vdd: *mut regulator,
    pub orientation: enum drm_panel_orientation,
}

    static inline struct anbernic_panel_td4310 *panel_to_anbernic_panel_td4310(struct drm_panel *panel)
    {
    return container_of(panel, struct anbernic_panel_td4310, panel);
    }
#[no_mangle]
unsafe extern "C" fn panel_anbernic_td4310_prepare(panel: *mut drm_panel) -> c_int {
    static int panel_anbernic_td4310_prepare(struct drm_panel *panel)
    {
    struct anbernic_panel_td4310 *ctx = panel_to_anbernic_panel_td4310(panel);
    struct mipi_dsi_device *dsi = ctx.dsi;
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = dsi };
    int ret;
    ret = regulator_enable(ctx.vdd);
    if (ret)
    return ret;
    ret = gpiod_set_value_cansleep(ctx.enable_gpio, 1);
    if (ret)
    goto err_enable;
    if (ctx.panel_info.enable_delay)
    mipi_dsi_msleep(&dsi_ctx, ctx.panel_info.enable_delay);
    ret = gpiod_set_value_cansleep(ctx.reset_gpio, 1);
    if (ret)
    goto err_reset;
    mipi_dsi_msleep(&dsi_ctx, 10);
    ret = gpiod_set_value_cansleep(ctx.reset_gpio, 0);
    if (ret)
    goto err_reset;
    if (ctx.panel_info.reset_delay)
    mipi_dsi_msleep(&dsi_ctx, ctx.panel_info.reset_delay);
    mipi_dsi_dcs_exit_sleep_mode_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, ctx.panel_info.prepare_delay);
    mipi_dsi_dcs_set_display_on_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, ctx.panel_info.prepare_delay);
    if (dsi_ctx.accum_err) {
    ret = dsi_ctx.accum_err;
    goto err_reset;
    }
    return 0;
    err_reset:
    gpiod_set_value_cansleep(ctx.enable_gpio, 0);
    err_enable:
    regulator_disable(ctx.vdd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn panel_anbernic_td4310_unprepare(panel: *mut drm_panel) -> c_int {
    static int panel_anbernic_td4310_unprepare(struct drm_panel *panel)
    {
    struct anbernic_panel_td4310 *ctx = panel_to_anbernic_panel_td4310(panel);
    struct mipi_dsi_device *dsi = ctx.dsi;
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = dsi };
    mipi_dsi_dcs_set_display_off_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, ctx.panel_info.unprepare_delay);
    mipi_dsi_dcs_enter_sleep_mode_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, ctx.panel_info.disable_delay);
    gpiod_set_value_cansleep(ctx.enable_gpio, 0);
    gpiod_set_value_cansleep(ctx.reset_gpio, 1);
    regulator_disable(ctx.vdd);
    return 0;
    }
    static int panel_anbernic_td4310_get_mode(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    struct anbernic_panel_td4310 *ctx = panel_to_anbernic_panel_td4310(panel);
    const struct anbernic_panel_td4310_info *panel_info = ctx.panel_info;
    connector.display_info.bpc = 8;
    connector.display_info.width_mm = panel_info.width_mm;
    connector.display_info.height_mm = panel_info.height_mm;
    connector.display_info.bus_flags = panel_info.bus_flags;
    return drm_connector_helper_get_modes_fixed(connector, panel_info.display_mode);
    }
#[no_mangle]
unsafe extern "C" fn panel_anbernic_td4310_get_orientation(panel: *mut drm_panel) -> enum drm_panel_orientation {
    static enum drm_panel_orientation panel_anbernic_td4310_get_orientation(struct drm_panel *panel)
    {
    struct anbernic_panel_td4310 *ctx = panel_to_anbernic_panel_td4310(panel);
    return ctx.orientation;
    }
    static const struct drm_panel_funcs panel_anbernic_td4310_funcs = {
    .prepare = panel_anbernic_td4310_prepare,
    .unprepare = panel_anbernic_td4310_unprepare,
    .get_modes = panel_anbernic_td4310_get_mode,
    .get_orientation = panel_anbernic_td4310_get_orientation,
    };
#[no_mangle]
unsafe extern "C" fn panel_anbernic_td4310_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int panel_anbernic_td4310_probe(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    struct anbernic_panel_td4310 *ctx;
    int ret;
    ctx = devm_drm_panel_alloc(dev, struct anbernic_panel_td4310, panel,
    &panel_anbernic_td4310_funcs,
    DRM_MODE_CONNECTOR_DSI);
    if (IS_ERR(ctx))
    return PTR_ERR(ctx);
    ctx.dev = dev;
    ctx.panel_info = of_device_get_match_data(dev);
    if (!ctx.panel_info)
    return -EINVAL;
    ret = drm_of_get_panel_orientation(dev.of_node, &ctx.orientation);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to get panel orientation\n");
    ctx.reset_gpio = devm_gpiod_get(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(ctx.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(ctx.reset_gpio),
    "Cannot get reset gpio\n");
    ctx.enable_gpio = devm_gpiod_get_optional(dev, "enable", GPIOD_OUT_LOW);
    if (IS_ERR(ctx.enable_gpio))
    return dev_err_probe(dev, PTR_ERR(ctx.enable_gpio),
    "Cannot get enable gpio\n");
    ctx.vdd = devm_regulator_get(dev, "vdd");
    if (IS_ERR(ctx.vdd))
    return dev_err_probe(dev, PTR_ERR(ctx.vdd),
    "Failed to request vdd regulator\n");
    ctx.dsi = dsi;
    mipi_dsi_set_drvdata(dsi, ctx);
    dsi.lanes = ctx.panel_info.lanes;
    dsi.format = ctx.panel_info.format;
    dsi.mode_flags = ctx.panel_info.mode_flags;
    ret = drm_panel_of_backlight(&ctx.panel);
    if (ret)
    return ret;
    devm_drm_panel_add(dev, &ctx.panel);
    ret = devm_mipi_dsi_attach(dev, dsi);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to attach to DSI host\n");
    return 0;
    }
    static const struct drm_display_mode anbernic_vitapro_mode = {
    .clock = 140020,
    .hdisplay = 1080,
    .hsync_start = 1080 + 50,
    .hsync_end = 1080 + 50 + 4,
    .htotal = 1080 + 50 + 4 + 50,
    .vdisplay = 1920,
    .vsync_start = 1920 + 15,
    .vsync_end = 1920 + 15 + 4,
    .vtotal = 1920 + 15 + 4 + 32,
    .flags = DRM_MODE_FLAG_NHSYNC | DRM_MODE_FLAG_NVSYNC,
    };
    static const struct anbernic_panel_td4310_info anbernic_vitapro_info = {
    .display_mode = &anbernic_vitapro_mode,
    .width_mm = 69,
    .height_mm = 121,
    .bus_flags = DRM_BUS_FLAG_DE_LOW | DRM_BUS_FLAG_PIXDATA_DRIVE_NEGEDGE,
    .mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_VIDEO_BURST |
    MIPI_DSI_MODE_LPM | MIPI_DSI_MODE_NO_EOT_PACKET |
    MIPI_DSI_CLOCK_NON_CONTINUOUS,
    .format = MIPI_DSI_FMT_RGB888,
    .lanes = 4,
    .prepare_delay = 50,
    .reset_delay = 220,
    .enable_delay = 120,
    .disable_delay = 50,
    .unprepare_delay = 20,
    };
    static const struct of_device_id panel_anbernic_td4310_of_match[] = {
    {
    .compatible = "anbernic,panel-vita-pro",
    .data = &anbernic_vitapro_info,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, panel_anbernic_td4310_of_match);
    static struct mipi_dsi_driver anbernic_panel_td4310_driver = {
    .driver = {
    .name = "panel-anbernic-td4310",
    .of_match_table = panel_anbernic_td4310_of_match,
    },
    .probe	= panel_anbernic_td4310_probe,
    };
    module_mipi_dsi_driver(anbernic_panel_td4310_driver);
    MODULE_AUTHOR("Chris Morgan <macromorgan@hotmail.com>");
    MODULE_DESCRIPTION("DRM driver for Anbernic TD4310 MIPI DSI panels");
    MODULE_LICENSE("GPL");
