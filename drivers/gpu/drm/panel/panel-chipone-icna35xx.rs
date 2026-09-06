//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-chipone-icna35xx.c
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
// Chipone ICNA35XX Driver IC panels driver
//
// Copyright (c) 2025 Teguh Sobirin <teguh@sobir.in>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panel_info {
    pub panel: drm_panel,
    pub connector: *mut drm_connector,
    pub dsi: *mut mipi_dsi_device,
    pub desc: *mut panel_desc,
    pub orientation: enum drm_panel_orientation,
    pub reset_gpio: *mut gpio_desc,
    pub supplies: *mut regulator_bulk_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panel_desc {
    pub width_mm: c_uint,
    pub height_mm: c_uint,
    pub bpc: c_uint,
    pub lanes: c_uint,
    pub mode_flags: c_ulong,
    pub format: enum mipi_dsi_pixel_format,
    pub modes: *const drm_display_mode,
    pub num_modes: c_uint,
    pub pinfo): *mut *mut int (init_sequence)(struct panel_info,
    pub dsc: drm_dsc_config,
}

    static const struct regulator_bulk_data panel_supplies[] = {
    { .supply = "vdd" },
    { .supply = "vddio" },
    { .supply = "vci" },
    { .supply = "disp" },
    { .supply = "blvdd" },
    };
    static inline struct panel_info *to_panel_info(struct drm_panel *panel)
    {
    return container_of(panel, struct panel_info, panel);
    }
#[no_mangle]
unsafe extern "C" fn icna3512_init_sequence(pinfo: *mut panel_info) -> c_int {
    static int icna3512_init_sequence(struct panel_info *pinfo)
    {
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = pinfo.dsi };
    struct drm_dsc_picture_parameter_set pps;
    pinfo.dsi.mode_flags |= MIPI_DSI_MODE_LPM;
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0x9C, 0xA5, 0xA5);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0xFD, 0x5A, 0x5A);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0x53, 0xE0);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0x35, 0x00);
    mipi_dsi_dcs_exit_sleep_mode_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 120);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0x9F, 0x0F);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0xCE, 0x22);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0x9F, 0x01);
// 165 hz
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0x48, 0x20);
    drm_dsc_pps_payload_pack(&pps, &pinfo.desc.dsc);
    mipi_dsi_picture_parameter_set_multi(&dsi_ctx, &pps);
    mipi_dsi_msleep(&dsi_ctx, 20);
    mipi_dsi_dcs_set_display_on_multi(&dsi_ctx);
    return dsi_ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn icna3520_init_sequence(pinfo: *mut panel_info) -> c_int {
    static int icna3520_init_sequence(struct panel_info *pinfo)
    {
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = pinfo.dsi };
    struct drm_dsc_picture_parameter_set pps;
    pinfo.dsi.mode_flags |= MIPI_DSI_MODE_LPM;
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0x9C, 0xA5, 0xA5);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0xFD, 0x5A, 0x5A);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0x53, 0xE0);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0x35, 0x00);
    mipi_dsi_dcs_exit_sleep_mode_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 120);
