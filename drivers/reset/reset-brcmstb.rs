//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-brcmstb.c
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
// Broadcom STB generic reset controller for SW_INIT style reset controller
//
// Author: Florian Fainelli <f.fainelli@gmail.com>
// Copyright (C) 2018 Broadcom
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmstb_reset {
    pub base: *mut void __iomem,
    pub rcdev: reset_controller_dev,
}

pub const SW_INIT_SET: c_uint = 0x00;
pub const SW_INIT_CLEAR: c_uint = 0x04;
pub const SW_INIT_STATUS: c_uint = 0x08;

// A full bank contains extra registers that we are not utilizing but still
// qualify as a single bank.
//
pub const SW_INIT_BANK_SIZE: c_uint = 0x18;
    static inline
    struct brcmstb_reset *to_brcmstb(struct reset_controller_dev *rcdev)
    {
    return container_of(rcdev, struct brcmstb_reset, rcdev);
    }
    static int brcmstb_reset_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    let mut off: c_uint = SW_INIT_BANK(id) * SW_INIT_BANK_SIZE;
    struct brcmstb_reset *priv = to_brcmstb(rcdev);
    writel_relaxed(SW_INIT_BIT(id), priv.base + off + SW_INIT_SET);
    return 0;
    }
    static int brcmstb_reset_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    let mut off: c_uint = SW_INIT_BANK(id) * SW_INIT_BANK_SIZE;
    struct brcmstb_reset *priv = to_brcmstb(rcdev);
    writel_relaxed(SW_INIT_BIT(id), priv.base + off + SW_INIT_CLEAR);
// Maximum reset delay after de-asserting a line and seeing block
// operation is typically 14us for the worst case, build some slack
// here.
//
    usleep_range(100, 200);
    return 0;
    }
    static int brcmstb_reset_status(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    let mut off: c_uint = SW_INIT_BANK(id) * SW_INIT_BANK_SIZE;
    struct brcmstb_reset *priv = to_brcmstb(rcdev);
    return readl_relaxed(priv.base + off + SW_INIT_STATUS) &
    SW_INIT_BIT(id);
    }
    static const struct reset_control_ops brcmstb_reset_ops = {
    .assert	= brcmstb_reset_assert,
    .deassert = brcmstb_reset_deassert,
    .status = brcmstb_reset_status,
    };
#[no_mangle]
unsafe extern "C" fn brcmstb_reset_probe(pdev: *mut platform_device) -> c_int {
    static int brcmstb_reset_probe(struct platform_device *pdev)
    {
    struct device *kdev = &pdev.dev;
    struct brcmstb_reset *priv;
    struct resource *res;
    priv = devm_kzalloc(kdev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    dev_set_drvdata(kdev, priv);
    priv.rcdev.owner = THIS_MODULE;
    priv.rcdev.nr_resets = DIV_ROUND_DOWN_ULL(resource_size(res),
    SW_INIT_BANK_SIZE) * 32;
    priv.rcdev.ops = &brcmstb_reset_ops;
    priv.rcdev.of_node = kdev.of_node;
// Use defaults: 1 cell and simple xlate function
    return devm_reset_controller_register(kdev, &priv.rcdev);
    }
    static const struct of_device_id brcmstb_reset_of_match[] = {
    { .compatible = "brcm,brcmstb-reset" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, brcmstb_reset_of_match);
    static struct platform_driver brcmstb_reset_driver = {
    .probe	= brcmstb_reset_probe,
    .driver	= {
    .name = "brcmstb-reset",
    .of_match_table = brcmstb_reset_of_match,
    },
    };
    module_platform_driver(brcmstb_reset_driver);
    MODULE_AUTHOR("Broadcom");
    MODULE_DESCRIPTION("Broadcom STB reset controller");
    MODULE_LICENSE("GPL");
