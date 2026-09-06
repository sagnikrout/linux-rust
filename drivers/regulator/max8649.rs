//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/max8649.c
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
// Regulators driver for Maxim max8649
//
// Copyright (C) 2009-2010 Marvell International Ltd.
// Haojian Zhuang <haojian.zhuang@marvell.com>
//

pub const MAX8649_VOL_MASK: c_uint = 0x3f;
// Registers
pub const MAX8649_MODE0: c_uint = 0x00;
pub const MAX8649_MODE1: c_uint = 0x01;
pub const MAX8649_MODE2: c_uint = 0x02;
pub const MAX8649_MODE3: c_uint = 0x03;
pub const MAX8649_CONTROL: c_uint = 0x04;
pub const MAX8649_SYNC: c_uint = 0x05;
pub const MAX8649_RAMP: c_uint = 0x06;
pub const MAX8649_CHIP_ID1: c_uint = 0x08;
pub const MAX8649_CHIP_ID2: c_uint = 0x09;
// Bits

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8649_regulator_info {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub /: *mut *mut unsigned mode:2; / bit[1:0] = VID1, VID0,
    pub extclk_freq:2: unsigned,
    pub extclk:1: unsigned,
    pub ramp_timing:3: unsigned,
    pub ramp_down:1: unsigned,
}

