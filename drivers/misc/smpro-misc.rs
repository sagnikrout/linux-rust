//! Automatically rewritten from C to Rust
//! Source: drivers/misc/smpro-misc.c
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
// Ampere Computing SoC's SMpro Misc Driver
//
// Copyright (c) 2022, Ampere Computing LLC
//

// Boot Stage/Progress Registers
pub const BOOTSTAGE: c_uint = 0xB0;
pub const BOOTSTAGE_LO: c_uint = 0xB1;
pub const CUR_BOOTSTAGE: c_uint = 0xB2;
pub const BOOTSTAGE_HI: c_uint = 0xB3;
// SOC State Registers
pub const SOC_POWER_LIMIT: c_uint = 0xE5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smpro_misc {
    pub regmap: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn boot_progress_show(dev: *mut device, da: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t boot_progress_show(struct device *dev, struct device_attribute *da, char *buf)
    {
    struct smpro_misc *misc = dev_get_drvdata(dev);
    u16 boot_progress[3] = { 0 };
    u32 bootstage;
    u8 boot_stage;
    u8 cur_stage;
    u32 reg_lo;
    u32 reg;
    int ret;
// Read current boot stage
    ret = regmap_read(misc.regmap, CUR_BOOTSTAGE, &reg);
    if (ret)
    return ret;
    cur_stage = reg & 0xff;
    ret = regmap_read(misc.regmap, BOOTSTAGE, &bootstage);
    if (ret)
    return ret;
    boot_stage = (bootstage >> 8) & 0xff;
    if (boot_stage > cur_stage)
    return -EINVAL;
    ret = regmap_read(misc.regmap,	BOOTSTAGE_LO, &reg_lo);
    if (!ret)
    ret = regmap_read(misc.regmap, BOOTSTAGE_HI, &reg);
    if (ret)
    return ret;
// Firmware to report new boot stage next time
    if (boot_stage < cur_stage) {
    ret = regmap_write(misc.regmap, BOOTSTAGE, ((bootstage & 0xff00) | 0x1));
    if (ret)
    return ret;
    }
    boot_progress[0] = bootstage;
    boot_progress[1] = swab16(reg);
    boot_progress[2] = swab16(reg_lo);
    return sysfs_emit(buf, "%*phN\n", (int)sizeof(boot_progress), boot_progress);
    }
    static DEVICE_ATTR_RO(boot_progress);
#[no_mangle]
unsafe extern "C" fn soc_power_limit_show(dev: *mut device, da: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t soc_power_limit_show(struct device *dev, struct device_attribute *da, char *buf)
    {
    struct smpro_misc *misc = dev_get_drvdata(dev);
    unsigned int value;
    int ret;
    ret = regmap_read(misc.regmap, SOC_POWER_LIMIT, &value);
    if (ret)
    return ret;
    return sysfs_emit(buf, "%d\n", value);
    }
    static ssize_t soc_power_limit_store(struct device *dev, struct device_attribute *da,
    const char *buf, size_t count)
    {
    struct smpro_misc *misc = dev_get_drvdata(dev);
    unsigned long val;
    s32 ret;
    ret = kstrtoul(buf, 0, &val);
    if (ret)
    return ret;
    ret = regmap_write(misc.regmap, SOC_POWER_LIMIT, (unsigned int)val);
    if (ret)
    return -EPROTO;
    return count;
    }
    static DEVICE_ATTR_RW(soc_power_limit);
    static struct attribute *smpro_misc_attrs[] = {
    &dev_attr_boot_progress.attr,
    &dev_attr_soc_power_limit.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(smpro_misc);
#[no_mangle]
unsafe extern "C" fn smpro_misc_probe(pdev: *mut platform_device) -> c_int {
    static int smpro_misc_probe(struct platform_device *pdev)
    {
    struct smpro_misc *misc;
    misc = devm_kzalloc(&pdev.dev, sizeof(struct smpro_misc), GFP_KERNEL);
    if (!misc)
    return -ENOMEM;
    platform_set_drvdata(pdev, misc);
    misc.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!misc.regmap)
    return -ENODEV;
    return 0;
    }
    static struct platform_driver smpro_misc_driver = {
    .probe		= smpro_misc_probe,
    .driver = {
    .name	= "smpro-misc",
    .dev_groups = smpro_misc_groups,
    },
    };
    module_platform_driver(smpro_misc_driver);
    MODULE_AUTHOR("Tung Nguyen <tungnguyen@os.amperecomputing.com>");
    MODULE_AUTHOR("Quan Nguyen <quan@os.amperecomputing.com>");
    MODULE_DESCRIPTION("Ampere Altra SMpro Misc driver");
    MODULE_LICENSE("GPL");
