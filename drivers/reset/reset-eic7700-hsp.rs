//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-eic7700-hsp.c
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
// Copyright 2026, Beijing ESWIN Computing Technology Co., Ltd..
// All rights reserved.
//
// ESWIN EIC7700 HSP Reset Driver
//
// Authors: Xuyang Dong <dongxuyang@eswincomputing.com>
//

//
// struct eic7700_hsp_reset_data - reset controller information structure
// @rcdev: reset controller entity
// @regmap: regmap handle containing the memory-mapped reset registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eic7700_hsp_reset_data {
    pub rcdev: reset_controller_dev,
    pub regmap: *mut regmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eic7700_hsp_reg {
    pub reg: u32,
    pub bit: u32,
    pub active_low: bool,
}

    static inline struct eic7700_hsp_reset_data *
    to_eic7700_hsp_reset(struct reset_controller_dev *rcdev)
    {
    return container_of(rcdev, struct eic7700_hsp_reset_data, rcdev);
    }
    static const struct eic7700_hsp_reg eic7700_hsp_reset[] = {
    [EIC7700_HSP_RST_SATA_P0]	= {0x340, BIT(0), false},
    [EIC7700_HSP_RST_SATA_PHY]	= {0x340, BIT(1), false},
    [EIC7700_HSP_RST_USB0]		= {0x800, BIT(24), true},
    [EIC7700_HSP_RST_USB1]		= {0x900, BIT(24), true},
    [EIC7700_HSP_RST_USB0_PHY]	= {0x800, BIT(25), false},
    [EIC7700_HSP_RST_USB1_PHY]	= {0x900, BIT(25), false},
    };
    static int eic7700_hsp_reset_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct eic7700_hsp_reset_data *data = to_eic7700_hsp_reset(rcdev);
    return regmap_assign_bits(data.regmap, eic7700_hsp_reset[id].reg,
    eic7700_hsp_reset[id].bit,
    !eic7700_hsp_reset[id].active_low);
    }
    static int eic7700_hsp_reset_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct eic7700_hsp_reset_data *data = to_eic7700_hsp_reset(rcdev);
    return regmap_assign_bits(data.regmap, eic7700_hsp_reset[id].reg,
    eic7700_hsp_reset[id].bit,
    eic7700_hsp_reset[id].active_low);
    }
    static const struct reset_control_ops eic7700_hsp_reset_ops = {
    .assert = eic7700_hsp_reset_assert,
    .deassert = eic7700_hsp_reset_deassert,
    };
    static int eic7700_hsp_reset_probe(struct auxiliary_device *adev,
    const struct auxiliary_device_id *id)
    {
    struct eic7700_hsp_reset_data *data;
    struct device *dev = &adev.dev;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!data.regmap)
    return dev_err_probe(dev, -ENODEV, "failed to get regmap!\n");
    data.rcdev.owner = THIS_MODULE;
    data.rcdev.ops = &eic7700_hsp_reset_ops;
    data.rcdev.of_node = dev.parent.of_node;
    data.rcdev.dev = dev;
    data.rcdev.nr_resets = ARRAY_SIZE(eic7700_hsp_reset);
    return devm_reset_controller_register(dev, &data.rcdev);
    }
    static const struct auxiliary_device_id eic7700_hsp_reset_ids[] = {
    { .name = "clk_eic7700_hsp.hsp-reset", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(auxiliary, eic7700_hsp_reset_ids);
    static struct auxiliary_driver eic7700_hsp_reset_driver = {
    .probe	= eic7700_hsp_reset_probe,
    .id_table = eic7700_hsp_reset_ids,
    };
    module_auxiliary_driver(eic7700_hsp_reset_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Xuyang Dong <dongxuyang@eswincomputing.com>");
    MODULE_DESCRIPTION("ESWIN EIC7700 HSP Reset Controller Driver");
