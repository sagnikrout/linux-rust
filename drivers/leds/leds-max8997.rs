//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-max8997.c
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
// leds-max8997.c - LED class driver for MAX8997 LEDs.
//
// Copyright (C) 2011 Samsung Electronics
// Donggeun Kim <dg77.kim@samsung.com>
//

pub const MAX8997_LED_FLASH_SHIFT: c_int = 3;
pub const MAX8997_LED_FLASH_CUR_MASK: c_uint = 0xf8;
pub const MAX8997_LED_MOVIE_SHIFT: c_int = 4;
pub const MAX8997_LED_MOVIE_CUR_MASK: c_uint = 0xf0;
pub const MAX8997_LED_FLASH_MAX_BRIGHTNESS: c_uint = 0x1f;
pub const MAX8997_LED_MOVIE_MAX_BRIGHTNESS: c_uint = 0xf;
pub const MAX8997_LED_NONE_MAX_BRIGHTNESS: c_int = 0;
pub const MAX8997_LED0_FLASH_MASK: c_uint = 0x1;
pub const MAX8997_LED0_FLASH_PIN_MASK: c_uint = 0x5;
pub const MAX8997_LED0_MOVIE_MASK: c_uint = 0x8;
pub const MAX8997_LED0_MOVIE_PIN_MASK: c_uint = 0x28;
pub const MAX8997_LED1_FLASH_MASK: c_uint = 0x2;
pub const MAX8997_LED1_FLASH_PIN_MASK: c_uint = 0x6;
pub const MAX8997_LED1_MOVIE_MASK: c_uint = 0x10;
pub const MAX8997_LED1_MOVIE_PIN_MASK: c_uint = 0x30;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8997_led {
    pub iodev: *mut max8997_dev,
    pub cdev: led_classdev,
    pub enabled: bool,
    pub id: c_int,
    pub led_mode: enum max8997_led_mode,
    pub mutex: mutex,
}

    static void max8997_led_set_mode(struct max8997_led *led,
    enum max8997_led_mode mode)
    {
    int ret;
    struct i2c_client *client = led.iodev.i2c;
    let mut mask: u8 = 0, val;
    switch (mode) {
    case MAX8997_FLASH_MODE:
    mask = MAX8997_LED1_FLASH_MASK | MAX8997_LED0_FLASH_MASK;
    val = led.id ?
    MAX8997_LED1_FLASH_MASK : MAX8997_LED0_FLASH_MASK;
    led.cdev.max_brightness = MAX8997_LED_FLASH_MAX_BRIGHTNESS;
    break;
    case MAX8997_MOVIE_MODE:
    mask = MAX8997_LED1_MOVIE_MASK | MAX8997_LED0_MOVIE_MASK;
    val = led.id ?
    MAX8997_LED1_MOVIE_MASK : MAX8997_LED0_MOVIE_MASK;
    led.cdev.max_brightness = MAX8997_LED_MOVIE_MAX_BRIGHTNESS;
    break;
    case MAX8997_FLASH_PIN_CONTROL_MODE:
    mask = MAX8997_LED1_FLASH_PIN_MASK |
    MAX8997_LED0_FLASH_PIN_MASK;
    val = led.id ?
    MAX8997_LED1_FLASH_PIN_MASK : MAX8997_LED0_FLASH_PIN_MASK;
    led.cdev.max_brightness = MAX8997_LED_FLASH_MAX_BRIGHTNESS;
    break;
    case MAX8997_MOVIE_PIN_CONTROL_MODE:
    mask = MAX8997_LED1_MOVIE_PIN_MASK |
    MAX8997_LED0_MOVIE_PIN_MASK;
    val = led.id ?
    MAX8997_LED1_MOVIE_PIN_MASK : MAX8997_LED0_MOVIE_PIN_MASK;
    led.cdev.max_brightness = MAX8997_LED_MOVIE_MAX_BRIGHTNESS;
    break;
    default:
    led.cdev.max_brightness = MAX8997_LED_NONE_MAX_BRIGHTNESS;
    break;
    }
    if (mask) {
    ret = max8997_update_reg(client, MAX8997_REG_LEN_CNTL, val,
    mask);
    if (ret)
    dev_err(led.iodev.dev,
    "failed to update register(%d)\n", ret);
    }
    led.led_mode = mode;
    }
