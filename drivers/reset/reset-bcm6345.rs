//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-bcm6345.c
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
// BCM6345 Reset Controller Driver
//
// Copyright (C) 2020 Álvaro Fernández Rojas <noltari@gmail.com>
//

pub const BCM6345_RESET_NUM: c_int = 32;
pub const BCM6345_RESET_SLEEP_MIN_US: c_int = 10000;
pub const BCM6345_RESET_SLEEP_MAX_US: c_int = 20000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm6345_reset {
    pub rcdev: reset_controller_dev,
    pub base: *mut void __iomem,
    pub lock: spinlock_t,
}

    static inline struct bcm6345_reset *
    to_bcm6345_reset(struct reset_controller_dev *rcdev)
    {
    return container_of(rcdev, struct bcm6345_reset, rcdev);
    }
    static int bcm6345_reset_update(struct reset_controller_dev *rcdev,
    unsigned long id, bool assert)
    {
    struct bcm6345_reset *bcm6345_reset = to_bcm6345_reset(rcdev);
    unsigned long flags;
    uint32_t val;
    spin_lock_irqsave(&bcm6345_reset.lock, flags);
    val = __raw_readl(bcm6345_reset.base);
    if (assert)
    val &= ~BIT(id);
    else
    val |= BIT(id);
    __raw_writel(val, bcm6345_reset.base);
    spin_unlock_irqrestore(&bcm6345_reset.lock, flags);
    return 0;
    }
    static int bcm6345_reset_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return bcm6345_reset_update(rcdev, id, true);
    }
    static int bcm6345_reset_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return bcm6345_reset_update(rcdev, id, false);
    }
    static int bcm6345_reset_reset(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    bcm6345_reset_update(rcdev, id, true);
    usleep_range(BCM6345_RESET_SLEEP_MIN_US,
    BCM6345_RESET_SLEEP_MAX_US);
    bcm6345_reset_update(rcdev, id, false);
//
// Ensure component is taken out reset state by sleeping also after
// deasserting the reset. Otherwise, the component may not be ready
// for operation.
//
    usleep_range(BCM6345_RESET_SLEEP_MIN_US,
    BCM6345_RESET_SLEEP_MAX_US);
    return 0;
    }
    static int bcm6345_reset_status(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct bcm6345_reset *bcm6345_reset = to_bcm6345_reset(rcdev);
    return !(__raw_readl(bcm6345_reset.base) & BIT(id));
    }
    static const struct reset_control_ops bcm6345_reset_ops = {
    .assert = bcm6345_reset_assert,
    .deassert = bcm6345_reset_deassert,
    .reset = bcm6345_reset_reset,
    .status = bcm6345_reset_status,
    };
#[no_mangle]
unsafe extern "C" fn bcm6345_reset_probe(pdev: *mut platform_device) -> c_int {
    static int bcm6345_reset_probe(struct platform_device *pdev)
    {
    struct bcm6345_reset *bcm6345_reset;
    bcm6345_reset = devm_kzalloc(&pdev.dev,
    sizeof(*bcm6345_reset), GFP_KERNEL);
    if (!bcm6345_reset)
    return -ENOMEM;
    bcm6345_reset.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(bcm6345_reset.base))
    return PTR_ERR(bcm6345_reset.base);
    spin_lock_init(&bcm6345_reset.lock);
    bcm6345_reset.rcdev.ops = &bcm6345_reset_ops;
    bcm6345_reset.rcdev.owner = THIS_MODULE;
    bcm6345_reset.rcdev.of_node = pdev.dev.of_node;
    bcm6345_reset.rcdev.of_reset_n_cells = 1;
    bcm6345_reset.rcdev.nr_resets = BCM6345_RESET_NUM;
    return devm_reset_controller_register(&pdev.dev,
    &bcm6345_reset.rcdev);
    }
    static const struct of_device_id bcm6345_reset_of_match[] = {
    { .compatible = "brcm,bcm6345-reset" },
    { .compatible = "brcm,bcm63xx-ephy-ctrl" },
    { /* sentinel */ },
    };
    static struct platform_driver bcm6345_reset_driver = {
    .probe = bcm6345_reset_probe,
    .driver	= {
    .name = "bcm6345-reset",
    .of_match_table = bcm6345_reset_of_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver(bcm6345_reset_driver);
