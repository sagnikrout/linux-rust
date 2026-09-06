//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-raydium-rm67191.c
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
// Raydium RM67191 MIPI-DSI panel driver
//
// Copyright 2019 NXP
//

// Panel specific color-format bits
pub const COL_FMT_16BPP: c_uint = 0x55;
pub const COL_FMT_18BPP: c_uint = 0x66;
pub const COL_FMT_24BPP: c_uint = 0x77;
// Write Manufacture Command Set Control
pub const WRMAUCCTR: c_uint = 0xFE;
// Manufacturer Command Set pages (CMD2)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_set_entry {
    pub cmd: u8,
    pub param: u8,
}

//
// There is no description in the Reference Manual about these commands.
// We received them from vendor, so just use them as is.
//
    static const struct cmd_set_entry manufacturer_cmd_set[] = {
    {0xFE, 0x0B},
    {0x28, 0x40},
    {0x29, 0x4F},
    {0xFE, 0x0E},
    {0x4B, 0x00},
    {0x4C, 0x0F},
    {0x4D, 0x20},
    {0x4E, 0x40},
    {0x4F, 0x60},
    {0x50, 0xA0},
    {0x51, 0xC0},
    {0x52, 0xE0},
    {0x53, 0xFF},
    {0xFE, 0x0D},
    {0x18, 0x08},
    {0x42, 0x00},
    {0x08, 0x41},
    {0x46, 0x02},
    {0x72, 0x09},
    {0xFE, 0x0A},
    {0x24, 0x17},
    {0x04, 0x07},
    {0x1A, 0x0C},
    {0x0F, 0x44},
    {0xFE, 0x04},
    {0x00, 0x0C},
    {0x05, 0x08},
    {0x06, 0x08},
    {0x08, 0x08},
    {0x09, 0x08},
    {0x0A, 0xE6},
    {0x0B, 0x8C},
    {0x1A, 0x12},
    {0x1E, 0xE0},
    {0x29, 0x93},
    {0x2A, 0x93},
    {0x2F, 0x02},
    {0x31, 0x02},
    {0x33, 0x05},
    {0x37, 0x2D},
    {0x38, 0x2D},
    {0x3A, 0x1E},
    {0x3B, 0x1E},
    {0x3D, 0x27},
    {0x3F, 0x80},
    {0x40, 0x40},
    {0x41, 0xE0},
    {0x4F, 0x2F},
    {0x50, 0x1E},
    {0xFE, 0x06},
    {0x00, 0xCC},
    {0x05, 0x05},
    {0x07, 0xA2},
    {0x08, 0xCC},
    {0x0D, 0x03},
    {0x0F, 0xA2},
    {0x32, 0xCC},
    {0x37, 0x05},
    {0x39, 0x83},
    {0x3A, 0xCC},
    {0x41, 0x04},
    {0x43, 0x83},
    {0x44, 0xCC},
    {0x49, 0x05},
    {0x4B, 0xA2},
    {0x4C, 0xCC},
    {0x51, 0x03},
    {0x53, 0xA2},
    {0x75, 0xCC},
    {0x7A, 0x03},
    {0x7C, 0x83},
    {0x7D, 0xCC},
    {0x82, 0x02},
    {0x84, 0x83},
    {0x85, 0xEC},
    {0x86, 0x0F},
    {0x87, 0xFF},
    {0x88, 0x00},
    {0x8A, 0x02},
    {0x8C, 0xA2},
    {0x8D, 0xEA},
    {0x8E, 0x01},
    {0x8F, 0xE8},
    {0xFE, 0x06},
    {0x90, 0x0A},
    {0x92, 0x06},
    {0x93, 0xA0},
    {0x94, 0xA8},
    {0x95, 0xEC},
    {0x96, 0x0F},
    {0x97, 0xFF},
    {0x98, 0x00},
    {0x9A, 0x02},
    {0x9C, 0xA2},
    {0xAC, 0x04},
    {0xFE, 0x06},
    {0xB1, 0x12},
    {0xB2, 0x17},
    {0xB3, 0x17},
    {0xB4, 0x17},
    {0xB5, 0x17},
    {0xB6, 0x11},
    {0xB7, 0x08},
    {0xB8, 0x09},
    {0xB9, 0x06},
    {0xBA, 0x07},
    {0xBB, 0x17},
    {0xBC, 0x17},
    {0xBD, 0x17},
    {0xBE, 0x17},
    {0xBF, 0x17},
    {0xC0, 0x17},
    {0xC1, 0x17},
    {0xC2, 0x17},
    {0xC3, 0x17},
    {0xC4, 0x0F},
    {0xC5, 0x0E},
    {0xC6, 0x00},
    {0xC7, 0x01},
    {0xC8, 0x10},
    {0xFE, 0x06},
    {0x95, 0xEC},
    {0x8D, 0xEE},
    {0x44, 0xEC},
    {0x4C, 0xEC},
    {0x32, 0xEC},
    {0x3A, 0xEC},
    {0x7D, 0xEC},
    {0x75, 0xEC},
    {0x00, 0xEC},
    {0x08, 0xEC},
    {0x85, 0xEC},
    {0xA6, 0x21},
    {0xA7, 0x05},
    {0xA9, 0x06},
    {0x82, 0x06},
    {0x41, 0x06},
    {0x7A, 0x07},
    {0x37, 0x07},
    {0x05, 0x06},
    {0x49, 0x06},
    {0x0D, 0x04},
    {0x51, 0x04},
    };
    static const u32 rad_bus_formats[] = {
    MEDIA_BUS_FMT_RGB888_1X24,
    MEDIA_BUS_FMT_RGB666_1X18,
    MEDIA_BUS_FMT_RGB565_1X16,
    };
    static const u32 rad_bus_flags = DRM_BUS_FLAG_DE_LOW |
    DRM_BUS_FLAG_PIXDATA_SAMPLE_POSEDGE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rad_panel {
    pub panel: drm_panel,
    pub dsi: *mut mipi_dsi_device,
    pub reset: *mut gpio_desc,
    pub backlight: *mut backlight_device,
    pub supplies: *mut regulator_bulk_data,
    pub num_supplies: c_uint,
    pub prepared: bool,
}

    static const struct drm_display_mode default_mode = {
    .clock = 132000,
    .hdisplay = 1080,
    .hsync_start = 1080 + 20,
    .hsync_end = 1080 + 20 + 2,
    .htotal = 1080 + 20 + 2 + 34,
    .vdisplay = 1920,
    .vsync_start = 1920 + 10,
    .vsync_end = 1920 + 10 + 2,
    .vtotal = 1920 + 10 + 2 + 4,
    .width_mm = 68,
    .height_mm = 121,
    .flags = DRM_MODE_FLAG_NHSYNC |
    DRM_MODE_FLAG_NVSYNC,
    };
    static inline struct rad_panel *to_rad_panel(struct drm_panel *panel)
    {
    return container_of(panel, struct rad_panel, panel);
    }
