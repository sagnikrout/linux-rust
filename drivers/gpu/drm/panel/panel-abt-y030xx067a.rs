//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-abt-y030xx067a.c
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
// Asia Better Technology Ltd. Y030XX067A IPS LCD panel driver
//
// Copyright (C) 2020, Paul Cercueil <paul@crapouillou.net>
// Copyright (C) 2020, Christophe Branchereau <cbranchereau@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct y030xx067a_info {
    pub display_modes: *const drm_display_mode,
    pub num_modes: c_uint,
    pub height_mm: u16 width_mm,,
    pub bus_flags: u32 bus_format,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct y030xx067a {
    pub panel: drm_panel,
    pub spi: *mut spi_device,
    pub map: *mut regmap,
    pub panel_info: *const y030xx067a_info,
    pub supply: *mut regulator,
    pub reset_gpio: *mut gpio_desc,
}

    static inline struct y030xx067a *to_y030xx067a(struct drm_panel *panel)
    {
    return container_of(panel, struct y030xx067a, panel);
    }
    static const struct reg_sequence y030xx067a_init_sequence[] = {
    { 0x00, REG00_VBRT_CTRL(0x7f) },
    { 0x01, REG01_COM_DC(0x3c) },
    { 0x02, REG02_VESA_SEL(0x3) | REG02_DA_CONTRAST(0x1f) },
    { 0x03, REG03_VPOSITION(0x0a) },
    { 0x04, REG04_HPOSITION1(0xd2) },
    { 0x05, REG05_CLIP | REG05_NVM_VREFRESH | REG05_SLBRCHARGE(0x2) },
    { 0x06, REG06_NT },
    { 0x07, 0 },
    { 0x08, REG08_PANEL(0x1) | REG08_CLOCK_DIV(0x2) },
    { 0x09, REG09_SUB_BRIGHT_R(0x20) },
    { 0x0a, REG0A_SUB_BRIGHT_B(0x20) },
    { 0x0b, REG0B_HD_FREERUN | REG0B_VD_FREERUN },
    { 0x0c, REG0C_CONTRAST_R(0x00) },
    { 0x0d, REG0D_CONTRAST_G(0x00) },
    { 0x0e, REG0E_CONTRAST_B(0x10) },
    { 0x0f, 0 },
    { 0x10, REG10_BRIGHT(0x7f) },
    { 0x11, REG11_SIGC_CNTL | REG11_SIG_GAIN(0x3f) },
    { 0x12, REG12_COLOR(0x20) | REG12_PWCKSEL(0x1) },
    { 0x13, REG13_4096LEVEL_CNTL(0x8) },
    { 0x14, 0 },
    { 0x15, 0 },
    };
