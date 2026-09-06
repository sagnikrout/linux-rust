//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-lp8788.c
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
// TI LP8788 MFD - keyled driver
//
// Copyright 2012 Texas Instruments
//
// Author: Milo(Woogyom) Kim <milo.kim@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp8788_led {
    pub lp: *mut lp8788,
    pub lock: mutex,
    pub led_dev: led_classdev,
    pub isink_num: enum lp8788_isink_number,
    pub on: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp8788_led_config {
    pub scale: enum lp8788_isink_scale,
    pub num: enum lp8788_isink_number,
    pub iout: c_int,
}

    static struct lp8788_led_config default_led_config = {
    .scale = LP8788_ISINK_SCALE_100mA,
    .num   = LP8788_ISINK_3,
    .iout  = 0,
    };
    static int lp8788_led_init_device(struct lp8788_led *led,
    struct lp8788_led_platform_data *pdata)
    {
    struct lp8788_led_config *cfg = &default_led_config;
    u8 addr, mask, val;
    int ret;
    if (pdata) {
    cfg.scale = pdata.scale;
    cfg.num = pdata.num;
    cfg.iout = pdata.iout_code;
    }
    led.isink_num = cfg.num;
// scale configuration
    addr = LP8788_ISINK_CTRL;
    mask = 1 << (cfg.num + LP8788_ISINK_SCALE_OFFSET);
    val = cfg.scale << (cfg.num + LP8788_ISINK_SCALE_OFFSET);
    ret = lp8788_update_bits(led.lp, addr, mask, val);
    if (ret)
    return ret;
// current configuration
    addr = lp8788_iout_addr[cfg.num];
    mask = lp8788_iout_mask[cfg.num];
    val = cfg.iout;
    return lp8788_update_bits(led.lp, addr, mask, val);
    }
    static int lp8788_led_enable(struct lp8788_led *led,
    enum lp8788_isink_number num, int on)
    {
    int ret;
    let mut mask: u8 = 1 << num;
    let mut val: u8 = on << num;
    ret = lp8788_update_bits(led.lp, LP8788_ISINK_CTRL, mask, val);
    if (ret == 0)
    led.on = on;
    return ret;
    }
    static int lp8788_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness val)
    {
    struct lp8788_led *led =
    container_of(led_cdev, struct lp8788_led, led_dev);
    let mut num: enum lp8788_isink_number = led.isink_num;
    int enable, ret;
    mutex_lock(&led.lock);
    switch (num) {
    case LP8788_ISINK_1:
    case LP8788_ISINK_2:
    case LP8788_ISINK_3:
    ret = lp8788_write_byte(led.lp, lp8788_pwm_addr[num], val);
    if (ret < 0)
    goto unlock;
    break;
    default:
    mutex_unlock(&led.lock);
    return -EINVAL;
    }
    enable = (val > 0) ? 1 : 0;
    if (enable != led.on)
    ret = lp8788_led_enable(led, num, enable);
    unlock:
    mutex_unlock(&led.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lp8788_led_probe(pdev: *mut platform_device) -> c_int {
    static int lp8788_led_probe(struct platform_device *pdev)
    {
    struct lp8788 *lp = dev_get_drvdata(pdev.dev.parent);
    struct lp8788_led_platform_data *led_pdata;
    struct lp8788_led *led;
    struct device *dev = &pdev.dev;
    int ret;
    led = devm_kzalloc(dev, sizeof(struct lp8788_led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    led.lp = lp;
    led.led_dev.max_brightness = MAX_BRIGHTNESS;
    led.led_dev.brightness_set_blocking = lp8788_brightness_set;
    led_pdata = lp.pdata ? lp.pdata.led_pdata : core::ptr::null_mut();
    if (!led_pdata || !led_pdata.name)
    led.led_dev.name = DEFAULT_LED_NAME;
    else
    led.led_dev.name = led_pdata.name;
    mutex_init(&led.lock);
    ret = lp8788_led_init_device(led, led_pdata);
    if (ret) {
    dev_err(dev, "led init device err: %d\n", ret);
    return ret;
    }
    ret = devm_led_classdev_register(dev, &led.led_dev);
    if (ret) {
    dev_err(dev, "led register err: %d\n", ret);
    return ret;
    }
    return 0;
    }
    static struct platform_driver lp8788_led_driver = {
    .probe = lp8788_led_probe,
    .driver = {
    .name = LP8788_DEV_KEYLED,
    },
    };
    module_platform_driver(lp8788_led_driver);
    MODULE_DESCRIPTION("Texas Instruments LP8788 Keyboard LED Driver");
    MODULE_AUTHOR("Milo Kim");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:lp8788-keyled");
