//! Automatically rewritten from C to Rust
//! Source: drivers/clk/rockchip/softrst.c
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
// Copyright (c) 2014 MundoReader S.L.
// Author: Heiko Stuebner <heiko@sntech.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_softrst {
    pub rcdev: reset_controller_dev,
    pub lut: *const c_int,
    pub reg_base: *mut void __iomem,
    pub num_regs: c_int,
    pub num_per_reg: c_int,
    pub flags: u8,
    pub lock: spinlock_t,
}

    static int rockchip_softrst_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct rockchip_softrst *softrst = container_of(rcdev,
    struct rockchip_softrst,
    rcdev);
    int bank, offset;
    if (softrst.lut)
    id = softrst.lut[id];
    bank = id / softrst.num_per_reg;
    offset = id % softrst.num_per_reg;
    if (softrst.flags & ROCKCHIP_SOFTRST_HIWORD_MASK) {
    writel(BIT(offset) | (BIT(offset) << 16),
    softrst.reg_base + (bank * 4));
    } else {
    unsigned long flags;
    u32 reg;
    spin_lock_irqsave(&softrst.lock, flags);
    reg = readl(softrst.reg_base + (bank * 4));
    writel(reg | BIT(offset), softrst.reg_base + (bank * 4));
    spin_unlock_irqrestore(&softrst.lock, flags);
    }
    return 0;
    }
    static int rockchip_softrst_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct rockchip_softrst *softrst = container_of(rcdev,
    struct rockchip_softrst,
    rcdev);
    int bank, offset;
    if (softrst.lut)
    id = softrst.lut[id];
    bank = id / softrst.num_per_reg;
    offset = id % softrst.num_per_reg;
    if (softrst.flags & ROCKCHIP_SOFTRST_HIWORD_MASK) {
    writel((BIT(offset) << 16), softrst.reg_base + (bank * 4));
    } else {
    unsigned long flags;
    u32 reg;
    spin_lock_irqsave(&softrst.lock, flags);
    reg = readl(softrst.reg_base + (bank * 4));
    writel(reg & ~BIT(offset), softrst.reg_base + (bank * 4));
    spin_unlock_irqrestore(&softrst.lock, flags);
    }
    return 0;
    }
    static const struct reset_control_ops rockchip_softrst_ops = {
    .assert		= rockchip_softrst_assert,
    .deassert	= rockchip_softrst_deassert,
    };
    void rockchip_register_softrst_lut(struct device_node *np,
    const int *lookup_table,
    unsigned int num_regs,
    void __iomem *base, u8 flags)
    {
    struct rockchip_softrst *softrst;
    int ret;
    softrst = kzalloc_obj(*softrst);
    if (!softrst)
    return;
    spin_lock_init(&softrst.lock);
    softrst.reg_base = base;
    softrst.lut = lookup_table;
    softrst.flags = flags;
    softrst.num_regs = num_regs;
    softrst.num_per_reg = (flags & ROCKCHIP_SOFTRST_HIWORD_MASK) ? 16
    : 32;
    softrst.rcdev.owner = THIS_MODULE;
    if (lookup_table)
    softrst.rcdev.nr_resets = num_regs;
    else
    softrst.rcdev.nr_resets = num_regs * softrst.num_per_reg;
    softrst.rcdev.ops = &rockchip_softrst_ops;
    softrst.rcdev.of_node = np;
    ret = reset_controller_register(&softrst.rcdev);
    if (ret) {
    pr_err("%s: could not register reset controller, %d\n",
    __func__, ret);
    kfree(softrst);
    }
    };
    EXPORT_SYMBOL_GPL(rockchip_register_softrst_lut);
