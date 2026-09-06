//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-adp5520.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// LEDs driver for Analog Devices ADP5520/ADP5501 MFD PMICs
//
// Copyright 2009 Analog Devices Inc.
//
// Loosely derived from leds-da903x:
// Copyright (C) 2008 Compulab, Ltd.
// Mike Rapoport <mike@compulab.co.il>
//
// Copyright (C) 2006-2008 Marvell International Ltd.
// Eric Miao <eric.miao@marvell.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp5520_led {
    pub cdev: led_classdev,
    pub master: *mut device,
    pub id: c_int,
    pub flags: c_int,
}

    static int adp5520_led_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct adp5520_led *led;
    led = container_of(led_cdev, struct adp5520_led, cdev);
    return adp5520_write(led.master, ADP5520_LED1_CURRENT + led.id - 1,
    value >> 2);
    }
#[no_mangle]
unsafe extern "C" fn adp5520_led_setup(led: *mut adp5520_led) -> c_int {
    static int adp5520_led_setup(struct adp5520_led *led)
    {
    struct device *dev = led.master;
    let mut flags: c_int = led.flags;
    let mut ret: c_int = 0;
    switch (led.id) {
    case FLAG_ID_ADP5520_LED1_ADP5501_LED0:
    ret |= adp5520_set_bits(dev, ADP5520_LED_TIME,
    (flags >> ADP5520_FLAG_OFFT_SHIFT) &
    ADP5520_FLAG_OFFT_MASK);
    ret |= adp5520_set_bits(dev, ADP5520_LED_CONTROL,
    ADP5520_LED1_EN);
    break;
    case FLAG_ID_ADP5520_LED2_ADP5501_LED1:
    ret |= adp5520_set_bits(dev,  ADP5520_LED_TIME,
    ((flags >> ADP5520_FLAG_OFFT_SHIFT) &
    ADP5520_FLAG_OFFT_MASK) << 2);
    ret |= adp5520_clr_bits(dev, ADP5520_LED_CONTROL,
    ADP5520_R3_MODE);
    ret |= adp5520_set_bits(dev, ADP5520_LED_CONTROL,
    ADP5520_LED2_EN);
    break;
    case FLAG_ID_ADP5520_LED3_ADP5501_LED2:
    ret |= adp5520_set_bits(dev,  ADP5520_LED_TIME,
    ((flags >> ADP5520_FLAG_OFFT_SHIFT) &
    ADP5520_FLAG_OFFT_MASK) << 4);
    ret |= adp5520_clr_bits(dev, ADP5520_LED_CONTROL,
    ADP5520_C3_MODE);
    ret |= adp5520_set_bits(dev, ADP5520_LED_CONTROL,
    ADP5520_LED3_EN);
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adp5520_led_prepare(pdev: *mut platform_device) -> c_int {
    static int adp5520_led_prepare(struct platform_device *pdev)
    {
    struct adp5520_leds_platform_data *pdata = dev_get_platdata(&pdev.dev);
    struct device *dev = pdev.dev.parent;
    let mut ret: c_int = 0;
    ret |= adp5520_write(dev, ADP5520_LED1_CURRENT, 0);
    ret |= adp5520_write(dev, ADP5520_LED2_CURRENT, 0);
    ret |= adp5520_write(dev, ADP5520_LED3_CURRENT, 0);
    ret |= adp5520_write(dev, ADP5520_LED_TIME, pdata.led_on_time << 6);
    ret |= adp5520_write(dev, ADP5520_LED_FADE, FADE_VAL(pdata.fade_in,
    pdata.fade_out));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adp5520_led_probe(pdev: *mut platform_device) -> c_int {
    static int adp5520_led_probe(struct platform_device *pdev)
    {
    struct adp5520_leds_platform_data *pdata = dev_get_platdata(&pdev.dev);
    struct adp5520_led *led, *led_dat;
    struct led_info *cur_led;
    int ret, i;
    if (pdata == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "missing platform data\n");
    return -ENODEV;
    }
    if (pdata.num_leds > ADP5520_01_MAXLEDS) {
    dev_err(&pdev.dev, "can't handle more than %d LEDS\n",
    ADP5520_01_MAXLEDS);
    return -EFAULT;
    }
    led = devm_kcalloc(&pdev.dev, pdata.num_leds, sizeof(*led),
    GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    ret = adp5520_led_prepare(pdev);
    if (ret) {
    dev_err(&pdev.dev, "failed to write\n");
    return ret;
    }
    for (i = 0; i < pdata.num_leds; ++i) {
    cur_led = &pdata.leds[i];
    led_dat = &led[i];
    led_dat.cdev.name = cur_led.name;
    led_dat.cdev.default_trigger = cur_led.default_trigger;
    led_dat.cdev.brightness_set_blocking = adp5520_led_set;
    led_dat.cdev.brightness = LED_OFF;
    if (cur_led.flags & ADP5520_FLAG_LED_MASK)
    led_dat.flags = cur_led.flags;
    else
    led_dat.flags = i + 1;
    led_dat.id = led_dat.flags & ADP5520_FLAG_LED_MASK;
    led_dat.master = pdev.dev.parent;
    ret = led_classdev_register(led_dat.master, &led_dat.cdev);
    if (ret) {
    dev_err(&pdev.dev, "failed to register LED %d\n",
    led_dat.id);
    goto err;
    }
    ret = adp5520_led_setup(led_dat);
    if (ret) {
    dev_err(&pdev.dev, "failed to write\n");
    i++;
    goto err;
    }
    }
    platform_set_drvdata(pdev, led);
    return 0;
    err:
    if (i > 0) {
    for (i = i - 1; i >= 0; i--)
    led_classdev_unregister(&led[i].cdev);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adp5520_led_remove(pdev: *mut platform_device) {
    static void adp5520_led_remove(struct platform_device *pdev)
    {
    struct adp5520_leds_platform_data *pdata = dev_get_platdata(&pdev.dev);
    struct adp5520_led *led;
    int i;
    led = platform_get_drvdata(pdev);
    adp5520_clr_bits(led.master, ADP5520_LED_CONTROL,
    ADP5520_LED1_EN | ADP5520_LED2_EN | ADP5520_LED3_EN);
    for (i = 0; i < pdata.num_leds; i++) {
    led_classdev_unregister(&led[i].cdev);
    }
    }
    static struct platform_driver adp5520_led_driver = {
    .driver	= {
    .name	= "adp5520-led",
    },
    .probe		= adp5520_led_probe,
    .remove		= adp5520_led_remove,
    };
    module_platform_driver(adp5520_led_driver);
    MODULE_AUTHOR("Michael Hennerich <hennerich@blackfin.uclinux.org>");
    MODULE_DESCRIPTION("LEDS ADP5520(01) Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:adp5520-led");
