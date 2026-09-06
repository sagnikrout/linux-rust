//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/aat2870-regulator.c
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
// linux/drivers/regulator/aat2870-regulator.c
//
// Copyright (c) 2011, NVIDIA Corporation.
// Author: Jin Park <jinyoungp@nvidia.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aat2870_regulator {
    pub aat2870: *mut aat2870_data,
    pub desc: regulator_desc,
    pub enable_addr: u8,
    pub enable_shift: u8,
    pub enable_mask: u8,
    pub voltage_addr: u8,
    pub voltage_shift: u8,
    pub voltage_mask: u8,
}

    static int aat2870_ldo_set_voltage_sel(struct regulator_dev *rdev,
    unsigned selector)
    {
    struct aat2870_regulator *ri = rdev_get_drvdata(rdev);
    struct aat2870_data *aat2870 = ri.aat2870;
    return aat2870.update(aat2870, ri.voltage_addr, ri.voltage_mask,
    selector << ri.voltage_shift);
    }
#[no_mangle]
unsafe extern "C" fn aat2870_ldo_get_voltage_sel(rdev: *mut regulator_dev) -> c_int {
    static int aat2870_ldo_get_voltage_sel(struct regulator_dev *rdev)
    {
    struct aat2870_regulator *ri = rdev_get_drvdata(rdev);
    struct aat2870_data *aat2870 = ri.aat2870;
    u8 val;
    int ret;
    ret = aat2870.read(aat2870, ri.voltage_addr, &val);
    if (ret)
    return ret;
    return (val & ri.voltage_mask) >> ri.voltage_shift;
    }
#[no_mangle]
unsafe extern "C" fn aat2870_ldo_enable(rdev: *mut regulator_dev) -> c_int {
    static int aat2870_ldo_enable(struct regulator_dev *rdev)
    {
    struct aat2870_regulator *ri = rdev_get_drvdata(rdev);
    struct aat2870_data *aat2870 = ri.aat2870;
    return aat2870.update(aat2870, ri.enable_addr, ri.enable_mask,
    ri.enable_mask);
    }
#[no_mangle]
unsafe extern "C" fn aat2870_ldo_disable(rdev: *mut regulator_dev) -> c_int {
    static int aat2870_ldo_disable(struct regulator_dev *rdev)
    {
    struct aat2870_regulator *ri = rdev_get_drvdata(rdev);
    struct aat2870_data *aat2870 = ri.aat2870;
    return aat2870.update(aat2870, ri.enable_addr, ri.enable_mask, 0);
    }
#[no_mangle]
unsafe extern "C" fn aat2870_ldo_is_enabled(rdev: *mut regulator_dev) -> c_int {
    static int aat2870_ldo_is_enabled(struct regulator_dev *rdev)
    {
    struct aat2870_regulator *ri = rdev_get_drvdata(rdev);
    struct aat2870_data *aat2870 = ri.aat2870;
    u8 val;
    int ret;
    ret = aat2870.read(aat2870, ri.enable_addr, &val);
    if (ret)
    return ret;
    return val & ri.enable_mask ? 1 : 0;
    }
    static const struct regulator_ops aat2870_ldo_ops = {
    .list_voltage = regulator_list_voltage_table,
    .map_voltage = regulator_map_voltage_ascend,
    .set_voltage_sel = aat2870_ldo_set_voltage_sel,
    .get_voltage_sel = aat2870_ldo_get_voltage_sel,
    .enable = aat2870_ldo_enable,
    .disable = aat2870_ldo_disable,
    .is_enabled = aat2870_ldo_is_enabled,
    };
    static const unsigned int aat2870_ldo_voltages[] = {
    1200000, 1300000, 1500000, 1600000,
    1800000, 2000000, 2200000, 2500000,
    2600000, 2700000, 2800000, 2900000,
    3000000, 3100000, 3200000, 3300000,
    };

    {						\
    .desc = {				\
    .name = #ids,			\
    .id = AAT2870_ID_##ids,		\
    .n_voltages = ARRAY_SIZE(aat2870_ldo_voltages),	\
    .volt_table = aat2870_ldo_voltages, \
    .ops = &aat2870_ldo_ops,	\
    .type = REGULATOR_VOLTAGE,	\
    .owner = THIS_MODULE,		\
    },					\
    }
    static struct aat2870_regulator aat2870_regulators[] = {
    AAT2870_LDO(LDOA),
    AAT2870_LDO(LDOB),
    AAT2870_LDO(LDOC),
    AAT2870_LDO(LDOD),
    };
    static struct aat2870_regulator *aat2870_get_regulator(int id)
    {
    struct aat2870_regulator *ri = core::ptr::null_mut();
    int i;
    for (i = 0; i < ARRAY_SIZE(aat2870_regulators); i++) {
    ri = &aat2870_regulators[i];
    if (ri.desc.id == id)
    break;
    }
    if (i == ARRAY_SIZE(aat2870_regulators))
    return core::ptr::null_mut();
    ri.enable_addr = AAT2870_LDO_EN;
    ri.enable_shift = id - AAT2870_ID_LDOA;
    ri.enable_mask = 0x1 << ri.enable_shift;
    ri.voltage_addr = (id - AAT2870_ID_LDOA) / 2 ?
    AAT2870_LDO_CD : AAT2870_LDO_AB;
    ri.voltage_shift = (id - AAT2870_ID_LDOA) % 2 ? 0 : 4;
    ri.voltage_mask = 0xF << ri.voltage_shift;
    return ri;
    }
#[no_mangle]
unsafe extern "C" fn aat2870_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int aat2870_regulator_probe(struct platform_device *pdev)
    {
    struct aat2870_regulator *ri;
    let mut config: regulator_config = { };
    struct regulator_dev *rdev;
    ri = aat2870_get_regulator(pdev.id);
    if (!ri) {
    dev_err(&pdev.dev, "Invalid device ID, %d\n", pdev.id);
    return -EINVAL;
    }
    ri.aat2870 = dev_get_drvdata(pdev.dev.parent);
    config.dev = &pdev.dev;
    config.driver_data = ri;
    config.init_data = dev_get_platdata(&pdev.dev);
    rdev = devm_regulator_register(&pdev.dev, &ri.desc, &config);
    if (IS_ERR(rdev)) {
    dev_err(&pdev.dev, "Failed to register regulator %s\n",
    ri.desc.name);
    return PTR_ERR(rdev);
    }
    platform_set_drvdata(pdev, rdev);
    return 0;
    }
    static struct platform_driver aat2870_regulator_driver = {
    .driver = {
    .name	= "aat2870-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe	= aat2870_regulator_probe,
    };
#[no_mangle]
unsafe extern "C" fn aat2870_regulator_init() -> int __init {
    static int __init aat2870_regulator_init(void)
    {
    return platform_driver_register(&aat2870_regulator_driver);
    }
    subsys_initcall(aat2870_regulator_init);
#[no_mangle]
unsafe extern "C" fn aat2870_regulator_exit() -> void __exit {
    static void __exit aat2870_regulator_exit(void)
    {
    platform_driver_unregister(&aat2870_regulator_driver);
    }
    module_exit(aat2870_regulator_exit);
    MODULE_DESCRIPTION("AnalogicTech AAT2870 Regulator");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Jin Park <jinyoungp@nvidia.com>");
    MODULE_ALIAS("platform:aat2870-regulator");
