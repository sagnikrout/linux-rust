//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-cobalt-qube.c
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
// Copyright 2006 - Florian Fainelli <florian@openwrt.org>
//
// Control the Cobalt Qube/RaQ front LED
//

pub const LED_FRONT_LEFT: c_uint = 0x01;
pub const LED_FRONT_RIGHT: c_uint = 0x02;
    static void __iomem *led_port;
    static u8 led_value;
    static void qube_front_led_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    if (brightness)
    led_value = LED_FRONT_LEFT | LED_FRONT_RIGHT;
    else
    led_value = ~(LED_FRONT_LEFT | LED_FRONT_RIGHT);
    writeb(led_value, led_port);
    }
    static struct led_classdev qube_front_led = {
    .name			= "qube::front",
    .brightness		= LED_FULL,
    .brightness_set		= qube_front_led_set,
    .default_trigger	= "default-on",
    };
#[no_mangle]
unsafe extern "C" fn cobalt_qube_led_probe(pdev: *mut platform_device) -> c_int {
    static int cobalt_qube_led_probe(struct platform_device *pdev)
    {
    struct resource *res;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -EBUSY;
    led_port = devm_ioremap(&pdev.dev, res.start, resource_size(res));
    if (!led_port)
    return -ENOMEM;
    led_value = LED_FRONT_LEFT | LED_FRONT_RIGHT;
    writeb(led_value, led_port);
    return devm_led_classdev_register(&pdev.dev, &qube_front_led);
    }
    static struct platform_driver cobalt_qube_led_driver = {
    .probe	= cobalt_qube_led_probe,
    .driver	= {
    .name	= "cobalt-qube-leds",
    },
    };
    module_platform_driver(cobalt_qube_led_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Front LED support for Cobalt Server");
    MODULE_AUTHOR("Florian Fainelli <florian@openwrt.org>");
    MODULE_ALIAS("platform:cobalt-qube-leds");