#[no_mangle]
unsafe extern "C" fn y030xx067a_prepare(panel: *mut drm_panel) -> c_int {
    static int y030xx067a_prepare(struct drm_panel *panel)
    {
    struct y030xx067a *priv = to_y030xx067a(panel);
    struct device *dev = &priv.spi.dev;
    int err;
    err = regulator_enable(priv.supply);
    if (err) {
    dev_err(dev, "Failed to enable power supply: %d\n", err);
    return err;
    }
// Reset the chip
    gpiod_set_value_cansleep(priv.reset_gpio, 1);
    usleep_range(1000, 20000);
    gpiod_set_value_cansleep(priv.reset_gpio, 0);
    usleep_range(1000, 20000);
    err = regmap_multi_reg_write(priv.map, y030xx067a_init_sequence,
    ARRAY_SIZE(y030xx067a_init_sequence));
    if (err) {
    dev_err(dev, "Failed to init registers: %d\n", err);
    goto err_disable_regulator;
    }
    return 0;
    err_disable_regulator:
    regulator_disable(priv.supply);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn y030xx067a_unprepare(panel: *mut drm_panel) -> c_int {
    static int y030xx067a_unprepare(struct drm_panel *panel)
    {
    struct y030xx067a *priv = to_y030xx067a(panel);
    gpiod_set_value_cansleep(priv.reset_gpio, 1);
    regulator_disable(priv.supply);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn y030xx067a_enable(panel: *mut drm_panel) -> c_int {
    static int y030xx067a_enable(struct drm_panel *panel)
    {
    struct y030xx067a *priv = to_y030xx067a(panel);
    regmap_set_bits(priv.map, 0x06, REG06_XPSAVE);
    if (panel.backlight) {
// Wait for the picture to be ready before enabling backlight
    msleep(120);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn y030xx067a_disable(panel: *mut drm_panel) -> c_int {
    static int y030xx067a_disable(struct drm_panel *panel)
    {
    struct y030xx067a *priv = to_y030xx067a(panel);
    regmap_clear_bits(priv.map, 0x06, REG06_XPSAVE);
    return 0;
    }
    static int y030xx067a_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    struct y030xx067a *priv = to_y030xx067a(panel);
    const struct y030xx067a_info *panel_info = priv.panel_info;
    struct drm_display_mode *mode;
    unsigned int i;
    for (i = 0; i < panel_info.num_modes; i++) {
    mode = drm_mode_duplicate(connector.dev,
    &panel_info.display_modes[i]);
    if (!mode)
    return -ENOMEM;
    drm_mode_set_name(mode);
    mode.type = DRM_MODE_TYPE_DRIVER;
    if (panel_info.num_modes == 1)
    mode.type |= DRM_MODE_TYPE_PREFERRED;
    drm_mode_probed_add(connector, mode);
    }
    connector.display_info.bpc = 8;
    connector.display_info.width_mm = panel_info.width_mm;
    connector.display_info.height_mm = panel_info.height_mm;
    drm_display_info_set_bus_formats(&connector.display_info,
    &panel_info.bus_format, 1);
    connector.display_info.bus_flags = panel_info.bus_flags;
    return panel_info.num_modes;
    }
    static const struct drm_panel_funcs y030xx067a_funcs = {
    .prepare	= y030xx067a_prepare,
    .unprepare	= y030xx067a_unprepare,
    .enable		= y030xx067a_enable,
    .disable	= y030xx067a_disable,
    .get_modes	= y030xx067a_get_modes,
    };
    static const struct regmap_config y030xx067a_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x15,
    .cache_type = REGCACHE_FLAT,
    };
#[no_mangle]
unsafe extern "C" fn y030xx067a_probe(spi: *mut spi_device) -> c_int {
    static int y030xx067a_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct y030xx067a *priv;
    int err;
    priv = devm_drm_panel_alloc(dev, struct y030xx067a, panel,
    &y030xx067a_funcs, DRM_MODE_CONNECTOR_DPI);
    if (IS_ERR(priv))
    return PTR_ERR(priv);
    priv.spi = spi;
    spi_set_drvdata(spi, priv);
    priv.map = devm_regmap_init_spi(spi, &y030xx067a_regmap_config);
    if (IS_ERR(priv.map)) {
    dev_err(dev, "Unable to init regmap\n");
    return PTR_ERR(priv.map);
    }
    priv.panel_info = of_device_get_match_data(dev);
    if (!priv.panel_info)
    return -EINVAL;
    priv.supply = devm_regulator_get(dev, "power");
    if (IS_ERR(priv.supply))
    return dev_err_probe(dev, PTR_ERR(priv.supply),
    "Failed to get power supply\n");
    priv.reset_gpio = devm_gpiod_get(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(priv.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(priv.reset_gpio),
    "Failed to get reset GPIO\n");
    err = drm_panel_of_backlight(&priv.panel);
    if (err)
    return err;
    drm_panel_add(&priv.panel);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn y030xx067a_remove(spi: *mut spi_device) {
    static void y030xx067a_remove(struct spi_device *spi)
    {
    struct y030xx067a *priv = spi_get_drvdata(spi);
    drm_panel_remove(&priv.panel);
    drm_panel_disable(&priv.panel);
    drm_panel_unprepare(&priv.panel);
    }
    static const struct drm_display_mode y030xx067a_modes[] = {
    { /* 60 Hz */
    .clock = 14400,
    .hdisplay = 320,
    .hsync_start = 320 + 10,
    .hsync_end = 320 + 10 + 37,
    .htotal = 320 + 10 + 37 + 33,
    .vdisplay = 480,
    .vsync_start = 480 + 84,
    .vsync_end = 480 + 84 + 20,
    .vtotal = 480 + 84 + 20 + 16,
    .flags = DRM_MODE_FLAG_NHSYNC | DRM_MODE_FLAG_NVSYNC,
    },
    { /* 50 Hz */
    .clock = 12000,
    .hdisplay = 320,
    .hsync_start = 320 + 10,
    .hsync_end = 320 + 10 + 37,
    .htotal = 320 + 10 + 37 + 33,
    .vdisplay = 480,
    .vsync_start = 480 + 84,
    .vsync_end = 480 + 84 + 20,
    .vtotal = 480 + 84 + 20 + 16,
    .flags = DRM_MODE_FLAG_NHSYNC | DRM_MODE_FLAG_NVSYNC,
    },
    };
    static const struct y030xx067a_info y030xx067a_info = {
    .display_modes = y030xx067a_modes,
    .num_modes = ARRAY_SIZE(y030xx067a_modes),
    .width_mm = 69,
    .height_mm = 51,
    .bus_format = MEDIA_BUS_FMT_RGB888_3X8_DELTA,
    .bus_flags = DRM_BUS_FLAG_PIXDATA_SAMPLE_POSEDGE | DRM_BUS_FLAG_DE_LOW,
    };
    static const struct of_device_id y030xx067a_of_match[] = {
    { .compatible = "abt,y030xx067a", .data = &y030xx067a_info },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, y030xx067a_of_match);
    static struct spi_driver y030xx067a_driver = {
    .driver = {
    .name = "abt-y030xx067a",
    .of_match_table = y030xx067a_of_match,
    },
    .probe = y030xx067a_probe,
    .remove = y030xx067a_remove,
    };
    module_spi_driver(y030xx067a_driver);
    MODULE_AUTHOR("Paul Cercueil <paul@crapouillou.net>");
    MODULE_AUTHOR("Christophe Branchereau <cbranchereau@gmail.com>");
    MODULE_DESCRIPTION("Asia Better Technology Ltd. Y030XX067A IPS LCD panel driver");
    MODULE_LICENSE("GPL v2");
