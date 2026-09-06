//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-sc27xx-bltc.c
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
// Copyright (C) 2018 Spreadtrum Communications Inc.

// PMIC global control register definition
pub const SC27XX_MODULE_EN0: c_uint = 0xc08;
pub const SC27XX_CLK_EN0: c_uint = 0xc18;
pub const SC27XX_RGB_CTRL: c_uint = 0xebc;

// Breathing light controller register definition
pub const SC27XX_LEDS_CTRL: c_uint = 0x00;
pub const SC27XX_LEDS_PRESCALE: c_uint = 0x04;
pub const SC27XX_LEDS_DUTY: c_uint = 0x08;
pub const SC27XX_LEDS_CURVE0: c_uint = 0x0c;
pub const SC27XX_LEDS_CURVE1: c_uint = 0x10;
pub const SC27XX_CTRL_SHIFT: c_int = 4;

pub const SC27XX_DUTY_SHIFT: c_int = 8;

pub const SC27XX_CURVE_SHIFT: c_int = 8;

pub const SC27XX_LEDS_OFFSET: c_uint = 0x10;
pub const SC27XX_LEDS_MAX: c_int = 3;
pub const SC27XX_LEDS_PATTERN_CNT: c_int = 4;
// Stage duration step, in milliseconds
pub const SC27XX_LEDS_STEP: c_int = 125;
// Minimum and maximum duration, in milliseconds

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sc27xx_led {
    pub fwnode: *mut fwnode_handle,
    pub ldev: led_classdev,
    pub priv: *mut sc27xx_led_priv,
    pub line: u8,
    pub active: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sc27xx_led_priv {
    pub leds: [sc27xx_led; SC27XX_LEDS_MAX],
    pub regmap: *mut regmap,
    pub lock: mutex,
    pub base: u32,
}

    container_of(ldev, struct sc27xx_led, ldev)
#[no_mangle]
unsafe extern "C" fn sc27xx_led_init(regmap: *mut regmap) -> c_int {
    static int sc27xx_led_init(struct regmap *regmap)
    {
    int err;
    err = regmap_update_bits(regmap, SC27XX_MODULE_EN0, SC27XX_BLTC_EN,
    SC27XX_BLTC_EN);
    if (err)
    return err;
    err = regmap_update_bits(regmap, SC27XX_CLK_EN0, SC27XX_RTC_EN,
    SC27XX_RTC_EN);
    if (err)
    return err;
    return regmap_update_bits(regmap, SC27XX_RGB_CTRL, SC27XX_RGB_PD, 0);
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_led_get_offset(leds: *mut sc27xx_led) -> u32 {
    static u32 sc27xx_led_get_offset(struct sc27xx_led *leds)
    {
    return leds.priv.base + SC27XX_LEDS_OFFSET * leds.line;
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_led_enable(leds: *mut sc27xx_led, value: enum led_brightness) -> c_int {
    static int sc27xx_led_enable(struct sc27xx_led *leds, enum led_brightness value)
    {
    let mut base: u32 = sc27xx_led_get_offset(leds);
    let mut ctrl_base: u32 = leds.priv.base + SC27XX_LEDS_CTRL;
    let mut ctrl_shift: u8 = SC27XX_CTRL_SHIFT * leds.line;
    struct regmap *regmap = leds.priv.regmap;
    int err;
    err = regmap_update_bits(regmap, base + SC27XX_LEDS_DUTY,
    SC27XX_DUTY_MASK,
    (value << SC27XX_DUTY_SHIFT) |
    SC27XX_MOD_MASK);
    if (err)
    return err;
    return regmap_update_bits(regmap, ctrl_base,
    (SC27XX_LED_RUN | SC27XX_LED_TYPE) << ctrl_shift,
    (SC27XX_LED_RUN | SC27XX_LED_TYPE) << ctrl_shift);
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_led_disable(leds: *mut sc27xx_led) -> c_int {
    static int sc27xx_led_disable(struct sc27xx_led *leds)
    {
    struct regmap *regmap = leds.priv.regmap;
    let mut ctrl_base: u32 = leds.priv.base + SC27XX_LEDS_CTRL;
    let mut ctrl_shift: u8 = SC27XX_CTRL_SHIFT * leds.line;
    return regmap_update_bits(regmap, ctrl_base,
    (SC27XX_LED_RUN | SC27XX_LED_TYPE) << ctrl_shift, 0);
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_led_set(ldev: *mut led_classdev, value: enum led_brightness) -> c_int {
    static int sc27xx_led_set(struct led_classdev *ldev, enum led_brightness value)
    {
    struct sc27xx_led *leds = to_sc27xx_led(ldev);
    int err;
    mutex_lock(&leds.priv.lock);
    if (value == LED_OFF)
    err = sc27xx_led_disable(leds);
    else
    err = sc27xx_led_enable(leds, value);
    mutex_unlock(&leds.priv.lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_led_clamp_align_delta_t(delta_t: *mut u32) {
    static void sc27xx_led_clamp_align_delta_t(u32 *delta_t)
    {
    u32 v, offset, t = *delta_t;
    v = t + SC27XX_LEDS_STEP / 2;
    v = clamp_t(u32, v, SC27XX_DELTA_T_MIN, SC27XX_DELTA_T_MAX);
    offset = v - SC27XX_DELTA_T_MIN;
    offset = SC27XX_LEDS_STEP * (offset / SC27XX_LEDS_STEP);
// delta_t = SC27XX_DELTA_T_MIN + offset;
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_led_pattern_clear(ldev: *mut led_classdev) -> c_int {
    static int sc27xx_led_pattern_clear(struct led_classdev *ldev)
    {
    struct sc27xx_led *leds = to_sc27xx_led(ldev);
    struct regmap *regmap = leds.priv.regmap;
    let mut base: u32 = sc27xx_led_get_offset(leds);
    let mut ctrl_base: u32 = leds.priv.base + SC27XX_LEDS_CTRL;
    let mut ctrl_shift: u8 = SC27XX_CTRL_SHIFT * leds.line;
    int err;
    mutex_lock(&leds.priv.lock);
// Reset the rise, high, fall and low time to zero.
    regmap_write(regmap, base + SC27XX_LEDS_CURVE0, 0);
    regmap_write(regmap, base + SC27XX_LEDS_CURVE1, 0);
    err = regmap_update_bits(regmap, ctrl_base,
    (SC27XX_LED_RUN | SC27XX_LED_TYPE) << ctrl_shift, 0);
    ldev.brightness = LED_OFF;
    mutex_unlock(&leds.priv.lock);
    return err;
    }
    static int sc27xx_led_pattern_set(struct led_classdev *ldev,
    struct led_pattern *pattern,
    u32 len, int repeat)
    {
    struct sc27xx_led *leds = to_sc27xx_led(ldev);
    let mut base: u32 = sc27xx_led_get_offset(leds);
    let mut ctrl_base: u32 = leds.priv.base + SC27XX_LEDS_CTRL;
    let mut ctrl_shift: u8 = SC27XX_CTRL_SHIFT * leds.line;
    struct regmap *regmap = leds.priv.regmap;
    int err;
//
// Must contain 4 tuples to configure the rise time, high time, fall
// time and low time to enable the breathing mode.
//
    if (len != SC27XX_LEDS_PATTERN_CNT)
    return -EINVAL;
    mutex_lock(&leds.priv.lock);
    sc27xx_led_clamp_align_delta_t(&pattern[0].delta_t);
    err = regmap_update_bits(regmap, base + SC27XX_LEDS_CURVE0,
    SC27XX_CURVE_L_MASK,
    pattern[0].delta_t / SC27XX_LEDS_STEP);
    if (err)
    goto out;
    sc27xx_led_clamp_align_delta_t(&pattern[1].delta_t);
    err = regmap_update_bits(regmap, base + SC27XX_LEDS_CURVE1,
    SC27XX_CURVE_L_MASK,
    pattern[1].delta_t / SC27XX_LEDS_STEP);
    if (err)
    goto out;
    sc27xx_led_clamp_align_delta_t(&pattern[2].delta_t);
    err = regmap_update_bits(regmap, base + SC27XX_LEDS_CURVE0,
    SC27XX_CURVE_H_MASK,
    (pattern[2].delta_t / SC27XX_LEDS_STEP) <<
    SC27XX_CURVE_SHIFT);
    if (err)
    goto out;
    sc27xx_led_clamp_align_delta_t(&pattern[3].delta_t);
    err = regmap_update_bits(regmap, base + SC27XX_LEDS_CURVE1,
    SC27XX_CURVE_H_MASK,
    (pattern[3].delta_t / SC27XX_LEDS_STEP) <<
    SC27XX_CURVE_SHIFT);
    if (err)
    goto out;
    err = regmap_update_bits(regmap, base + SC27XX_LEDS_DUTY,
    SC27XX_DUTY_MASK,
    (pattern[1].brightness << SC27XX_DUTY_SHIFT) |
    SC27XX_MOD_MASK);
    if (err)
    goto out;
// Enable the LED breathing mode
    err = regmap_update_bits(regmap, ctrl_base,
    SC27XX_LED_RUN << ctrl_shift,
    SC27XX_LED_RUN << ctrl_shift);
    if (!err)
    ldev.brightness = pattern[1].brightness;
    out:
    mutex_unlock(&leds.priv.lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_led_register(dev: *mut device, priv: *mut sc27xx_led_priv) -> c_int {
    static int sc27xx_led_register(struct device *dev, struct sc27xx_led_priv *priv)
    {
    int i, err;
    err = sc27xx_led_init(priv.regmap);
    if (err)
    return err;
    for (i = 0; i < SC27XX_LEDS_MAX; i++) {
    struct sc27xx_led *led = &priv.leds[i];
    let mut init_data: led_init_data = {};
    if (!led.active)
    continue;
    led.line = i;
    led.priv = priv;
    led.ldev.brightness_set_blocking = sc27xx_led_set;
    led.ldev.pattern_set = sc27xx_led_pattern_set;
    led.ldev.pattern_clear = sc27xx_led_pattern_clear;
    led.ldev.default_trigger = "pattern";
    init_data.fwnode = led.fwnode;
    init_data.devicename = "sc27xx";
    init_data.default_label = ":";
    err = devm_led_classdev_register_ext(dev, &led.ldev,
    &init_data);
    if (err)
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_led_probe(pdev: *mut platform_device) -> c_int {
    static int sc27xx_led_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev_of_node(dev);
    struct sc27xx_led_priv *priv;
    u32 base, count, reg;
    int err;
    count = of_get_available_child_count(np);
    if (!count || count > SC27XX_LEDS_MAX)
    return -EINVAL;
    err = of_property_read_u32(np, "reg", &base);
    if (err) {
    dev_err(dev, "fail to get reg of property\n");
    return err;
    }
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    platform_set_drvdata(pdev, priv);
    priv.base = base;
    priv.regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!priv.regmap) {
    err = -ENODEV;
    dev_err(dev, "failed to get regmap: %d\n", err);
    return err;
    }
    for_each_available_child_of_node_scoped(np, child) {
    err = of_property_read_u32(child, "reg", &reg);
    if (err)
    return err;
    if (reg >= SC27XX_LEDS_MAX || priv.leds[reg].active)
    return -EINVAL;
    priv.leds[reg].fwnode = of_fwnode_handle(child);
    priv.leds[reg].active = true;
    }
    mutex_init(&priv.lock);
    err = sc27xx_led_register(dev, priv);
    if (err)
    mutex_destroy(&priv.lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_led_remove(pdev: *mut platform_device) {
    static void sc27xx_led_remove(struct platform_device *pdev)
    {
    struct sc27xx_led_priv *priv = platform_get_drvdata(pdev);
    mutex_destroy(&priv.lock);
    }
    static const struct of_device_id sc27xx_led_of_match[] = {
    { .compatible = "sprd,sc2731-bltc", },
    { }
    };
    MODULE_DEVICE_TABLE(of, sc27xx_led_of_match);
    static struct platform_driver sc27xx_led_driver = {
    .driver = {
    .name = "sprd-bltc",
    .of_match_table = sc27xx_led_of_match,
    },
    .probe = sc27xx_led_probe,
    .remove = sc27xx_led_remove,
    };
    module_platform_driver(sc27xx_led_driver);
    MODULE_DESCRIPTION("Spreadtrum SC27xx breathing light controller driver");
    MODULE_AUTHOR("Xiaotong Lu <xiaotong.lu@spreadtrum.com>");
    MODULE_AUTHOR("Baolin Wang <baolin.wang@linaro.org>");
    MODULE_LICENSE("GPL v2");
