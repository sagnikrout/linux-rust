//! Automatically rewritten from C to Rust
//! Source: drivers/bus/arm-integrator-lm.c
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
// ARM Integrator Logical Module bus driver
// Copyright (C) 2020 Linaro Ltd.
// Author: Linus Walleij <linus.walleij@linaro.org>
//
// See the device tree bindings for this block for more details on the
// hardware.
//

// All information about the connected logic modules are in here
pub const INTEGRATOR_SC_DEC_OFFSET: c_uint = 0x10;
// Base address for the expansion modules
pub const INTEGRATOR_AP_EXP_BASE: c_uint = 0xc0000000;
pub const INTEGRATOR_AP_EXP_STRIDE: c_uint = 0x10000000;
#[no_mangle]
unsafe extern "C" fn integrator_lm_populate(num: c_int, dev: *mut device) -> c_int {
    static int integrator_lm_populate(int num, struct device *dev)
    {
    struct device_node *np = dev.of_node;
    struct device_node *child;
    u32 base;
    int ret;
    base = INTEGRATOR_AP_EXP_BASE + (num * INTEGRATOR_AP_EXP_STRIDE);
// Walk over the child nodes and see what chipselects we use
    for_each_available_child_of_node(np, child) {
    struct resource res;
    ret = of_address_to_resource(child, 0, &res);
    if (ret) {
    dev_info(dev, "no valid address on child\n");
    continue;
    }
// First populate the syscon then any devices
    if (res.start == base) {
    dev_info(dev, "populate module @0x%08x from DT\n",
    base);
    ret = of_platform_default_populate(child, core::ptr::null_mut(), dev);
    if (ret) {
    dev_err(dev, "failed to populate module\n");
    of_node_put(child);
    return ret;
    }
    }
    }
    return 0;
    }
    static const struct of_device_id integrator_ap_syscon_match[] = {
    { .compatible = "arm,integrator-ap-syscon"},
    { },
    };
#[no_mangle]
unsafe extern "C" fn integrator_ap_lm_probe(pdev: *mut platform_device) -> c_int {
    static int integrator_ap_lm_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *syscon;
    static struct regmap *map;
    u32 val;
    int ret;
    int i;
// Look up the system controller
    syscon = of_find_matching_node(core::ptr::null_mut(), integrator_ap_syscon_match);
    if (!syscon) {
    dev_err(dev,
    "could not find Integrator/AP system controller\n");
    return -ENODEV;
    }
    map = syscon_node_to_regmap(syscon);
    of_node_put(syscon);
    if (IS_ERR(map)) {
    dev_err(dev,
    "could not find Integrator/AP system controller\n");
    return PTR_ERR(map);
    }
    ret = regmap_read(map, INTEGRATOR_SC_DEC_OFFSET, &val);
    if (ret) {
    dev_err(dev, "could not read from Integrator/AP syscon\n");
    return ret;
    }
// Loop over the connected modules
    for (i = 0; i < 4; i++) {
    if (!(val & BIT(4 + i)))
    continue;
    dev_info(dev, "detected module in slot %d\n", i);
    ret = integrator_lm_populate(i, dev);
    if (ret)
    return ret;
    }
    return 0;
    }
    static const struct of_device_id integrator_ap_lm_match[] = {
    { .compatible = "arm,integrator-ap-lm"},
    { },
    };
    static struct platform_driver integrator_ap_lm_driver = {
    .probe = integrator_ap_lm_probe,
    .driver = {
    .name = "integratorap-lm",
    .of_match_table = integrator_ap_lm_match,
    },
    };
    module_platform_driver(integrator_ap_lm_driver);
    MODULE_AUTHOR("Linus Walleij <linus.walleij@linaro.org>");
    MODULE_DESCRIPTION("Integrator AP Logical Module driver");
