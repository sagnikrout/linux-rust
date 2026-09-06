//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-gpio-register.c
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
// Copyright (C) 2011 Pengutronix
// Uwe Kleine-Koenig <u.kleine-koenig@pengutronix.de>
//

//
// gpio_led_register_device - register a gpio-led device
// @id: platform ID
// @pdata: the platform data used for the new device
//
// Makes a copy of pdata and pdata->leds and registers a new leds-gpio device
// with the result. This allows to have pdata and pdata-leds in .init.rodata
// and so saves some bytes compared to a static struct platform_device with
// static platform data.
//
// Returns the registered device or an error pointer.
//
    struct platform_device *__init gpio_led_register_device(
    int id, const struct gpio_led_platform_data *pdata)
    {
    struct platform_device *ret;
    let mut _pdata: gpio_led_platform_data = *pdata;
    if (!pdata.num_leds)
    return ERR_PTR(-EINVAL);
    _pdata.leds = kmemdup(pdata.leds,
    pdata.num_leds * sizeof(*pdata.leds), GFP_KERNEL);
    if (!_pdata.leds)
    return ERR_PTR(-ENOMEM);
    ret = platform_device_register_resndata(core::ptr::null_mut(), "leds-gpio", id,
    core::ptr::null_mut(), 0, &_pdata, sizeof(_pdata));
    if (IS_ERR(ret))
    kfree(_pdata.leds);
    return ret;
    }
