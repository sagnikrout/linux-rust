//! Automatically rewritten from C to Rust
//! Source: drivers/clk/rockchip/clk-inverter.c
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
// Copyright 2015 Heiko Stuebner <heiko@sntech.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_inv_clock {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub shift: c_int,
    pub flags: c_int,
    pub lock: *mut spinlock_t,
}

pub const INVERTER_MASK: c_uint = 0x1;
#[no_mangle]
unsafe extern "C" fn rockchip_inv_get_phase(hw: *mut clk_hw) -> c_int {
    static int rockchip_inv_get_phase(struct clk_hw *hw)
    {
    struct rockchip_inv_clock *inv_clock = to_inv_clock(hw);
    u32 val;
    val = readl(inv_clock.reg) >> inv_clock.shift;
    val &= INVERTER_MASK;
    return val ? 180 : 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_inv_set_phase(hw: *mut clk_hw, degrees: c_int) -> c_int {
    static int rockchip_inv_set_phase(struct clk_hw *hw, int degrees)
    {
    struct rockchip_inv_clock *inv_clock = to_inv_clock(hw);
    u32 val;
    if (degrees % 180 == 0) {
    val = !!degrees;
    } else {
    pr_err("%s: unsupported phase %d for %s\n",
    __func__, degrees, clk_hw_get_name(hw));
    return -EINVAL;
    }
    if (inv_clock.flags & ROCKCHIP_INVERTER_HIWORD_MASK) {
    writel(HIWORD_UPDATE(val, INVERTER_MASK, inv_clock.shift),
    inv_clock.reg);
    } else {
    unsigned long flags;
    u32 reg;
    spin_lock_irqsave(inv_clock.lock, flags);
    reg = readl(inv_clock.reg);
    reg &= ~BIT(inv_clock.shift);
    reg |= val;
    writel(reg, inv_clock.reg);
    spin_unlock_irqrestore(inv_clock.lock, flags);
    }
    return 0;
    }
    static const struct clk_ops rockchip_inv_clk_ops = {
    .get_phase	= rockchip_inv_get_phase,
    .set_phase	= rockchip_inv_set_phase,
    };
    struct clk *rockchip_clk_register_inverter(const char *name,
    const char *const *parent_names, u8 num_parents,
    void __iomem *reg, int shift, int flags,
    spinlock_t *lock)
    {
    struct clk_init_data init;
    struct rockchip_inv_clock *inv_clock;
    struct clk *clk;
    inv_clock = kmalloc_obj(*inv_clock);
    if (!inv_clock)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.num_parents = num_parents;
    init.flags = CLK_SET_RATE_PARENT;
    init.parent_names = parent_names;
    init.ops = &rockchip_inv_clk_ops;
    inv_clock.hw.init = &init;
    inv_clock.reg = reg;
    inv_clock.shift = shift;
    inv_clock.flags = flags;
    inv_clock.lock = lock;
    clk = clk_register(core::ptr::null_mut(), &inv_clock.hw);
    if (IS_ERR(clk))
    kfree(inv_clock);
    return clk;
    }