#[no_mangle]
unsafe extern "C" fn rad_panel_push_cmd_list(dsi: *mut mipi_dsi_device) -> c_int {
    static int rad_panel_push_cmd_list(struct mipi_dsi_device *dsi)
    {
    size_t i;
    let mut count: usize = ARRAY_SIZE(manufacturer_cmd_set);
    let mut ret: c_int = 0;
    for (i = 0; i < count; i++) {
    const struct cmd_set_entry *entry = &manufacturer_cmd_set[i];
    u8 buffer[2] = { entry.cmd, entry.param };
    ret = mipi_dsi_generic_write(dsi, &buffer, sizeof(buffer));
    if (ret < 0)
    return ret;
    }
    return ret;
    };
#[no_mangle]
unsafe extern "C" fn color_format_from_dsi_format(format: enum mipi_dsi_pixel_format) -> c_int {
    static int color_format_from_dsi_format(enum mipi_dsi_pixel_format format)
    {
    switch (format) {
    case MIPI_DSI_FMT_RGB565:
    return COL_FMT_16BPP;
    case MIPI_DSI_FMT_RGB666:
    case MIPI_DSI_FMT_RGB666_PACKED:
    return COL_FMT_18BPP;
    case MIPI_DSI_FMT_RGB888:
    return COL_FMT_24BPP;
    default:
    return COL_FMT_24BPP; /* for backward compatibility */
    }
    };
#[no_mangle]
unsafe extern "C" fn rad_panel_prepare(panel: *mut drm_panel) -> c_int {
    static int rad_panel_prepare(struct drm_panel *panel)
    {
    struct rad_panel *rad = to_rad_panel(panel);
    int ret;
    ret = regulator_bulk_enable(rad.num_supplies, rad.supplies);
    if (ret)
    return ret;
    if (rad.reset) {
    gpiod_set_value_cansleep(rad.reset, 1);
    usleep_range(3000, 5000);
    gpiod_set_value_cansleep(rad.reset, 0);
    usleep_range(18000, 20000);
    }
    rad.prepared = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rad_panel_unprepare(panel: *mut drm_panel) -> c_int {
    static int rad_panel_unprepare(struct drm_panel *panel)
    {
    struct rad_panel *rad = to_rad_panel(panel);
    int ret;
//
// Right after asserting the reset, we need to release it, so that the
// touch driver can have an active connection with the touch controller
// even after the display is turned off.
//
    if (rad.reset) {
    gpiod_set_value_cansleep(rad.reset, 1);
    usleep_range(15000, 17000);
    gpiod_set_value_cansleep(rad.reset, 0);
    }
    ret = regulator_bulk_disable(rad.num_supplies, rad.supplies);
    if (ret)
    return ret;
    rad.prepared = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rad_panel_enable(panel: *mut drm_panel) -> c_int {
    static int rad_panel_enable(struct drm_panel *panel)
    {
    struct rad_panel *rad = to_rad_panel(panel);
    struct mipi_dsi_device *dsi = rad.dsi;
    struct device *dev = &dsi.dev;
    let mut color_format: c_int = color_format_from_dsi_format(dsi.format);
    int ret;
    dsi.mode_flags |= MIPI_DSI_MODE_LPM;
    ret = rad_panel_push_cmd_list(dsi);
    if (ret < 0) {
    dev_err(dev, "Failed to send MCS (%d)\n", ret);
    goto fail;
    }
// Select User Command Set table (CMD1)
    ret = mipi_dsi_generic_write(dsi, (u8[]){ WRMAUCCTR, 0x00 }, 2);
    if (ret < 0)
    goto fail;
// Software reset
    ret = mipi_dsi_dcs_soft_reset(dsi);
    if (ret < 0) {
    dev_err(dev, "Failed to do Software Reset (%d)\n", ret);
    goto fail;
    }
    usleep_range(15000, 17000);
// Set DSI mode
    ret = mipi_dsi_generic_write(dsi, (u8[]){ 0xC2, 0x0B }, 2);
    if (ret < 0) {
    dev_err(dev, "Failed to set DSI mode (%d)\n", ret);
    goto fail;
    }
// Set tear ON
    ret = mipi_dsi_dcs_set_tear_on(dsi, MIPI_DSI_DCS_TEAR_MODE_VBLANK);
    if (ret < 0) {
    dev_err(dev, "Failed to set tear ON (%d)\n", ret);
    goto fail;
    }
// Set tear scanline
    ret = mipi_dsi_dcs_set_tear_scanline(dsi, 0x380);
    if (ret < 0) {
    dev_err(dev, "Failed to set tear scanline (%d)\n", ret);
    goto fail;
    }
// Set pixel format
    ret = mipi_dsi_dcs_set_pixel_format(dsi, color_format);
    dev_dbg(dev, "Interface color format set to 0x%x\n", color_format);
    if (ret < 0) {
    dev_err(dev, "Failed to set pixel format (%d)\n", ret);
    goto fail;
    }
// Exit sleep mode
    ret = mipi_dsi_dcs_exit_sleep_mode(dsi);
    if (ret < 0) {
    dev_err(dev, "Failed to exit sleep mode (%d)\n", ret);
    goto fail;
    }
    usleep_range(5000, 7000);
    ret = mipi_dsi_dcs_set_display_on(dsi);
    if (ret < 0) {
    dev_err(dev, "Failed to set display ON (%d)\n", ret);
    goto fail;
    }
    backlight_enable(rad.backlight);
    return 0;
    fail:
    gpiod_set_value_cansleep(rad.reset, 1);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rad_panel_disable(panel: *mut drm_panel) -> c_int {
    static int rad_panel_disable(struct drm_panel *panel)
    {
    struct rad_panel *rad = to_rad_panel(panel);
    struct mipi_dsi_device *dsi = rad.dsi;
    struct device *dev = &dsi.dev;
    int ret;
    dsi.mode_flags |= MIPI_DSI_MODE_LPM;
    backlight_disable(rad.backlight);
    usleep_range(10000, 12000);
    ret = mipi_dsi_dcs_set_display_off(dsi);
    if (ret < 0) {
    dev_err(dev, "Failed to set display OFF (%d)\n", ret);
    return ret;
    }
    usleep_range(5000, 10000);
    ret = mipi_dsi_dcs_enter_sleep_mode(dsi);
    if (ret < 0) {
    dev_err(dev, "Failed to enter sleep mode (%d)\n", ret);
    return ret;
    }
    return 0;
    }
    static int rad_panel_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    struct drm_display_mode *mode;
    mode = drm_mode_duplicate(connector.dev, &default_mode);
    if (!mode) {
    dev_err(panel.dev, "failed to add mode %ux%u@%u\n",
    default_mode.hdisplay, default_mode.vdisplay,
    drm_mode_vrefresh(&default_mode));
    return -ENOMEM;
    }
    drm_mode_set_name(mode);
    mode.type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED;
    drm_mode_probed_add(connector, mode);
    connector.display_info.width_mm = mode.width_mm;
    connector.display_info.height_mm = mode.height_mm;
    connector.display_info.bus_flags = rad_bus_flags;
    drm_display_info_set_bus_formats(&connector.display_info,
    rad_bus_formats,
    ARRAY_SIZE(rad_bus_formats));
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn rad_bl_get_brightness(bl: *mut backlight_device) -> c_int {
    static int rad_bl_get_brightness(struct backlight_device *bl)
    {
    struct mipi_dsi_device *dsi = bl_get_data(bl);
    struct rad_panel *rad = mipi_dsi_get_drvdata(dsi);
    u16 brightness;
    int ret;
    if (!rad.prepared)
    return 0;
    dsi.mode_flags &= ~MIPI_DSI_MODE_LPM;
    ret = mipi_dsi_dcs_get_display_brightness(dsi, &brightness);
    if (ret < 0)
    return ret;
    bl.props.brightness = brightness;
    return brightness & 0xff;
    }
#[no_mangle]
unsafe extern "C" fn rad_bl_update_status(bl: *mut backlight_device) -> c_int {
    static int rad_bl_update_status(struct backlight_device *bl)
    {
    struct mipi_dsi_device *dsi = bl_get_data(bl);
    struct rad_panel *rad = mipi_dsi_get_drvdata(dsi);
    let mut ret: c_int = 0;
    if (!rad.prepared)
    return 0;
    dsi.mode_flags &= ~MIPI_DSI_MODE_LPM;
    ret = mipi_dsi_dcs_set_display_brightness(dsi, bl.props.brightness);
    if (ret < 0)
    return ret;
    return 0;
    }
    static const struct backlight_ops rad_bl_ops = {
    .update_status = rad_bl_update_status,
    .get_brightness = rad_bl_get_brightness,
    };
    static const struct drm_panel_funcs rad_panel_funcs = {
    .prepare = rad_panel_prepare,
    .unprepare = rad_panel_unprepare,
    .enable = rad_panel_enable,
    .disable = rad_panel_disable,
    .get_modes = rad_panel_get_modes,
    };
    static const char * const rad_supply_names[] = {
    "v3p3",
    "v1p8",
    };
#[no_mangle]
unsafe extern "C" fn rad_init_regulators(rad: *mut rad_panel) -> c_int {
    static int rad_init_regulators(struct rad_panel *rad)
    {
    struct device *dev = &rad.dsi.dev;
    int i;
    rad.num_supplies = ARRAY_SIZE(rad_supply_names);
    rad.supplies = devm_kcalloc(dev, rad.num_supplies,
    sizeof(*rad.supplies), GFP_KERNEL);
    if (!rad.supplies)
    return -ENOMEM;
    for (i = 0; i < rad.num_supplies; i++)
    rad.supplies[i].supply = rad_supply_names[i];
    return devm_regulator_bulk_get(dev, rad.num_supplies, rad.supplies);
    };
#[no_mangle]
unsafe extern "C" fn rad_panel_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int rad_panel_probe(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    struct device_node *np = dev.of_node;
    struct rad_panel *panel;
    struct backlight_properties bl_props;
    int ret;
    u32 video_mode;
    panel = devm_drm_panel_alloc(dev, struct rad_panel, panel,
    &rad_panel_funcs,
    DRM_MODE_CONNECTOR_DSI);
    if (IS_ERR(panel))
    return PTR_ERR(panel);
    mipi_dsi_set_drvdata(dsi, panel);
    panel.dsi = dsi;
    dsi.format = MIPI_DSI_FMT_RGB888;
    dsi.mode_flags =  MIPI_DSI_MODE_VIDEO_HSE | MIPI_DSI_MODE_VIDEO;
    ret = of_property_read_u32(np, "video-mode", &video_mode);
    if (!ret) {
    switch (video_mode) {
    case 0:
// burst mode
    dsi.mode_flags |= MIPI_DSI_MODE_VIDEO_BURST;
    break;
    case 1:
// non-burst mode with sync event
    break;
    case 2:
// non-burst mode with sync pulse
    dsi.mode_flags |= MIPI_DSI_MODE_VIDEO_SYNC_PULSE;
    break;
    default:
    dev_warn(dev, "invalid video mode %d\n", video_mode);
    break;
    }
    }
    ret = of_property_read_u32(np, "dsi-lanes", &dsi.lanes);
    if (ret) {
    dev_err(dev, "Failed to get dsi-lanes property (%d)\n", ret);
    return ret;
    }
    panel.reset = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(panel.reset))
    return PTR_ERR(panel.reset);
    memset(&bl_props, 0, sizeof(bl_props));
    bl_props.type = BACKLIGHT_RAW;
    bl_props.brightness = 255;
    bl_props.max_brightness = 255;
    panel.backlight = devm_backlight_device_register(dev, dev_name(dev),
    dev, dsi, &rad_bl_ops,
    &bl_props);
    if (IS_ERR(panel.backlight)) {
    ret = PTR_ERR(panel.backlight);
    dev_err(dev, "Failed to register backlight (%d)\n", ret);
    return ret;
    }
    ret = rad_init_regulators(panel);
    if (ret)
    return ret;
    dev_set_drvdata(dev, panel);
    drm_panel_add(&panel.panel);
    ret = mipi_dsi_attach(dsi);
    if (ret)
    drm_panel_remove(&panel.panel);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rad_panel_remove(dsi: *mut mipi_dsi_device) {
    static void rad_panel_remove(struct mipi_dsi_device *dsi)
    {
    struct rad_panel *rad = mipi_dsi_get_drvdata(dsi);
    struct device *dev = &dsi.dev;
    int ret;
    ret = mipi_dsi_detach(dsi);
    if (ret)
    dev_err(dev, "Failed to detach from host (%d)\n", ret);
    drm_panel_remove(&rad.panel);
    }
    static const struct of_device_id rad_of_match[] = {
    { .compatible = "raydium,rm67191", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rad_of_match);
    static struct mipi_dsi_driver rad_panel_driver = {
    .driver = {
    .name = "panel-raydium-rm67191",
    .of_match_table = rad_of_match,
    },
    .probe = rad_panel_probe,
    .remove = rad_panel_remove,
    };
    module_mipi_dsi_driver(rad_panel_driver);
    MODULE_AUTHOR("Robert Chiras <robert.chiras@nxp.com>");
    MODULE_DESCRIPTION("DRM Driver for Raydium RM67191 MIPI DSI panel");
    MODULE_LICENSE("GPL v2");
