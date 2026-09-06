//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/tps65911-comparator.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// tps65910.c  --  TI TPS6591x
//
// Copyright 2010 Texas Instruments Inc.
//
// Author: Jorge Eduardo Candelaria <jedu@slimlogic.co.uk>
//

pub const COMP1: c_int = 0;
pub const COMP2: c_int = 1;
// Comparator 1 voltage selection table in millivolts
    static const u16 COMP_VSEL_TABLE[] = {
    0, 2500, 2500, 2500, 2500, 2550, 2600, 2650,
    2700, 2750, 2800, 2850, 2900, 2950, 3000, 3050,
    3100, 3150, 3200, 3250, 3300, 3350, 3400, 3450,
    3500,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct comparator {
    pub name: *const c_char,
    pub reg: c_int,
    pub uV_max: c_int,
    pub vsel_table: *const u16,
}

    static struct comparator tps_comparators[] = {
    {
    .name = "COMP1",
    .reg = TPS65911_VMBCH,
    .uV_max = 3500,
    .vsel_table = COMP_VSEL_TABLE,
    },
    {
    .name = "COMP2",
    .reg = TPS65911_VMBCH2,
    .uV_max = 3500,
    .vsel_table = COMP_VSEL_TABLE,
    },
    };
#[no_mangle]
unsafe extern "C" fn comp_threshold_set(tps65910: *mut tps65910, id: c_int, voltage: c_int) -> c_int {
    static int comp_threshold_set(struct tps65910 *tps65910, int id, int voltage)
    {
    let mut tps_comp: comparator = tps_comparators[id];
    let mut curr_voltage: c_int = 0;
    int ret;
    let mut index: u8 = 0, val;
    while (curr_voltage < tps_comp.uV_max) {
    curr_voltage = tps_comp.vsel_table[index];
    if (curr_voltage >= voltage)
    break;
#[no_mangle]
pub unsafe extern "C" fn if(voltage: curr_voltage <) -> else {
    else if (curr_voltage < voltage)
    index ++;
    }
    if (curr_voltage > tps_comp.uV_max)
    return -EINVAL;
    val = index << 1;
    ret = regmap_write(tps65910.regmap, tps_comp.reg, val);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn comp_threshold_get(tps65910: *mut tps65910, id: c_int) -> c_int {
    static int comp_threshold_get(struct tps65910 *tps65910, int id)
    {
    let mut tps_comp: comparator = tps_comparators[id];
    unsigned int val;
    int ret;
    ret = regmap_read(tps65910.regmap, tps_comp.reg, &val);
    if (ret < 0)
    return ret;
    val >>= 1;
    return tps_comp.vsel_table[val];
    }
    static ssize_t comp_threshold_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct tps65910 *tps65910 = dev_get_drvdata(dev.parent);
    let mut comp_attr: attribute = attr.attr;
    int id, uVolt;
    if (!strcmp(comp_attr.name, "comp1_threshold"))
    id = COMP1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(comp_attr.name, _arg: "comp2_threshold")) -> else {
    else if (!strcmp(comp_attr.name, "comp2_threshold"))
    id = COMP2;
    else
    return -EINVAL;
    uVolt = comp_threshold_get(tps65910, id);
    return sprintf(buf, "%d\n", uVolt);
    }
    static DEVICE_ATTR(comp1_threshold, S_IRUGO, comp_threshold_show, core::ptr::null_mut());
    static DEVICE_ATTR(comp2_threshold, S_IRUGO, comp_threshold_show, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn tps65911_comparator_probe(pdev: *mut platform_device) -> c_int {
    static int tps65911_comparator_probe(struct platform_device *pdev)
    {
    struct tps65910 *tps65910 = dev_get_drvdata(pdev.dev.parent);
    struct tps65910_board *pdata = dev_get_platdata(tps65910.dev);
    int ret;
    ret = comp_threshold_set(tps65910, COMP1,  pdata.vmbch_threshold);
    if (ret < 0) {
    dev_err(&pdev.dev, "cannot set COMP1 threshold\n");
    return ret;
    }
    ret = comp_threshold_set(tps65910, COMP2, pdata.vmbch2_threshold);
    if (ret < 0) {
    dev_err(&pdev.dev, "cannot set COMP2 threshold\n");
    return ret;
    }
// Create sysfs entry
    ret = device_create_file(&pdev.dev, &dev_attr_comp1_threshold);
    if (ret < 0)
    dev_err(&pdev.dev, "failed to add COMP1 sysfs file\n");
    ret = device_create_file(&pdev.dev, &dev_attr_comp2_threshold);
    if (ret < 0)
    dev_err(&pdev.dev, "failed to add COMP2 sysfs file\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tps65911_comparator_remove(pdev: *mut platform_device) {
    static void tps65911_comparator_remove(struct platform_device *pdev)
    {
    struct tps65910 *tps65910;
    tps65910 = dev_get_drvdata(pdev.dev.parent);
    device_remove_file(&pdev.dev, &dev_attr_comp2_threshold);
    device_remove_file(&pdev.dev, &dev_attr_comp1_threshold);
    }
    static struct platform_driver tps65911_comparator_driver = {
    .driver = {
    .name = "tps65911-comparator",
    },
    .probe = tps65911_comparator_probe,
    .remove = tps65911_comparator_remove,
    };
#[no_mangle]
unsafe extern "C" fn tps65911_comparator_init() -> int __init {
    static int __init tps65911_comparator_init(void)
    {
    return platform_driver_register(&tps65911_comparator_driver);
    }
    subsys_initcall(tps65911_comparator_init);
#[no_mangle]
unsafe extern "C" fn tps65911_comparator_exit() -> void __exit {
    static void __exit tps65911_comparator_exit(void)
    {
    platform_driver_unregister(&tps65911_comparator_driver);
    }
    module_exit(tps65911_comparator_exit);
    MODULE_AUTHOR("Jorge Eduardo Candelaria <jedu@slimlogic.co.uk>");
    MODULE_DESCRIPTION("TPS65911 comparator driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:tps65911-comparator");
