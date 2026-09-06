//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-samsung-s6e3ha8.c
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
// Generated with linux-mdss-dsi-panel-driver-generator from vendor device tree:
// Copyright (c) 2013, The Linux Foundation. All rights reserved.
// Copyright (c) 2024 Dzmitry Sankouski <dsankouski@gmail.com>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s6e3ha8 {
    pub panel: drm_panel,
    pub dsi: *mut mipi_dsi_device,
    pub dsc: drm_dsc_config,
    pub reset_gpio: *mut gpio_desc,
    pub supplies: *mut regulator_bulk_data,
}

    static const struct regulator_bulk_data s6e3ha8_supplies[] = {
    { .supply = "vdd3" },
    { .supply = "vci" },
    { .supply = "vddr" },
    };
    static inline
    struct s6e3ha8 *to_s6e3ha8_amb577px01_wqhd(struct drm_panel *panel)
    {
    return container_of(panel, struct s6e3ha8, panel);
    }

    mipi_dsi_dcs_write_seq_multi(ctx, 0xf0, 0x5a, 0x5a)

    mipi_dsi_dcs_write_seq_multi(ctx, 0xf0, 0xa5, 0xa5)

    mipi_dsi_dcs_write_seq_multi(ctx, 0xfc, 0x5a, 0x5a)

    mipi_dsi_dcs_write_seq_multi(ctx, 0xfc, 0xa5, 0xa5)

    mipi_dsi_dcs_write_seq_multi(ctx, 0x9f, 0xa5, 0xa5)

    mipi_dsi_dcs_write_seq_multi(ctx, 0x9f, 0x5a, 0x5a)

    mipi_dsi_dcs_write_seq_multi(ctx, 0xe2, 0x00, 0x00)
