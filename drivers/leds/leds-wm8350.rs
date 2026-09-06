//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-wm8350.c
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
// LED driver for WM8350 driven LEDS.
//
// Copyright(C) 2007, 2008 Wolfson Microelectronics PLC.
//

// Microamps
    static const int isink_cur[] = {
    4,
    5,
    6,
    7,
    8,
    10,
    11,
    14,
    16,
    19,
    23,
    27,
    32,
    39,
    46,
    54,
    65,
    77,
    92,
    109,
    130,
    154,
    183,
    218,
    259,
    308,
    367,
    436,
    518,
    616,
    733,
    872,
    1037,
    1233,
    1466,
    1744,
    2073,
    2466,
    2933,
    3487,
    4147,
    4932,
    5865,
    6975,
    8294,
    9864,
    11730,
    13949,
    16589,
    19728,
    23460,
    27899,
    33178,
    39455,
    46920,
    55798,
    66355,
    78910,
    93840,
    111596,
    132710,
    157820,
    187681,
    223191
    };

    container_of(led_cdev, struct wm8350_led, cdev)
#[no_mangle]
unsafe extern "C" fn wm8350_led_enable(led: *mut wm8350_led) -> c_int {
    static int wm8350_led_enable(struct wm8350_led *led)
    {
    let mut ret: c_int = 0;
    if (led.enabled)
    return ret;
    ret = regulator_enable(led.isink);
    if (ret != 0) {
    dev_err(led.cdev.dev, "Failed to enable ISINK: %d\n", ret);
    return ret;
    }
    ret = regulator_enable(led.dcdc);
    if (ret != 0) {
    dev_err(led.cdev.dev, "Failed to enable DCDC: %d\n", ret);
    regulator_disable(led.isink);
    return ret;
    }
    led.enabled = 1;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn wm8350_led_disable(led: *mut wm8350_led) -> c_int {
    static int wm8350_led_disable(struct wm8350_led *led)
    {
    let mut ret: c_int = 0;
    if (!led.enabled)
    return ret;
    ret = regulator_disable(led.dcdc);
    if (ret != 0) {
    dev_err(led.cdev.dev, "Failed to disable DCDC: %d\n", ret);
    return ret;
    }
    ret = regulator_disable(led.isink);
    if (ret != 0) {
    dev_err(led.cdev.dev, "Failed to disable ISINK: %d\n", ret);
    ret = regulator_enable(led.dcdc);
    if (ret != 0)
    dev_err(led.cdev.dev, "Failed to reenable DCDC: %d\n",
    ret);
    return ret;
    }
    led.enabled = 0;
    return ret;
    }
    static int wm8350_led_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct wm8350_led *led = to_wm8350_led(led_cdev);
    unsigned long flags;
    int ret;
    int uA;
    led.value = value;
    spin_lock_irqsave(&led.value_lock, flags);
    if (led.value == LED_OFF) {
    spin_unlock_irqrestore(&led.value_lock, flags);
    return wm8350_led_disable(led);
    }
// This scales linearly into the index of valid current
// settings which results in a linear scaling of perceived
// brightness due to the non-linear current settings provided
// by the hardware.
//
    uA = (led.max_uA_index * led.value) / LED_FULL;
    spin_unlock_irqrestore(&led.value_lock, flags);
    BUG_ON(uA >= ARRAY_SIZE(isink_cur));
    ret = regulator_set_current_limit(led.isink, isink_cur[uA],
    isink_cur[uA]);
    if (ret != 0) {
    dev_err(led.cdev.dev, "Failed to set %duA: %d\n",
    isink_cur[uA], ret);
    return ret;
    }
    return wm8350_led_enable(led);
    }
#[no_mangle]
unsafe extern "C" fn wm8350_led_shutdown(pdev: *mut platform_device) {
    static void wm8350_led_shutdown(struct platform_device *pdev)
    {
    struct wm8350_led *led = platform_get_drvdata(pdev);
    led.value = LED_OFF;
    wm8350_led_disable(led);
    }
#[no_mangle]
unsafe extern "C" fn wm8350_led_probe(pdev: *mut platform_device) -> c_int {
    static int wm8350_led_probe(struct platform_device *pdev)
    {
    struct regulator *isink, *dcdc;
    struct wm8350_led *led;
    struct wm8350_led_platform_data *pdata = dev_get_platdata(&pdev.dev);
    int i;
    if (pdata == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "no platform data\n");
    return -ENODEV;
    }
    if (pdata.max_uA < isink_cur[0]) {
    dev_err(&pdev.dev, "Invalid maximum current %duA\n",
    pdata.max_uA);
    return -EINVAL;
    }
    isink = devm_regulator_get(&pdev.dev, "led_isink");
    if (IS_ERR(isink)) {
    dev_err(&pdev.dev, "%s: can't get ISINK\n", __func__);
    return PTR_ERR(isink);
    }
    dcdc = devm_regulator_get(&pdev.dev, "led_vcc");
    if (IS_ERR(dcdc)) {
    dev_err(&pdev.dev, "%s: can't get DCDC\n", __func__);
    return PTR_ERR(dcdc);
    }
    led = devm_kzalloc(&pdev.dev, sizeof(*led), GFP_KERNEL);
    if (led == core::ptr::null_mut())
    return -ENOMEM;
    led.cdev.brightness_set_blocking = wm8350_led_set;
    led.cdev.default_trigger = pdata.default_trigger;
    led.cdev.name = pdata.name;
    led.cdev.flags |= LED_CORE_SUSPENDRESUME;
    led.enabled = regulator_is_enabled(isink);
    led.isink = isink;
    led.dcdc = dcdc;
    for (i = 0; i < ARRAY_SIZE(isink_cur) - 1; i++)
    if (isink_cur[i] >= pdata.max_uA)
    break;
    led.max_uA_index = i;
    if (pdata.max_uA != isink_cur[i])
    dev_warn(&pdev.dev,
    "Maximum current %duA is not directly supported,"
    " check platform data\n",
    pdata.max_uA);
    spin_lock_init(&led.value_lock);
    led.value = LED_OFF;
    platform_set_drvdata(pdev, led);
    return led_classdev_register(&pdev.dev, &led.cdev);
    }
#[no_mangle]
unsafe extern "C" fn wm8350_led_remove(pdev: *mut platform_device) {
    static void wm8350_led_remove(struct platform_device *pdev)
    {
    struct wm8350_led *led = platform_get_drvdata(pdev);
    led_classdev_unregister(&led.cdev);
    wm8350_led_disable(led);
    }
    static struct platform_driver wm8350_led_driver = {
    .driver = {
    .name = "wm8350-led",
    },
    .probe = wm8350_led_probe,
    .remove = wm8350_led_remove,
    .shutdown = wm8350_led_shutdown,
    };
    module_platform_driver(wm8350_led_driver);
    MODULE_AUTHOR("Mark Brown");
    MODULE_DESCRIPTION("WM8350 LED driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:wm8350-led");