// 120 hz
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0x48, 0x00);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0x9F, 0x00);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0xB3,
    0x00, 0xD8, 0x00, 0x1C, 0x00, 0x4C);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0x9F, 0x01);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0xB2, 0x00);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0x9F, 0x0D);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0xB2, 0x27);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0xB6, 0x03);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0xBB, 0x01);
    mipi_dsi_generic_write_seq_multi(&dsi_ctx, 0xB2, 0x24);
    drm_dsc_pps_payload_pack(&pps, &pinfo.desc.dsc);
    mipi_dsi_picture_parameter_set_multi(&dsi_ctx, &pps);
    mipi_dsi_msleep(&dsi_ctx, 20);
    mipi_dsi_dcs_set_display_on_multi(&dsi_ctx);
    return dsi_ctx.accum_err;
    }
    static const struct drm_display_mode odin2portal_modes[] = {
    {
// 165Hz
    .clock = (1080 + 98 + 1 + 23) * (1920 + 20 + 1 + 15) * 165 / 1000,
    .hdisplay = 1080,
    .hsync_start = 1080 + 98,
    .hsync_end = 1080 + 98 + 1,
    .htotal = 1080 + 98 + 1 + 23,
    .vdisplay = 1920,
    .vsync_start = 1920 + 20,
    .vsync_end = 1920 + 20 + 1,
    .vtotal = 1920 + 20 + 1 + 15,
    }
    };
    static const struct drm_display_mode thor_top_modes[] = {
    {
// 120Hz
    .clock = (1080 + 24 + 1 + 24) * (1920 + 28 + 1 + 28) * 120 / 1000,
    .hdisplay = 1080,
    .hsync_start = 1080 + 24,
    .hsync_end = 1080 + 24 + 1,
    .htotal = 1080 + 24 + 1 + 24,
    .vdisplay = 1920,
    .vsync_start = 1920 + 28,
    .vsync_end = 1920 + 28 + 1,
    .vtotal = 1920 + 28 + 1 + 28,
    }
    };
    static struct panel_desc odin2portal_desc = {
    .modes = odin2portal_modes,
    .num_modes = ARRAY_SIZE(odin2portal_modes),
    .width_mm = 160,
    .height_mm = 89,
    .bpc = 8,
    .lanes = 4,
    .format = MIPI_DSI_FMT_RGB888,
    .mode_flags = MIPI_DSI_MODE_NO_EOT_PACKET | MIPI_DSI_CLOCK_NON_CONTINUOUS |
    MIPI_DSI_MODE_LPM,
    .init_sequence = icna3512_init_sequence,
    .dsc = {
    .dsc_version_major = 0x1,
    .dsc_version_minor = 0x1,
    .slice_height = 20,
    .slice_width = 540,
    .slice_count = 2,
    .bits_per_component = 8,
    .bits_per_pixel = 8 << 4,
    .block_pred_enable = true,
    },
    };
    static struct panel_desc thor_top_desc = {
    .modes = thor_top_modes,
    .num_modes = ARRAY_SIZE(thor_top_modes),
    .width_mm = 136,
    .height_mm = 68,
    .bpc = 8,
    .lanes = 4,
    .format = MIPI_DSI_FMT_RGB888,
    .mode_flags =  MIPI_DSI_MODE_NO_EOT_PACKET | MIPI_DSI_CLOCK_NON_CONTINUOUS |
    MIPI_DSI_MODE_LPM,
    .init_sequence = icna3520_init_sequence,
    .dsc = {
    .dsc_version_major = 0x1,
    .dsc_version_minor = 0x1,
    .slice_height = 12,
    .slice_width = 540,
    .slice_count = 2,
    .bits_per_component = 8,
    .bits_per_pixel = 8 << 4,
    .block_pred_enable = true,
    },
    };
