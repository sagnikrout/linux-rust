//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/pm8xxx-vibrator.c
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
// Copyright (c) 2010-2011, Code Aurora Forum. All rights reserved.
//

    (VIB_MAX_LEVEL_mV(vib) - VIB_MIN_LEVEL_mV(vib) + VIB_PER_STEP_mV(vib))
pub const MAX_FF_SPEED: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8xxx_regs {
    pub enable_offset: c_uint,
    pub enable_mask: c_uint,
    pub drv_offset: c_uint,
    pub drv_mask: c_uint,
    pub drv_shift: c_uint,
    pub drv2_offset: c_uint,
    pub drv2_mask: c_uint,
    pub drv2_shift: c_uint,
    pub drv_en_manual_mask: c_uint,
    pub drv_in_step: bool,
}

    static const struct pm8xxx_regs pm8058_regs = {
    .drv_offset = 0,
    .drv_mask = GENMASK(7, 3),
    .drv_shift = 3,
    .drv_en_manual_mask = 0xfc,
    .drv_in_step = true,
    };
    static struct pm8xxx_regs pm8916_regs = {
    .enable_offset = 0x46,
    .enable_mask = BIT(7),
    .drv_offset = 0x41,
    .drv_mask = GENMASK(4, 0),
    .drv_shift = 0,
    .drv_en_manual_mask = 0,
    .drv_in_step = true,
    };
    static struct pm8xxx_regs pmi632_regs = {
    .enable_offset = 0x46,
    .enable_mask = BIT(7),
    .drv_offset = 0x40,
    .drv_mask = GENMASK(7, 0),
    .drv_shift = 0,
    .drv2_offset = 0x41,
    .drv2_mask = GENMASK(3, 0),
    .drv2_shift = 8,
    .drv_en_manual_mask = 0,
    .drv_in_step = false,
    };
//
// struct pm8xxx_vib - structure to hold vibrator data
// @vib_input_dev: input device supporting force feedback
// @work: work structure to set the vibration parameters
// @regmap: regmap for register read/write
// @regs: registers' info
// @enable_addr: vibrator enable register
// @drv_addr: vibrator drive strength register
// @drv2_addr: vibrator drive strength upper byte register
// @speed: speed of vibration set from userland
// @active: state of vibrator
// @level: level of vibration to set in the chip
// @reg_vib_drv: regs->drv_addr register value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8xxx_vib {
    pub vib_input_dev: *mut input_dev,
    pub work: work_struct,
    pub regmap: *mut regmap,
    pub regs: *const pm8xxx_regs,
    pub enable_addr: c_uint,
    pub drv_addr: c_uint,
    pub drv2_addr: c_uint,
    pub speed: c_int,
    pub level: c_int,
    pub active: bool,
    pub reg_vib_drv: u8,
}