#[no_mangle]
unsafe extern "C" fn s6e3ha8_amb577px01_wqhd_reset(priv: *mut s6e3ha8) {
    static void s6e3ha8_amb577px01_wqhd_reset(struct s6e3ha8 *priv)
    {
    gpiod_set_value_cansleep(priv.reset_gpio, 1);
    usleep_range(5000, 6000);
    gpiod_set_value_cansleep(priv.reset_gpio, 0);
    usleep_range(5000, 6000);
    gpiod_set_value_cansleep(priv.reset_gpio, 1);
    usleep_range(5000, 6000);
    }
#[no_mangle]
unsafe extern "C" fn s6e3ha8_amb577px01_wqhd_on(priv: *mut s6e3ha8) -> c_int {
    static int s6e3ha8_amb577px01_wqhd_on(struct s6e3ha8 *priv)
    {
    struct mipi_dsi_device *dsi = priv.dsi;
    let mut ctx: mipi_dsi_multi_context = { .dsi = dsi };
    dsi.mode_flags |= MIPI_DSI_MODE_LPM;
    s6e3ha8_test_key_on_lvl1(&ctx);
    s6e3ha8_test_key_on_lvl2(&ctx);
    mipi_dsi_compression_mode_multi(&ctx, true);
    s6e3ha8_test_key_off_lvl2(&ctx);
    mipi_dsi_dcs_exit_sleep_mode_multi(&ctx);
    usleep_range(5000, 6000);
    s6e3ha8_test_key_on_lvl2(&ctx);
    mipi_dsi_dcs_write_seq_multi(&ctx, 0xf2, 0x13);
    s6e3ha8_test_key_off_lvl2(&ctx);
    usleep_range(10000, 11000);
    s6e3ha8_test_key_on_lvl2(&ctx);
    mipi_dsi_dcs_write_seq_multi(&ctx, 0xf2, 0x13);
    s6e3ha8_test_key_off_lvl2(&ctx);
// OMOK setting 1 (Initial setting) - Scaler Latch Setting Guide
    s6e3ha8_test_key_on_lvl2(&ctx);
    mipi_dsi_dcs_write_seq_multi(&ctx, 0xb0, 0x07);
// latch setting 1 : Scaler on/off & address setting & PPS setting -> Image update latch
    mipi_dsi_dcs_write_seq_multi(&ctx, 0xf2, 0x3c, 0x10);
    mipi_dsi_dcs_write_seq_multi(&ctx, 0xb0, 0x0b);
// latch setting 2 : Ratio change mode -> Image update latch
    mipi_dsi_dcs_write_seq_multi(&ctx, 0xf2, 0x30);
// OMOK setting 2 - Seamless setting guide : WQHD
    mipi_dsi_dcs_write_seq_multi(&ctx, 0x2a, 0x00, 0x00, 0x05, 0x9f); /* CASET */
    mipi_dsi_dcs_write_seq_multi(&ctx, 0x2b, 0x00, 0x00, 0x0b, 0x8f); /* PASET */
    mipi_dsi_dcs_write_seq_multi(&ctx, 0xba, 0x01); /* scaler setup : scaler off */
    s6e3ha8_test_key_off_lvl2(&ctx);
    mipi_dsi_dcs_write_seq_multi(&ctx, 0x35, 0x00); /* TE Vsync ON */
    s6e3ha8_test_key_on_lvl2(&ctx);
    mipi_dsi_dcs_write_seq_multi(&ctx, 0xed, 0x4c); /* ERR_FG */
    s6e3ha8_test_key_off_lvl2(&ctx);
    s6e3ha8_test_key_on_lvl3(&ctx);
// FFC Setting 897.6Mbps
    mipi_dsi_dcs_write_seq_multi(&ctx, 0xc5, 0x0d, 0x10, 0xb4, 0x3e, 0x01);
    s6e3ha8_test_key_off_lvl3(&ctx);
    s6e3ha8_test_key_on_lvl2(&ctx);
    mipi_dsi_dcs_write_seq_multi(&ctx, 0xb9,
    0x00, 0xb0, 0x81, 0x09, 0x00, 0x00, 0x00,
    0x11, 0x03); /* TSP HSYNC Setting */
    s6e3ha8_test_key_off_lvl2(&ctx);
    s6e3ha8_test_key_on_lvl2(&ctx);
    mipi_dsi_dcs_write_seq_multi(&ctx, 0xb0, 0x03);
    mipi_dsi_dcs_write_seq_multi(&ctx, 0xf6, 0x43);
    s6e3ha8_test_key_off_lvl2(&ctx);
    s6e3ha8_test_key_on_lvl2(&ctx);
// Brightness condition set
    mipi_dsi_dcs_write_seq_multi(&ctx, 0xca,
    0x07, 0x00, 0x00, 0x00, 0x80, 0x80, 0x80,
    0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
    0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
    0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
    0x80, 0x80, 0x80, 0x00, 0x00, 0x00);
    mipi_dsi_dcs_write_seq_multi(&ctx, 0xb1, 0x00, 0x0c); /* AID Set : 0% */
    mipi_dsi_dcs_write_seq_multi(&ctx, 0xb5,
    0x19, 0xdc, 0x16, 0x01, 0x34, 0x67, 0x9a,
    0xcd, 0x01, 0x22, 0x33, 0x44, 0x00, 0x00,
    0x05, 0x55, 0xcc, 0x0c, 0x01, 0x11, 0x11,
    0x10); /* MPS/ELVSS Setting */
    mipi_dsi_dcs_write_seq_multi(&ctx, 0xf4, 0xeb, 0x28); /* VINT */
    mipi_dsi_dcs_write_seq_multi(&ctx, 0xf7, 0x03); /* Gamma, LTPS(AID) update */
    s6e3ha8_test_key_off_lvl2(&ctx);
    s6e3ha8_test_key_off_lvl1(&ctx);
    return ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn s6e3ha8_enable(panel: *mut drm_panel) -> c_int {
    static int s6e3ha8_enable(struct drm_panel *panel)
    {
    struct s6e3ha8 *priv = to_s6e3ha8_amb577px01_wqhd(panel);
    struct mipi_dsi_device *dsi = priv.dsi;
    let mut ctx: mipi_dsi_multi_context = { .dsi = dsi };
    s6e3ha8_test_key_on_lvl1(&ctx);
    mipi_dsi_dcs_set_display_on_multi(&ctx);
    s6e3ha8_test_key_off_lvl1(&ctx);
    return ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn s6e3ha8_disable(panel: *mut drm_panel) -> c_int {
    static int s6e3ha8_disable(struct drm_panel *panel)
    {
    struct s6e3ha8 *priv = to_s6e3ha8_amb577px01_wqhd(panel);
    struct mipi_dsi_device *dsi = priv.dsi;
    let mut ctx: mipi_dsi_multi_context = { .dsi = dsi };
    s6e3ha8_test_key_on_lvl1(&ctx);
    mipi_dsi_dcs_set_display_off_multi(&ctx);
    s6e3ha8_test_key_off_lvl1(&ctx);
    mipi_dsi_msleep(&ctx, 20);
    s6e3ha8_test_key_on_lvl2(&ctx);
    s6e3ha8_afc_off(&ctx);
    s6e3ha8_test_key_off_lvl2(&ctx);
    mipi_dsi_msleep(&ctx, 160);
    return ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn s6e3ha8_amb577px01_wqhd_prepare(panel: *mut drm_panel) -> c_int {
    static int s6e3ha8_amb577px01_wqhd_prepare(struct drm_panel *panel)
    {
    struct s6e3ha8 *priv = to_s6e3ha8_amb577px01_wqhd(panel);
    struct mipi_dsi_device *dsi = priv.dsi;
    let mut ctx: mipi_dsi_multi_context = { .dsi = dsi };
    struct drm_dsc_picture_parameter_set pps;
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(s6e3ha8_supplies), priv.supplies);
    if (ret < 0)
    return ret;
    mipi_dsi_msleep(&ctx, 120);
    s6e3ha8_amb577px01_wqhd_reset(priv);
    ret = s6e3ha8_amb577px01_wqhd_on(priv);
    if (ret < 0) {
    gpiod_set_value_cansleep(priv.reset_gpio, 1);
    goto err;
    }
    drm_dsc_pps_payload_pack(&pps, &priv.dsc);
    s6e3ha8_test_key_on_lvl1(&ctx);
    mipi_dsi_picture_parameter_set_multi(&ctx, &pps);
    s6e3ha8_test_key_off_lvl1(&ctx);
    mipi_dsi_msleep(&ctx, 28);
    return ctx.accum_err;
    err:
    regulator_bulk_disable(ARRAY_SIZE(s6e3ha8_supplies), priv.supplies);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn s6e3ha8_amb577px01_wqhd_unprepare(panel: *mut drm_panel) -> c_int {
    static int s6e3ha8_amb577px01_wqhd_unprepare(struct drm_panel *panel)
    {
    struct s6e3ha8 *priv = to_s6e3ha8_amb577px01_wqhd(panel);
    return regulator_bulk_disable(ARRAY_SIZE(s6e3ha8_supplies), priv.supplies);
    }
    static const struct drm_display_mode s6e3ha8_amb577px01_wqhd_mode = {
    .clock = (1440 + 116 + 44 + 120) * (2960 + 120 + 80 + 124) * 60 / 1000,
    .hdisplay = 1440,
    .hsync_start = 1440 + 116,
    .hsync_end = 1440 + 116 + 44,
    .htotal = 1440 + 116 + 44 + 120,
    .vdisplay = 2960,
    .vsync_start = 2960 + 120,
    .vsync_end = 2960 + 120 + 80,
    .vtotal = 2960 + 120 + 80 + 124,
    .width_mm = 64,
    .height_mm = 132,
    };
    static int s6e3ha8_amb577px01_wqhd_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    return drm_connector_helper_get_modes_fixed(connector, &s6e3ha8_amb577px01_wqhd_mode);
    }
    static const struct drm_panel_funcs s6e3ha8_amb577px01_wqhd_panel_funcs = {
    .prepare = s6e3ha8_amb577px01_wqhd_prepare,
    .unprepare = s6e3ha8_amb577px01_wqhd_unprepare,
    .get_modes = s6e3ha8_amb577px01_wqhd_get_modes,
    .enable = s6e3ha8_enable,
    .disable = s6e3ha8_disable,
    };
#[no_mangle]
unsafe extern "C" fn s6e3ha8_amb577px01_wqhd_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int s6e3ha8_amb577px01_wqhd_probe(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    struct s6e3ha8 *priv;
    int ret;
    priv = devm_drm_panel_alloc(dev, struct s6e3ha8, panel,
    &s6e3ha8_amb577px01_wqhd_panel_funcs,
    DRM_MODE_CONNECTOR_DSI);
    if (IS_ERR(priv))
    return PTR_ERR(priv);
    ret = devm_regulator_bulk_get_const(dev, ARRAY_SIZE(s6e3ha8_supplies),
    s6e3ha8_supplies,
    &priv.supplies);
    if (ret < 0) {
    dev_err(dev, "failed to get regulators: %d\n", ret);
    return ret;
    }
    priv.reset_gpio = devm_gpiod_get(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(priv.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(priv.reset_gpio),
    "Failed to get reset-gpios\n");
    priv.dsi = dsi;
    mipi_dsi_set_drvdata(dsi, priv);
    dsi.lanes = 4;
    dsi.format = MIPI_DSI_FMT_RGB888;
    dsi.mode_flags = MIPI_DSI_CLOCK_NON_CONTINUOUS |
    MIPI_DSI_MODE_VIDEO_NO_HFP | MIPI_DSI_MODE_VIDEO_NO_HBP |
    MIPI_DSI_MODE_VIDEO_NO_HSA | MIPI_DSI_MODE_NO_EOT_PACKET;
    priv.panel.prepare_prev_first = true;
    drm_panel_add(&priv.panel);
// This panel only supports DSC; unconditionally enable it
    dsi.dsc = &priv.dsc;
    priv.dsc.dsc_version_major = 1;
    priv.dsc.dsc_version_minor = 1;
    priv.dsc.slice_height = 40;
    priv.dsc.slice_width = 720;
    WARN_ON(1440 % priv.dsc.slice_width);
    priv.dsc.slice_count = 1440 / priv.dsc.slice_width;
    priv.dsc.bits_per_component = 8;
    priv.dsc.bits_per_pixel = 8 << 4; /* 4 fractional bits */
    priv.dsc.block_pred_enable = true;
    ret = mipi_dsi_attach(dsi);
    if (ret < 0) {
    dev_err(dev, "Failed to attach to DSI host: %d\n", ret);
    drm_panel_remove(&priv.panel);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s6e3ha8_amb577px01_wqhd_remove(dsi: *mut mipi_dsi_device) {
    static void s6e3ha8_amb577px01_wqhd_remove(struct mipi_dsi_device *dsi)
    {
    struct s6e3ha8 *priv = mipi_dsi_get_drvdata(dsi);
    int ret;
    ret = mipi_dsi_detach(dsi);
    if (ret < 0)
    dev_err(&dsi.dev, "Failed to detach from DSI host: %d\n", ret);
    drm_panel_remove(&priv.panel);
    }
    static const struct of_device_id s6e3ha8_amb577px01_wqhd_of_match[] = {
    { .compatible = "samsung,s6e3ha8" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, s6e3ha8_amb577px01_wqhd_of_match);
    static struct mipi_dsi_driver s6e3ha8_amb577px01_wqhd_driver = {
    .probe = s6e3ha8_amb577px01_wqhd_probe,
    .remove = s6e3ha8_amb577px01_wqhd_remove,
    .driver = {
    .name = "panel-s6e3ha8",
    .of_match_table = s6e3ha8_amb577px01_wqhd_of_match,
    },
    };
    module_mipi_dsi_driver(s6e3ha8_amb577px01_wqhd_driver);
    MODULE_AUTHOR("Dzmitry Sankouski <dsankouski@gmail.com>");
    MODULE_DESCRIPTION("DRM driver for S6E3HA8 panel");
    MODULE_LICENSE("GPL");
