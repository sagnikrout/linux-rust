//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-sony-td4353-jdi.c
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
// Copyright (c) 2022 Konrad Dybcio <konrad.dybcio@somainline.org>
//
// Generated with linux-mdss-dsi-panel-driver-generator with a
// substantial amount of manual adjustments.
//
// SONY Downstream kernel calls this one:
// - "JDI ID3" for Akari  (XZ2)
// - "JDI ID4" for Apollo (XZ2 Compact)
//

    enum {
    TYPE_TAMA_60HZ,
//
// Leaving room for expansion - SONY very often uses
// *truly reliably* overclockable panels on their flagships!
//
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sony_td4353_jdi {
    pub panel: drm_panel,
    pub dsi: *mut mipi_dsi_device,
    pub supplies: [regulator_bulk_data; 3],
    pub panel_reset_gpio: *mut gpio_desc,
    pub touch_reset_gpio: *mut gpio_desc,
    pub type: c_int,
}

    static inline struct sony_td4353_jdi *to_sony_td4353_jdi(struct drm_panel *panel)
    {
    return container_of(panel, struct sony_td4353_jdi, panel);
    }
#[no_mangle]
unsafe extern "C" fn sony_td4353_jdi_on(ctx: *mut sony_td4353_jdi) -> c_int {
    static int sony_td4353_jdi_on(struct sony_td4353_jdi *ctx)
    {
    struct mipi_dsi_device *dsi = ctx.dsi;
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = dsi };
    dsi.mode_flags |= MIPI_DSI_MODE_LPM;
    mipi_dsi_dcs_set_column_address_multi(&dsi_ctx, 0x0000, 1080 - 1);
    mipi_dsi_dcs_set_page_address_multi(&dsi_ctx, 0x0000, 2160 - 1);
    mipi_dsi_dcs_set_tear_scanline_multi(&dsi_ctx, 0);
    mipi_dsi_dcs_set_tear_on_multi(&dsi_ctx, MIPI_DSI_DCS_TEAR_MODE_VBLANK);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, MIPI_DCS_SET_ADDRESS_MODE, 0x00);
    mipi_dsi_dcs_set_pixel_format_multi(&dsi_ctx, 0x77);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, MIPI_DCS_SET_PARTIAL_ROWS,
    0x00, 0x00, 0x08, 0x6f);
    mipi_dsi_dcs_exit_sleep_mode_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 70);
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, MIPI_DCS_WRITE_MEMORY_START);
    mipi_dsi_dcs_set_display_on_multi(&dsi_ctx);
    return dsi_ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn sony_td4353_jdi_off(ctx: *mut sony_td4353_jdi) {
    static void sony_td4353_jdi_off(struct sony_td4353_jdi *ctx)
    {
    struct mipi_dsi_device *dsi = ctx.dsi;
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = dsi };
    dsi.mode_flags &= ~MIPI_DSI_MODE_LPM;
    mipi_dsi_dcs_set_display_off_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 22);
    mipi_dsi_dcs_set_tear_off_multi(&dsi_ctx);
    mipi_dsi_dcs_enter_sleep_mode_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 80);
    }
