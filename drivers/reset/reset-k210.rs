//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-k210.c
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
// Copyright (c) 2020 Western Digital Corporation or its affiliates.
//

pub const K210_RST_MASK: c_uint = 0x27FFFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k210_rst {
    pub map: *mut regmap,
    pub rcdev: reset_controller_dev,
}

    static inline struct k210_rst *
    to_k210_rst(struct reset_controller_dev *rcdev)
    {
    return container_of(rcdev, struct k210_rst, rcdev);
    }
    static inline int k210_rst_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct k210_rst *ksr = to_k210_rst(rcdev);
    return regmap_update_bits(ksr.map, K210_SYSCTL_PERI_RESET, BIT(id), 1);
    }
    static inline int k210_rst_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct k210_rst *ksr = to_k210_rst(rcdev);
    return regmap_update_bits(ksr.map, K210_SYSCTL_PERI_RESET, BIT(id), 0);
    }
    static int k210_rst_reset(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    int ret;
    ret = k210_rst_assert(rcdev, id);
    if (ret == 0) {
    udelay(10);
    ret = k210_rst_deassert(rcdev, id);
    }
    return ret;
    }
    static int k210_rst_status(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct k210_rst *ksr = to_k210_rst(rcdev);
    u32 reg, bit = BIT(id);
    int ret;
    ret = regmap_read(ksr.map, K210_SYSCTL_PERI_RESET, &reg);
    if (ret)
    return ret;
    return reg & bit;
    }
    static int k210_rst_xlate(struct reset_controller_dev *rcdev,
    const struct of_phandle_args *reset_spec)
    {
    let mut id: c_ulong = reset_spec.args[0];
    if (!(BIT(id) & K210_RST_MASK))
    return -EINVAL;
    return id;
    }
    static const struct reset_control_ops k210_rst_ops = {
    .assert		= k210_rst_assert,
    .deassert	= k210_rst_deassert,
    .reset		= k210_rst_reset,
    .status		= k210_rst_status,
    };
#[no_mangle]
unsafe extern "C" fn k210_rst_probe(pdev: *mut platform_device) -> c_int {
    static int k210_rst_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *parent_np;
    struct k210_rst *ksr;
    dev_info(dev, "K210 reset controller\n");
    ksr = devm_kzalloc(dev, sizeof(*ksr), GFP_KERNEL);
    if (!ksr)
    return -ENOMEM;
    parent_np = of_get_parent(dev.of_node);
    ksr.map = syscon_node_to_regmap(parent_np);
    of_node_put(parent_np);
    if (IS_ERR(ksr.map))
    return PTR_ERR(ksr.map);
    ksr.rcdev.owner = THIS_MODULE;
    ksr.rcdev.dev = dev;
    ksr.rcdev.of_node = dev.of_node;
    ksr.rcdev.ops = &k210_rst_ops;
    ksr.rcdev.nr_resets = fls(K210_RST_MASK);
    ksr.rcdev.of_reset_n_cells = 1;
    ksr.rcdev.of_xlate = k210_rst_xlate;
    return devm_reset_controller_register(dev, &ksr.rcdev);
    }
    static const struct of_device_id k210_rst_dt_ids[] = {
    { .compatible = "canaan,k210-rst" },
    { /* sentinel */ },
    };
    static struct platform_driver k210_rst_driver = {
    .probe	= k210_rst_probe,
    .driver = {
    .name		= "k210-rst",
    .of_match_table	= k210_rst_dt_ids,
    },
    };
    builtin_platform_driver(k210_rst_driver);
