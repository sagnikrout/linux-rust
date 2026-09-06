//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-gate-93.c
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
// Copyright 2022 NXP
//
// Peng Fan <peng.fan@nxp.com>
//

pub const DIRECT_OFFSET: c_uint = 0x0;
//
// 0b000 - LPCG will be OFF in any CPU mode.
// 0b100 - LPCG will be ON in any CPU mode.
//
pub const LPM_SETTING_OFF: c_uint = 0x0;
pub const LPM_SETTING_ON: c_uint = 0x4;
pub const LPM_CUR_OFFSET: c_uint = 0x1c;
pub const AUTHEN_OFFSET: c_uint = 0x30;

pub const TZ_NS_SHIFT: c_int = 9;

pub const WHITE_LIST_SHIFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx93_clk_gate {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub bit_idx: u32,
    pub val: u32,
    pub mask: u32,
    pub lock: *mut spinlock_t,
    pub share_count: *mut c_uint,
}

#[no_mangle]
unsafe extern "C" fn imx93_clk_gate_do_hardware(hw: *mut clk_hw, enable: bool) {
    static void imx93_clk_gate_do_hardware(struct clk_hw *hw, bool enable)
    {
    struct imx93_clk_gate *gate = to_imx93_clk_gate(hw);
    u32 val;
    val = readl(gate.reg + AUTHEN_OFFSET);
    if (val & CPULPM_EN) {
    val = enable ? LPM_SETTING_ON : LPM_SETTING_OFF;
    writel(val, gate.reg + LPM_CUR_OFFSET);
    } else {
    val = readl(gate.reg + DIRECT_OFFSET);
    val &= ~(gate.mask << gate.bit_idx);
    if (enable)
    val |= (gate.val & gate.mask) << gate.bit_idx;
    writel(val, gate.reg + DIRECT_OFFSET);
    }
    }
#[no_mangle]
unsafe extern "C" fn imx93_clk_gate_enable(hw: *mut clk_hw) -> c_int {
    static int imx93_clk_gate_enable(struct clk_hw *hw)
    {
    struct imx93_clk_gate *gate = to_imx93_clk_gate(hw);
    unsigned long flags;
    spin_lock_irqsave(gate.lock, flags);
    if (gate.share_count && (*gate.share_count)++ > 0)
    goto out;
    imx93_clk_gate_do_hardware(hw, true);
    out:
    spin_unlock_irqrestore(gate.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx93_clk_gate_disable(hw: *mut clk_hw) {
    static void imx93_clk_gate_disable(struct clk_hw *hw)
    {
    struct imx93_clk_gate *gate = to_imx93_clk_gate(hw);
    unsigned long flags;
    spin_lock_irqsave(gate.lock, flags);
    if (gate.share_count) {
    if (WARN_ON(*gate.share_count == 0))
    goto out;
#[no_mangle]
pub unsafe extern "C" fn if(0: *mut *mut --(gate->share_count) >) -> else {
    else if (--(*gate.share_count) > 0)
    goto out;
    }
    imx93_clk_gate_do_hardware(hw, false);
    out:
    spin_unlock_irqrestore(gate.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn imx93_clk_gate_reg_is_enabled(gate: *mut imx93_clk_gate) -> c_int {
    static int imx93_clk_gate_reg_is_enabled(struct imx93_clk_gate *gate)
    {
    let mut val: u32 = readl(gate.reg + AUTHEN_OFFSET);
    if (val & CPULPM_EN) {
    val = readl(gate.reg + LPM_CUR_OFFSET);
    if (val == LPM_SETTING_ON)
    return 1;
    } else {
    val = readl(gate.reg);
    if (((val >> gate.bit_idx) & gate.mask) == gate.val)
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx93_clk_gate_is_enabled(hw: *mut clk_hw) -> c_int {
    static int imx93_clk_gate_is_enabled(struct clk_hw *hw)
    {
    struct imx93_clk_gate *gate = to_imx93_clk_gate(hw);
    unsigned long flags;
    int ret;
    spin_lock_irqsave(gate.lock, flags);
    ret = imx93_clk_gate_reg_is_enabled(gate);
    spin_unlock_irqrestore(gate.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx93_clk_gate_disable_unused(hw: *mut clk_hw) {
    static void imx93_clk_gate_disable_unused(struct clk_hw *hw)
    {
    struct imx93_clk_gate *gate = to_imx93_clk_gate(hw);
    unsigned long flags;
    spin_lock_irqsave(gate.lock, flags);
    if (!gate.share_count || *gate.share_count == 0)
    imx93_clk_gate_do_hardware(hw, false);
    spin_unlock_irqrestore(gate.lock, flags);
    }
    static const struct clk_ops imx93_clk_gate_ops = {
    .enable = imx93_clk_gate_enable,
    .disable = imx93_clk_gate_disable,
    .disable_unused = imx93_clk_gate_disable_unused,
    .is_enabled = imx93_clk_gate_is_enabled,
    };
    static const struct clk_ops imx93_clk_gate_ro_ops = {
    .is_enabled = imx93_clk_gate_is_enabled,
    };
    struct clk_hw *imx93_clk_gate(struct device *dev, const char *name, const char *parent_name,
    unsigned long flags, void __iomem *reg, u32 bit_idx, u32 val,
    u32 mask, u32 domain_id, unsigned int *share_count)
    {
    struct imx93_clk_gate *gate;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    u32 authen;
    gate = kzalloc_obj(struct imx93_clk_gate);
    if (!gate)
    return ERR_PTR(-ENOMEM);
    gate.reg = reg;
    gate.lock = &imx_ccm_lock;
    gate.bit_idx = bit_idx;
    gate.val = val;
    gate.mask = mask;
    gate.share_count = share_count;
    init.name = name;
    init.ops = &imx93_clk_gate_ops;
    init.flags = flags | CLK_SET_RATE_PARENT | CLK_OPS_PARENT_ENABLE;
    init.parent_names = parent_name ? &parent_name : core::ptr::null_mut();
    init.num_parents = parent_name ? 1 : 0;
    gate.hw.init = &init;
    hw = &gate.hw;
    authen = readl(reg + AUTHEN_OFFSET);
    if (!(authen & TZ_NS_MASK) || !(authen & BIT(WHITE_LIST_SHIFT + domain_id)))
    init.ops = &imx93_clk_gate_ro_ops;
    ret = clk_hw_register(dev, hw);
    if (ret) {
    kfree(gate);
    return ERR_PTR(ret);
    }
    return hw;
    }
    EXPORT_SYMBOL_GPL(imx93_clk_gate);
