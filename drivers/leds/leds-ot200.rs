//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-ot200.c
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


//
// Bachmann ot200 leds driver.
//
// Author: Sebastian Andrzej Siewior <bigeasy@linutronix.de>
// Christian Gmeiner <christian.gmeiner@gmail.com>
//
// License: GPL as published by the FSF.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ot200_led {
    pub cdev: led_classdev,
    pub name: *const c_char,
    pub port: c_ulong,
    pub mask: u8,
}

//
// The device has three leds on the back panel (led_err, led_init and led_run)
// and can handle up to seven leds on the front panel.
//
    static struct ot200_led leds[] = {
    {
    .name = "led_run",
    .port = 0x5a,
    .mask = BIT(0),
    },
    {
    .name = "led_init",
    .port = 0x5a,
    .mask = BIT(1),
    },
    {
    .name = "led_err",
    .port = 0x5a,
    .mask = BIT(2),
    },
    {
    .name = "led_1",
    .port = 0x49,
    .mask = BIT(6),
    },
    {
    .name = "led_2",
    .port = 0x49,
    .mask = BIT(5),
    },
    {
    .name = "led_3",
    .port = 0x49,
    .mask = BIT(4),
    },
    {
    .name = "led_4",
    .port = 0x49,
    .mask = BIT(3),
    },
    {
    .name = "led_5",
    .port = 0x49,
    .mask = BIT(2),
    },
    {
    .name = "led_6",
    .port = 0x49,
    .mask = BIT(1),
    },
    {
    .name = "led_7",
    .port = 0x49,
    .mask = BIT(0),
    }
    };
    static DEFINE_SPINLOCK(value_lock);
//
// we need to store the current led states, as it is not
// possible to read the current led state via inb().
//
    static u8 leds_back;
    static u8 leds_front;
    static void ot200_led_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct ot200_led *led = container_of(led_cdev, struct ot200_led, cdev);
    u8 *val;
    unsigned long flags;
    spin_lock_irqsave(&value_lock, flags);
    if (led.port == 0x49)
    val = &leds_front;
#[no_mangle]
pub unsafe extern "C" fn if(0x5a: led->port ==) -> else {
    else if (led.port == 0x5a)
    val = &leds_back;
    else
    BUG();
    if (value == LED_OFF)
// val &= ~led->mask;
    else
// val |= led->mask;
    outb(*val, led.port);
    spin_unlock_irqrestore(&value_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ot200_led_probe(pdev: *mut platform_device) -> c_int {
    static int ot200_led_probe(struct platform_device *pdev)
    {
    int i;
    int ret;
    for (i = 0; i < ARRAY_SIZE(leds); i++) {
    leds[i].cdev.name = leds[i].name;
    leds[i].cdev.brightness_set = ot200_led_brightness_set;
    ret = devm_led_classdev_register(&pdev.dev, &leds[i].cdev);
    if (ret < 0)
    return ret;
    }
    leds_front = 0;		/* turn off all front leds */
    leds_back = BIT(1);	/* turn on init led */
    outb(leds_front, 0x49);
    outb(leds_back, 0x5a);
    return 0;
    }
    static struct platform_driver ot200_led_driver = {
    .probe		= ot200_led_probe,
    .driver		= {
    .name	= "leds-ot200",
    },
    };
    module_platform_driver(ot200_led_driver);
    MODULE_AUTHOR("Sebastian A. Siewior <bigeasy@linutronix.de>");
    MODULE_DESCRIPTION("ot200 LED driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:leds-ot200");
