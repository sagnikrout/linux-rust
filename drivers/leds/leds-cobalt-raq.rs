//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-cobalt-raq.c
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
// LEDs driver for the Cobalt Raq series.
//
// Copyright (C) 2007  Yoichi Yuasa <yuasa@linux-mips.org>
//

pub const LED_WEB: c_uint = 0x04;
pub const LED_POWER_OFF: c_uint = 0x08;
    static void __iomem *led_port;
    static u8 led_value;
    static DEFINE_SPINLOCK(led_value_lock);
    static void raq_web_led_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    unsigned long flags;
    spin_lock_irqsave(&led_value_lock, flags);
    if (brightness)
    led_value |= LED_WEB;
    else
    led_value &= ~LED_WEB;
    writeb(led_value, led_port);
    spin_unlock_irqrestore(&led_value_lock, flags);
    }
    static struct led_classdev raq_web_led = {
    .name		= "raq::web",
    .brightness_set	= raq_web_led_set,
    };
    static void raq_power_off_led_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    unsigned long flags;
    spin_lock_irqsave(&led_value_lock, flags);
    if (brightness)
    led_value |= LED_POWER_OFF;
    else
    led_value &= ~LED_POWER_OFF;
    writeb(led_value, led_port);
    spin_unlock_irqrestore(&led_value_lock, flags);
    }
    static struct led_classdev raq_power_off_led = {
    .name			= "raq::power-off",
    .brightness_set		= raq_power_off_led_set,
    .default_trigger	= "power-off",
    };
#[no_mangle]
unsafe extern "C" fn cobalt_raq_led_probe(pdev: *mut platform_device) -> c_int {
    static int cobalt_raq_led_probe(struct platform_device *pdev)
    {
    struct resource *res;
    int retval;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -EBUSY;
    led_port = devm_ioremap(&pdev.dev, res.start, resource_size(res));
    if (!led_port)
    return -ENOMEM;
    retval = led_classdev_register(&pdev.dev, &raq_power_off_led);
    if (retval)
    goto err_null;
    retval = led_classdev_register(&pdev.dev, &raq_web_led);
    if (retval)
    goto err_unregister;
    return 0;
    err_unregister:
    led_classdev_unregister(&raq_power_off_led);
    err_null:
    led_port = core::ptr::null_mut();
    return retval;
    }
    static struct platform_driver cobalt_raq_led_driver = {
    .probe	= cobalt_raq_led_probe,
    .driver = {
    .name	= "cobalt-raq-leds",
    },
    };
    builtin_platform_driver(cobalt_raq_led_driver);