#[no_mangle]
unsafe extern "C" fn icna35xx_reset(pinfo: *mut panel_info) {
    static void icna35xx_reset(struct panel_info *pinfo)
    {
    gpiod_set_value_cansleep(pinfo.reset_gpio, 0);
    usleep_range(20000, 21000);
    gpiod_set_value_cansleep(pinfo.reset_gpio, 1);
    usleep_range(20000, 21000);
    gpiod_set_value_cansleep(pinfo.reset_gpio, 0);
    usleep_range(20000, 21000);
    }
#[no_mangle]
unsafe extern "C" fn icna35xx_prepare(panel: *mut drm_panel) -> c_int {
    static int icna35xx_prepare(struct drm_panel *panel)
    {
    struct panel_info *pinfo = to_panel_info(panel);
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(panel_supplies), pinfo.supplies);
    if (ret < 0) {
    dev_err(panel.dev, "failed to enable regulators: %d\n", ret);
    return ret;
    }
    icna35xx_reset(pinfo);
    ret = pinfo.desc.init_sequence(pinfo);
    if (ret < 0) {
    regulator_bulk_disable(ARRAY_SIZE(panel_supplies), pinfo.supplies);
    dev_err(panel.dev, "failed to initialize panel: %d\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn icna35xx_disable(panel: *mut drm_panel) -> c_int {
    static int icna35xx_disable(struct drm_panel *panel)
    {
    struct panel_info *pinfo = to_panel_info(panel);
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = pinfo.dsi };
    pinfo.dsi.mode_flags &= ~MIPI_DSI_MODE_LPM;
    mipi_dsi_dcs_set_display_off_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 50);
    mipi_dsi_dcs_enter_sleep_mode_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 120);
    return dsi_ctx.accum_err;
    }
#[no_mangle]
unsafe extern "C" fn icna35xx_unprepare(panel: *mut drm_panel) -> c_int {
    static int icna35xx_unprepare(struct drm_panel *panel)
    {
    struct panel_info *pinfo = to_panel_info(panel);
    gpiod_set_value_cansleep(pinfo.reset_gpio, 1);
    regulator_bulk_disable(ARRAY_SIZE(panel_supplies), pinfo.supplies);
    return 0;
    }
    static int icna35xx_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    struct panel_info *pinfo = to_panel_info(panel);
    return drm_connector_helper_get_modes_fixed(connector, pinfo.desc.modes);
    }
#[no_mangle]
unsafe extern "C" fn icna35xx_get_orientation(panel: *mut drm_panel) -> enum drm_panel_orientation {
    static enum drm_panel_orientation icna35xx_get_orientation(struct drm_panel *panel)
    {
    struct panel_info *pinfo = to_panel_info(panel);
    return pinfo.orientation;
    }
    static const struct drm_panel_funcs icna35xx_panel_funcs = {
    .disable = icna35xx_disable,
    .prepare = icna35xx_prepare,
    .unprepare = icna35xx_unprepare,
    .get_modes = icna35xx_get_modes,
    .get_orientation = icna35xx_get_orientation,
    };
#[no_mangle]
unsafe extern "C" fn icna35xx_bl_update_status(bl: *mut backlight_device) -> c_int {
    static int icna35xx_bl_update_status(struct backlight_device *bl)
    {
    struct mipi_dsi_device *dsi = bl_get_data(bl);
    let mut brightness: u16 = backlight_get_brightness(bl);
    int ret;
    dsi.mode_flags &= ~MIPI_DSI_MODE_LPM;
    ret = mipi_dsi_dcs_set_display_brightness_large(dsi, brightness);
    dsi.mode_flags |= MIPI_DSI_MODE_LPM;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn icna35xx_bl_get_brightness(bl: *mut backlight_device) -> c_int {
    static int icna35xx_bl_get_brightness(struct backlight_device *bl)
    {
    struct mipi_dsi_device *dsi = bl_get_data(bl);
    u16 brightness;
    int ret;
    dsi.mode_flags &= ~MIPI_DSI_MODE_LPM;
    ret = mipi_dsi_dcs_get_display_brightness_large(dsi, &brightness);
    dsi.mode_flags |= MIPI_DSI_MODE_LPM;
    return ret < 0 ? ret : brightness;
    }
    static const struct backlight_ops icna35xx_bl_ops = {
    .update_status = icna35xx_bl_update_status,
    .get_brightness = icna35xx_bl_get_brightness,
    };
    static struct backlight_device *icna35xx_create_backlight(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    const struct backlight_properties props = {
    .type = BACKLIGHT_RAW,
    .brightness = 4096,
    .max_brightness = 4096,
    };
    return devm_backlight_device_register(dev, dev_name(dev), dev, dsi,
    &icna35xx_bl_ops, &props);
    }
#[no_mangle]
unsafe extern "C" fn icna35xx_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int icna35xx_probe(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    struct panel_info *pinfo;
    int ret;
    pinfo = devm_drm_panel_alloc(dev, __typeof(*pinfo), panel,
    &icna35xx_panel_funcs,
    DRM_MODE_CONNECTOR_DSI);
    if (IS_ERR(pinfo))
    return PTR_ERR(pinfo);
    ret = devm_regulator_bulk_get_const(dev, ARRAY_SIZE(panel_supplies),
    panel_supplies, &pinfo.supplies);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to get regulators\n");
    pinfo.reset_gpio = devm_gpiod_get(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(pinfo.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(pinfo.reset_gpio), "failed to get reset gpio\n");
    pinfo.desc = (struct panel_desc *)of_device_get_match_data(dev);
    if (!pinfo.desc)
    return -ENODEV;
    pinfo.dsi = dsi;
    mipi_dsi_set_drvdata(dsi, pinfo);
    ret = drm_of_get_panel_orientation(dev.of_node, &pinfo.orientation);
    if (ret < 0) {
    dev_err(dev, "%pOF: failed to get orientation %d\n", dev.of_node, ret);
    return ret;
    }
    pinfo.panel.prepare_prev_first = true;
    pinfo.panel.backlight = icna35xx_create_backlight(dsi);
    if (IS_ERR(pinfo.panel.backlight))
    return dev_err_probe(dev, PTR_ERR(pinfo.panel.backlight),
    "Failed to create backlight\n");
    ret = devm_drm_panel_add(dev, &pinfo.panel);
    if (ret)
    return ret;
    pinfo.dsi.lanes = pinfo.desc.lanes;
    pinfo.dsi.format = pinfo.desc.format;
    pinfo.dsi.mode_flags = pinfo.desc.mode_flags;
    pinfo.dsi.dsc = &pinfo.desc.dsc;
    return devm_mipi_dsi_attach(dev, dsi);
    }
    static const struct of_device_id icna35xx_of_match[] = {
    { .compatible = "ayaneo,pocketds-panel-top", .data = &odin2portal_desc },
    { .compatible = "ayntec,odin2portal-panel", .data = &odin2portal_desc },
    { .compatible = "ayntec,odin3-panel", .data = &thor_top_desc },
    { .compatible = "ayntec,thor-panel-top", .data = &thor_top_desc },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, icna35xx_of_match);
    static struct mipi_dsi_driver icna35xx_driver = {
    .probe = icna35xx_probe,
    .driver = {
    .name = "panel-chipone-icna35xx",
    .of_match_table = icna35xx_of_match,
    },
    };
    module_mipi_dsi_driver(icna35xx_driver);
    MODULE_AUTHOR("Teguh Sobirin <teguh@sobir.in>");
    MODULE_DESCRIPTION("DRM driver for Chipone ICNA35XX based MIPI DSI panels");
    MODULE_LICENSE("GPL");
