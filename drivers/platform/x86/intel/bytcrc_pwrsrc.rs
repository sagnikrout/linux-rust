//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/bytcrc_pwrsrc.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Power-source driver for Bay Trail Crystal Cove PMIC
//
// Copyright (c) 2023 Hans de Goede <hdegoede@redhat.com>
//
// Based on intel_crystalcove_pwrsrc.c from Android kernel sources, which is:
// Copyright (C) 2013 Intel Corporation
//

pub const CRYSTALCOVE_PWRSRC_IRQ: c_uint = 0x03;
pub const CRYSTALCOVE_SPWRSRC_REG: c_uint = 0x1E;

pub const CRYSTALCOVE_RESETSRC0_REG: c_uint = 0x20;
pub const CRYSTALCOVE_RESETSRC1_REG: c_uint = 0x21;
pub const CRYSTALCOVE_WAKESRC_REG: c_uint = 0x22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crc_pwrsrc_data {
    pub regmap: *mut regmap,
    pub debug_dentry: *mut dentry,
    pub psy: *mut power_supply,
    pub resetsrc0: c_uint,
    pub resetsrc1: c_uint,
    pub wakesrc: c_uint,
}

    static const char * const pwrsrc_pwrsrc_info[] = {
// bit 0 */ "USB",
// bit 1 */ "DC in",
// bit 2 */ "Battery",
    core::ptr::null_mut(),
    };
    static const char * const pwrsrc_resetsrc0_info[] = {
// bit 0 */ "SOC reporting a thermal event",
// bit 1 */ "critical PMIC temperature",
// bit 2 */ "critical system temperature",
// bit 3 */ "critical battery temperature",
// bit 4 */ "VSYS under voltage",
// bit 5 */ "VSYS over voltage",
// bit 6 */ "battery removal",
    core::ptr::null_mut(),
    };
    static const char * const pwrsrc_resetsrc1_info[] = {
// bit 0 */ "VCRIT threshold",
// bit 1 */ "BATID reporting battery removal",
// bit 2 */ "user pressing the power button",
    core::ptr::null_mut(),
    };
    static const char * const pwrsrc_wakesrc_info[] = {
// bit 0 */ "user pressing the power button",
// bit 1 */ "a battery insertion",
// bit 2 */ "a USB charger insertion",
// bit 3 */ "an adapter insertion",
    core::ptr::null_mut(),
    };
    static void crc_pwrsrc_log(struct seq_file *seq, const char *prefix,
    const char * const *info, unsigned int reg_val)
    {
    int i;
    for (i = 0; info[i]; i++) {
    if (reg_val & BIT(i))
    seq_printf(seq, "%s by %s\n", prefix, info[i]);
    }
    }
