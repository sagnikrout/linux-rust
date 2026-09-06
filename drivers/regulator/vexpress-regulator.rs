//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/vexpress-regulator.c
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
// Copyright (C) 2012 ARM Limited

#[no_mangle]
unsafe extern "C" fn vexpress_regulator_get_voltage(regdev: *mut regulator_dev) -> c_int {
    static int vexpress_regulator_get_voltage(struct regulator_dev *regdev)
    {
    unsigned int uV;
    let mut err: c_int = regmap_read(regdev.regmap, 0, &uV);
    return err ? err : uV;
    }
    static int vexpress_regulator_set_voltage(struct regulator_dev *regdev,
    int min_uV, int max_uV, unsigned *selector)
    {
    return regmap_write(regdev.regmap, 0, min_uV);
    }
    static const struct regulator_ops vexpress_regulator_ops_ro = {
    .get_voltage = vexpress_regulator_get_voltage,
    };
    static const struct regulator_ops vexpress_regulator_ops = {
    .get_voltage = vexpress_regulator_get_voltage,
    .set_voltage = vexpress_regulator_set_voltage,
    };
#[no_mangle]
unsafe extern "C" fn vexpress_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int vexpress_regulator_probe(struct platform_device *pdev)
    {
    struct regulator_desc *desc;
    struct regulator_init_data *init_data;
    let mut config: regulator_config = { };
    struct regulator_dev *rdev;
    struct regmap *regmap;
    desc = devm_kzalloc(&pdev.dev, sizeof(*desc), GFP_KERNEL);
    if (!desc)
    return -ENOMEM;
    regmap = devm_regmap_init_vexpress_config(&pdev.dev);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    desc.name = dev_name(&pdev.dev);
    desc.type = REGULATOR_VOLTAGE;
    desc.owner = THIS_MODULE;
    desc.continuous_voltage_range = true;
    init_data = of_get_regulator_init_data(&pdev.dev, pdev.dev.of_node,
    desc);
    if (!init_data)
    return -EINVAL;
    init_data.constraints.apply_uV = 0;
    if (init_data.constraints.min_uV && init_data.constraints.max_uV)
    desc.ops = &vexpress_regulator_ops;
    else
    desc.ops = &vexpress_regulator_ops_ro;
    config.regmap = regmap;
    config.dev = &pdev.dev;
    config.init_data = init_data;
    config.of_node = pdev.dev.of_node;
    rdev = devm_regulator_register(&pdev.dev, desc, &config);
    return PTR_ERR_OR_ZERO(rdev);
    }
    static const struct of_device_id vexpress_regulator_of_match[] = {
    { .compatible = "arm,vexpress-volt", },
    { }
    };
    MODULE_DEVICE_TABLE(of, vexpress_regulator_of_match);
    static struct platform_driver vexpress_regulator_driver = {
    .probe = vexpress_regulator_probe,
    .driver	= {
    .name = DRVNAME,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = vexpress_regulator_of_match,
    },
    };
    module_platform_driver(vexpress_regulator_driver);
    MODULE_AUTHOR("Pawel Moll <pawel.moll@arm.com>");
    MODULE_DESCRIPTION("Versatile Express regulator");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:vexpress-regulator");
