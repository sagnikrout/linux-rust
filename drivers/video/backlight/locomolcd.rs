//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/locomolcd.c
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
// Backlight control code for Sharp Zaurus SL-5500
//
// Copyright 2005 John Lenz <lenz@cs.wisc.edu>
// Maintainer: Pavel Machek <pavel@ucw.cz> (unless John wants to :-)
//
// This driver assumes single CPU. That's okay, because collie is
// slightly old hardware, and no one is going to retrofit second CPU to
// old PDA.
//
// LCD power functions

    static struct backlight_device *locomolcd_bl_device;
    static struct locomo_dev *locomolcd_dev;
    static unsigned long locomolcd_flags;
pub const LOCOMOLCD_SUSPENDED: c_uint = 0x01;
#[no_mangle]
unsafe extern "C" fn locomolcd_on(comadj: c_int) {
    static void locomolcd_on(int comadj)
    {
    locomo_gpio_set_dir(locomolcd_dev.dev.parent, LOCOMO_GPIO_LCD_VSHA_ON, 0);
    locomo_gpio_write(locomolcd_dev.dev.parent, LOCOMO_GPIO_LCD_VSHA_ON, 1);
    mdelay(2);
    locomo_gpio_set_dir(locomolcd_dev.dev.parent, LOCOMO_GPIO_LCD_VSHD_ON, 0);
    locomo_gpio_write(locomolcd_dev.dev.parent, LOCOMO_GPIO_LCD_VSHD_ON, 1);
    mdelay(2);
    locomo_m62332_senddata(locomolcd_dev, comadj, 0);
    mdelay(5);
    locomo_gpio_set_dir(locomolcd_dev.dev.parent, LOCOMO_GPIO_LCD_VEE_ON, 0);
    locomo_gpio_write(locomolcd_dev.dev.parent, LOCOMO_GPIO_LCD_VEE_ON, 1);
    mdelay(10);
// TFTCRST | CPSOUT=0 | CPSEN
    locomo_writel(0x01, locomolcd_dev.mapbase + LOCOMO_TC);
// Set CPSD
    locomo_writel(6, locomolcd_dev.mapbase + LOCOMO_CPSD);
// TFTCRST | CPSOUT=0 | CPSEN
    locomo_writel((0x04 | 0x01), locomolcd_dev.mapbase + LOCOMO_TC);
    mdelay(10);
    locomo_gpio_set_dir(locomolcd_dev.dev.parent, LOCOMO_GPIO_LCD_MOD, 0);
    locomo_gpio_write(locomolcd_dev.dev.parent, LOCOMO_GPIO_LCD_MOD, 1);
    }
#[no_mangle]
unsafe extern "C" fn locomolcd_off(comadj: c_int) {
    static void locomolcd_off(int comadj)
    {
// TFTCRST=1 | CPSOUT=1 | CPSEN = 0
    locomo_writel(0x06, locomolcd_dev.mapbase + LOCOMO_TC);
    mdelay(1);
    locomo_gpio_write(locomolcd_dev.dev.parent, LOCOMO_GPIO_LCD_VSHA_ON, 0);
    mdelay(110);
    locomo_gpio_write(locomolcd_dev.dev.parent, LOCOMO_GPIO_LCD_VEE_ON, 0);
    mdelay(700);
// TFTCRST=0 | CPSOUT=0 | CPSEN = 0
    locomo_writel(0, locomolcd_dev.mapbase + LOCOMO_TC);
    locomo_gpio_write(locomolcd_dev.dev.parent, LOCOMO_GPIO_LCD_MOD, 0);
    locomo_gpio_write(locomolcd_dev.dev.parent, LOCOMO_GPIO_LCD_VSHD_ON, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn locomolcd_power(on: c_int) {
    void locomolcd_power(int on)
    {
    let mut comadj: c_int = sharpsl_param.comadj;
    unsigned long flags;
    local_irq_save(flags);
    if (!locomolcd_dev) {
    local_irq_restore(flags);
    return;
    }
// read comadj
    if (comadj == -1 && machine_is_collie())
    comadj = 128;
    if (on)
    locomolcd_on(comadj);
    else
    locomolcd_off(comadj);
    local_irq_restore(flags);
    }
    EXPORT_SYMBOL(locomolcd_power);
    static int current_intensity;
#[no_mangle]
unsafe extern "C" fn locomolcd_set_intensity(bd: *mut backlight_device) -> c_int {
    static int locomolcd_set_intensity(struct backlight_device *bd)
    {
    let mut intensity: c_int = backlight_get_brightness(bd);
    if (locomolcd_flags & LOCOMOLCD_SUSPENDED)
    intensity = 0;
    switch (intensity) {
//
// AC and non-AC are handled differently,
// but produce same results in sharp code?
//
    case 0:
    locomo_frontlight_set(locomolcd_dev, 0, 0, 161);
    break;
    case 1:
    locomo_frontlight_set(locomolcd_dev, 117, 0, 161);
    break;
    case 2:
    locomo_frontlight_set(locomolcd_dev, 163, 0, 148);
    break;
    case 3:
    locomo_frontlight_set(locomolcd_dev, 194, 0, 161);
    break;
    case 4:
    locomo_frontlight_set(locomolcd_dev, 194, 1, 161);
    break;
    default:
    return -ENODEV;
    }
    current_intensity = intensity;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn locomolcd_get_intensity(bd: *mut backlight_device) -> c_int {
    static int locomolcd_get_intensity(struct backlight_device *bd)
    {
    return current_intensity;
    }
    static const struct backlight_ops locomobl_data = {
    .get_brightness = locomolcd_get_intensity,
    .update_status  = locomolcd_set_intensity,
    };

#[no_mangle]
unsafe extern "C" fn locomolcd_suspend(dev: *mut device) -> c_int {
    static int locomolcd_suspend(struct device *dev)
    {
    locomolcd_flags |= LOCOMOLCD_SUSPENDED;
    locomolcd_set_intensity(locomolcd_bl_device);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn locomolcd_resume(dev: *mut device) -> c_int {
    static int locomolcd_resume(struct device *dev)
    {
    locomolcd_flags &= ~LOCOMOLCD_SUSPENDED;
    locomolcd_set_intensity(locomolcd_bl_device);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(locomolcd_pm_ops, locomolcd_suspend, locomolcd_resume);
#[no_mangle]
unsafe extern "C" fn locomolcd_probe(ldev: *mut locomo_dev) -> c_int {
    static int locomolcd_probe(struct locomo_dev *ldev)
    {
    struct backlight_properties props;
    unsigned long flags;
    local_irq_save(flags);
    locomolcd_dev = ldev;
    locomo_gpio_set_dir(ldev.dev.parent, LOCOMO_GPIO_FL_VR, 0);
    local_irq_restore(flags);
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = 4;
    locomolcd_bl_device = backlight_device_register("locomo-bl",
    &ldev.dev, core::ptr::null_mut(),
    &locomobl_data, &props);
    if (IS_ERR(locomolcd_bl_device))
    return PTR_ERR(locomolcd_bl_device);
// Set up frontlight so that screen is readable
    locomolcd_bl_device.props.brightness = 2;
    locomolcd_set_intensity(locomolcd_bl_device);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn locomolcd_remove(dev: *mut locomo_dev) {
    static void locomolcd_remove(struct locomo_dev *dev)
    {
    unsigned long flags;
    locomolcd_bl_device.props.brightness = 0;
    locomolcd_bl_device.props.power = 0;
    locomolcd_set_intensity(locomolcd_bl_device);
    backlight_device_unregister(locomolcd_bl_device);
    local_irq_save(flags);
    locomolcd_dev = core::ptr::null_mut();
    local_irq_restore(flags);
    }
    static struct locomo_driver poodle_lcd_driver = {
    .drv = {
    .name	= "locomo-backlight",
    .pm	= &locomolcd_pm_ops,
    },
    .devid	= LOCOMO_DEVID_BACKLIGHT,
    .probe	= locomolcd_probe,
    .remove	= locomolcd_remove,
    };
#[no_mangle]
unsafe extern "C" fn locomolcd_init() -> int __init {
    static int __init locomolcd_init(void)
    {
    return locomo_driver_register(&poodle_lcd_driver);
    }
#[no_mangle]
unsafe extern "C" fn locomolcd_exit() -> void __exit {
    static void __exit locomolcd_exit(void)
    {
    locomo_driver_unregister(&poodle_lcd_driver);
    }
    module_init(locomolcd_init);
    module_exit(locomolcd_exit);
    MODULE_AUTHOR("John Lenz <lenz@cs.wisc.edu>, Pavel Machek <pavel@ucw.cz>");
    MODULE_DESCRIPTION("Collie LCD driver");
    MODULE_LICENSE("GPL");
