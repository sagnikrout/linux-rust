//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/rc5t583-regulator.c
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
// Regulator driver for RICOH RC5T583 power management chip.
//
// Copyright (c) 2011-2012, NVIDIA CORPORATION.  All rights reserved.
// Author: Laxman dewangan <ldewangan@nvidia.com>
//
// based on code
// Copyright (C) 2011 RICOH COMPANY,LTD
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rc5t583_regulator_info {
    pub deepsleep_id: c_int,
// Regulator register address.
    pub reg_disc_reg: u8,
    pub disc_bit: u8,
    pub deepsleep_reg: u8,
// Regulator specific turn-on delay  and voltage settling time
    pub enable_uv_per_us: c_int,
// Used by regulator core
    pub desc: regulator_desc,
}

#[no_mangle]
unsafe extern "C" fn rc5t583_regulator_enable_time(rdev: *mut regulator_dev) -> c_int {
    static int rc5t583_regulator_enable_time(struct regulator_dev *rdev)
    {
    struct rc5t583_regulator_info *reg_info = rdev_get_drvdata(rdev);
    let mut vsel: c_int = regulator_get_voltage_sel_regmap(rdev);
    let mut curr_uV: c_int = regulator_list_voltage_linear(rdev, vsel);
    return DIV_ROUND_UP(curr_uV, reg_info.enable_uv_per_us);
    }
    static const struct regulator_ops rc5t583_ops = {
    .is_enabled		= regulator_is_enabled_regmap,
    .enable			= regulator_enable_regmap,
    .disable		= regulator_disable_regmap,
    .enable_time		= rc5t583_regulator_enable_time,
    .get_voltage_sel	= regulator_get_voltage_sel_regmap,
    .set_voltage_sel	= regulator_set_voltage_sel_regmap,
    .list_voltage		= regulator_list_voltage_linear,
    .map_voltage		= regulator_map_voltage_linear,
    .set_voltage_time_sel	= regulator_set_voltage_time_sel,
    };

    _vout_mask, _min_mv, _max_mv, _step_uV, _enable_mv) \
    {								\
    .reg_disc_reg	= RC5T583_REG_##_disc_reg,		\
    .disc_bit	= _disc_bit,				\
    .deepsleep_reg	= RC5T583_REG_##_id##DAC_DS,		\
    .enable_uv_per_us = _enable_mv * 1000,			\
    .deepsleep_id	= RC5T583_DS_##_id,			\
    .desc = {						\
    .name = "rc5t583-regulator-"#_id,		\
    .id = RC5T583_REGULATOR_##_id,			\
    .n_voltages = (_max_mv - _min_mv) * 1000 / _step_uV + 1, \
    .ops = &rc5t583_ops,				\
    .type = REGULATOR_VOLTAGE,			\
    .owner = THIS_MODULE,				\
    .vsel_reg = RC5T583_REG_##_id##DAC,		\
    .vsel_mask = _vout_mask,			\
    .enable_reg = RC5T583_REG_##_en_reg,		\
    .enable_mask = BIT(_en_bit),			\
    .min_uV	= _min_mv * 1000,			\
    .uV_step = _step_uV,				\
    .ramp_delay = 40 * 1000,			\
    },							\
    }
    static struct rc5t583_regulator_info rc5t583_reg_info[RC5T583_REGULATOR_MAX] = {
    RC5T583_REG(DC0, DC0CTL, 0, DC0CTL, 1, 0x7F, 700, 1500, 12500, 4),
    RC5T583_REG(DC1, DC1CTL, 0, DC1CTL, 1, 0x7F, 700, 1500, 12500, 14),
    RC5T583_REG(DC2, DC2CTL, 0, DC2CTL, 1, 0x7F, 900, 2400, 12500, 14),
    RC5T583_REG(DC3, DC3CTL, 0, DC3CTL, 1, 0x7F, 900, 2400, 12500, 14),
    RC5T583_REG(LDO0, LDOEN2, 0, LDODIS2, 0, 0x7F, 900, 3400, 25000, 160),
    RC5T583_REG(LDO1, LDOEN2, 1, LDODIS2, 1, 0x7F, 900, 3400, 25000, 160),
    RC5T583_REG(LDO2, LDOEN2, 2, LDODIS2, 2, 0x7F, 900, 3400, 25000, 160),
    RC5T583_REG(LDO3, LDOEN2, 3, LDODIS2, 3, 0x7F, 900, 3400, 25000, 160),
    RC5T583_REG(LDO4, LDOEN2, 4, LDODIS2, 4, 0x3F, 750, 1500, 12500, 133),
    RC5T583_REG(LDO5, LDOEN2, 5, LDODIS2, 5, 0x7F, 900, 3400, 25000, 267),
    RC5T583_REG(LDO6, LDOEN2, 6, LDODIS2, 6, 0x7F, 900, 3400, 25000, 133),
    RC5T583_REG(LDO7, LDOEN2, 7, LDODIS2, 7, 0x7F, 900, 3400, 25000, 233),
    RC5T583_REG(LDO8, LDOEN1, 0, LDODIS1, 0, 0x7F, 900, 3400, 25000, 233),
    RC5T583_REG(LDO9, LDOEN1, 1, LDODIS1, 1, 0x7F, 900, 3400, 25000, 133),
    };
