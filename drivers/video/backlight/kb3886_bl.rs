//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/kb3886_bl.c
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
// Backlight Driver for the KB3886 Backlight
//
// Copyright (c) 2007-2008 Claudio Nieder
//
// Based on corgi_bl.c by Richard Purdie and kb3886 driver by Robert Woerle
//

pub const KB3886_PARENT: c_uint = 0x64;
pub const KB3886_IO: c_uint = 0x60;
pub const KB3886_ADC_DAC_PWM: c_uint = 0xC4;
pub const KB3886_PWM0_WRITE: c_uint = 0x81;
pub const KB3886_PWM0_READ: c_uint = 0x41;
    static DEFINE_MUTEX(bl_mutex);
#[no_mangle]
unsafe extern "C" fn kb3886_bl_set_intensity(intensity: c_int) {
    static void kb3886_bl_set_intensity(int intensity)
    {
    mutex_lock(&bl_mutex);
    intensity = intensity&0xff;
    outb(KB3886_ADC_DAC_PWM, KB3886_PARENT);
    usleep_range(10000, 11000);
    outb(KB3886_PWM0_WRITE, KB3886_IO);
    usleep_range(10000, 11000);
    outb(intensity, KB3886_IO);
    mutex_unlock(&bl_mutex);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kb3886bl_machinfo {
    pub max_intensity: c_int,
    pub default_intensity: c_int,
    pub limit_mask: c_int,
    pub intensity): *mut *mut void (set_bl_intensity)(int,
}

    static struct kb3886bl_machinfo kb3886_bl_machinfo = {
    .max_intensity = 0xff,
    .default_intensity = 0xa0,
    .limit_mask = 0x7f,
    .set_bl_intensity = kb3886_bl_set_intensity,
    };
    static struct platform_device kb3886bl_device = {
    .name		= "kb3886-bl",
    .dev		= {
    .platform_data	= &kb3886_bl_machinfo,
    },
    .id		= -1,
    };
    static struct platform_device *devices[] __initdata = {
    &kb3886bl_device,
    };
//
// Back to driver
//
    static int kb3886bl_intensity;
    static struct backlight_device *kb3886_backlight_device;
    static struct kb3886bl_machinfo *bl_machinfo;
    static unsigned long kb3886bl_flags;
pub const KB3886BL_SUSPENDED: c_uint = 0x01;
    static const struct dmi_system_id kb3886bl_device_table[] __initconst = {
    {
    .ident = "Sahara Touch-iT",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "SDV"),
    DMI_MATCH(DMI_PRODUCT_NAME, "iTouch T201"),
    },
    },
    { }
    };
#[no_mangle]
unsafe extern "C" fn kb3886bl_send_intensity(bd: *mut backlight_device) -> c_int {
    static int kb3886bl_send_intensity(struct backlight_device *bd)
    {
    let mut intensity: c_int = backlight_get_brightness(bd);
    if (kb3886bl_flags & KB3886BL_SUSPENDED)
    intensity = 0;
    bl_machinfo.set_bl_intensity(intensity);
    kb3886bl_intensity = intensity;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn kb3886bl_suspend(dev: *mut device) -> c_int {
    static int kb3886bl_suspend(struct device *dev)
    {
    struct backlight_device *bd = dev_get_drvdata(dev);
    kb3886bl_flags |= KB3886BL_SUSPENDED;
    backlight_update_status(bd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kb3886bl_resume(dev: *mut device) -> c_int {
    static int kb3886bl_resume(struct device *dev)
    {
    struct backlight_device *bd = dev_get_drvdata(dev);
    kb3886bl_flags &= ~KB3886BL_SUSPENDED;
    backlight_update_status(bd);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(kb3886bl_pm_ops, kb3886bl_suspend, kb3886bl_resume);
#[no_mangle]
unsafe extern "C" fn kb3886bl_get_intensity(bd: *mut backlight_device) -> c_int {
    static int kb3886bl_get_intensity(struct backlight_device *bd)
    {
    return kb3886bl_intensity;
    }
    static const struct backlight_ops kb3886bl_ops = {
    .get_brightness = kb3886bl_get_intensity,
    .update_status  = kb3886bl_send_intensity,
    };
#[no_mangle]
unsafe extern "C" fn kb3886bl_probe(pdev: *mut platform_device) -> c_int {
    static int kb3886bl_probe(struct platform_device *pdev)
    {
    struct backlight_properties props;
    struct kb3886bl_machinfo *machinfo = dev_get_platdata(&pdev.dev);
    bl_machinfo = machinfo;
    if (!machinfo.limit_mask)
    machinfo.limit_mask = -1;
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = machinfo.max_intensity;
    kb3886_backlight_device = devm_backlight_device_register(&pdev.dev,
    "kb3886-bl", &pdev.dev,
    core::ptr::null_mut(), &kb3886bl_ops,
    &props);
    if (IS_ERR(kb3886_backlight_device))
    return PTR_ERR(kb3886_backlight_device);
    platform_set_drvdata(pdev, kb3886_backlight_device);
    kb3886_backlight_device.props.power = BACKLIGHT_POWER_ON;
    kb3886_backlight_device.props.brightness = machinfo.default_intensity;
    backlight_update_status(kb3886_backlight_device);
    return 0;
    }
    static struct platform_driver kb3886bl_driver = {
    .probe		= kb3886bl_probe,
    .driver		= {
    .name	= "kb3886-bl",
    .pm	= &kb3886bl_pm_ops,
    },
    };
#[no_mangle]
unsafe extern "C" fn kb3886_init() -> int __init {
    static int __init kb3886_init(void)
    {
    if (!dmi_check_system(kb3886bl_device_table))
    return -ENODEV;
    platform_add_devices(devices, ARRAY_SIZE(devices));
    return platform_driver_register(&kb3886bl_driver);
    }
#[no_mangle]
unsafe extern "C" fn kb3886_exit() -> void __exit {
    static void __exit kb3886_exit(void)
    {
    platform_driver_unregister(&kb3886bl_driver);
    }
    module_init(kb3886_init);
    module_exit(kb3886_exit);
    MODULE_AUTHOR("Claudio Nieder <private@claudio.ch>");
    MODULE_DESCRIPTION("Tabletkiosk Sahara Touch-iT Backlight Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("dmi:*:svnSDV:pniTouchT201:*");
