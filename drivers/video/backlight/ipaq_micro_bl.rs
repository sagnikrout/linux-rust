//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/ipaq_micro_bl.c
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
// iPAQ microcontroller backlight support
// Author : Linus Walleij <linus.walleij@linaro.org>
//

#[no_mangle]
unsafe extern "C" fn micro_bl_update_status(bd: *mut backlight_device) -> c_int {
    static int micro_bl_update_status(struct backlight_device *bd)
    {
    struct ipaq_micro *micro = dev_get_drvdata(&bd.dev);
    let mut intensity: c_int = backlight_get_brightness(bd);
    struct ipaq_micro_msg msg = {
    .id = MSG_BACKLIGHT,
    .tx_len = 3,
    };
//
// Message format:
// Byte 0: backlight instance (usually 1)
// Byte 1: on/off
// Byte 2: intensity, 0-255
//
    msg.tx_data[0] = 0x01;
    msg.tx_data[1] = intensity > 0 ? 1 : 0;
    msg.tx_data[2] = intensity;
    return ipaq_micro_tx_msg_sync(micro, &msg);
    }
    static const struct backlight_ops micro_bl_ops = {
    .options = BL_CORE_SUSPENDRESUME,
    .update_status  = micro_bl_update_status,
    };
    static const struct backlight_properties micro_bl_props = {
    .type = BACKLIGHT_RAW,
    .max_brightness = 255,
    .power = BACKLIGHT_POWER_ON,
    .brightness = 64,
    };
#[no_mangle]
unsafe extern "C" fn micro_backlight_probe(pdev: *mut platform_device) -> c_int {
    static int micro_backlight_probe(struct platform_device *pdev)
    {
    struct backlight_device *bd;
    struct ipaq_micro *micro = dev_get_drvdata(pdev.dev.parent);
    bd = devm_backlight_device_register(&pdev.dev, "ipaq-micro-backlight",
    &pdev.dev, micro, &micro_bl_ops,
    &micro_bl_props);
    if (IS_ERR(bd))
    return PTR_ERR(bd);
    platform_set_drvdata(pdev, bd);
    backlight_update_status(bd);
    return 0;
    }
    static struct platform_driver micro_backlight_device_driver = {
    .driver = {
    .name    = "ipaq-micro-backlight",
    },
    .probe   = micro_backlight_probe,
    };
    module_platform_driver(micro_backlight_device_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("driver for iPAQ Atmel micro backlight");
    MODULE_ALIAS("platform:ipaq-micro-backlight");
