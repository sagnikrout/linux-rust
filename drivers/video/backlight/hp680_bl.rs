//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/hp680_bl.c
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
// Backlight Driver for HP Jornada 680
//
// Copyright (c) 2005 Andriy Skulysh
//
// Based on Sharp's Corgi Backlight Driver
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//

pub const HP680_MAX_INTENSITY: c_int = 255;
pub const HP680_DEFAULT_INTENSITY: c_int = 10;
    static int hp680bl_suspended;
    static int current_intensity;
    static DEFINE_SPINLOCK(bl_lock);
#[no_mangle]
unsafe extern "C" fn hp680bl_send_intensity(bd: *mut backlight_device) {
    static void hp680bl_send_intensity(struct backlight_device *bd)
    {
    unsigned long flags;
    u16 v;
    let mut intensity: c_int = backlight_get_brightness(bd);
    if (hp680bl_suspended)
    intensity = 0;
    spin_lock_irqsave(&bl_lock, flags);
    if (intensity && current_intensity == 0) {
    sh_dac_enable(DAC_LCD_BRIGHTNESS);
    v = inw(HD64461_GPBDR);
    v &= ~HD64461_GPBDR_LCDOFF;
    outw(v, HD64461_GPBDR);
    sh_dac_output(255-(u8)intensity, DAC_LCD_BRIGHTNESS);
    } else if (intensity == 0 && current_intensity != 0) {
    sh_dac_output(255-(u8)intensity, DAC_LCD_BRIGHTNESS);
    sh_dac_disable(DAC_LCD_BRIGHTNESS);
    v = inw(HD64461_GPBDR);
    v |= HD64461_GPBDR_LCDOFF;
    outw(v, HD64461_GPBDR);
    } else if (intensity) {
    sh_dac_output(255-(u8)intensity, DAC_LCD_BRIGHTNESS);
    }
    spin_unlock_irqrestore(&bl_lock, flags);
    current_intensity = intensity;
    }

#[no_mangle]
unsafe extern "C" fn hp680bl_suspend(dev: *mut device) -> c_int {
    static int hp680bl_suspend(struct device *dev)
    {
    struct backlight_device *bd = dev_get_drvdata(dev);
    hp680bl_suspended = 1;
    hp680bl_send_intensity(bd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hp680bl_resume(dev: *mut device) -> c_int {
    static int hp680bl_resume(struct device *dev)
    {
    struct backlight_device *bd = dev_get_drvdata(dev);
    hp680bl_suspended = 0;
    hp680bl_send_intensity(bd);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(hp680bl_pm_ops, hp680bl_suspend, hp680bl_resume);
#[no_mangle]
unsafe extern "C" fn hp680bl_set_intensity(bd: *mut backlight_device) -> c_int {
    static int hp680bl_set_intensity(struct backlight_device *bd)
    {
    hp680bl_send_intensity(bd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hp680bl_get_intensity(bd: *mut backlight_device) -> c_int {
    static int hp680bl_get_intensity(struct backlight_device *bd)
    {
    return current_intensity;
    }
    static const struct backlight_ops hp680bl_ops = {
    .get_brightness = hp680bl_get_intensity,
    .update_status  = hp680bl_set_intensity,
    };
#[no_mangle]
unsafe extern "C" fn hp680bl_probe(pdev: *mut platform_device) -> c_int {
    static int hp680bl_probe(struct platform_device *pdev)
    {
    struct backlight_properties props;
    struct backlight_device *bd;
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = HP680_MAX_INTENSITY;
    bd = devm_backlight_device_register(&pdev.dev, "hp680-bl", &pdev.dev,
    core::ptr::null_mut(), &hp680bl_ops, &props);
    if (IS_ERR(bd))
    return PTR_ERR(bd);
    platform_set_drvdata(pdev, bd);
    bd.props.brightness = HP680_DEFAULT_INTENSITY;
    hp680bl_send_intensity(bd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hp680bl_remove(pdev: *mut platform_device) {
    static void hp680bl_remove(struct platform_device *pdev)
    {
    struct backlight_device *bd = platform_get_drvdata(pdev);
    bd.props.brightness = 0;
    bd.props.power = 0;
    hp680bl_send_intensity(bd);
    }
    static struct platform_driver hp680bl_driver = {
    .probe		= hp680bl_probe,
    .remove		= hp680bl_remove,
    .driver		= {
    .name	= "hp680-bl",
    .pm	= &hp680bl_pm_ops,
    },
    };
    static struct platform_device *hp680bl_device;
#[no_mangle]
unsafe extern "C" fn hp680bl_init() -> int __init {
    static int __init hp680bl_init(void)
    {
    int ret;
    ret = platform_driver_register(&hp680bl_driver);
    if (ret)
    return ret;
    hp680bl_device = platform_device_register_simple("hp680-bl", -1,
    core::ptr::null_mut(), 0);
    if (IS_ERR(hp680bl_device)) {
    platform_driver_unregister(&hp680bl_driver);
    return PTR_ERR(hp680bl_device);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hp680bl_exit() -> void __exit {
    static void __exit hp680bl_exit(void)
    {
    platform_device_unregister(hp680bl_device);
    platform_driver_unregister(&hp680bl_driver);
    }
    module_init(hp680bl_init);
    module_exit(hp680bl_exit);
    MODULE_AUTHOR("Andriy Skulysh <askulysh@gmail.com>");
    MODULE_DESCRIPTION("HP Jornada 680 Backlight Driver");
    MODULE_LICENSE("GPL");
