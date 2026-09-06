//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-gate-exclusive.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2014 Freescale Semiconductor, Inc.
//

//
// struct clk_gate_exclusive - i.MX specific gate clock which is mutually
// exclusive with other gate clocks
//
// @gate: the parent class
// @exclusive_mask: mask of gate bits which are mutually exclusive to this
// gate clock
//
// The imx exclusive gate clock is a subclass of basic clk_gate
// with an additional mask to indicate which other gate bits in the same
// register is mutually exclusive to this gate clock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_gate_exclusive {
    pub gate: clk_gate,
    pub exclusive_mask: u32,
}

#[no_mangle]
unsafe extern "C" fn clk_gate_exclusive_enable(hw: *mut clk_hw) -> c_int {
    static int clk_gate_exclusive_enable(struct clk_hw *hw)
    {
    struct clk_gate *gate = to_clk_gate(hw);
    struct clk_gate_exclusive *exgate = container_of(gate,
    struct clk_gate_exclusive, gate);
    let mut val: u32 = readl(gate.reg);
    if (val & exgate.exclusive_mask)
    return -EBUSY;
    return clk_gate_ops.enable(hw);
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_exclusive_disable(hw: *mut clk_hw) {
    static void clk_gate_exclusive_disable(struct clk_hw *hw)
    {
    clk_gate_ops.disable(hw);
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_exclusive_is_enabled(hw: *mut clk_hw) -> c_int {
    static int clk_gate_exclusive_is_enabled(struct clk_hw *hw)
    {
    return clk_gate_ops.is_enabled(hw);
    }
    static const struct clk_ops clk_gate_exclusive_ops = {
    .enable = clk_gate_exclusive_enable,
    .disable = clk_gate_exclusive_disable,
    .is_enabled = clk_gate_exclusive_is_enabled,
    };
    struct clk_hw *imx_clk_hw_gate_exclusive(const char *name, const char *parent,
    void __iomem *reg, u8 shift, u32 exclusive_mask)
    {
    struct clk_gate_exclusive *exgate;
    struct clk_gate *gate;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    if (exclusive_mask == 0)
    return ERR_PTR(-EINVAL);
    exgate = kzalloc_obj(*exgate);
    if (!exgate)
    return ERR_PTR(-ENOMEM);
    gate = &exgate.gate;
    init.name = name;
    init.ops = &clk_gate_exclusive_ops;
    init.flags = CLK_SET_RATE_PARENT;
    init.parent_names = parent ? &parent : core::ptr::null_mut();
    init.num_parents = parent ? 1 : 0;
    gate.reg = reg;
    gate.bit_idx = shift;
    gate.lock = &imx_ccm_lock;
    gate.hw.init = &init;
    exgate.exclusive_mask = exclusive_mask;
    hw = &gate.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    kfree(gate);
    return ERR_PTR(ret);
    }
    return hw;
    }
