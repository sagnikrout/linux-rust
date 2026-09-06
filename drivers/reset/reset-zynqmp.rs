//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-zynqmp.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2018 Xilinx, Inc.
//

pub const VERSAL_NR_RESETS: c_int = 95;
pub const VERSAL_NET_NR_RESETS: c_int = 176;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynqmp_reset_soc_data {
    pub reset_id: u32,
    pub num_resets: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynqmp_reset_data {
    pub rcdev: reset_controller_dev,
    pub data: *const zynqmp_reset_soc_data,
}

    static inline struct zynqmp_reset_data *
    to_zynqmp_reset_data(struct reset_controller_dev *rcdev)
    {
    return container_of(rcdev, struct zynqmp_reset_data, rcdev);
    }
    static int zynqmp_reset_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct zynqmp_reset_data *priv = to_zynqmp_reset_data(rcdev);
    return zynqmp_pm_reset_assert(priv.data.reset_id + id,
    PM_RESET_ACTION_ASSERT);
    }
    static int zynqmp_reset_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct zynqmp_reset_data *priv = to_zynqmp_reset_data(rcdev);
    return zynqmp_pm_reset_assert(priv.data.reset_id + id,
    PM_RESET_ACTION_RELEASE);
    }
    static int zynqmp_reset_status(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct zynqmp_reset_data *priv = to_zynqmp_reset_data(rcdev);
    int err;
    u32 val;
    err = zynqmp_pm_reset_get_status(priv.data.reset_id + id, &val);
    if (err)
    return err;
    return val;
    }
    static int zynqmp_reset_reset(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct zynqmp_reset_data *priv = to_zynqmp_reset_data(rcdev);
    return zynqmp_pm_reset_assert(priv.data.reset_id + id,
    PM_RESET_ACTION_PULSE);
    }
    static int zynqmp_reset_of_xlate(struct reset_controller_dev *rcdev,
    const struct of_phandle_args *reset_spec)
    {
    return reset_spec.args[0];
    }
    static const struct zynqmp_reset_soc_data zynqmp_reset_data = {
    .reset_id = ZYNQMP_RESET_ID,
    .num_resets = ZYNQMP_NR_RESETS,
    };
    static const struct zynqmp_reset_soc_data versal_reset_data = {
    .reset_id = 0,
    .num_resets = VERSAL_NR_RESETS,
    };
    static const struct zynqmp_reset_soc_data versal_net_reset_data = {
    .reset_id = 0,
    .num_resets = VERSAL_NET_NR_RESETS,
    };
    static const struct reset_control_ops zynqmp_reset_ops = {
    .reset = zynqmp_reset_reset,
    .assert = zynqmp_reset_assert,
    .deassert = zynqmp_reset_deassert,
    .status = zynqmp_reset_status,
    };
#[no_mangle]
unsafe extern "C" fn zynqmp_reset_probe(pdev: *mut platform_device) -> c_int {
    static int zynqmp_reset_probe(struct platform_device *pdev)
    {
    struct zynqmp_reset_data *priv;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.data = of_device_get_match_data(&pdev.dev);
    if (!priv.data)
    return -EINVAL;
    priv.rcdev.ops = &zynqmp_reset_ops;
    priv.rcdev.owner = THIS_MODULE;
    priv.rcdev.of_node = pdev.dev.of_node;
    priv.rcdev.nr_resets = priv.data.num_resets;
    priv.rcdev.of_reset_n_cells = 1;
    priv.rcdev.of_xlate = zynqmp_reset_of_xlate;
    return devm_reset_controller_register(&pdev.dev, &priv.rcdev);
    }
    static const struct of_device_id zynqmp_reset_dt_ids[] = {
    { .compatible = "xlnx,zynqmp-reset", .data = &zynqmp_reset_data, },
    { .compatible = "xlnx,versal-reset", .data = &versal_reset_data, },
    { .compatible = "xlnx,versal-net-reset", .data = &versal_net_reset_data, },
    { /* sentinel */ },
    };
    static struct platform_driver zynqmp_reset_driver = {
    .probe	= zynqmp_reset_probe,
    .driver = {
    .name		= KBUILD_MODNAME,
    .of_match_table	= zynqmp_reset_dt_ids,
    },
    };
#[no_mangle]
unsafe extern "C" fn zynqmp_reset_init() -> int __init {
    static int __init zynqmp_reset_init(void)
    {
    return platform_driver_register(&zynqmp_reset_driver);
    }
    arch_initcall(zynqmp_reset_init);
