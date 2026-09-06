//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-ath79.c
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
// AR71xx Reset Controller Driver
// Author: Alban Bedel
//
// Copyright (C) 2015 Alban Bedel <albeu@free.fr>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath79_reset {
    pub rcdev: reset_controller_dev,
    pub base: *mut void __iomem,
    pub lock: spinlock_t,
}

pub const FULL_CHIP_RESET: c_int = 24;
    static int ath79_reset_update(struct reset_controller_dev *rcdev,
    unsigned long id, bool assert)
    {
    struct ath79_reset *ath79_reset =
    container_of(rcdev, struct ath79_reset, rcdev);
    unsigned long flags;
    u32 val;
    spin_lock_irqsave(&ath79_reset.lock, flags);
    val = readl(ath79_reset.base);
    if (assert)
    val |= BIT(id);
    else
    val &= ~BIT(id);
    writel(val, ath79_reset.base);
    spin_unlock_irqrestore(&ath79_reset.lock, flags);
    return 0;
    }
    static int ath79_reset_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return ath79_reset_update(rcdev, id, true);
    }
    static int ath79_reset_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return ath79_reset_update(rcdev, id, false);
    }
    static int ath79_reset_status(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct ath79_reset *ath79_reset =
    container_of(rcdev, struct ath79_reset, rcdev);
    u32 val;
    val = readl(ath79_reset.base);
    return !!(val & BIT(id));
    }
    static const struct reset_control_ops ath79_reset_ops = {
    .assert = ath79_reset_assert,
    .deassert = ath79_reset_deassert,
    .status = ath79_reset_status,
    };
#[no_mangle]
unsafe extern "C" fn ath79_reset_restart_handler(data: *mut sys_off_data) -> c_int {
    static int ath79_reset_restart_handler(struct sys_off_data *data)
    {
    struct ath79_reset *ath79_reset = data.cb_data;
    ath79_reset_assert(&ath79_reset.rcdev, FULL_CHIP_RESET);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn ath79_reset_probe(pdev: *mut platform_device) -> c_int {
    static int ath79_reset_probe(struct platform_device *pdev)
    {
    struct ath79_reset *ath79_reset;
    int err;
    ath79_reset = devm_kzalloc(&pdev.dev,
    sizeof(*ath79_reset), GFP_KERNEL);
    if (!ath79_reset)
    return -ENOMEM;
    ath79_reset.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ath79_reset.base))
    return PTR_ERR(ath79_reset.base);
    spin_lock_init(&ath79_reset.lock);
    ath79_reset.rcdev.ops = &ath79_reset_ops;
    ath79_reset.rcdev.owner = THIS_MODULE;
    ath79_reset.rcdev.of_node = pdev.dev.of_node;
    ath79_reset.rcdev.of_reset_n_cells = 1;
    ath79_reset.rcdev.nr_resets = 32;
    err = devm_reset_controller_register(&pdev.dev, &ath79_reset.rcdev);
    if (err)
    return err;
    err = devm_register_restart_handler(&pdev.dev, ath79_reset_restart_handler, ath79_reset);
    if (err)
    dev_warn(&pdev.dev, "Failed to register restart handler\n");
    return 0;
    }
    static const struct of_device_id ath79_reset_dt_ids[] = {
    { .compatible = "qca,ar7100-reset", },
    { },
    };
    static struct platform_driver ath79_reset_driver = {
    .probe	= ath79_reset_probe,
    .driver = {
    .name			= "ath79-reset",
    .of_match_table		= ath79_reset_dt_ids,
    .suppress_bind_attrs	= true,
    },
    };
    builtin_platform_driver(ath79_reset_driver);
