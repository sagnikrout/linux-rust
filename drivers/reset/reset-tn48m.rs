//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-tn48m.c
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
// Delta TN48M CPLD reset driver
//
// Copyright (C) 2021 Sartura Ltd.
//
// Author: Robert Marko <robert.marko@sartura.hr>
//

pub const TN48M_RESET_REG: c_uint = 0x10;
pub const TN48M_RESET_TIMEOUT_US: c_int = 125000;
pub const TN48M_RESET_SLEEP_US: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn48_reset_map {
    pub bit: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn48_reset_data {
    pub rcdev: reset_controller_dev,
    pub regmap: *mut regmap,
}

    static const struct tn48_reset_map tn48m_resets[] = {
    [CPU_88F7040_RESET] = {0},
    [CPU_88F6820_RESET] = {1},
    [MAC_98DX3265_RESET] = {2},
    [PHY_88E1680_RESET] = {4},
    [PHY_88E1512_RESET] = {6},
    [POE_RESET] = {7},
    };
    static inline struct tn48_reset_data *to_tn48_reset_data(
    struct reset_controller_dev *rcdev)
    {
    return container_of(rcdev, struct tn48_reset_data, rcdev);
    }
    static int tn48m_control_reset(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct tn48_reset_data *data = to_tn48_reset_data(rcdev);
    unsigned int val;
    regmap_update_bits(data.regmap, TN48M_RESET_REG,
    BIT(tn48m_resets[id].bit), 0);
    return regmap_read_poll_timeout(data.regmap,
    TN48M_RESET_REG,
    val,
    val & BIT(tn48m_resets[id].bit),
    TN48M_RESET_SLEEP_US,
    TN48M_RESET_TIMEOUT_US);
    }
    static int tn48m_control_status(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct tn48_reset_data *data = to_tn48_reset_data(rcdev);
    unsigned int regval;
    int ret;
    ret = regmap_read(data.regmap, TN48M_RESET_REG, &regval);
    if (ret < 0)
    return ret;
    if (BIT(tn48m_resets[id].bit) & regval)
    return 0;
    else
    return 1;
    }
    static const struct reset_control_ops tn48_reset_ops = {
    .reset		= tn48m_control_reset,
    .status		= tn48m_control_status,
    };
#[no_mangle]
unsafe extern "C" fn tn48m_reset_probe(pdev: *mut platform_device) -> c_int {
    static int tn48m_reset_probe(struct platform_device *pdev)
    {
    struct tn48_reset_data *data;
    struct regmap *regmap;
    regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!regmap)
    return -ENODEV;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.regmap = regmap;
    data.rcdev.owner = THIS_MODULE;
    data.rcdev.ops = &tn48_reset_ops;
    data.rcdev.nr_resets = ARRAY_SIZE(tn48m_resets);
    data.rcdev.of_node = pdev.dev.of_node;
    return devm_reset_controller_register(&pdev.dev, &data.rcdev);
    }
    static const struct of_device_id tn48m_reset_of_match[] = {
    { .compatible = "delta,tn48m-reset" },
    { }
    };
    MODULE_DEVICE_TABLE(of, tn48m_reset_of_match);
    static struct platform_driver tn48m_reset_driver = {
    .driver = {
    .name = "delta-tn48m-reset",
    .of_match_table = tn48m_reset_of_match,
    },
    .probe = tn48m_reset_probe,
    };
    module_platform_driver(tn48m_reset_driver);
    MODULE_AUTHOR("Robert Marko <robert.marko@sartura.hr>");
    MODULE_DESCRIPTION("Delta TN48M CPLD reset driver");
    MODULE_LICENSE("GPL");
