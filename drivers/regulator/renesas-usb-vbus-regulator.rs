//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/renesas-usb-vbus-regulator.c
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
// Renesas USB VBUS output regulator driver
//
// Copyright (C) 2024 Renesas Electronics Corporation
//

    static const struct regulator_ops rzg2l_usb_vbus_reg_ops = {
    .enable     = regulator_enable_regmap,
    .disable    = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    };
    static const struct regulator_desc rzg2l_usb_vbus_rdesc = {
    .name = "vbus",
    .of_match = of_match_ptr("regulator-vbus"),
    .ops = &rzg2l_usb_vbus_reg_ops,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .enable_reg  = 0,
    .enable_mask = BIT(0),
    .enable_is_inverted = true,
    .fixed_uV	= 5000000,
    .n_voltages	= 1,
    };
#[no_mangle]
unsafe extern "C" fn rzg2l_usb_vbus_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int rzg2l_usb_vbus_regulator_probe(struct platform_device *pdev)
    {
    let mut config: regulator_config = { };
    struct device *dev = &pdev.dev;
    struct regulator_dev *rdev;
    config.regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!config.regmap)
    return dev_err_probe(dev, -ENOENT, "Failed to get regmap\n");
    config.dev = dev;
    config.of_node = of_get_child_by_name(dev.parent.of_node, "regulator-vbus");
    if (!config.of_node)
    return dev_err_probe(dev, -ENODEV, "regulator node not found\n");
    rdev = devm_regulator_register(dev, &rzg2l_usb_vbus_rdesc, &config);
    of_node_put(config.of_node);
    if (IS_ERR(rdev))
    return dev_err_probe(dev, PTR_ERR(rdev),
    "not able to register vbus regulator\n");
    return 0;
    }
    static struct platform_driver rzg2l_usb_vbus_regulator_driver = {
    .probe = rzg2l_usb_vbus_regulator_probe,
    .driver	= {
    .name = "rzg2l-usb-vbus-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
    module_platform_driver(rzg2l_usb_vbus_regulator_driver);
    MODULE_AUTHOR("Biju Das <biju.das.jz@bp.renesas.com>");
    MODULE_DESCRIPTION("Renesas RZ/G2L USB Vbus Regulator Driver");
    MODULE_LICENSE("GPL");
