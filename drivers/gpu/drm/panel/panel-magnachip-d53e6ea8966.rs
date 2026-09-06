//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-magnachip-d53e6ea8966.c
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
// Magnachip d53e6ea8966 MIPI-DSI panel driver
// Copyright (C) 2023 Chris Morgan
//

// Forward declaration for use in backlight function
    struct d53e6ea8966;
// Panel info, unique to each panel
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d53e6ea8966_panel_info {
// @display_modes: the supported display modes
    pub display_modes: *const drm_display_mode,
// @num_modes: the number of supported display modes
    pub num_modes: c_uint,
// @width_mm: panel width in mm
    pub width_mm: u16,
// @height_mm: panel height in mm
    pub height_mm: u16,
// @bus_flags: drm bus flags for panel
    pub bus_flags: u32,
// @panel_init_seq: panel specific init sequence
    pub db): *mut *mut void (panel_init_seq)(struct d53e6ea8966,
// @backlight_register: panel backlight registration or NULL
    pub db): *mut *mut int (backlight_register)(struct d53e6ea8966,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct d53e6ea8966 {
// @dev: the container device
    pub dev: *mut device,
// @dbi: the DBI bus abstraction handle
    pub dbi: mipi_dbi,
// @panel: the DRM panel instance for this device
    pub panel: drm_panel,
// @reset: reset GPIO line
    pub reset: *mut gpio_desc,
// @enable: enable GPIO line
    pub enable: *mut gpio_desc,
// @reg_vdd: VDD supply regulator for panel logic
    pub reg_vdd: *mut regulator,
// @reg_elvdd: ELVDD supply regulator for panel display
    pub reg_elvdd: *mut regulator,
// @dsi_dev: DSI child device (panel)
    pub dsi_dev: *mut mipi_dsi_device,
// @bl_dev: pseudo-backlight device for oled panel
    pub bl_dev: *mut backlight_device,
// @panel_info: struct containing panel timing and info
    pub panel_info: *const d53e6ea8966_panel_info,
}

pub const NUM_GAMMA_LEVELS: c_int = 16;
pub const GAMMA_TABLE_COUNT: c_int = 23;

pub const MCS_ELVSS_ON: c_uint = 0xb1;
pub const MCS_TEMP_SWIRE: c_uint = 0xb2;
pub const MCS_PASSWORD_0: c_uint = 0xf0;
pub const MCS_PASSWORD_1: c_uint = 0xf1;
pub const MCS_ANALOG_PWR_CTL_0: c_uint = 0xf4;
pub const MCS_ANALOG_PWR_CTL_1: c_uint = 0xf5;
pub const MCS_GTCON_SET: c_uint = 0xf7;
pub const MCS_GATELESS_SIGNAL_SET: c_uint = 0xf8;
pub const MCS_SET_GAMMA: c_uint = 0xf9;
    static inline struct d53e6ea8966 *to_d53e6ea8966(struct drm_panel *panel)
    {
    return container_of(panel, struct d53e6ea8966, panel);
    }
// Table of gamma values provided in datasheet
    static u8 ams495qa01_gamma[NUM_GAMMA_LEVELS][GAMMA_TABLE_COUNT] = {
    {0x01, 0x79, 0x78, 0x8d, 0xd9, 0xdf, 0xd5, 0xcb, 0xcf, 0xc5,
    0xe5, 0xe0, 0xe4, 0xdc, 0xb8, 0xd4, 0xfa, 0xed, 0xe6, 0x2f,
    0x00, 0x2f},
    {0x01, 0x7d, 0x7c, 0x92, 0xd7, 0xdd, 0xd2, 0xcb, 0xd0, 0xc6,
    0xe5, 0xe1, 0xe3, 0xda, 0xbd, 0xd3, 0xfa, 0xed, 0xe6, 0x2f,
    0x00, 0x2f},
    {0x01, 0x7f, 0x7e, 0x95, 0xd7, 0xde, 0xd2, 0xcb, 0xcf, 0xc5,
    0xe5, 0xe3, 0xe3, 0xda, 0xbf, 0xd3, 0xfa, 0xed, 0xe6, 0x2f,
    0x00, 0x2f},
    {0x01, 0x82, 0x81, 0x99, 0xd6, 0xdd, 0xd1, 0xca, 0xcf, 0xc3,
    0xe4, 0xe3, 0xe3, 0xda, 0xc2, 0xd3, 0xfa, 0xed, 0xe6, 0x2f,
    0x00, 0x2f},
    {0x01, 0x84, 0x83, 0x9b, 0xd7, 0xde, 0xd2, 0xc8, 0xce, 0xc2,
    0xe4, 0xe3, 0xe2, 0xd9, 0xc3, 0xd3, 0xfa, 0xed, 0xe6, 0x2f,
    0x00, 0x2f},
    {0x01, 0x87, 0x86, 0x9f, 0xd6, 0xdd, 0xd1, 0xc7, 0xce, 0xc1,
    0xe4, 0xe3, 0xe2, 0xd9, 0xc6, 0xd3, 0xfa, 0xed, 0xe6, 0x2f,
    0x00, 0x2f},
    {0x01, 0x89, 0x89, 0xa2, 0xd5, 0xdb, 0xcf, 0xc8, 0xcf, 0xc2,
    0xe3, 0xe3, 0xe1, 0xd9, 0xc7, 0xd3, 0xfa, 0xed, 0xe6, 0x2f,
    0x00, 0x2f},
    {0x01, 0x8b, 0x8b, 0xa5, 0xd5, 0xdb, 0xcf, 0xc7, 0xce, 0xc0,
    0xe3, 0xe3, 0xe1, 0xd8, 0xc7, 0xd3, 0xfa, 0xed, 0xe6, 0x2f,
    0x00, 0x2f},
    {0x01, 0x8d, 0x8d, 0xa7, 0xd5, 0xdb, 0xcf, 0xc6, 0xce, 0xc0,
    0xe4, 0xe4, 0xe1, 0xd7, 0xc8, 0xd3, 0xfa, 0xed, 0xe6, 0x2f,
    0x00, 0x2f},
    {0x01, 0x8f, 0x8f, 0xaa, 0xd4, 0xdb, 0xce, 0xc6, 0xcd, 0xbf,
    0xe3, 0xe3, 0xe1, 0xd7, 0xca, 0xd3, 0xfa, 0xed, 0xe6, 0x2f,
    0x00, 0x2f},
    {0x01, 0x91, 0x91, 0xac, 0xd3, 0xda, 0xce, 0xc5, 0xcd, 0xbe,
    0xe3, 0xe3, 0xe0, 0xd7, 0xca, 0xd3, 0xfa, 0xed, 0xe6, 0x2f,
    0x00, 0x2f},
    {0x01, 0x93, 0x93, 0xaf, 0xd3, 0xda, 0xcd, 0xc5, 0xcd, 0xbe,
    0xe2, 0xe3, 0xdf, 0xd6, 0xca, 0xd3, 0xfa, 0xed, 0xe6, 0x2f,
    0x00, 0x2f},
    {0x01, 0x95, 0x95, 0xb1, 0xd2, 0xd9, 0xcc, 0xc4, 0xcd, 0xbe,
    0xe2, 0xe3, 0xdf, 0xd7, 0xcc, 0xd3, 0xfa, 0xed, 0xe6, 0x2f,
    0x00, 0x2f},
    {0x01, 0x99, 0x99, 0xb6, 0xd1, 0xd9, 0xcc, 0xc3, 0xcb, 0xbc,
    0xe2, 0xe4, 0xdf, 0xd6, 0xcc, 0xd3, 0xfa, 0xed, 0xe6, 0x2f,
    0x00, 0x2f},
    {0x01, 0x9c, 0x9c, 0xba, 0xd0, 0xd8, 0xcb, 0xc3, 0xcb, 0xbb,
    0xe2, 0xe4, 0xdf, 0xd6, 0xce, 0xd3, 0xfa, 0xed, 0xe6, 0x2f,
    0x00, 0x2f},
    {0x01, 0x9f, 0x9f, 0xbe, 0xcf, 0xd7, 0xc9, 0xc2, 0xcb, 0xbb,
    0xe1, 0xe3, 0xde, 0xd6, 0xd0, 0xd3, 0xfa, 0xed, 0xe6, 0x2f,
    0x00, 0x2f},
    };
//
// Table of elvss values provided in datasheet and corresponds to
// gamma values.
//
    static u8 ams495qa01_elvss[NUM_GAMMA_LEVELS] = {
    0x15, 0x15, 0x15, 0x15, 0x15, 0x15, 0x15, 0x15, 0x15, 0x15,
    0x15, 0x15, 0x14, 0x14, 0x13, 0x12,
    };
#[no_mangle]
unsafe extern "C" fn ams495qa01_update_gamma(dbi: *mut mipi_dbi, brightness: c_int) -> c_int {
    static int ams495qa01_update_gamma(struct mipi_dbi *dbi, int brightness)
    {
    let mut tmp: c_int = brightness;
    mipi_dbi_command_buf(dbi, MCS_SET_GAMMA, ams495qa01_gamma[tmp],
    ARRAY_SIZE(ams495qa01_gamma[tmp]));
    mipi_dbi_command(dbi, MCS_SET_GAMMA, 0x00);
// Undocumented command
    mipi_dbi_command(dbi, 0x26, 0x00);
    mipi_dbi_command(dbi, MCS_TEMP_SWIRE, ams495qa01_elvss[tmp]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ams495qa01_panel_init(db: *mut d53e6ea8966) {
    static void ams495qa01_panel_init(struct d53e6ea8966 *db)
    {
    struct mipi_dbi *dbi = &db.dbi;
    mipi_dbi_command(dbi, MCS_PASSWORD_0, 0x5a, 0x5a);
    mipi_dbi_command(dbi, MCS_PASSWORD_1, 0x5a, 0x5a);
// Undocumented commands
    mipi_dbi_command(dbi, 0xb0, 0x02);
    mipi_dbi_command(dbi, 0xf3, 0x3b);
    mipi_dbi_command(dbi, MCS_ANALOG_PWR_CTL_0, 0x33, 0x42, 0x00, 0x08);
    mipi_dbi_command(dbi, MCS_ANALOG_PWR_CTL_1, 0x00, 0x06, 0x26, 0x35, 0x03);
// Undocumented commands
    mipi_dbi_command(dbi, 0xf6, 0x02);
    mipi_dbi_command(dbi, 0xc6, 0x0b, 0x00, 0x00, 0x3c, 0x00, 0x22,
    0x00, 0x00, 0x00, 0x00);
    mipi_dbi_command(dbi, MCS_GTCON_SET, 0x20);
    mipi_dbi_command(dbi, MCS_TEMP_SWIRE, 0x06, 0x06, 0x06, 0x06);
    mipi_dbi_command(dbi, MCS_ELVSS_ON, 0x07, 0x00, 0x10);
    mipi_dbi_command(dbi, MCS_GATELESS_SIGNAL_SET, 0x7f, 0x7a,
    0x89, 0x67, 0x26, 0x38, 0x00, 0x00, 0x09,
    0x67, 0x70, 0x88, 0x7a, 0x76, 0x05, 0x09,
    0x23, 0x23, 0x23);
// Undocumented commands
    mipi_dbi_command(dbi, 0xb5, 0xff, 0xef, 0x35, 0x42, 0x0d, 0xd7,
    0xff, 0x07, 0xff, 0xff, 0xfd, 0x00, 0x01,
    0xff, 0x05, 0x12, 0x0f, 0xff, 0xff, 0xff,
    0xff);
    mipi_dbi_command(dbi, 0xb4, 0x15);
    mipi_dbi_command(dbi, 0xb3, 0x00);
    ams495qa01_update_gamma(dbi, MAX_BRIGHTNESS);
    }
#[no_mangle]
unsafe extern "C" fn d53e6ea8966_prepare(panel: *mut drm_panel) -> c_int {
    static int d53e6ea8966_prepare(struct drm_panel *panel)
    {
    struct d53e6ea8966 *db = to_d53e6ea8966(panel);
    int ret;
// Power up
    ret = regulator_enable(db.reg_vdd);
    if (ret) {
    dev_err(db.dev, "failed to enable vdd regulator: %d\n", ret);
    return ret;
    }
    if (db.reg_elvdd) {
    ret = regulator_enable(db.reg_elvdd);
    if (ret) {
    dev_err(db.dev,
    "failed to enable elvdd regulator: %d\n", ret);
    regulator_disable(db.reg_vdd);
    return ret;
    }
    }
// Enable
    if (db.enable)
    gpiod_set_value_cansleep(db.enable, 1);
    msleep(50);
// Reset
    gpiod_set_value_cansleep(db.reset, 1);
    usleep_range(1000, 5000);
    gpiod_set_value_cansleep(db.reset, 0);
    msleep(20);
    db.panel_info.panel_init_seq(db);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn d53e6ea8966_enable(panel: *mut drm_panel) -> c_int {
    static int d53e6ea8966_enable(struct drm_panel *panel)
    {
    struct d53e6ea8966 *db = to_d53e6ea8966(panel);
    struct mipi_dbi *dbi = &db.dbi;
    mipi_dbi_command(dbi, MIPI_DCS_EXIT_SLEEP_MODE);
    msleep(200);
    mipi_dbi_command(dbi, MIPI_DCS_SET_DISPLAY_ON);
    usleep_range(10000, 15000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn d53e6ea8966_disable(panel: *mut drm_panel) -> c_int {
    static int d53e6ea8966_disable(struct drm_panel *panel)
    {
    struct d53e6ea8966 *db = to_d53e6ea8966(panel);
    struct mipi_dbi *dbi = &db.dbi;
    mipi_dbi_command(dbi, MIPI_DCS_SET_DISPLAY_OFF);
    msleep(20);
    mipi_dbi_command(dbi, MIPI_DCS_ENTER_SLEEP_MODE);
    msleep(100);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn d53e6ea8966_unprepare(panel: *mut drm_panel) -> c_int {
    static int d53e6ea8966_unprepare(struct drm_panel *panel)
    {
    struct d53e6ea8966 *db = to_d53e6ea8966(panel);
    if (db.enable)
    gpiod_set_value_cansleep(db.enable, 0);
    gpiod_set_value_cansleep(db.reset, 1);
    if (db.reg_elvdd)
    regulator_disable(db.reg_elvdd);
    regulator_disable(db.reg_vdd);
    msleep(100);
    return 0;
    }
    static int d53e6ea8966_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    struct d53e6ea8966 *db = to_d53e6ea8966(panel);
    const struct d53e6ea8966_panel_info *panel_info = db.panel_info;
    struct drm_display_mode *mode;
    let mut bus_format: static u32 = MEDIA_BUS_FMT_RGB888_1X24;
    unsigned int i;
    for (i = 0; i < panel_info.num_modes; i++) {
    mode = drm_mode_duplicate(connector.dev,
    &panel_info.display_modes[i]);
    if (!mode)
    return -ENOMEM;
    drm_mode_set_name(mode);
    drm_mode_probed_add(connector, mode);
    }
    connector.display_info.bpc = 8;
    connector.display_info.width_mm = panel_info.width_mm;
    connector.display_info.height_mm = panel_info.height_mm;
    connector.display_info.bus_flags = panel_info.bus_flags;
    drm_display_info_set_bus_formats(&connector.display_info,
    &bus_format, 1);
    return 1;
    }
    static const struct drm_panel_funcs d53e6ea8966_panel_funcs = {
    .disable = d53e6ea8966_disable,
    .enable = d53e6ea8966_enable,
    .get_modes = d53e6ea8966_get_modes,
    .prepare = d53e6ea8966_prepare,
    .unprepare = d53e6ea8966_unprepare,
    };
#[no_mangle]
unsafe extern "C" fn ams495qa01_set_brightness(bd: *mut backlight_device) -> c_int {
    static int ams495qa01_set_brightness(struct backlight_device *bd)
    {
    struct d53e6ea8966 *db = bl_get_data(bd);
    struct mipi_dbi *dbi = &db.dbi;
    let mut brightness: c_int = backlight_get_brightness(bd);
    ams495qa01_update_gamma(dbi, brightness);
    return 0;
    }
    static const struct backlight_ops ams495qa01_backlight_ops = {
    .update_status	= ams495qa01_set_brightness,
    };
#[no_mangle]
unsafe extern "C" fn ams495qa01_backlight_register(db: *mut d53e6ea8966) -> c_int {
    static int ams495qa01_backlight_register(struct d53e6ea8966 *db)
    {
    struct backlight_properties props = {
    .type		= BACKLIGHT_RAW,
    .brightness	= MAX_BRIGHTNESS,
    .max_brightness = MAX_BRIGHTNESS,
    };
    struct device *dev = db.dev;
    let mut ret: c_int = 0;
    db.bl_dev = devm_backlight_device_register(dev, "panel", dev, db,
    &ams495qa01_backlight_ops,
    &props);
    if (IS_ERR(db.bl_dev)) {
    ret = PTR_ERR(db.bl_dev);
    dev_err(dev, "error registering backlight device (%d)\n", ret);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn d53e6ea8966_probe(spi: *mut spi_device) -> c_int {
    static int d53e6ea8966_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct mipi_dsi_host *dsi_host;
    struct d53e6ea8966 *db;
    int ret;
    struct mipi_dsi_device_info info = {
    .type = "d53e6ea8966",
    .channel = 0,
    .node = core::ptr::null_mut(),
    };
    db = devm_drm_panel_alloc(dev, struct d53e6ea8966, panel,
    &d53e6ea8966_panel_funcs,
    DRM_MODE_CONNECTOR_DSI);
    if (IS_ERR(db))
    return PTR_ERR(db);
    spi_set_drvdata(spi, db);
    db.dev = dev;
    db.panel_info = of_device_get_match_data(dev);
    if (!db.panel_info)
    return -EINVAL;
    db.reg_vdd = devm_regulator_get(dev, "vdd");
    if (IS_ERR(db.reg_vdd))
    return dev_err_probe(dev, PTR_ERR(db.reg_vdd),
    "Failed to get vdd supply\n");
    db.reg_elvdd = devm_regulator_get_optional(dev, "elvdd");
    if (IS_ERR(db.reg_elvdd))
    db.reg_elvdd = core::ptr::null_mut();
    db.reset = devm_gpiod_get(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(db.reset)) {
    ret = PTR_ERR(db.reset);
    return dev_err_probe(dev, ret, "no RESET GPIO\n");
    }
    db.enable = devm_gpiod_get_optional(dev, "enable", GPIOD_OUT_LOW);
    if (IS_ERR(db.enable)) {
    ret = PTR_ERR(db.enable);
    return dev_err_probe(dev, ret, "cannot get ENABLE GPIO\n");
    }
    ret = mipi_dbi_spi_init(spi, &db.dbi, core::ptr::null_mut());
    if (ret)
    return dev_err_probe(dev, ret, "MIPI DBI init failed\n");
    dsi_host = drm_of_get_dsi_bus(dev);
    if (IS_ERR(dsi_host)) {
    ret = PTR_ERR(dsi_host);
    return dev_err_probe(dev, ret, "Error attaching DSI bus\n");
    }
    db.dsi_dev = devm_mipi_dsi_device_register_full(dev, dsi_host, &info);
    if (IS_ERR(db.dsi_dev)) {
    dev_err(dev, "failed to register dsi device: %ld\n",
    PTR_ERR(db.dsi_dev));
    return PTR_ERR(db.dsi_dev);
    }
    db.dsi_dev.lanes = 2;
    db.dsi_dev.format = MIPI_DSI_FMT_RGB888;
    db.dsi_dev.mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_VIDEO_BURST |
    MIPI_DSI_MODE_LPM | MIPI_DSI_MODE_NO_EOT_PACKET;
    if (db.panel_info.backlight_register) {
    ret = db.panel_info.backlight_register(db);
    if (ret < 0)
    return ret;
    db.panel.backlight = db.bl_dev;
    }
    drm_panel_add(&db.panel);
    ret = devm_mipi_dsi_attach(dev, db.dsi_dev);
    if (ret < 0) {
    dev_err(dev, "mipi_dsi_attach failed: %d\n", ret);
    drm_panel_remove(&db.panel);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn d53e6ea8966_remove(spi: *mut spi_device) {
    static void d53e6ea8966_remove(struct spi_device *spi)
    {
    struct d53e6ea8966 *db = spi_get_drvdata(spi);
    drm_panel_remove(&db.panel);
    }
    static const struct drm_display_mode ams495qa01_modes[] = {
    { /* 60hz */
    .clock = 33500,
    .hdisplay = 960,
    .hsync_start = 960 + 10,
    .hsync_end = 960 + 10 + 2,
    .htotal = 960 + 10 + 2 + 10,
    .vdisplay = 544,
    .vsync_start = 544 + 10,
    .vsync_end = 544 + 10 + 2,
    .vtotal = 544 + 10 + 2 + 10,
    .flags = DRM_MODE_FLAG_NHSYNC | DRM_MODE_FLAG_NVSYNC,
    .type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED,
    },
    { /* 50hz */
    .clock = 27800,
    .hdisplay = 960,
    .hsync_start = 960 + 10,
    .hsync_end = 960 + 10 + 2,
    .htotal = 960 + 10 + 2 + 10,
    .vdisplay = 544,
    .vsync_start = 544 + 10,
    .vsync_end = 544 + 10 + 2,
    .vtotal = 544 + 10 + 2 + 10,
    .flags = DRM_MODE_FLAG_NHSYNC | DRM_MODE_FLAG_NVSYNC,
    .type = DRM_MODE_TYPE_DRIVER,
    },
    };
    static const struct d53e6ea8966_panel_info ams495qa01_info = {
    .display_modes = ams495qa01_modes,
    .num_modes = ARRAY_SIZE(ams495qa01_modes),
    .width_mm = 117,
    .height_mm = 74,
    .bus_flags = DRM_BUS_FLAG_DE_LOW | DRM_BUS_FLAG_PIXDATA_DRIVE_NEGEDGE,
    .panel_init_seq = ams495qa01_panel_init,
    .backlight_register = ams495qa01_backlight_register,
    };
    static const struct of_device_id d53e6ea8966_match[] = {
    { .compatible = "samsung,ams495qa01", .data = &ams495qa01_info },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, d53e6ea8966_match);
    static const struct spi_device_id d53e6ea8966_ids[] = {
    { "ams495qa01", 0 },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(spi, d53e6ea8966_ids);
    static struct spi_driver d53e6ea8966_driver = {
    .driver		= {
    .name	= "d53e6ea8966-panel",
    .of_match_table = d53e6ea8966_match,
    },
    .id_table	= d53e6ea8966_ids,
    .probe		= d53e6ea8966_probe,
    .remove		= d53e6ea8966_remove,
    };
    module_spi_driver(d53e6ea8966_driver);
    MODULE_AUTHOR("Chris Morgan <macromorgan@hotmail.com>");
    MODULE_DESCRIPTION("Magnachip d53e6ea8966 panel driver");
    MODULE_LICENSE("GPL");
