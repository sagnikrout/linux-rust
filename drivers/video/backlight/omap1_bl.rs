//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/omap1_bl.c
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
// Backlight driver for OMAP based boards.
//
// Copyright (c) 2006 Andrzej Zaborowski  <balrog@zabor.org>
//

pub const OMAPBL_MAX_INTENSITY: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_backlight {
    pub enabled: bool,
    pub current_intensity: c_int,
    pub dev: *mut device,
    pub pdata: *mut omap_backlight_config,
}

#[no_mangle]
pub unsafe extern "C" fn omapbl_send_intensity(intensity: c_int) {
    static inline void omapbl_send_intensity(int intensity)
    {
    omap_writeb(intensity, OMAP_PWL_ENABLE);
    }
#[no_mangle]
pub unsafe extern "C" fn omapbl_send_enable(enable: c_int) {
    static inline void omapbl_send_enable(int enable)
    {
    omap_writeb(enable, OMAP_PWL_CLK_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn omapbl_enable(bl: *mut omap_backlight, enable: bool) {
    static void omapbl_enable(struct omap_backlight *bl, bool enable)
    {
    if (enable) {
    omapbl_send_intensity(bl.current_intensity);
    omapbl_send_enable(1);
    } else {
    omapbl_send_intensity(0);
    omapbl_send_enable(0);
    }
    }

#[no_mangle]
unsafe extern "C" fn omapbl_suspend(dev: *mut device) -> c_int {
    static int omapbl_suspend(struct device *dev)
    {
    struct backlight_device *bl_dev = dev_get_drvdata(dev);
    struct omap_backlight *bl = bl_get_data(bl_dev);
    omapbl_enable(bl, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omapbl_resume(dev: *mut device) -> c_int {
    static int omapbl_resume(struct device *dev)
    {
    struct backlight_device *bl_dev = dev_get_drvdata(dev);
    struct omap_backlight *bl = bl_get_data(bl_dev);
    omapbl_enable(bl, bl.enabled);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn omapbl_set_enabled(dev: *mut backlight_device, enable: bool) {
    static void omapbl_set_enabled(struct backlight_device *dev, bool enable)
    {
    struct omap_backlight *bl = bl_get_data(dev);
    omapbl_enable(bl, enable);
    bl.enabled = enable;
    }
#[no_mangle]
unsafe extern "C" fn omapbl_update_status(dev: *mut backlight_device) -> c_int {
    static int omapbl_update_status(struct backlight_device *dev)
    {
    struct omap_backlight *bl = bl_get_data(dev);
    bool enable;
    if (bl.current_intensity != dev.props.brightness) {
    if (bl.enabled)
    omapbl_send_intensity(dev.props.brightness);
    bl.current_intensity = dev.props.brightness;
    }
    enable = !backlight_is_blank(dev);
    if (enable != bl.enabled)
    omapbl_set_enabled(dev, enable);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omapbl_get_intensity(dev: *mut backlight_device) -> c_int {
    static int omapbl_get_intensity(struct backlight_device *dev)
    {
    struct omap_backlight *bl = bl_get_data(dev);
    return bl.current_intensity;
    }
    static const struct backlight_ops omapbl_ops = {
    .get_brightness = omapbl_get_intensity,
    .update_status  = omapbl_update_status,
    };
#[no_mangle]
unsafe extern "C" fn omapbl_probe(pdev: *mut platform_device) -> c_int {
    static int omapbl_probe(struct platform_device *pdev)
    {
    struct backlight_properties props;
    struct backlight_device *dev;
    struct omap_backlight *bl;
    struct omap_backlight_config *pdata = dev_get_platdata(&pdev.dev);
    if (!pdata)
    return -ENXIO;
    bl = devm_kzalloc(&pdev.dev, sizeof(struct omap_backlight),
    GFP_KERNEL);
    if (unlikely(!bl))
    return -ENOMEM;
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = OMAPBL_MAX_INTENSITY;
    dev = devm_backlight_device_register(&pdev.dev, "omap-bl", &pdev.dev,
    bl, &omapbl_ops, &props);
    if (IS_ERR(dev))
    return PTR_ERR(dev);
    bl.enabled = false;
    bl.current_intensity = 0;
    bl.pdata = pdata;
    bl.dev = &pdev.dev;
    platform_set_drvdata(pdev, dev);
    omap_cfg_reg(PWL);	/* Conflicts with UART3 */
    dev.props.brightness = pdata.default_intensity;
    omapbl_update_status(dev);
    dev_info(&pdev.dev, "OMAP LCD backlight initialised\n");
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(omapbl_pm_ops, omapbl_suspend, omapbl_resume);
    static struct platform_driver omapbl_driver = {
    .probe		= omapbl_probe,
    .driver		= {
    .name	= "omap-bl",
    .pm	= &omapbl_pm_ops,
    },
    };
    module_platform_driver(omapbl_driver);
    MODULE_AUTHOR("Andrzej Zaborowski <balrog@zabor.org>");
    MODULE_DESCRIPTION("OMAP LCD Backlight driver");
    MODULE_LICENSE("GPL");