#[no_mangle]
unsafe extern "C" fn pwrsrc_show(seq: *mut seq_file, unused: *mut c_void) -> c_int {
    static int pwrsrc_show(struct seq_file *seq, void *unused)
    {
    struct crc_pwrsrc_data *data = seq.private;
    unsigned int reg_val;
    int ret;
    ret = regmap_read(data.regmap, CRYSTALCOVE_SPWRSRC_REG, &reg_val);
    if (ret)
    return ret;
    crc_pwrsrc_log(seq, "System powered", pwrsrc_pwrsrc_info, reg_val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn resetsrc_show(seq: *mut seq_file, unused: *mut c_void) -> c_int {
    static int resetsrc_show(struct seq_file *seq, void *unused)
    {
    struct crc_pwrsrc_data *data = seq.private;
    crc_pwrsrc_log(seq, "Last shutdown caused", pwrsrc_resetsrc0_info, data.resetsrc0);
    crc_pwrsrc_log(seq, "Last shutdown caused", pwrsrc_resetsrc1_info, data.resetsrc1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wakesrc_show(seq: *mut seq_file, unused: *mut c_void) -> c_int {
    static int wakesrc_show(struct seq_file *seq, void *unused)
    {
    struct crc_pwrsrc_data *data = seq.private;
    crc_pwrsrc_log(seq, "Last wake caused", pwrsrc_wakesrc_info, data.wakesrc);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(pwrsrc);
    DEFINE_SHOW_ATTRIBUTE(resetsrc);
    DEFINE_SHOW_ATTRIBUTE(wakesrc);
    static int crc_pwrsrc_read_and_clear(struct crc_pwrsrc_data *data,
    unsigned int reg, unsigned int *val)
    {
    int ret;
    ret = regmap_read(data.regmap, reg, val);
    if (ret)
    return ret;
    return regmap_write(data.regmap, reg, *val);
    }
#[no_mangle]
unsafe extern "C" fn crc_pwrsrc_irq_handler(irq: c_int, _data: *mut c_void) -> irqreturn_t {
    static irqreturn_t crc_pwrsrc_irq_handler(int irq, void *_data)
    {
    struct crc_pwrsrc_data *data = _data;
    unsigned int irq_mask;
    if (regmap_read(data.regmap, CRYSTALCOVE_PWRSRC_IRQ, &irq_mask))
    return IRQ_NONE;
    regmap_write(data.regmap, CRYSTALCOVE_PWRSRC_IRQ, irq_mask);
    power_supply_changed(data.psy);
    return IRQ_HANDLED;
    }
    static int crc_pwrsrc_psy_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct crc_pwrsrc_data *data = power_supply_get_drvdata(psy);
    unsigned int pwrsrc;
    int ret;
    if (psp != POWER_SUPPLY_PROP_ONLINE)
    return -EINVAL;
    ret = regmap_read(data.regmap, CRYSTALCOVE_SPWRSRC_REG, &pwrsrc);
    if (ret)
    return ret;
    val.intval = !!(pwrsrc & (CRYSTALCOVE_SPWRSRC_USB |
    CRYSTALCOVE_SPWRSRC_DC));
    return 0;
    }
    static const enum power_supply_property crc_pwrsrc_psy_props[] = {
    POWER_SUPPLY_PROP_ONLINE,
    };
    static const struct power_supply_desc crc_pwrsrc_psy_desc = {
    .name = "crystal_cove_pwrsrc",
    .type = POWER_SUPPLY_TYPE_MAINS,
    .properties = crc_pwrsrc_psy_props,
    .num_properties = ARRAY_SIZE(crc_pwrsrc_psy_props),
    .get_property = crc_pwrsrc_psy_get_property,
    };
#[no_mangle]
unsafe extern "C" fn crc_pwrsrc_probe(pdev: *mut platform_device) -> c_int {
    static int crc_pwrsrc_probe(struct platform_device *pdev)
    {
    struct intel_soc_pmic *pmic = dev_get_drvdata(pdev.dev.parent);
    struct device *dev = &pdev.dev;
    struct crc_pwrsrc_data *data;
    int irq, ret;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.regmap = pmic.regmap;
//
// Read + clear resetsrc0/1 and wakesrc now, so that they get
// cleared even if the debugfs interface is never used.
//
// Properly clearing the wakesrc is important, leaving bit 0 of it
// set turns reboot into poweroff on some tablets.
//
    ret = crc_pwrsrc_read_and_clear(data, CRYSTALCOVE_RESETSRC0_REG, &data.resetsrc0);
    if (ret)
    return ret;
    ret = crc_pwrsrc_read_and_clear(data, CRYSTALCOVE_RESETSRC1_REG, &data.resetsrc1);
    if (ret)
    return ret;
    ret = crc_pwrsrc_read_and_clear(data, CRYSTALCOVE_WAKESRC_REG, &data.wakesrc);
    if (ret)
    return ret;
    if (device_property_read_bool(dev.parent, "linux,register-pwrsrc-power_supply")) {
    let mut psy_cfg: power_supply_config = { .drv_data = data };
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    data.psy = devm_power_supply_register(dev, &crc_pwrsrc_psy_desc, &psy_cfg);
    if (IS_ERR(data.psy))
    return dev_err_probe(dev, PTR_ERR(data.psy), "registering power-supply\n");
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(),
    crc_pwrsrc_irq_handler,
    IRQF_ONESHOT, KBUILD_MODNAME, data);
    if (ret)
    return dev_err_probe(dev, ret, "requesting IRQ\n");
    }
    data.debug_dentry = debugfs_create_dir(KBUILD_MODNAME, core::ptr::null_mut());
    debugfs_create_file("pwrsrc", 0444, data.debug_dentry, data, &pwrsrc_fops);
    debugfs_create_file("resetsrc", 0444, data.debug_dentry, data, &resetsrc_fops);
    debugfs_create_file("wakesrc", 0444, data.debug_dentry, data, &wakesrc_fops);
    platform_set_drvdata(pdev, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crc_pwrsrc_remove(pdev: *mut platform_device) {
    static void crc_pwrsrc_remove(struct platform_device *pdev)
    {
    struct crc_pwrsrc_data *data = platform_get_drvdata(pdev);
    debugfs_remove_recursive(data.debug_dentry);
    }
    static struct platform_driver crc_pwrsrc_driver = {
    .probe = crc_pwrsrc_probe,
    .remove = crc_pwrsrc_remove,
    .driver = {
    .name = "crystal_cove_pwrsrc",
    },
    };
    module_platform_driver(crc_pwrsrc_driver);
    MODULE_ALIAS("platform:crystal_cove_pwrsrc");
    MODULE_AUTHOR("Hans de Goede <hdegoede@redhat.com>");
    MODULE_DESCRIPTION("Power-source driver for Bay Trail Crystal Cove PMIC");
    MODULE_LICENSE("GPL");
