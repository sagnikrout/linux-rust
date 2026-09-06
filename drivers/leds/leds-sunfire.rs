//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-sunfire.c
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
// leds-sunfire.c: SUNW,Ultra-Enterprise LED driver.
//
// Copyright (C) 2008 David S. Miller <davem@davemloft.net>
//

    MODULE_AUTHOR("David S. Miller <davem@davemloft.net>");
    MODULE_DESCRIPTION("Sun Fire LED driver");
    MODULE_LICENSE("GPL");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunfire_led {
    pub led_cdev: led_classdev,
    pub reg: *mut void __iomem,
}

    static void __clockboard_set(struct led_classdev *led_cdev,
    enum led_brightness led_val, u8 bit)
    {
    struct sunfire_led *p = to_sunfire_led(led_cdev);
    let mut reg: u8 = upa_readb(p.reg);
    switch (bit) {
    case CLOCK_CTRL_LLED:
    if (led_val)
    reg &= ~bit;
    else
    reg |= bit;
    break;
    default:
    if (led_val)
    reg |= bit;
    else
    reg &= ~bit;
    break;
    }
    upa_writeb(reg, p.reg);
    }
    static void clockboard_left_set(struct led_classdev *led_cdev,
    enum led_brightness led_val)
    {
    __clockboard_set(led_cdev, led_val, CLOCK_CTRL_LLED);
    }
    static void clockboard_middle_set(struct led_classdev *led_cdev,
    enum led_brightness led_val)
    {
    __clockboard_set(led_cdev, led_val, CLOCK_CTRL_MLED);
    }
    static void clockboard_right_set(struct led_classdev *led_cdev,
    enum led_brightness led_val)
    {
    __clockboard_set(led_cdev, led_val, CLOCK_CTRL_RLED);
    }
    static void __fhc_set(struct led_classdev *led_cdev,
    enum led_brightness led_val, u32 bit)
    {
    struct sunfire_led *p = to_sunfire_led(led_cdev);
    let mut reg: u32 = upa_readl(p.reg);
    switch (bit) {
    case FHC_CONTROL_LLED:
    if (led_val)
    reg &= ~bit;
    else
    reg |= bit;
    break;
    default:
    if (led_val)
    reg |= bit;
    else
    reg &= ~bit;
    break;
    }
    upa_writel(reg, p.reg);
    }
    static void fhc_left_set(struct led_classdev *led_cdev,
    enum led_brightness led_val)
    {
    __fhc_set(led_cdev, led_val, FHC_CONTROL_LLED);
    }
    static void fhc_middle_set(struct led_classdev *led_cdev,
    enum led_brightness led_val)
    {
    __fhc_set(led_cdev, led_val, FHC_CONTROL_MLED);
    }
    static void fhc_right_set(struct led_classdev *led_cdev,
    enum led_brightness led_val)
    {
    __fhc_set(led_cdev, led_val, FHC_CONTROL_RLED);
    }
    typedef void (*set_handler)(struct led_classdev *, enum led_brightness);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_type {
    pub name: *const c_char,
    pub handler: set_handler,
    pub default_trigger: *const c_char,
}

pub const NUM_LEDS_PER_BOARD: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunfire_drvdata {
    pub leds: [sunfire_led; NUM_LEDS_PER_BOARD],
}

    static int sunfire_led_generic_probe(struct platform_device *pdev,
    struct led_type *types)
    {
    struct sunfire_drvdata *p;
    int i, err;
    if (pdev.num_resources != 1) {
    dev_err(&pdev.dev, "Wrong number of resources %d, should be 1\n",
    pdev.num_resources);
    return -EINVAL;
    }
    p = devm_kzalloc(&pdev.dev, sizeof(*p), GFP_KERNEL);
    if (!p)
    return -ENOMEM;
    for (i = 0; i < NUM_LEDS_PER_BOARD; i++) {
    struct led_classdev *lp = &p.leds[i].led_cdev;
    p.leds[i].reg = (void __iomem *) pdev.resource[0].start;
    lp.name = types[i].name;
    lp.brightness = LED_FULL;
    lp.brightness_set = types[i].handler;
    lp.default_trigger = types[i].default_trigger;
    err = led_classdev_register(&pdev.dev, lp);
    if (err) {
    dev_err(&pdev.dev, "Could not register %s LED\n",
    lp.name);
    for (i--; i >= 0; i--)
    led_classdev_unregister(&p.leds[i].led_cdev);
    return err;
    }
    }
    platform_set_drvdata(pdev, p);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sunfire_led_generic_remove(pdev: *mut platform_device) {
    static void sunfire_led_generic_remove(struct platform_device *pdev)
    {
    struct sunfire_drvdata *p = platform_get_drvdata(pdev);
    int i;
    for (i = 0; i < NUM_LEDS_PER_BOARD; i++)
    led_classdev_unregister(&p.leds[i].led_cdev);
    }
    static struct led_type clockboard_led_types[NUM_LEDS_PER_BOARD] = {
    {
    .name		= "clockboard-left",
    .handler	= clockboard_left_set,
    },
    {
    .name		= "clockboard-middle",
    .handler	= clockboard_middle_set,
    },
    {
    .name		= "clockboard-right",
    .handler	= clockboard_right_set,
    .default_trigger = "heartbeat",
    },
    };
#[no_mangle]
unsafe extern "C" fn sunfire_clockboard_led_probe(pdev: *mut platform_device) -> c_int {
    static int sunfire_clockboard_led_probe(struct platform_device *pdev)
    {
    return sunfire_led_generic_probe(pdev, clockboard_led_types);
    }
    static struct led_type fhc_led_types[NUM_LEDS_PER_BOARD] = {
    {
    .name		= "fhc-left",
    .handler	= fhc_left_set,
    },
    {
    .name		= "fhc-middle",
    .handler	= fhc_middle_set,
    },
    {
    .name		= "fhc-right",
    .handler	= fhc_right_set,
    .default_trigger = "heartbeat",
    },
    };
#[no_mangle]
unsafe extern "C" fn sunfire_fhc_led_probe(pdev: *mut platform_device) -> c_int {
    static int sunfire_fhc_led_probe(struct platform_device *pdev)
    {
    return sunfire_led_generic_probe(pdev, fhc_led_types);
    }
    MODULE_ALIAS("platform:sunfire-clockboard-leds");
    MODULE_ALIAS("platform:sunfire-fhc-leds");
    static struct platform_driver sunfire_clockboard_led_driver = {
    .probe		= sunfire_clockboard_led_probe,
    .remove		= sunfire_led_generic_remove,
    .driver		= {
    .name	= "sunfire-clockboard-leds",
    },
    };
    static struct platform_driver sunfire_fhc_led_driver = {
    .probe		= sunfire_fhc_led_probe,
    .remove		= sunfire_led_generic_remove,
    .driver		= {
    .name	= "sunfire-fhc-leds",
    },
    };
    static struct platform_driver * const drivers[] = {
    &sunfire_clockboard_led_driver,
    &sunfire_fhc_led_driver,
    };
#[no_mangle]
unsafe extern "C" fn sunfire_leds_init() -> int __init {
    static int __init sunfire_leds_init(void)
    {
    return platform_register_drivers(drivers, ARRAY_SIZE(drivers));
    }
#[no_mangle]
unsafe extern "C" fn sunfire_leds_exit() -> void __exit {
    static void __exit sunfire_leds_exit(void)
    {
    platform_unregister_drivers(drivers, ARRAY_SIZE(drivers));
    }
    module_init(sunfire_leds_init);
    module_exit(sunfire_leds_exit);
