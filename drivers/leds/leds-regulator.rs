//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-regulator.c
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
// leds-regulator.c - LED class driver for regulator driven LEDs.
//
// Copyright (C) 2009 Antonio Ospite <ospite@studenti.unina.it>
//
// Inspired by leds-wm8350 driver.
//

    container_of(led_cdev, struct regulator_led, cdev)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_led {
    pub cdev: led_classdev,
    pub enabled: c_int,
    pub mutex: mutex,
    pub vcc: *mut regulator,
}

#[no_mangle]
pub unsafe extern "C" fn led_regulator_get_max_brightness(supply: *mut regulator) -> c_int {
    static inline int led_regulator_get_max_brightness(struct regulator *supply)
    {
    int ret;
    let mut voltage: c_int = regulator_list_voltage(supply, 0);
    if (voltage <= 0)
    return 1;
// even if regulator can't change voltages,
// we still assume it can change status
// and the LED can be turned on and off.
//
    ret = regulator_set_voltage(supply, voltage, voltage);
    if (ret < 0)
    return 1;
    return regulator_count_voltages(supply);
    }
    static int led_regulator_get_voltage(struct regulator *supply,
    enum led_brightness brightness)
    {
    if (brightness == 0)
    return -EINVAL;
    return regulator_list_voltage(supply, brightness - 1);
    }
#[no_mangle]
unsafe extern "C" fn regulator_led_enable(led: *mut regulator_led) {
    static void regulator_led_enable(struct regulator_led *led)
    {
    int ret;
    if (led.enabled)
    return;
    ret = regulator_enable(led.vcc);
    if (ret != 0) {
    dev_err(led.cdev.dev, "Failed to enable vcc: %d\n", ret);
    return;
    }
    led.enabled = 1;
    }
#[no_mangle]
unsafe extern "C" fn regulator_led_disable(led: *mut regulator_led) {
    static void regulator_led_disable(struct regulator_led *led)
    {
    int ret;
    if (!led.enabled)
    return;
    ret = regulator_disable(led.vcc);
    if (ret != 0) {
    dev_err(led.cdev.dev, "Failed to disable vcc: %d\n", ret);
    return;
    }
    led.enabled = 0;
    }
    static int regulator_led_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct regulator_led *led = to_regulator_led(led_cdev);
    int voltage;
    let mut ret: c_int = 0;
    mutex_lock(&led.mutex);
    if (value == LED_OFF) {
    regulator_led_disable(led);
    goto out;
    }
    if (led.cdev.max_brightness > 1) {
    voltage = led_regulator_get_voltage(led.vcc, value);
    dev_dbg(led.cdev.dev, "brightness: %d voltage: %d\n",
    value, voltage);
    ret = regulator_set_voltage(led.vcc, voltage, voltage);
    if (ret != 0)
    dev_err(led.cdev.dev, "Failed to set voltage %d: %d\n",
    voltage, ret);
    }
    regulator_led_enable(led);
    out:
    mutex_unlock(&led.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn regulator_led_probe(pdev: *mut platform_device) -> c_int {
    static int regulator_led_probe(struct platform_device *pdev)
    {
    struct led_regulator_platform_data *pdata =
    dev_get_platdata(&pdev.dev);
    struct device *dev = &pdev.dev;
    let mut init_data: led_init_data = {};
    struct regulator_led *led;
    struct regulator *vcc;
    let mut ret: c_int = 0;
    vcc = devm_regulator_get_exclusive(dev, "vled");
    if (IS_ERR(vcc)) {
    dev_err(dev, "Cannot get vcc\n");
    return PTR_ERR(vcc);
    }
    led = devm_kzalloc(dev, sizeof(*led), GFP_KERNEL);
    if (led == core::ptr::null_mut())
    return -ENOMEM;
    init_data.fwnode = dev.fwnode;
    led.cdev.max_brightness = led_regulator_get_max_brightness(vcc);
// Legacy platform data label assignment
    if (pdata) {
    if (pdata.brightness > led.cdev.max_brightness) {
    dev_err(dev, "Invalid default brightness %d\n",
    pdata.brightness);
    return -EINVAL;
    }
    led.cdev.brightness = pdata.brightness;
    init_data.default_label = pdata.name;
    }
    led.cdev.brightness_set_blocking = regulator_led_brightness_set;
    led.cdev.flags |= LED_CORE_SUSPENDRESUME;
    led.vcc = vcc;
// to handle correctly an already enabled regulator
    if (regulator_is_enabled(led.vcc))
    led.enabled = 1;
    mutex_init(&led.mutex);
    platform_set_drvdata(pdev, led);
    ret = led_classdev_register_ext(dev, &led.cdev, &init_data);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regulator_led_remove(pdev: *mut platform_device) {
    static void regulator_led_remove(struct platform_device *pdev)
    {
    struct regulator_led *led = platform_get_drvdata(pdev);
    led_classdev_unregister(&led.cdev);
    regulator_led_disable(led);
    }
    static const struct of_device_id regulator_led_of_match[] = {
    { .compatible = "regulator-led", },
    {}
    };
    MODULE_DEVICE_TABLE(of, regulator_led_of_match);
    static struct platform_driver regulator_led_driver = {
    .driver = {
    .name  = "leds-regulator",
    .of_match_table = regulator_led_of_match,
    },
    .probe  = regulator_led_probe,
    .remove = regulator_led_remove,
    };
    module_platform_driver(regulator_led_driver);
    MODULE_AUTHOR("Antonio Ospite <ospite@studenti.unina.it>");
    MODULE_DESCRIPTION("Regulator driven LED driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:leds-regulator");