#[no_mangle]
unsafe extern "C" fn max8997_led_enable(led: *mut max8997_led, enable: bool) {
    static void max8997_led_enable(struct max8997_led *led, bool enable)
    {
    int ret;
    struct i2c_client *client = led.iodev.i2c;
    let mut val: u8 = 0, mask = MAX8997_LED_BOOST_ENABLE_MASK;
    if (led.enabled == enable)
    return;
    val = enable ? MAX8997_LED_BOOST_ENABLE_MASK : 0;
    ret = max8997_update_reg(client, MAX8997_REG_BOOST_CNTL, val, mask);
    if (ret)
    dev_err(led.iodev.dev,
    "failed to update register(%d)\n", ret);
    led.enabled = enable;
    }
    static void max8997_led_set_current(struct max8997_led *led,
    enum led_brightness value)
    {
    int ret;
    struct i2c_client *client = led.iodev.i2c;
    let mut val: u8 = 0, mask = 0, reg = 0;
    switch (led.led_mode) {
    case MAX8997_FLASH_MODE:
    case MAX8997_FLASH_PIN_CONTROL_MODE:
    val = value << MAX8997_LED_FLASH_SHIFT;
    mask = MAX8997_LED_FLASH_CUR_MASK;
    reg = led.id ? MAX8997_REG_FLASH2_CUR : MAX8997_REG_FLASH1_CUR;
    break;
    case MAX8997_MOVIE_MODE:
    case MAX8997_MOVIE_PIN_CONTROL_MODE:
    val = value << MAX8997_LED_MOVIE_SHIFT;
    mask = MAX8997_LED_MOVIE_CUR_MASK;
    reg = MAX8997_REG_MOVIE_CUR;
    break;
    default:
    break;
    }
    if (mask) {
    ret = max8997_update_reg(client, reg, val, mask);
    if (ret)
    dev_err(led.iodev.dev,
    "failed to update register(%d)\n", ret);
    }
    }
    static void max8997_led_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct max8997_led *led =
    container_of(led_cdev, struct max8997_led, cdev);
    if (value) {
    max8997_led_set_current(led, value);
    max8997_led_enable(led, true);
    } else {
    max8997_led_set_current(led, value);
    max8997_led_enable(led, false);
    }
    }
    static ssize_t mode_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct led_classdev *led_cdev = dev_get_drvdata(dev);
    struct max8997_led *led =
    container_of(led_cdev, struct max8997_led, cdev);
    let mut ret: isize = 0;
    mutex_lock(&led.mutex);
    switch (led.led_mode) {
    case MAX8997_FLASH_MODE:
    ret += sprintf(buf, "FLASH\n");
    break;
    case MAX8997_MOVIE_MODE:
    ret += sprintf(buf, "MOVIE\n");
    break;
    case MAX8997_FLASH_PIN_CONTROL_MODE:
    ret += sprintf(buf, "FLASH_PIN_CONTROL\n");
    break;
    case MAX8997_MOVIE_PIN_CONTROL_MODE:
    ret += sprintf(buf, "MOVIE_PIN_CONTROL\n");
    break;
    default:
    ret += sprintf(buf, "NONE\n");
    break;
    }
    mutex_unlock(&led.mutex);
    return ret;
    }
    static ssize_t mode_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    struct led_classdev *led_cdev = dev_get_drvdata(dev);
    struct max8997_led *led =
    container_of(led_cdev, struct max8997_led, cdev);
    enum max8997_led_mode mode;
    mutex_lock(&led.mutex);
    if (!strncmp(buf, "FLASH_PIN_CONTROL", 17))
    mode = MAX8997_FLASH_PIN_CONTROL_MODE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(buf, _arg: "MOVIE_PIN_CONTROL", _arg: 17)) -> else {
    else if (!strncmp(buf, "MOVIE_PIN_CONTROL", 17))
    mode = MAX8997_MOVIE_PIN_CONTROL_MODE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(buf, _arg: "FLASH", _arg: 5)) -> else {
    else if (!strncmp(buf, "FLASH", 5))
    mode = MAX8997_FLASH_MODE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(buf, _arg: "MOVIE", _arg: 5)) -> else {
    else if (!strncmp(buf, "MOVIE", 5))
    mode = MAX8997_MOVIE_MODE;
    else
    mode = MAX8997_NONE;
    max8997_led_set_mode(led, mode);
    mutex_unlock(&led.mutex);
    return size;
    }
    static DEVICE_ATTR_RW(mode);
    static struct attribute *max8997_attrs[] = {
    &dev_attr_mode.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(max8997);
#[no_mangle]
unsafe extern "C" fn max8997_led_probe(pdev: *mut platform_device) -> c_int {
    static int max8997_led_probe(struct platform_device *pdev)
    {
    struct max8997_dev *iodev = dev_get_drvdata(pdev.dev.parent);
    struct max8997_platform_data *pdata = dev_get_platdata(iodev.dev);
    struct max8997_led *led;
    char name[20];
    let mut ret: c_int = 0;
    led = devm_kzalloc(&pdev.dev, sizeof(*led), GFP_KERNEL);
    if (led == core::ptr::null_mut())
    return -ENOMEM;
    led.id = pdev.id;
    snprintf(name, sizeof(name), "max8997-led%d", pdev.id);
    led.cdev.name = name;
    led.cdev.brightness_set = max8997_led_brightness_set;
    led.cdev.flags |= LED_CORE_SUSPENDRESUME;
    led.cdev.brightness = 0;
    led.cdev.groups = max8997_groups;
    led.iodev = iodev;
// initialize mode and brightness according to platform_data
    if (pdata && pdata.led_pdata) {
    let mut mode: u8 = 0, brightness = 0;
    mode = pdata.led_pdata.mode[led.id];
    brightness = pdata.led_pdata.brightness[led.id];
    max8997_led_set_mode(led, mode);
    if (brightness > led.cdev.max_brightness)
    brightness = led.cdev.max_brightness;
    max8997_led_set_current(led, brightness);
    led.cdev.brightness = brightness;
    } else {
    max8997_led_set_mode(led, MAX8997_NONE);
    max8997_led_set_current(led, 0);
    }
    mutex_init(&led.mutex);
    ret = devm_led_classdev_register(&pdev.dev, &led.cdev);
    if (ret < 0)
    return ret;
    return 0;
    }
    static struct platform_driver max8997_led_driver = {
    .driver = {
    .name  = "max8997-led",
    },
    .probe  = max8997_led_probe,
    };
    module_platform_driver(max8997_led_driver);
    MODULE_AUTHOR("Donggeun Kim <dg77.kim@samsung.com>");
    MODULE_DESCRIPTION("MAX8997 LED driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:max8997-led");