#[no_mangle]
unsafe extern "C" fn rc5t583_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int rc5t583_regulator_probe(struct platform_device *pdev)
    {
    struct rc5t583 *rc5t583 = dev_get_drvdata(pdev.dev.parent);
    struct rc5t583_platform_data *pdata = dev_get_platdata(rc5t583.dev);
    let mut config: regulator_config = { };
    struct regulator_dev *rdev;
    struct rc5t583_regulator_info *ri;
    int ret;
    int id;
    if (!pdata) {
    dev_err(&pdev.dev, "No platform data, exiting...\n");
    return -ENODEV;
    }
    for (id = 0; id < RC5T583_REGULATOR_MAX; ++id) {
    ri = &rc5t583_reg_info[id];
    if (ri.deepsleep_id == RC5T583_DS_NONE)
    goto skip_ext_pwr_config;
    ret = rc5t583_ext_power_req_config(rc5t583.dev,
    ri.deepsleep_id,
    pdata.regulator_ext_pwr_control[id],
    pdata.regulator_deepsleep_slot[id]);
//
// Configuring external control is not a major issue,
// just give warning.
//
    if (ret < 0)
    dev_warn(&pdev.dev,
    "Failed to configure ext control %d\n", id);
    skip_ext_pwr_config:
    config.dev = &pdev.dev;
    config.init_data = pdata.reg_init_data[id];
    config.driver_data = ri;
    config.regmap = rc5t583.regmap;
    rdev = devm_regulator_register(&pdev.dev, &ri.desc, &config);
    if (IS_ERR(rdev)) {
    dev_err(&pdev.dev, "Failed to register regulator %s\n",
    ri.desc.name);
    return PTR_ERR(rdev);
    }
    }
    return 0;
    }
    static struct platform_driver rc5t583_regulator_driver = {
    .driver	= {
    .name	= "rc5t583-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe		= rc5t583_regulator_probe,
    };
#[no_mangle]
unsafe extern "C" fn rc5t583_regulator_init() -> int __init {
    static int __init rc5t583_regulator_init(void)
    {
    return platform_driver_register(&rc5t583_regulator_driver);
    }
    subsys_initcall(rc5t583_regulator_init);
#[no_mangle]
unsafe extern "C" fn rc5t583_regulator_exit() -> void __exit {
    static void __exit rc5t583_regulator_exit(void)
    {
    platform_driver_unregister(&rc5t583_regulator_driver);
    }
    module_exit(rc5t583_regulator_exit);
    MODULE_AUTHOR("Laxman Dewangan <ldewangan@nvidia.com>");
    MODULE_DESCRIPTION("RC5T583 regulator driver");
    MODULE_ALIAS("platform:rc5t583-regulator");
    MODULE_LICENSE("GPL v2");
