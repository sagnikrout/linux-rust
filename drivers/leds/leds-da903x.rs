//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-da903x.c
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
// LEDs driver for Dialog Semiconductor DA9030/DA9034
//
// Copyright (C) 2008 Compulab, Ltd.
// Mike Rapoport <mike@compulab.co.il>
//
// Copyright (C) 2006-2008 Marvell International Ltd.
// Eric Miao <eric.miao@marvell.com>
//

pub const DA9030_LED1_CONTROL: c_uint = 0x20;
pub const DA9030_LED2_CONTROL: c_uint = 0x21;
pub const DA9030_LED3_CONTROL: c_uint = 0x22;
pub const DA9030_LED4_CONTROL: c_uint = 0x23;
pub const DA9030_LEDPC_CONTROL: c_uint = 0x24;
pub const DA9030_MISC_CONTROL_A: c_uint = 0x26	/* Vibrator Control */;
pub const DA9034_LED1_CONTROL: c_uint = 0x35;
pub const DA9034_LED2_CONTROL: c_uint = 0x36;
pub const DA9034_VIBRA: c_uint = 0x40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da903x_led {
    pub cdev: led_classdev,
    pub master: *mut device,
    pub id: c_int,
    pub flags: c_int,
}

    static int da903x_led_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct da903x_led *led =
    container_of(led_cdev, struct da903x_led, cdev);
    uint8_t val;
    int offset, ret = -EINVAL;
    switch (led.id) {
    case DA9030_ID_LED_1:
    case DA9030_ID_LED_2:
    case DA9030_ID_LED_3:
    case DA9030_ID_LED_4:
    case DA9030_ID_LED_PC:
    offset = DA9030_LED_OFFSET(led.id);
    val = led.flags & ~0x87;
    val |= value ? 0x80 : 0; /* EN bit */
    val |= (0x7 - (value >> 5)) & 0x7; /* PWM<2:0> */
    ret = da903x_write(led.master, DA9030_LED1_CONTROL + offset,
    val);
    break;
    case DA9030_ID_VIBRA:
    val = led.flags & ~0x80;
    val |= value ? 0x80 : 0; /* EN bit */
    ret = da903x_write(led.master, DA9030_MISC_CONTROL_A, val);
    break;
    case DA9034_ID_LED_1:
    case DA9034_ID_LED_2:
    offset = DA9034_LED_OFFSET(led.id);
    val = (value * 0x5f / LED_FULL) & 0x7f;
    val |= (led.flags & DA9034_LED_RAMP) ? 0x80 : 0;
    ret = da903x_write(led.master, DA9034_LED1_CONTROL + offset,
    val);
    break;
    case DA9034_ID_VIBRA:
    val = value & 0xfe;
    ret = da903x_write(led.master, DA9034_VIBRA, val);
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn da903x_led_probe(pdev: *mut platform_device) -> c_int {
    static int da903x_led_probe(struct platform_device *pdev)
    {
    struct led_info *pdata = dev_get_platdata(&pdev.dev);
    struct da903x_led *led;
    int id, ret;
    if (pdata == core::ptr::null_mut())
    return 0;
    id = pdev.id;
    if (!((id >= DA9030_ID_LED_1 && id <= DA9030_ID_VIBRA) ||
    (id >= DA9034_ID_LED_1 && id <= DA9034_ID_VIBRA))) {
    dev_err(&pdev.dev, "invalid LED ID (%d) specified\n", id);
    return -EINVAL;
    }
    led = devm_kzalloc(&pdev.dev, sizeof(struct da903x_led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    led.cdev.name = pdata.name;
    led.cdev.default_trigger = pdata.default_trigger;
    led.cdev.brightness_set_blocking = da903x_led_set;
    led.cdev.brightness = LED_OFF;
    led.id = id;
    led.flags = pdata.flags;
    led.master = pdev.dev.parent;
    ret = led_classdev_register(led.master, &led.cdev);
    if (ret) {
    dev_err(&pdev.dev, "failed to register LED %d\n", id);
    return ret;
    }
    platform_set_drvdata(pdev, led);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da903x_led_remove(pdev: *mut platform_device) {
    static void da903x_led_remove(struct platform_device *pdev)
    {
    struct da903x_led *led = platform_get_drvdata(pdev);
    led_classdev_unregister(&led.cdev);
    }
    static struct platform_driver da903x_led_driver = {
    .driver	= {
    .name	= "da903x-led",
    },
    .probe		= da903x_led_probe,
    .remove		= da903x_led_remove,
    };
    module_platform_driver(da903x_led_driver);
    MODULE_DESCRIPTION("LEDs driver for Dialog Semiconductor DA9030/DA9034");
    MODULE_AUTHOR("Eric Miao <eric.miao@marvell.com>");
    MODULE_AUTHOR("Mike Rapoport <mike@compulab.co.il>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:da903x-led");
