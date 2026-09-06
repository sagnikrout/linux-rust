//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/rave-sp-backlight.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// LCD Backlight driver for RAVE SP
//
// Copyright (C) 2018 Zodiac Inflight Innovations
//

#[no_mangle]
unsafe extern "C" fn rave_sp_backlight_update_status(bd: *mut backlight_device) -> c_int {
    static int rave_sp_backlight_update_status(struct backlight_device *bd)
    {
    const struct backlight_properties *p = &bd.props;
    const u8 intensity =
    (p.power == BACKLIGHT_POWER_ON) ? p.brightness : 0;
    struct rave_sp *sp = dev_get_drvdata(&bd.dev);
    u8 cmd[] = {
    [0] = RAVE_SP_CMD_SET_BACKLIGHT,
    [1] = 0,
    [2] = intensity ? RAVE_SP_BACKLIGHT_LCD_EN | intensity : 0,
    [3] = 0,
    [4] = 0,
    };
    return rave_sp_exec(sp, cmd, sizeof(cmd), core::ptr::null_mut(), 0);
    }
    static const struct backlight_ops rave_sp_backlight_ops = {
    .options	= BL_CORE_SUSPENDRESUME,
    .update_status	= rave_sp_backlight_update_status,
    };
    static struct backlight_properties rave_sp_backlight_props = {
    .type		= BACKLIGHT_PLATFORM,
    .max_brightness = 100,
    .brightness	= 50,
    };
#[no_mangle]
unsafe extern "C" fn rave_sp_backlight_probe(pdev: *mut platform_device) -> c_int {
    static int rave_sp_backlight_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct backlight_device *bd;
    bd = devm_backlight_device_register(dev, pdev.name, dev,
    dev_get_drvdata(dev.parent),
    &rave_sp_backlight_ops,
    &rave_sp_backlight_props);
    if (IS_ERR(bd))
    return PTR_ERR(bd);
//
// If there is a phandle pointing to the device node we can
// assume that another device will manage the status changes.
// If not we make sure the backlight is in a consistent state.
//
    if (!dev.of_node.phandle)
    backlight_update_status(bd);
    return 0;
    }
    static const struct of_device_id rave_sp_backlight_of_match[] = {
    { .compatible = "zii,rave-sp-backlight" },
    {}
    };
    static struct platform_driver rave_sp_backlight_driver = {
    .probe = rave_sp_backlight_probe,
    .driver	= {
    .name = KBUILD_MODNAME,
    .of_match_table = rave_sp_backlight_of_match,
    },
    };
    module_platform_driver(rave_sp_backlight_driver);
    MODULE_DEVICE_TABLE(of, rave_sp_backlight_of_match);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Andrey Vostrikov <andrey.vostrikov@cogentembedded.com>");
    MODULE_AUTHOR("Nikita Yushchenko <nikita.yoush@cogentembedded.com>");
    MODULE_AUTHOR("Andrey Smirnov <andrew.smirnov@gmail.com>");
    MODULE_DESCRIPTION("RAVE SP Backlight driver");
