//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-ipaq-micro.c
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
// h3xxx atmel micro companion support, notification LED subdevice
//
// Author : Linus Walleij <linus.walleij@linaro.org>
//

pub const LED_YELLOW: c_uint = 0x00;
pub const LED_GREEN: c_uint = 0x01;

    static int micro_leds_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct ipaq_micro *micro = dev_get_drvdata(led_cdev.dev.parent.parent);
//
// In this message:
// Byte 0 = LED color: 0 = yellow, 1 = green
// yellow LED is always ~30 blinks per minute
// Byte 1 = duration (flags?) appears to be ignored
// Byte 2 = green ontime in 1/10 sec (deciseconds)
// 1 = 1/10 second
// 0 = 256/10 second
// Byte 3 = green offtime in 1/10 sec (deciseconds)
// 1 = 1/10 second
// 0 = 256/10 seconds
//
    struct ipaq_micro_msg msg = {
    .id = MSG_NOTIFY_LED,
    .tx_len = 4,
    };
    msg.tx_data[0] = LED_GREEN;
    msg.tx_data[1] = 0;
    if (value) {
    msg.tx_data[2] = 0; /* Duty cycle 256 */
    msg.tx_data[3] = 1;
    } else {
    msg.tx_data[2] = 1;
    msg.tx_data[3] = 0; /* Duty cycle 256 */
    }
    return ipaq_micro_tx_msg_sync(micro, &msg);
    }
// Maximum duty cycle in ms 256/10 sec = 25600 ms
pub const IPAQ_LED_MAX_DUTY: c_int = 25600;
    static int micro_leds_blink_set(struct led_classdev *led_cdev,
    unsigned long *delay_on,
    unsigned long *delay_off)
    {
    struct ipaq_micro *micro = dev_get_drvdata(led_cdev.dev.parent.parent);
//
// In this message:
// Byte 0 = LED color: 0 = yellow, 1 = green
// yellow LED is always ~30 blinks per minute
// Byte 1 = duration (flags?) appears to be ignored
// Byte 2 = green ontime in 1/10 sec (deciseconds)
// 1 = 1/10 second
// 0 = 256/10 second
// Byte 3 = green offtime in 1/10 sec (deciseconds)
// 1 = 1/10 second
// 0 = 256/10 seconds
//
    struct ipaq_micro_msg msg = {
    .id = MSG_NOTIFY_LED,
    .tx_len = 4,
    };
    msg.tx_data[0] = LED_GREEN;
    if (*delay_on > IPAQ_LED_MAX_DUTY ||
// delay_off > IPAQ_LED_MAX_DUTY)
    return -EINVAL;
    if (*delay_on == 0 && *delay_off == 0) {
// delay_on = 100;
// delay_off = 100;
    }
    msg.tx_data[1] = 0;
    if (*delay_on >= IPAQ_LED_MAX_DUTY)
    msg.tx_data[2] = 0;
    else
    msg.tx_data[2] = (u8) DIV_ROUND_CLOSEST(*delay_on, 100);
    if (*delay_off >= IPAQ_LED_MAX_DUTY)
    msg.tx_data[3] = 0;
    else
    msg.tx_data[3] = (u8) DIV_ROUND_CLOSEST(*delay_off, 100);
    return ipaq_micro_tx_msg_sync(micro, &msg);
    }
    static struct led_classdev micro_led = {
    .name			= "led-ipaq-micro",
    .brightness_set_blocking = micro_leds_brightness_set,
    .blink_set		= micro_leds_blink_set,
    .flags			= LED_CORE_SUSPENDRESUME,
    };
#[no_mangle]
unsafe extern "C" fn micro_leds_probe(pdev: *mut platform_device) -> c_int {
    static int micro_leds_probe(struct platform_device *pdev)
    {
    int ret;
    ret = devm_led_classdev_register(&pdev.dev, &micro_led);
    if (ret) {
    dev_err(&pdev.dev, "registering led failed: %d\n", ret);
    return ret;
    }
    dev_info(&pdev.dev, "iPAQ micro notification LED driver\n");
    return 0;
    }
    static struct platform_driver micro_leds_device_driver = {
    .driver = {
    .name    = "ipaq-micro-leds",
    },
    .probe   = micro_leds_probe,
    };
    module_platform_driver(micro_leds_device_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("driver for iPAQ Atmel micro leds");
    MODULE_ALIAS("platform:ipaq-micro-leds");
