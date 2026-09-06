//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-lxd-m9189a.c
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
pub const EK79007AD3_GAMMA1: c_uint = 0x80;
pub const EK79007AD3_GAMMA2: c_uint = 0x81;
pub const EK79007AD3_GAMMA3: c_uint = 0x82;
pub const EK79007AD3_GAMMA4: c_uint = 0x83;
pub const EK79007AD3_GAMMA5: c_uint = 0x84;
pub const EK79007AD3_GAMMA6: c_uint = 0x85;
pub const EK79007AD3_GAMMA7: c_uint = 0x86;
pub const EK79007AD3_PANEL_CTRL3: c_uint = 0xB2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct m9189_panel {
    pub panel: drm_panel,
    pub dsi: *mut mipi_dsi_device,
    pub supply: *mut regulator,
    pub reset_gpio: *mut gpio_desc,
    pub standby_gpio: *mut gpio_desc,
}

    static inline struct m9189_panel *to_m9189_panel(struct drm_panel *panel)
    {
    return container_of(panel, struct m9189_panel, panel);
    }
#[no_mangle]
unsafe extern "C" fn m9189_reset(m9189: *mut m9189_panel) {
    static void m9189_reset(struct m9189_panel *m9189)
    {
    gpiod_set_value_cansleep(m9189.reset_gpio, 0);
    msleep(20);
    gpiod_set_value_cansleep(m9189.reset_gpio, 1);
    msleep(30);
    gpiod_set_value_cansleep(m9189.reset_gpio, 0);
    msleep(55);
    }
#[no_mangle]
unsafe extern "C" fn m9189_on(m9189: *mut m9189_panel) -> c_int {
    static int m9189_on(struct m9189_panel *m9189)
    {
    let mut ctx: mipi_dsi_multi_context = { .dsi = m9189.dsi };
    ctx.dsi.mode_flags |= MIPI_DSI_MODE_LPM;
// Gamma 2.2
    mipi_dsi_dcs_write_seq_multi(&ctx, EK79007AD3_GAMMA1, 0x48);
    mipi_dsi_dcs_write_seq_multi(&ctx, EK79007AD3_GAMMA2, 0xB8);
    mipi_dsi_dcs_write_seq_multi(&ctx, EK79007AD3_GAMMA3, 0x88);
    mipi_dsi_dcs_write_seq_multi(&ctx, EK79007AD3_GAMMA4, 0x88);
    mipi_dsi_dcs_write_seq_multi(&ctx, EK79007AD3_GAMMA5, 0x58);
    mipi_dsi_dcs_write_seq_multi(&ctx, EK79007AD3_GAMMA6, 0xD2);
    mipi_dsi_dcs_write_seq_multi(&ctx, EK79007AD3_GAMMA7, 0x88);
    mipi_dsi_msleep(&ctx, 50);
// 4 Lanes
    mipi_dsi_generic_write_multi(&ctx, (u8[]){ EK79007AD3_PANEL_CTRL3, 0x70 }, 2);
    mipi_dsi_dcs_exit_sleep_mode_multi(&ctx);
    mipi_dsi_msleep(&ctx, 120);
    mipi_dsi_dcs_set_display_on_multi(&ctx);
    mipi_dsi_msleep(&ctx, 120);
    return ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn m9189_disable(panel: *mut drm_panel) -> c_int {
    static int m9189_disable(struct drm_panel *panel)
    {
    struct m9189_panel *m9189 = to_m9189_panel(panel);
    let mut ctx: mipi_dsi_multi_context = { .dsi = m9189.dsi };
    ctx.dsi.mode_flags &= ~MIPI_DSI_MODE_LPM;
    mipi_dsi_dcs_enter_sleep_mode_multi(&ctx);
    mipi_dsi_msleep(&ctx, 120);
    gpiod_set_value_cansleep(m9189.standby_gpio, 1);
    return ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn m9189_prepare(panel: *mut drm_panel) -> c_int {
    static int m9189_prepare(struct drm_panel *panel)
    {
    struct m9189_panel *m9189 = to_m9189_panel(panel);
    struct device *dev = &m9189.dsi.dev;
    int ret;
    ret = regulator_enable(m9189.supply);
    if (ret < 0) {
    dev_err(dev, "Failed to enable regulators: %d\n", ret);
    return ret;
    }
    gpiod_set_value_cansleep(m9189.standby_gpio, 0);
    msleep(20);
    m9189_reset(m9189);
    ret = m9189_on(m9189);
    if (ret < 0) {
    dev_err(dev, "Failed to initialize panel: %d\n", ret);
    gpiod_set_value_cansleep(m9189.reset_gpio, 1);
    regulator_disable(m9189.supply);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn m9189_unprepare(panel: *mut drm_panel) -> c_int {
    static int m9189_unprepare(struct drm_panel *panel)
    {
    struct m9189_panel *m9189 = to_m9189_panel(panel);
    gpiod_set_value_cansleep(m9189.standby_gpio, 1);
    msleep(50);
    gpiod_set_value_cansleep(m9189.reset_gpio, 1);
    regulator_disable(m9189.supply);
    return 0;
    }
    static const struct drm_display_mode m9189_mode = {
    .clock = (1024 + 160 + 160 + 10) * (600 + 12 + 23 + 1) * 60 / 1000,
    .hdisplay = 1024,
    .hsync_start = 1024 + 160,
    .hsync_end = 1024 + 160 + 160,
    .htotal = 1024 + 160 + 160 + 10,
    .vdisplay = 600,
    .vsync_start = 600 + 12,
    .vsync_end = 600 + 12 + 23,
    .vtotal = 600 + 12 + 23 + 1,
    .width_mm = 154,
    .height_mm = 86,
    };
    static int m9189_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    return drm_connector_helper_get_modes_fixed(connector, &m9189_mode);
    }
    static const struct drm_panel_funcs m9189_panel_funcs = {
    .prepare = m9189_prepare,
    .unprepare = m9189_unprepare,
    .disable = m9189_disable,
    .get_modes = m9189_get_modes,
    };
#[no_mangle]
unsafe extern "C" fn lxd_m9189_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int lxd_m9189_probe(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    struct m9189_panel *m9189;
    int ret;
    m9189 = devm_drm_panel_alloc(dev, __typeof(*m9189), panel,
    &m9189_panel_funcs,
    DRM_MODE_CONNECTOR_DSI);
    if (IS_ERR(m9189))
    return PTR_ERR(m9189);
    m9189.supply = devm_regulator_get(dev, "power");
    if (IS_ERR(m9189.supply))
    return dev_err_probe(dev, PTR_ERR(m9189.supply),
    "Failed to get power-supply\n");
    m9189.reset_gpio = devm_gpiod_get(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(m9189.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(m9189.reset_gpio),
    "Failed to get reset-gpios\n");
    m9189.standby_gpio = devm_gpiod_get(dev, "standby", GPIOD_OUT_LOW);
    if (IS_ERR(m9189.standby_gpio))
    return dev_err_probe(dev, PTR_ERR(m9189.standby_gpio),
    "Failed to get standby-gpios\n");
    m9189.dsi = dsi;
    mipi_dsi_set_drvdata(dsi, m9189);
    dsi.lanes = 4;
    dsi.format = MIPI_DSI_FMT_RGB888;
    dsi.mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_VIDEO_BURST;
    m9189.panel.prepare_prev_first = true;
    ret = drm_panel_of_backlight(&m9189.panel);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to get backlight\n");
    drm_panel_add(&m9189.panel);
    ret = mipi_dsi_attach(dsi);
    if (ret < 0) {
    dev_err_probe(dev, ret, "Failed to attach to DSI host\n");
    drm_panel_remove(&m9189.panel);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lxd_m9189_remove(dsi: *mut mipi_dsi_device) {
    static void lxd_m9189_remove(struct mipi_dsi_device *dsi)
    {
    struct m9189_panel *m9189 = mipi_dsi_get_drvdata(dsi);
    int ret;
    ret = mipi_dsi_detach(dsi);
    if (ret < 0)
    dev_err(&dsi.dev, "Failed to detach from DSI host: %d\n", ret);
    drm_panel_remove(&m9189.panel);
    }
    static const struct of_device_id lxd_m9189_of_match[] = {
    { .compatible = "lxd,m9189a" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, lxd_m9189_of_match);
    static struct mipi_dsi_driver lxd_m9189_driver = {
    .probe = lxd_m9189_probe,
    .remove = lxd_m9189_remove,
    .driver = {
    .name = "panel-lxd-m9189a",
    .of_match_table = lxd_m9189_of_match,
    },
    };
    module_mipi_dsi_driver(lxd_m9189_driver);
    MODULE_DESCRIPTION("DRM driver for LXD M9189A MIPI-DSI panels");
    MODULE_LICENSE("GPL");
