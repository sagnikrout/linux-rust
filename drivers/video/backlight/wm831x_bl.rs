//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/wm831x_bl.c
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
// Backlight driver for Wolfson Microelectronics WM831x PMICs
//
// Copyright 2009 Wolfson Microelectonics plc
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm831x_backlight_data {
    pub wm831x: *mut wm831x,
    pub isink_reg: c_int,
    pub current_brightness: c_int,
}

#[no_mangle]
unsafe extern "C" fn wm831x_backlight_set(bl: *mut backlight_device, brightness: c_int) -> c_int {
    static int wm831x_backlight_set(struct backlight_device *bl, int brightness)
    {
    struct wm831x_backlight_data *data = bl_get_data(bl);
    struct wm831x *wm831x = data.wm831x;
    let mut power_up: c_int = !data.current_brightness && brightness;
    let mut power_down: c_int = data.current_brightness && !brightness;
    int ret;
    if (power_up) {
// Enable the ISINK
    ret = wm831x_set_bits(wm831x, data.isink_reg,
    WM831X_CS1_ENA, WM831X_CS1_ENA);
    if (ret < 0)
    goto err;
// Enable the DC-DC
    ret = wm831x_set_bits(wm831x, WM831X_DCDC_ENABLE,
    WM831X_DC4_ENA, WM831X_DC4_ENA);
    if (ret < 0)
    goto err;
    }
    if (power_down) {
// DCDC first
    ret = wm831x_set_bits(wm831x, WM831X_DCDC_ENABLE,
    WM831X_DC4_ENA, 0);
    if (ret < 0)
    goto err;
// ISINK
    ret = wm831x_set_bits(wm831x, data.isink_reg,
    WM831X_CS1_DRIVE | WM831X_CS1_ENA, 0);
    if (ret < 0)
    goto err;
    }
// Set the new brightness
    ret = wm831x_set_bits(wm831x, data.isink_reg,
    WM831X_CS1_ISEL_MASK, brightness);
    if (ret < 0)
    goto err;
    if (power_up) {
// Drive current through the ISINK
    ret = wm831x_set_bits(wm831x, data.isink_reg,
    WM831X_CS1_DRIVE, WM831X_CS1_DRIVE);
    if (ret < 0)
    return ret;
    }
    data.current_brightness = brightness;
    return 0;
    err:
// If we were in the middle of a power transition always shut down
// for safety.
//
    if (power_up || power_down) {
    wm831x_set_bits(wm831x, WM831X_DCDC_ENABLE, WM831X_DC4_ENA, 0);
    wm831x_set_bits(wm831x, data.isink_reg, WM831X_CS1_ENA, 0);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn wm831x_backlight_update_status(bl: *mut backlight_device) -> c_int {
    static int wm831x_backlight_update_status(struct backlight_device *bl)
    {
    return wm831x_backlight_set(bl, backlight_get_brightness(bl));
    }
#[no_mangle]
unsafe extern "C" fn wm831x_backlight_get_brightness(bl: *mut backlight_device) -> c_int {
    static int wm831x_backlight_get_brightness(struct backlight_device *bl)
    {
    struct wm831x_backlight_data *data = bl_get_data(bl);
    return data.current_brightness;
    }
    static const struct backlight_ops wm831x_backlight_ops = {
    .options = BL_CORE_SUSPENDRESUME,
    .update_status	= wm831x_backlight_update_status,
    .get_brightness	= wm831x_backlight_get_brightness,
    };
#[no_mangle]
unsafe extern "C" fn wm831x_backlight_probe(pdev: *mut platform_device) -> c_int {
    static int wm831x_backlight_probe(struct platform_device *pdev)
    {
    struct wm831x *wm831x = dev_get_drvdata(pdev.dev.parent);
    struct wm831x_pdata *wm831x_pdata = dev_get_platdata(pdev.dev.parent);
    struct wm831x_backlight_pdata *pdata;
    struct wm831x_backlight_data *data;
    struct backlight_device *bl;
    struct backlight_properties props;
    int ret, i, max_isel, isink_reg, dcdc_cfg;
// We need platform data
    if (wm831x_pdata)
    pdata = wm831x_pdata.backlight;
    else
    pdata = core::ptr::null_mut();
    if (!pdata) {
    dev_err(&pdev.dev, "No platform data supplied\n");
    return -EINVAL;
    }
// Figure out the maximum current we can use
    for (i = 0; i < WM831X_ISINK_MAX_ISEL; i++) {
    if (wm831x_isinkv_values[i] > pdata.max_uA)
    break;
    }
    if (i == 0) {
    dev_err(&pdev.dev, "Invalid max_uA: %duA\n", pdata.max_uA);
    return -EINVAL;
    }
    max_isel = i - 1;
    if (pdata.max_uA != wm831x_isinkv_values[max_isel])
    dev_warn(&pdev.dev,
    "Maximum current is %duA not %duA as requested\n",
    wm831x_isinkv_values[max_isel], pdata.max_uA);
    switch (pdata.isink) {
    case 1:
    isink_reg = WM831X_CURRENT_SINK_1;
    dcdc_cfg = 0;
    break;
    case 2:
    isink_reg = WM831X_CURRENT_SINK_2;
    dcdc_cfg = WM831X_DC4_FBSRC;
    break;
    default:
    dev_err(&pdev.dev, "Invalid ISINK %d\n", pdata.isink);
    return -EINVAL;
    }
// Configure the ISINK to use for feedback
    ret = wm831x_reg_unlock(wm831x);
    if (ret < 0)
    return ret;
    ret = wm831x_set_bits(wm831x, WM831X_DC4_CONTROL, WM831X_DC4_FBSRC,
    dcdc_cfg);
    wm831x_reg_lock(wm831x);
    if (ret < 0)
    return ret;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (data == core::ptr::null_mut())
    return -ENOMEM;
    data.wm831x = wm831x;
    data.current_brightness = 0;
    data.isink_reg = isink_reg;
    memset(&props, 0, sizeof(props));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = max_isel;
    bl = devm_backlight_device_register(&pdev.dev, "wm831x", &pdev.dev,
    data, &wm831x_backlight_ops, &props);
    if (IS_ERR(bl)) {
    dev_err(&pdev.dev, "failed to register backlight\n");
    return PTR_ERR(bl);
    }
    bl.props.brightness = max_isel;
    platform_set_drvdata(pdev, bl);
// Disable the DCDC if it was started so we can bootstrap
    wm831x_set_bits(wm831x, WM831X_DCDC_ENABLE, WM831X_DC4_ENA, 0);
    backlight_update_status(bl);
    return 0;
    }
    static struct platform_driver wm831x_backlight_driver = {
    .driver		= {
    .name	= "wm831x-backlight",
    },
    .probe		= wm831x_backlight_probe,
    };
    module_platform_driver(wm831x_backlight_driver);
    MODULE_DESCRIPTION("Backlight Driver for WM831x PMICs");
    MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:wm831x-backlight");