#[no_mangle]
unsafe extern "C" fn sony_td4353_assert_reset_gpios(ctx: *mut sony_td4353_jdi, mode: c_int) {
    static void sony_td4353_assert_reset_gpios(struct sony_td4353_jdi *ctx, int mode)
    {
    gpiod_set_value_cansleep(ctx.touch_reset_gpio, mode);
    gpiod_set_value_cansleep(ctx.panel_reset_gpio, mode);
    usleep_range(5000, 5100);
    }
#[no_mangle]
unsafe extern "C" fn sony_td4353_jdi_prepare(panel: *mut drm_panel) -> c_int {
    static int sony_td4353_jdi_prepare(struct drm_panel *panel)
    {
    struct sony_td4353_jdi *ctx = to_sony_td4353_jdi(panel);
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(ctx.supplies), ctx.supplies);
    if (ret < 0)
    return ret;
    msleep(100);
    sony_td4353_assert_reset_gpios(ctx, 1);
    ret = sony_td4353_jdi_on(ctx);
    if (ret < 0) {
    sony_td4353_assert_reset_gpios(ctx, 0);
    regulator_bulk_disable(ARRAY_SIZE(ctx.supplies), ctx.supplies);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sony_td4353_jdi_unprepare(panel: *mut drm_panel) -> c_int {
    static int sony_td4353_jdi_unprepare(struct drm_panel *panel)
    {
    struct sony_td4353_jdi *ctx = to_sony_td4353_jdi(panel);
    sony_td4353_jdi_off(ctx);
    sony_td4353_assert_reset_gpios(ctx, 0);
    regulator_bulk_disable(ARRAY_SIZE(ctx.supplies), ctx.supplies);
    return 0;
    }
    static const struct drm_display_mode sony_td4353_jdi_mode_tama_60hz = {
    .clock = (1080 + 4 + 8 + 8) * (2160 + 259 + 8 + 8) * 60 / 1000,
    .hdisplay = 1080,
    .hsync_start = 1080 + 4,
    .hsync_end = 1080 + 4 + 8,
    .htotal = 1080 + 4 + 8 + 8,
    .vdisplay = 2160,
    .vsync_start = 2160 + 259,
    .vsync_end = 2160 + 259 + 8,
    .vtotal = 2160 + 259 + 8 + 8,
    .width_mm = 64,
    .height_mm = 128,
    };
    static int sony_td4353_jdi_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    struct sony_td4353_jdi *ctx = to_sony_td4353_jdi(panel);
    struct drm_display_mode *mode = core::ptr::null_mut();
    if (ctx.type == TYPE_TAMA_60HZ)
    mode = drm_mode_duplicate(connector.dev, &sony_td4353_jdi_mode_tama_60hz);
    else
    return -EINVAL;
    if (!mode)
    return -ENOMEM;
    drm_mode_set_name(mode);
    mode.type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED;
    connector.display_info.width_mm = mode.width_mm;
    connector.display_info.height_mm = mode.height_mm;
    drm_mode_probed_add(connector, mode);
    return 1;
    }
    static const struct drm_panel_funcs sony_td4353_jdi_panel_funcs = {
    .prepare = sony_td4353_jdi_prepare,
    .unprepare = sony_td4353_jdi_unprepare,
    .get_modes = sony_td4353_jdi_get_modes,
    };
#[no_mangle]
unsafe extern "C" fn sony_td4353_jdi_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int sony_td4353_jdi_probe(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    struct sony_td4353_jdi *ctx;
    int ret;
    ctx = devm_drm_panel_alloc(dev, struct sony_td4353_jdi, panel,
    &sony_td4353_jdi_panel_funcs,
    DRM_MODE_CONNECTOR_DSI);
    if (IS_ERR(ctx))
    return PTR_ERR(ctx);
    ctx.type = (uintptr_t)of_device_get_match_data(dev);
    ctx.supplies[0].supply = "vddio";
    ctx.supplies[1].supply = "vsp";
    ctx.supplies[2].supply = "vsn";
    ret = devm_regulator_bulk_get(dev, ARRAY_SIZE(ctx.supplies),
    ctx.supplies);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to get regulators\n");
    ctx.panel_reset_gpio = devm_gpiod_get(dev, "panel-reset", GPIOD_ASIS);
    if (IS_ERR(ctx.panel_reset_gpio))
    return dev_err_probe(dev, PTR_ERR(ctx.panel_reset_gpio),
    "Failed to get panel-reset-gpios\n");
    ctx.touch_reset_gpio = devm_gpiod_get(dev, "touch-reset", GPIOD_ASIS);
    if (IS_ERR(ctx.touch_reset_gpio))
    return dev_err_probe(dev, PTR_ERR(ctx.touch_reset_gpio),
    "Failed to get touch-reset-gpios\n");
    ctx.dsi = dsi;
    mipi_dsi_set_drvdata(dsi, ctx);
    dsi.lanes = 4;
    dsi.format = MIPI_DSI_FMT_RGB888;
    dsi.mode_flags = MIPI_DSI_CLOCK_NON_CONTINUOUS;
    ret = drm_panel_of_backlight(&ctx.panel);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to get backlight\n");
    ctx.panel.prepare_prev_first = true;
    drm_panel_add(&ctx.panel);
    ret = mipi_dsi_attach(dsi);
    if (ret < 0) {
    dev_err(dev, "Failed to attach to DSI host: %d\n", ret);
    drm_panel_remove(&ctx.panel);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sony_td4353_jdi_remove(dsi: *mut mipi_dsi_device) {
    static void sony_td4353_jdi_remove(struct mipi_dsi_device *dsi)
    {
    struct sony_td4353_jdi *ctx = mipi_dsi_get_drvdata(dsi);
    int ret;
    ret = mipi_dsi_detach(dsi);
    if (ret < 0)
    dev_err(&dsi.dev, "Failed to detach from DSI host: %d\n", ret);
    drm_panel_remove(&ctx.panel);
    }
    static const struct of_device_id sony_td4353_jdi_of_match[] = {
    { .compatible = "sony,td4353-jdi-tama", .data = (void *)TYPE_TAMA_60HZ },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, sony_td4353_jdi_of_match);
    static struct mipi_dsi_driver sony_td4353_jdi_driver = {
    .probe = sony_td4353_jdi_probe,
    .remove = sony_td4353_jdi_remove,
    .driver = {
    .name = "panel-sony-td4353-jdi",
    .of_match_table = sony_td4353_jdi_of_match,
    },
    };
    module_mipi_dsi_driver(sony_td4353_jdi_driver);
    MODULE_AUTHOR("Konrad Dybcio <konrad.dybcio@somainline.org>");
    MODULE_DESCRIPTION("DRM panel driver for SONY Xperia XZ2/XZ2c JDI panel");
    MODULE_LICENSE("GPL");