#[no_mangle]
unsafe extern "C" fn max8649_enable_time(rdev: *mut regulator_dev) -> c_int {
    static int max8649_enable_time(struct regulator_dev *rdev)
    {
    struct max8649_regulator_info *info = rdev_get_drvdata(rdev);
    int voltage, rate, ret;
    unsigned int val;
// get voltage
    ret = regmap_read(info.regmap, rdev.desc.vsel_reg, &val);
    if (ret != 0)
    return ret;
    val &= MAX8649_VOL_MASK;
    voltage = regulator_list_voltage_linear(rdev, (unsigned char)val);
// get rate
    ret = regmap_read(info.regmap, MAX8649_RAMP, &val);
    if (ret != 0)
    return ret;
    ret = (val & MAX8649_RAMP_MASK) >> 5;
    rate = (32 * 1000) >> ret;	/* uV/uS */
    return DIV_ROUND_UP(voltage, rate);
    }
#[no_mangle]
unsafe extern "C" fn max8649_set_mode(rdev: *mut regulator_dev, mode: c_uint) -> c_int {
    static int max8649_set_mode(struct regulator_dev *rdev, unsigned int mode)
    {
    struct max8649_regulator_info *info = rdev_get_drvdata(rdev);
    switch (mode) {
    case REGULATOR_MODE_FAST:
    regmap_update_bits(info.regmap, rdev.desc.vsel_reg,
    MAX8649_FORCE_PWM, MAX8649_FORCE_PWM);
    break;
    case REGULATOR_MODE_NORMAL:
    regmap_update_bits(info.regmap, rdev.desc.vsel_reg,
    MAX8649_FORCE_PWM, 0);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max8649_get_mode(rdev: *mut regulator_dev) -> c_uint {
    static unsigned int max8649_get_mode(struct regulator_dev *rdev)
    {
    struct max8649_regulator_info *info = rdev_get_drvdata(rdev);
    unsigned int val;
    int ret;
    ret = regmap_read(info.regmap, rdev.desc.vsel_reg, &val);
    if (ret != 0)
    return ret;
    if (val & MAX8649_FORCE_PWM)
    return REGULATOR_MODE_FAST;
    return REGULATOR_MODE_NORMAL;
    }
    static const struct regulator_ops max8649_dcdc_ops = {
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .list_voltage	= regulator_list_voltage_linear,
    .map_voltage	= regulator_map_voltage_linear,
    .enable		= regulator_enable_regmap,
    .disable	= regulator_disable_regmap,
    .is_enabled	= regulator_is_enabled_regmap,
    .enable_time	= max8649_enable_time,
    .set_mode	= max8649_set_mode,
    .get_mode	= max8649_get_mode,
    };
    static struct regulator_desc dcdc_desc = {
    .name		= "max8649",
    .ops		= &max8649_dcdc_ops,
    .type		= REGULATOR_VOLTAGE,
    .n_voltages	= 1 << 6,
    .owner		= THIS_MODULE,
    .vsel_mask	= MAX8649_VOL_MASK,
    .min_uV		= MAX8649_DCDC_VMIN,
    .uV_step	= MAX8649_DCDC_STEP,
    .enable_reg	= MAX8649_CONTROL,
    .enable_mask	= MAX8649_EN_PD,
    .enable_is_inverted = true,
    };
    static const struct regmap_config max8649_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
#[no_mangle]
unsafe extern "C" fn max8649_regulator_probe(client: *mut i2c_client) -> c_int {
    static int max8649_regulator_probe(struct i2c_client *client)
    {
    struct max8649_platform_data *pdata = dev_get_platdata(&client.dev);
    struct max8649_regulator_info *info = core::ptr::null_mut();
    struct regulator_dev *regulator;
    let mut config: regulator_config = { };
    unsigned int val;
    unsigned char data;
    int ret;
    info = devm_kzalloc(&client.dev, sizeof(struct max8649_regulator_info),
    GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    info.regmap = devm_regmap_init_i2c(client, &max8649_regmap_config);
    if (IS_ERR(info.regmap)) {
    ret = PTR_ERR(info.regmap);
    dev_err(&client.dev, "Failed to allocate register map: %d\n", ret);
    return ret;
    }
    info.dev = &client.dev;
    i2c_set_clientdata(client, info);
    info.mode = pdata.mode;
    switch (info.mode) {
    case 0:
    dcdc_desc.vsel_reg = MAX8649_MODE0;
    break;
    case 1:
    dcdc_desc.vsel_reg = MAX8649_MODE1;
    break;
    case 2:
    dcdc_desc.vsel_reg = MAX8649_MODE2;
    break;
    case 3:
    dcdc_desc.vsel_reg = MAX8649_MODE3;
    break;
    default:
    break;
    }
    ret = regmap_read(info.regmap, MAX8649_CHIP_ID1, &val);
    if (ret != 0) {
    dev_err(info.dev, "Failed to detect ID of MAX8649:%d\n",
    ret);
    return ret;
    }
    dev_info(info.dev, "Detected MAX8649 (ID:%x)\n", val);
// enable VID0 & VID1
    regmap_update_bits(info.regmap, MAX8649_CONTROL, MAX8649_VID_MASK, 0);
// enable/disable external clock synchronization
    info.extclk = pdata.extclk;
    data = (info.extclk) ? MAX8649_SYNC_EXTCLK : 0;
    regmap_update_bits(info.regmap, dcdc_desc.vsel_reg,
    MAX8649_SYNC_EXTCLK, data);
    if (info.extclk) {
// set external clock frequency
    info.extclk_freq = pdata.extclk_freq;
    regmap_update_bits(info.regmap, MAX8649_SYNC, MAX8649_EXT_MASK,
    info.extclk_freq << 6);
    }
    if (pdata.ramp_timing) {
    info.ramp_timing = pdata.ramp_timing;
    regmap_update_bits(info.regmap, MAX8649_RAMP, MAX8649_RAMP_MASK,
    info.ramp_timing << 5);
    }
    info.ramp_down = pdata.ramp_down;
    if (info.ramp_down) {
    regmap_update_bits(info.regmap, MAX8649_RAMP, MAX8649_RAMP_DOWN,
    MAX8649_RAMP_DOWN);
    }
    config.dev = &client.dev;
    config.init_data = pdata.regulator;
    config.driver_data = info;
    config.regmap = info.regmap;
    regulator = devm_regulator_register(&client.dev, &dcdc_desc,
    &config);
    if (IS_ERR(regulator)) {
    dev_err(info.dev, "failed to register regulator %s\n",
    dcdc_desc.name);
    return PTR_ERR(regulator);
    }
    return 0;
    }
    static const struct i2c_device_id max8649_id[] = {
    { .name = "max8649" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max8649_id);
    static struct i2c_driver max8649_driver = {
    .probe		= max8649_regulator_probe,
    .driver		= {
    .name	= "max8649",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .id_table	= max8649_id,
    };
#[no_mangle]
unsafe extern "C" fn max8649_init() -> int __init {
    static int __init max8649_init(void)
    {
    return i2c_add_driver(&max8649_driver);
    }
    subsys_initcall(max8649_init);
#[no_mangle]
unsafe extern "C" fn max8649_exit() -> void __exit {
    static void __exit max8649_exit(void)
    {
    i2c_del_driver(&max8649_driver);
    }
    module_exit(max8649_exit);
// Module information
    MODULE_DESCRIPTION("MAXIM 8649 voltage regulator driver");
    MODULE_AUTHOR("Haojian Zhuang <haojian.zhuang@marvell.com>");
    MODULE_LICENSE("GPL");
