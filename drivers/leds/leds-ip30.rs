//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-ip30.c
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
//
// LED Driver for SGI Octane machines
//

pub const IP30_LED_SYSTEM: c_int = 0;
pub const IP30_LED_FAULT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip30_led {
    pub cdev: led_classdev,
    pub reg: *mut u32 __iomem,
}

    static void ip30led_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct ip30_led *led = container_of(led_cdev, struct ip30_led, cdev);
    writel(value, led.reg);
    }
#[no_mangle]
unsafe extern "C" fn ip30led_create(pdev: *mut platform_device, num: c_int) -> c_int {
    static int ip30led_create(struct platform_device *pdev, int num)
    {
    struct ip30_led *data;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.reg = devm_platform_ioremap_resource(pdev, num);
    if (IS_ERR(data.reg))
    return PTR_ERR(data.reg);
    switch (num) {
    case IP30_LED_SYSTEM:
    data.cdev.name = "white:power";
    break;
    case IP30_LED_FAULT:
    data.cdev.name = "red:fault";
    break;
    default:
    return -EINVAL;
    }
    data.cdev.brightness = readl(data.reg);
    data.cdev.max_brightness = 1;
    data.cdev.brightness_set = ip30led_set;
    return devm_led_classdev_register(&pdev.dev, &data.cdev);
    }
#[no_mangle]
unsafe extern "C" fn ip30led_probe(pdev: *mut platform_device) -> c_int {
    static int ip30led_probe(struct platform_device *pdev)
    {
    int ret;
    ret = ip30led_create(pdev, IP30_LED_SYSTEM);
    if (ret < 0)
    return ret;
    return ip30led_create(pdev, IP30_LED_FAULT);
    }
    static struct platform_driver ip30led_driver = {
    .probe		= ip30led_probe,
    .driver		= {
    .name		= "ip30-leds",
    },
    };
    module_platform_driver(ip30led_driver);
    MODULE_AUTHOR("Thomas Bogendoerfer <tbogendoerfer@suse.de>");
    MODULE_DESCRIPTION("SGI Octane LED driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:ip30-leds");
