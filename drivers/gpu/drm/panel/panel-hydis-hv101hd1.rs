//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-hydis-hv101hd1.c
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
pub struct hv101hd1 {
    pub panel: drm_panel,
    pub dsi: *mut mipi_dsi_device,
    pub supplies: *mut regulator_bulk_data,
}

    static const struct regulator_bulk_data hv101hd1_supplies[] = {
    { .supply = "vdd" },
    { .supply = "vio" },
    };
    static inline struct hv101hd1 *to_hv101hd1(struct drm_panel *panel)
    {
    return container_of(panel, struct hv101hd1, panel);
    }
#[no_mangle]
unsafe extern "C" fn hv101hd1_prepare(panel: *mut drm_panel) -> c_int {
    static int hv101hd1_prepare(struct drm_panel *panel)
    {
    struct hv101hd1 *hv = to_hv101hd1(panel);
    let mut ctx: mipi_dsi_multi_context = { .dsi = hv.dsi };
    struct device *dev = &hv.dsi.dev;
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(hv101hd1_supplies), hv.supplies);
    if (ret) {
    dev_err(dev, "error enabling regulators (%d)\n", ret);
    return ret;
    }
    mipi_dsi_dcs_exit_sleep_mode_multi(&ctx);
    mipi_dsi_msleep(&ctx, 20);
    mipi_dsi_dcs_set_display_on_multi(&ctx);
    mipi_dsi_msleep(&ctx, 20);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hv101hd1_disable(panel: *mut drm_panel) -> c_int {
    static int hv101hd1_disable(struct drm_panel *panel)
    {
    struct hv101hd1 *hv = to_hv101hd1(panel);
    let mut ctx: mipi_dsi_multi_context = { .dsi = hv.dsi };
    mipi_dsi_dcs_set_display_off_multi(&ctx);
    mipi_dsi_msleep(&ctx, 120);
    mipi_dsi_dcs_enter_sleep_mode_multi(&ctx);
    mipi_dsi_msleep(&ctx, 20);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hv101hd1_unprepare(panel: *mut drm_panel) -> c_int {
    static int hv101hd1_unprepare(struct drm_panel *panel)
    {
    struct hv101hd1 *hv = to_hv101hd1(panel);
    return regulator_bulk_disable(ARRAY_SIZE(hv101hd1_supplies),
    hv.supplies);
    }
    static const struct drm_display_mode hv101hd1_mode = {
    .clock = (1366 + 74 + 36 + 24) * (768 + 21 + 7 + 4) * 60 / 1000,
    .hdisplay = 1366,
    .hsync_start = 1366 + 74,
    .hsync_end = 1366 + 74 + 36,
    .htotal = 1366 + 74 + 36 + 24,
    .vdisplay = 768,
    .vsync_start = 768 + 21,
    .vsync_end = 768 + 21 + 7,
    .vtotal = 768 + 21 + 7 + 4,
    .width_mm = 140,
    .height_mm = 220,
    };
#[no_mangle]
unsafe extern "C" fn hv101hd1_get_modes(panel: *mut drm_panel, connector: *mut drm_connector) -> c_int {
    static int hv101hd1_get_modes(struct drm_panel *panel, struct drm_connector *connector)
    {
    struct drm_display_mode *mode;
    mode = drm_mode_duplicate(connector.dev, &hv101hd1_mode);
    if (!mode)
    return -ENOMEM;
    drm_mode_set_name(mode);
    mode.type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED;
    connector.display_info.width_mm = mode.width_mm;
    connector.display_info.height_mm = mode.height_mm;
    drm_mode_probed_add(connector, mode);
    return 1;
    }
    static const struct drm_panel_funcs hv101hd1_panel_funcs = {
    .prepare = hv101hd1_prepare,
    .disable = hv101hd1_disable,
    .unprepare = hv101hd1_unprepare,
    .get_modes = hv101hd1_get_modes,
    };
#[no_mangle]
unsafe extern "C" fn hv101hd1_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int hv101hd1_probe(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    struct hv101hd1 *hv;
    int ret;
    hv = devm_drm_panel_alloc(dev, struct hv101hd1, panel,
    &hv101hd1_panel_funcs,
    DRM_MODE_CONNECTOR_DSI);
    if (IS_ERR(hv))
    return PTR_ERR(hv);
    ret = devm_regulator_bulk_get_const(dev, ARRAY_SIZE(hv101hd1_supplies),
    hv101hd1_supplies, &hv.supplies);
    if (ret)
    return dev_err_probe(dev, ret, "failed to get regulators\n");
    hv.dsi = dsi;
    mipi_dsi_set_drvdata(dsi, hv);
    dsi.lanes = 2;
    dsi.format = MIPI_DSI_FMT_RGB888;
    dsi.mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_LPM;
    ret = drm_panel_of_backlight(&hv.panel);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to get backlight\n");
    drm_panel_add(&hv.panel);
    ret = mipi_dsi_attach(dsi);
    if (ret) {
    drm_panel_remove(&hv.panel);
    return dev_err_probe(dev, ret, "Failed to attach to DSI host\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hv101hd1_remove(dsi: *mut mipi_dsi_device) {
    static void hv101hd1_remove(struct mipi_dsi_device *dsi)
    {
    struct hv101hd1 *hv = mipi_dsi_get_drvdata(dsi);
    int ret;
    ret = mipi_dsi_detach(dsi);
    if (ret < 0)
    dev_err(&dsi.dev,
    "Failed to detach from DSI host: %d\n", ret);
    drm_panel_remove(&hv.panel);
    }
    static const struct of_device_id hv101hd1_of_match[] = {
    { .compatible = "hydis,hv101hd1" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, hv101hd1_of_match);
    static struct mipi_dsi_driver hv101hd1_driver = {
    .driver = {
    .name = "panel-hv101hd1",
    .of_match_table = hv101hd1_of_match,
    },
    .probe = hv101hd1_probe,
    .remove = hv101hd1_remove,
    };
    module_mipi_dsi_driver(hv101hd1_driver);
    MODULE_AUTHOR("Svyatoslav Ryhel <clamor95@gmail.com>");
    MODULE_DESCRIPTION("DRM driver for Hydis HV101HD1 panel");
    MODULE_LICENSE("GPL");
