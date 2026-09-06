//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-sharp-ls037v7dw01.c
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
// Sharp LS037V7DW01 LCD Panel Driver
//
// Copyright (C) 2019 Texas Instruments Incorporated
//
// Based on the omapdrm-specific panel-sharp-ls037v7dw01 driver
//
// Copyright (C) 2013 Texas Instruments Incorporated
// Author: Tomi Valkeinen <tomi.valkeinen@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls037v7dw01_panel {
    pub panel: drm_panel,
    pub pdev: *mut platform_device,
    pub vdd: *mut regulator,
    pub /: *mut *mut *mut gpio_desc resb_gpio; / low = reset active min 20 us,
    pub /: *mut *mut *mut gpio_desc ini_gpio; / high = power on,
    pub /: *mut *mut *mut gpio_desc mo_gpio; / low = 480x640, high = 240x320,
    pub /: *mut *mut *mut gpio_desc lr_gpio; / high = conventional horizontal scanning,
    pub /: *mut *mut *mut gpio_desc ud_gpio; / high = conventional vertical scanning,
}

    container_of(p, struct ls037v7dw01_panel, panel)
#[no_mangle]
unsafe extern "C" fn ls037v7dw01_disable(panel: *mut drm_panel) -> c_int {
    static int ls037v7dw01_disable(struct drm_panel *panel)
    {
    struct ls037v7dw01_panel *lcd = to_ls037v7dw01_device(panel);
    gpiod_set_value_cansleep(lcd.ini_gpio, 0);
    gpiod_set_value_cansleep(lcd.resb_gpio, 0);
// Wait at least 5 vsyncs after disabling the LCD.
    msleep(100);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls037v7dw01_unprepare(panel: *mut drm_panel) -> c_int {
    static int ls037v7dw01_unprepare(struct drm_panel *panel)
    {
    struct ls037v7dw01_panel *lcd = to_ls037v7dw01_device(panel);
    regulator_disable(lcd.vdd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls037v7dw01_prepare(panel: *mut drm_panel) -> c_int {
    static int ls037v7dw01_prepare(struct drm_panel *panel)
    {
    struct ls037v7dw01_panel *lcd = to_ls037v7dw01_device(panel);
    int ret;
    ret = regulator_enable(lcd.vdd);
    if (ret < 0)
    dev_err(&lcd.pdev.dev, "%s: failed to enable regulator\n",
    __func__);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ls037v7dw01_enable(panel: *mut drm_panel) -> c_int {
    static int ls037v7dw01_enable(struct drm_panel *panel)
    {
    struct ls037v7dw01_panel *lcd = to_ls037v7dw01_device(panel);
// Wait couple of vsyncs before enabling the LCD.
    msleep(50);
    gpiod_set_value_cansleep(lcd.resb_gpio, 1);
    gpiod_set_value_cansleep(lcd.ini_gpio, 1);
    return 0;
    }
    static const struct drm_display_mode ls037v7dw01_mode = {
    .clock = 19200,
    .hdisplay = 480,
    .hsync_start = 480 + 1,
    .hsync_end = 480 + 1 + 2,
    .htotal = 480 + 1 + 2 + 28,
    .vdisplay = 640,
    .vsync_start = 640 + 1,
    .vsync_end = 640 + 1 + 1,
    .vtotal = 640 + 1 + 1 + 1,
    .type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED,
    .flags = DRM_MODE_FLAG_NHSYNC | DRM_MODE_FLAG_NVSYNC,
    .width_mm = 56,
    .height_mm = 75,
    };
    static int ls037v7dw01_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    struct drm_display_mode *mode;
    mode = drm_mode_duplicate(connector.dev, &ls037v7dw01_mode);
    if (!mode)
    return -ENOMEM;
    drm_mode_set_name(mode);
    drm_mode_probed_add(connector, mode);
    connector.display_info.width_mm = ls037v7dw01_mode.width_mm;
    connector.display_info.height_mm = ls037v7dw01_mode.height_mm;
//
// FIXME: According to the datasheet pixel data is sampled on the
// rising edge of the clock, but the code running on the SDP3430
// indicates sampling on the negative edge. This should be tested on a
// real device.
//
    connector.display_info.bus_flags = DRM_BUS_FLAG_DE_HIGH
    | DRM_BUS_FLAG_SYNC_SAMPLE_POSEDGE
    | DRM_BUS_FLAG_PIXDATA_SAMPLE_NEGEDGE;
    return 1;
    }
    static const struct drm_panel_funcs ls037v7dw01_funcs = {
    .disable = ls037v7dw01_disable,
    .unprepare = ls037v7dw01_unprepare,
    .prepare = ls037v7dw01_prepare,
    .enable = ls037v7dw01_enable,
    .get_modes = ls037v7dw01_get_modes,
    };
#[no_mangle]
unsafe extern "C" fn ls037v7dw01_probe(pdev: *mut platform_device) -> c_int {
    static int ls037v7dw01_probe(struct platform_device *pdev)
    {
    struct ls037v7dw01_panel *lcd;
    lcd = devm_drm_panel_alloc(&pdev.dev, struct ls037v7dw01_panel, panel,
    &ls037v7dw01_funcs, DRM_MODE_CONNECTOR_DPI);
    if (IS_ERR(lcd))
    return PTR_ERR(lcd);
    platform_set_drvdata(pdev, lcd);
    lcd.pdev = pdev;
    lcd.vdd = devm_regulator_get(&pdev.dev, "envdd");
    if (IS_ERR(lcd.vdd))
    return dev_err_probe(&pdev.dev, PTR_ERR(lcd.vdd),
    "failed to get regulator\n");
    lcd.ini_gpio = devm_gpiod_get(&pdev.dev, "enable", GPIOD_OUT_LOW);
    if (IS_ERR(lcd.ini_gpio))
    return dev_err_probe(&pdev.dev, PTR_ERR(lcd.ini_gpio),
    "failed to get enable gpio\n");
    lcd.resb_gpio = devm_gpiod_get(&pdev.dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(lcd.resb_gpio))
    return dev_err_probe(&pdev.dev, PTR_ERR(lcd.resb_gpio),
    "failed to get reset gpio\n");
    lcd.mo_gpio = devm_gpiod_get_index(&pdev.dev, "mode", 0,
    GPIOD_OUT_LOW);
    if (IS_ERR(lcd.mo_gpio)) {
    dev_err(&pdev.dev, "failed to get mode[0] gpio\n");
    return PTR_ERR(lcd.mo_gpio);
    }
    lcd.lr_gpio = devm_gpiod_get_index(&pdev.dev, "mode", 1,
    GPIOD_OUT_LOW);
    if (IS_ERR(lcd.lr_gpio)) {
    dev_err(&pdev.dev, "failed to get mode[1] gpio\n");
    return PTR_ERR(lcd.lr_gpio);
    }
    lcd.ud_gpio = devm_gpiod_get_index(&pdev.dev, "mode", 2,
    GPIOD_OUT_LOW);
    if (IS_ERR(lcd.ud_gpio)) {
    dev_err(&pdev.dev, "failed to get mode[2] gpio\n");
    return PTR_ERR(lcd.ud_gpio);
    }
    drm_panel_add(&lcd.panel);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls037v7dw01_remove(pdev: *mut platform_device) {
    static void ls037v7dw01_remove(struct platform_device *pdev)
    {
    struct ls037v7dw01_panel *lcd = platform_get_drvdata(pdev);
    drm_panel_remove(&lcd.panel);
    drm_panel_disable(&lcd.panel);
    drm_panel_unprepare(&lcd.panel);
    }
    static const struct of_device_id ls037v7dw01_of_match[] = {
    { .compatible = "sharp,ls037v7dw01", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, ls037v7dw01_of_match);
    static struct platform_driver ls037v7dw01_driver = {
    .probe		= ls037v7dw01_probe,
    .remove		= ls037v7dw01_remove,
    .driver		= {
    .name = "panel-sharp-ls037v7dw01",
    .of_match_table = ls037v7dw01_of_match,
    },
    };
    module_platform_driver(ls037v7dw01_driver);
    MODULE_AUTHOR("Tomi Valkeinen <tomi.valkeinen@ti.com>");
    MODULE_DESCRIPTION("Sharp LS037V7DW01 Panel Driver");
    MODULE_LICENSE("GPL");
