//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-samsung-ltl106hl02.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_ltl106hl02 {
    pub panel: drm_panel,
    pub dsi: *mut mipi_dsi_device,
    pub supply: *mut regulator,
    pub reset_gpio: *mut gpio_desc,
}

    static inline struct samsung_ltl106hl02 *to_samsung_ltl106hl02(struct drm_panel *panel)
    {
    return container_of(panel, struct samsung_ltl106hl02, panel);
    }
#[no_mangle]
unsafe extern "C" fn samsung_ltl106hl02_reset(ctx: *mut samsung_ltl106hl02) {
    static void samsung_ltl106hl02_reset(struct samsung_ltl106hl02 *ctx)
    {
    gpiod_set_value_cansleep(ctx.reset_gpio, 1);
    usleep_range(10000, 11000);
    gpiod_set_value_cansleep(ctx.reset_gpio, 0);
    usleep_range(2000, 3000);
    }
#[no_mangle]
unsafe extern "C" fn samsung_ltl106hl02_prepare(panel: *mut drm_panel) -> c_int {
    static int samsung_ltl106hl02_prepare(struct drm_panel *panel)
    {
    struct samsung_ltl106hl02 *ctx = to_samsung_ltl106hl02(panel);
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = ctx.dsi };
    struct device *dev = &ctx.dsi.dev;
    int ret;
    ret = regulator_enable(ctx.supply);
    if (ret < 0) {
    dev_err(dev, "failed to enable power supply %d\n", ret);
    return ret;
    }
    if (ctx.reset_gpio)
    samsung_ltl106hl02_reset(ctx);
    mipi_dsi_dcs_exit_sleep_mode_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 70);
    mipi_dsi_dcs_set_display_on_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 5);
    return dsi_ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn samsung_ltl106hl02_unprepare(panel: *mut drm_panel) -> c_int {
    static int samsung_ltl106hl02_unprepare(struct drm_panel *panel)
    {
    struct samsung_ltl106hl02 *ctx = to_samsung_ltl106hl02(panel);
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = ctx.dsi };
    mipi_dsi_dcs_set_display_off_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 50);
    mipi_dsi_dcs_enter_sleep_mode_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 150);
    if (ctx.reset_gpio)
    gpiod_set_value_cansleep(ctx.reset_gpio, 1);
    regulator_disable(ctx.supply);
    return 0;
    }
    static const struct drm_display_mode samsung_ltl106hl02_mode = {
    .clock = (1920 + 32 + 32 + 64) * (1080 + 6 + 3 + 22) * 60 / 1000,
    .hdisplay = 1920,
    .hsync_start = 1920 + 32,
    .hsync_end = 1920 + 32 + 32,
    .htotal = 1920 + 32 + 32 + 64,
    .vdisplay = 1080,
    .vsync_start = 1080 + 6,
    .vsync_end = 1080 + 6 + 3,
    .vtotal = 1080 + 6 + 3 + 22,
    .width_mm = 235,
    .height_mm = 132,
    .type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED,
    };
    static int samsung_ltl106hl02_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    return drm_connector_helper_get_modes_fixed(connector, &samsung_ltl106hl02_mode);
    }
    static const struct drm_panel_funcs samsung_ltl106hl02_panel_funcs = {
    .prepare = samsung_ltl106hl02_prepare,
    .unprepare = samsung_ltl106hl02_unprepare,
    .get_modes = samsung_ltl106hl02_get_modes,
    };
#[no_mangle]
unsafe extern "C" fn samsung_ltl106hl02_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int samsung_ltl106hl02_probe(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    struct samsung_ltl106hl02 *ctx;
    int ret;
    ctx = devm_drm_panel_alloc(dev, struct samsung_ltl106hl02, panel,
    &samsung_ltl106hl02_panel_funcs,
    DRM_MODE_CONNECTOR_DSI);
    if (IS_ERR(ctx))
    return PTR_ERR(ctx);
    ctx.supply = devm_regulator_get(dev, "power");
    if (IS_ERR(ctx.supply))
    return dev_err_probe(dev, PTR_ERR(ctx.supply),
    "Failed to get power regulator\n");
    ctx.reset_gpio = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(ctx.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(ctx.reset_gpio),
    "Failed to get reset-gpios\n");
    ctx.dsi = dsi;
    mipi_dsi_set_drvdata(dsi, ctx);
    dsi.lanes = 4;
    dsi.format = MIPI_DSI_FMT_RGB888;
    dsi.mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_LPM;
    ret = drm_panel_of_backlight(&ctx.panel);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to get backlight\n");
    drm_panel_add(&ctx.panel);
    ret = devm_mipi_dsi_attach(dev, dsi);
    if (ret < 0) {
    drm_panel_remove(&ctx.panel);
    return dev_err_probe(dev, ret, "Failed to attach to DSI host\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn samsung_ltl106hl02_remove(dsi: *mut mipi_dsi_device) {
    static void samsung_ltl106hl02_remove(struct mipi_dsi_device *dsi)
    {
    struct samsung_ltl106hl02 *ctx = mipi_dsi_get_drvdata(dsi);
    drm_panel_remove(&ctx.panel);
    }
    static const struct of_device_id samsung_ltl106hl02_of_match[] = {
    { .compatible = "samsung,ltl106hl02-001" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, samsung_ltl106hl02_of_match);
    static struct mipi_dsi_driver samsung_ltl106hl02_driver = {
    .driver = {
    .name = "panel-samsung-ltl106hl02",
    .of_match_table = samsung_ltl106hl02_of_match,
    },
    .probe = samsung_ltl106hl02_probe,
    .remove = samsung_ltl106hl02_remove,
    };
    module_mipi_dsi_driver(samsung_ltl106hl02_driver);
    MODULE_AUTHOR("Anton Bambura <jenneron@protonmail.com>");
    MODULE_DESCRIPTION("DRM driver for Samsung LTL106HL02 video mode DSI panel");
    MODULE_LICENSE("GPL");