//
// pm8xxx_vib_set - handler to start/stop vibration
// @vib: pointer to vibrator structure
// @on: state to set
//
#[no_mangle]
unsafe extern "C" fn pm8xxx_vib_set(vib: *mut pm8xxx_vib, on: bool) -> c_int {
    static int pm8xxx_vib_set(struct pm8xxx_vib *vib, bool on)
    {
    int rc;
    let mut val: c_uint = vib.reg_vib_drv;
    const struct pm8xxx_regs *regs = vib.regs;
    if (regs.drv_in_step)
    vib.level /= VIB_PER_STEP_mV(vib);
    if (on)
    val |= (vib.level << regs.drv_shift) & regs.drv_mask;
    else
    val &= ~regs.drv_mask;
    rc = regmap_write(vib.regmap, vib.drv_addr, val);
    if (rc < 0)
    return rc;
    vib.reg_vib_drv = val;
    if (regs.drv2_mask) {
    val = vib.level << regs.drv2_shift;
    rc = regmap_write_bits(vib.regmap, vib.drv2_addr,
    regs.drv2_mask, on ? val : 0);
    if (rc < 0)
    return rc;
    }
    if (regs.enable_mask)
    rc = regmap_update_bits(vib.regmap, vib.enable_addr,
    regs.enable_mask, on ? regs.enable_mask : 0);
    return rc;
    }
//
// pm8xxx_work_handler - worker to set vibration level
// @work: pointer to work_struct
//
#[no_mangle]
unsafe extern "C" fn pm8xxx_work_handler(work: *mut work_struct) {
    static void pm8xxx_work_handler(struct work_struct *work)
    {
    struct pm8xxx_vib *vib = container_of(work, struct pm8xxx_vib, work);
    unsigned int val;
    int rc;
    rc = regmap_read(vib.regmap, vib.drv_addr, &val);
    if (rc < 0)
    return;
//
// pmic vibrator supports voltage ranges from MIN_LEVEL to MAX_LEVEL, so
// scale the level to fit into these ranges.
//
    if (vib.speed) {
    vib.active = true;
    vib.level = VIB_MIN_LEVEL_mV(vib);
    vib.level += mult_frac(VIB_MAX_LEVELS(vib), vib.speed, MAX_FF_SPEED);
    } else {
    vib.active = false;
    vib.level = VIB_MIN_LEVEL_mV(vib);
    }
    pm8xxx_vib_set(vib, vib.active);
    }
//
// pm8xxx_vib_close - callback of input close callback
// @dev: input device pointer
//
// Turns off the vibrator.
//
#[no_mangle]
unsafe extern "C" fn pm8xxx_vib_close(dev: *mut input_dev) {
    static void pm8xxx_vib_close(struct input_dev *dev)
    {
    struct pm8xxx_vib *vib = input_get_drvdata(dev);
    cancel_work_sync(&vib.work);
    if (vib.active)
    pm8xxx_vib_set(vib, false);
    }
//
// pm8xxx_vib_play_effect - function to handle vib effects.
// @dev: input device pointer
// @data: data of effect
// @effect: effect to play
//
// Currently this driver supports only rumble effects.
//
    static int pm8xxx_vib_play_effect(struct input_dev *dev, void *data,
    struct ff_effect *effect)
    {
    struct pm8xxx_vib *vib = input_get_drvdata(dev);
    vib.speed = effect.u.rumble.strong_magnitude >> 8;
    if (!vib.speed)
    vib.speed = effect.u.rumble.weak_magnitude >> 9;
    schedule_work(&vib.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm8xxx_vib_probe(pdev: *mut platform_device) -> c_int {
    static int pm8xxx_vib_probe(struct platform_device *pdev)
    {
    struct pm8xxx_vib *vib;
    struct input_dev *input_dev;
    int error;
    unsigned int val, reg_base = 0;
    const struct pm8xxx_regs *regs;
    vib = devm_kzalloc(&pdev.dev, sizeof(*vib), GFP_KERNEL);
    if (!vib)
    return -ENOMEM;
    vib.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!vib.regmap)
    return -ENODEV;
    input_dev = devm_input_allocate_device(&pdev.dev);
    if (!input_dev)
    return -ENOMEM;
    INIT_WORK(&vib.work, pm8xxx_work_handler);
    vib.vib_input_dev = input_dev;
    error = fwnode_property_read_u32(pdev.dev.fwnode, "reg", &reg_base);
    if (error < 0)
    return dev_err_probe(&pdev.dev, error, "Failed to read reg address\n");
    regs = of_device_get_match_data(&pdev.dev);
    vib.enable_addr = reg_base + regs.enable_offset;
    vib.drv_addr = reg_base + regs.drv_offset;
    vib.drv2_addr = reg_base + regs.drv2_offset;
// operate in manual mode
    error = regmap_read(vib.regmap, vib.drv_addr, &val);
    if (error < 0)
    return error;
    val &= regs.drv_en_manual_mask;
    error = regmap_write(vib.regmap, vib.drv_addr, val);
    if (error < 0)
    return error;
    vib.regs = regs;
    vib.reg_vib_drv = val;
    input_dev.name = "pm8xxx_vib_ffmemless";
    input_dev.id.version = 1;
    input_dev.close = pm8xxx_vib_close;
    input_set_drvdata(input_dev, vib);
    input_set_capability(vib.vib_input_dev, EV_FF, FF_RUMBLE);
    error = input_ff_create_memless(input_dev, core::ptr::null_mut(),
    pm8xxx_vib_play_effect);
    if (error) {
    dev_err(&pdev.dev,
    "couldn't register vibrator as FF device\n");
    return error;
    }
    error = input_register_device(input_dev);
    if (error) {
    dev_err(&pdev.dev, "couldn't register input device\n");
    return error;
    }
    platform_set_drvdata(pdev, vib);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm8xxx_vib_suspend(dev: *mut device) -> c_int {
    static int pm8xxx_vib_suspend(struct device *dev)
    {
    struct pm8xxx_vib *vib = dev_get_drvdata(dev);
// Turn off the vibrator
    pm8xxx_vib_set(vib, false);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(pm8xxx_vib_pm_ops, pm8xxx_vib_suspend, core::ptr::null_mut());
    static const struct of_device_id pm8xxx_vib_id_table[] = {
    { .compatible = "qcom,pm8058-vib", .data = &pm8058_regs },
    { .compatible = "qcom,pm8921-vib", .data = &pm8058_regs },
    { .compatible = "qcom,pm8916-vib", .data = &pm8916_regs },
    { .compatible = "qcom,pmi632-vib", .data = &pmi632_regs },
    { }
    };
    MODULE_DEVICE_TABLE(of, pm8xxx_vib_id_table);
    static struct platform_driver pm8xxx_vib_driver = {
    .probe		= pm8xxx_vib_probe,
    .driver		= {
    .name	= "pm8xxx-vib",
    .pm	= pm_sleep_ptr(&pm8xxx_vib_pm_ops),
    .of_match_table = pm8xxx_vib_id_table,
    },
    };
    module_platform_driver(pm8xxx_vib_driver);
    MODULE_ALIAS("platform:pm8xxx_vib");
    MODULE_DESCRIPTION("PMIC8xxx vibrator driver based on ff-memless framework");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Amy Maloche <amaloche@codeaurora.org>");
