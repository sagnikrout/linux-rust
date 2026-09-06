//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/ep93xx_bl.c
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
// Driver for the Cirrus EP93xx lcd backlight
//
// Copyright (c) 2010 H Hartley Sweeten <hsweeten@visionengravers.com>
//
// This driver controls the pulse width modulated brightness control output,
// BRIGHT, on the Cirrus EP9307, EP9312, and EP9315 processors.
//

pub const EP93XX_MAX_COUNT: c_int = 255;
pub const EP93XX_MAX_BRIGHT: c_int = 255;
pub const EP93XX_DEF_BRIGHT: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep93xxbl {
    pub mmio: *mut void __iomem,
    pub brightness: c_int,
}

#[no_mangle]
unsafe extern "C" fn ep93xxbl_set(bl: *mut backlight_device, brightness: c_int) -> c_int {
    static int ep93xxbl_set(struct backlight_device *bl, int brightness)
    {
    struct ep93xxbl *ep93xxbl = bl_get_data(bl);
    writel((brightness << 8) | EP93XX_MAX_COUNT, ep93xxbl.mmio);
    ep93xxbl.brightness = brightness;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ep93xxbl_update_status(bl: *mut backlight_device) -> c_int {
    static int ep93xxbl_update_status(struct backlight_device *bl)
    {
    return ep93xxbl_set(bl, backlight_get_brightness(bl));
    }
#[no_mangle]
unsafe extern "C" fn ep93xxbl_get_brightness(bl: *mut backlight_device) -> c_int {
    static int ep93xxbl_get_brightness(struct backlight_device *bl)
    {
    struct ep93xxbl *ep93xxbl = bl_get_data(bl);
    return ep93xxbl.brightness;
    }
    static const struct backlight_ops ep93xxbl_ops = {
    .update_status	= ep93xxbl_update_status,
    .get_brightness	= ep93xxbl_get_brightness,
    };
#[no_mangle]
unsafe extern "C" fn ep93xxbl_probe(dev: *mut platform_device) -> c_int {
    static int ep93xxbl_probe(struct platform_device *dev)
    {
    struct ep93xxbl *ep93xxbl;
    struct backlight_device *bl;
    struct backlight_properties props;
    struct resource *res;
    ep93xxbl = devm_kzalloc(&dev.dev, sizeof(*ep93xxbl), GFP_KERNEL);
    if (!ep93xxbl)
    return -ENOMEM;
    res = platform_get_resource(dev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENXIO;
//
// FIXME - We don't do a request_mem_region here because we are
// sharing the register space with the framebuffer driver (see
// drivers/video/ep93xx-fb.c) and doing so will cause the second
// loaded driver to return -EBUSY.
//
// NOTE: No locking is required; the framebuffer does not touch
// this register.
//
    ep93xxbl.mmio = devm_ioremap(&dev.dev, res.start,
    resource_size(res));
    if (!ep93xxbl.mmio)
    return -ENXIO;
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = EP93XX_MAX_BRIGHT;
    bl = devm_backlight_device_register(&dev.dev, dev.name, &dev.dev,
    ep93xxbl, &ep93xxbl_ops, &props);
    if (IS_ERR(bl))
    return PTR_ERR(bl);
    bl.props.brightness = EP93XX_DEF_BRIGHT;
    platform_set_drvdata(dev, bl);
    ep93xxbl_update_status(bl);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ep93xxbl_suspend(dev: *mut device) -> c_int {
    static int ep93xxbl_suspend(struct device *dev)
    {
    struct backlight_device *bl = dev_get_drvdata(dev);
    return ep93xxbl_set(bl, 0);
    }
#[no_mangle]
unsafe extern "C" fn ep93xxbl_resume(dev: *mut device) -> c_int {
    static int ep93xxbl_resume(struct device *dev)
    {
    struct backlight_device *bl = dev_get_drvdata(dev);
    backlight_update_status(bl);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(ep93xxbl_pm_ops, ep93xxbl_suspend, ep93xxbl_resume);
    static struct platform_driver ep93xxbl_driver = {
    .driver		= {
    .name	= "ep93xx-bl",
    .pm	= &ep93xxbl_pm_ops,
    },
    .probe		= ep93xxbl_probe,
    };
    module_platform_driver(ep93xxbl_driver);
    MODULE_DESCRIPTION("EP93xx Backlight Driver");
    MODULE_AUTHOR("H Hartley Sweeten <hsweeten@visionengravers.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:ep93xx-bl");
