//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-composite-7ulp.c
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
// Copyright (C) 2016 Freescale Semiconductor, Inc.
// Copyright 2017~2018 NXP
//

pub const PCG_PCS_SHIFT: c_int = 24;
pub const PCG_PCS_MASK: c_uint = 0x7;
pub const PCG_CGC_SHIFT: c_int = 30;
pub const PCG_FRAC_SHIFT: c_int = 3;
pub const PCG_FRAC_WIDTH: c_int = 1;
pub const PCG_PCD_SHIFT: c_int = 0;
pub const PCG_PCD_WIDTH: c_int = 3;

#[no_mangle]
unsafe extern "C" fn pcc_gate_enable(hw: *mut clk_hw) -> c_int {
    static int pcc_gate_enable(struct clk_hw *hw)
    {
    struct clk_gate *gate = to_clk_gate(hw);
    unsigned long flags;
    u32 val;
    int ret;
    ret = clk_gate_ops.enable(hw);
    if (ret)
    return ret;
// Make sure the IP's clock is ready before release reset
    udelay(1);
    spin_lock_irqsave(gate.lock, flags);
//
// release the sw reset for peripherals associated with
// with this pcc clock.
//
    val = readl(gate.reg);
    val |= SW_RST;
    writel(val, gate.reg);
    spin_unlock_irqrestore(gate.lock, flags);
//
// Read back the register to make sure the previous write has been
// done in the target HW register. For IP like GPU, after deassert
// the reset, need to wait for a while to make sure the sync reset
// is done
//
    readl(gate.reg);
    udelay(1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcc_gate_disable(hw: *mut clk_hw) {
    static void pcc_gate_disable(struct clk_hw *hw)
    {
    clk_gate_ops.disable(hw);
    }
#[no_mangle]
unsafe extern "C" fn pcc_gate_is_enabled(hw: *mut clk_hw) -> c_int {
    static int pcc_gate_is_enabled(struct clk_hw *hw)
    {
    return clk_gate_ops.is_enabled(hw);
    }
    static const struct clk_ops pcc_gate_ops = {
    .enable = pcc_gate_enable,
    .disable = pcc_gate_disable,
    .is_enabled = pcc_gate_is_enabled,
    };
    static struct clk_hw *imx_ulp_clk_hw_composite(const char *name,
    const char * const *parent_names,
    int num_parents, bool mux_present,
    bool rate_present, bool gate_present,
    void __iomem *reg, bool has_swrst)
    {
    struct clk_hw *mux_hw = core::ptr::null_mut(), *fd_hw = core::ptr::null_mut(), *gate_hw = core::ptr::null_mut();
    struct clk_fractional_divider *fd = core::ptr::null_mut();
    struct clk_gate *gate = core::ptr::null_mut();
    struct clk_mux *mux = core::ptr::null_mut();
    struct clk_hw *hw;
    u32 val;
    val = readl(reg);
    if (!(val & PCG_PR_MASK)) {
    pr_info("PCC PR is 0 for clk:%s, bypass\n", name);
    return core::ptr::null_mut();
    }
    if (mux_present) {
    mux = kzalloc_obj(*mux);
    if (!mux)
    return ERR_PTR(-ENOMEM);
    mux_hw = &mux.hw;
    mux.reg = reg;
    mux.shift = PCG_PCS_SHIFT;
    mux.mask = PCG_PCS_MASK;
    if (has_swrst)
    mux.lock = &imx_ccm_lock;
    }
    if (rate_present) {
    fd = kzalloc_obj(*fd);
    if (!fd) {
    kfree(mux);
    return ERR_PTR(-ENOMEM);
    }
    fd_hw = &fd.hw;
    fd.reg = reg;
    fd.mshift = PCG_FRAC_SHIFT;
    fd.mwidth = PCG_FRAC_WIDTH;
    fd.nshift = PCG_PCD_SHIFT;
    fd.nwidth = PCG_PCD_WIDTH;
    fd.flags = CLK_FRAC_DIVIDER_ZERO_BASED;
    if (has_swrst)
    fd.lock = &imx_ccm_lock;
    }
    if (gate_present) {
    gate = kzalloc_obj(*gate);
    if (!gate) {
    kfree(mux);
    kfree(fd);
    return ERR_PTR(-ENOMEM);
    }
    gate_hw = &gate.hw;
    gate.reg = reg;
    gate.bit_idx = PCG_CGC_SHIFT;
    if (has_swrst)
    gate.lock = &imx_ccm_lock;
//
// make sure clock is gated during clock tree initialization,
// the HW ONLY allow clock parent/rate changed with clock gated,
// during clock tree initialization, clocks could be enabled
// by bootloader, so the HW status will mismatch with clock tree
// prepare count, then clock core driver will allow parent/rate
// change since the prepare count is zero, but HW actually
// prevent the parent/rate change due to the clock is enabled.
//
    val = readl_relaxed(reg);
    val &= ~(1 << PCG_CGC_SHIFT);
    writel_relaxed(val, reg);
    }
    hw = clk_hw_register_composite(core::ptr::null_mut(), name, parent_names, num_parents,
    mux_hw, &clk_mux_ops, fd_hw,
    &clk_fractional_divider_ops, gate_hw,
    has_swrst ? &pcc_gate_ops : &clk_gate_ops, CLK_SET_RATE_GATE |
    CLK_SET_PARENT_GATE | CLK_SET_RATE_NO_REPARENT);
    if (IS_ERR(hw)) {
    kfree(mux);
    kfree(fd);
    kfree(gate);
    }
    return hw;
    }
    struct clk_hw *imx7ulp_clk_hw_composite(const char *name, const char * const *parent_names,
    int num_parents, bool mux_present, bool rate_present,
    bool gate_present, void __iomem *reg)
    {
    return imx_ulp_clk_hw_composite(name, parent_names, num_parents, mux_present, rate_present,
    gate_present, reg, false);
    }
    struct clk_hw *imx8ulp_clk_hw_composite(const char *name, const char * const *parent_names,
    int num_parents, bool mux_present, bool rate_present,
    bool gate_present, void __iomem *reg, bool has_swrst)
    {
    return imx_ulp_clk_hw_composite(name, parent_names, num_parents, mux_present, rate_present,
    gate_present, reg, has_swrst);
    }
    EXPORT_SYMBOL_GPL(imx8ulp_clk_hw_composite);
