//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-gate.c
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
// Copyright (C) 2010-2011 Canonical Ltd <jeremy.kerr@canonical.com>
// Copyright (C) 2011-2012 Mike Turquette, Linaro Ltd <mturquette@linaro.org>
//
// Gated clock implementation
//

//
// DOC: basic gateable clock which can gate and ungate its output
//
// Traits of this clock:
// prepare - clk_(un)prepare only ensures parent is (un)prepared
// enable - clk_enable and clk_disable are functional & control gating
// rate - inherits rate from parent.  No clk_set_rate support
// parent - fixed parent.  No clk_set_parent support
//
#[no_mangle]
pub unsafe extern "C" fn clk_gate_readl(gate: *mut clk_gate) -> u32 {
    static inline u32 clk_gate_readl(struct clk_gate *gate)
    {
    if (gate.flags & CLK_GATE_BIG_ENDIAN)
    return ioread32be(gate.reg);
    return readl(gate.reg);
    }
#[no_mangle]
pub unsafe extern "C" fn clk_gate_writel(gate: *mut clk_gate, val: u32) {
    static inline void clk_gate_writel(struct clk_gate *gate, u32 val)
    {
    if (gate.flags & CLK_GATE_BIG_ENDIAN)
    iowrite32be(val, gate.reg);
    else
    writel(val, gate.reg);
    }
//
// It works on following logic:
//
// For enabling clock, enable = 1
// set2dis = 1	-> clear bit	-> set = 0
// set2dis = 0	-> set bit	-> set = 1
//
// For disabling clock, enable = 0
// set2dis = 1	-> set bit	-> set = 1
// set2dis = 0	-> clear bit	-> set = 0
//
// So, result is always: enable xor set2dis.
//
#[no_mangle]
unsafe extern "C" fn clk_gate_endisable(hw: *mut clk_hw, enable: c_int) {
    static void clk_gate_endisable(struct clk_hw *hw, int enable)
    {
    struct clk_gate *gate = to_clk_gate(hw);
    let mut set: c_int = gate.flags & CLK_GATE_SET_TO_DISABLE ? 1 : 0;
    unsigned long flags;
    u32 reg;
    set ^= enable;
    if (gate.lock)
    spin_lock_irqsave(gate.lock, flags);
    else
    __acquire(gate.lock);
    if (gate.flags & CLK_GATE_HIWORD_MASK) {
    reg = BIT(gate.bit_idx + 16);
    if (set)
    reg |= BIT(gate.bit_idx);
    } else {
    reg = clk_gate_readl(gate);
    if (set)
    reg |= BIT(gate.bit_idx);
    else
    reg &= ~BIT(gate.bit_idx);
    }
    clk_gate_writel(gate, reg);
    if (gate.lock)
    spin_unlock_irqrestore(gate.lock, flags);
    else
    __release(gate.lock);
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_enable(hw: *mut clk_hw) -> c_int {
    static int clk_gate_enable(struct clk_hw *hw)
    {
    clk_gate_endisable(hw, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_disable(hw: *mut clk_hw) {
    static void clk_gate_disable(struct clk_hw *hw)
    {
    clk_gate_endisable(hw, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn clk_gate_is_enabled(hw: *mut clk_hw) -> c_int {
    int clk_gate_is_enabled(struct clk_hw *hw)
    {
    u32 reg;
    struct clk_gate *gate = to_clk_gate(hw);
    reg = clk_gate_readl(gate);
// if a set bit disables this clk, flip it before masking
    if (gate.flags & CLK_GATE_SET_TO_DISABLE)
    reg ^= BIT(gate.bit_idx);
    reg &= BIT(gate.bit_idx);
    return reg ? 1 : 0;
    }
    EXPORT_SYMBOL_GPL(clk_gate_is_enabled);
    const struct clk_ops clk_gate_ops = {
    .enable = clk_gate_enable,
    .disable = clk_gate_disable,
    .is_enabled = clk_gate_is_enabled,
    };
    EXPORT_SYMBOL_GPL(clk_gate_ops);
    struct clk_hw *__clk_hw_register_gate(struct device *dev,
    struct device_node *np, const char *name,
    const char *parent_name, const struct clk_hw *parent_hw,
    const struct clk_parent_data *parent_data,
    unsigned long flags,
    void __iomem *reg, u8 bit_idx,
    u8 clk_gate_flags, spinlock_t *lock)
    {
    struct clk_gate *gate;
    struct clk_hw *hw;
    let mut init: clk_init_data = {};
    let mut ret: c_int = -EINVAL;
    if (clk_gate_flags & CLK_GATE_HIWORD_MASK) {
    if (bit_idx > 15) {
    pr_err("gate bit exceeds LOWORD field\n");
    return ERR_PTR(-EINVAL);
    }
    }
// allocate the gate
    gate = kzalloc_obj(*gate);
    if (!gate)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &clk_gate_ops;
    init.flags = flags;
    init.parent_names = parent_name ? &parent_name : core::ptr::null_mut();
    init.parent_hws = parent_hw ? &parent_hw : core::ptr::null_mut();
    init.parent_data = parent_data;
    if (parent_name || parent_hw || parent_data)
    init.num_parents = 1;
    else
    init.num_parents = 0;
// struct clk_gate assignments
    gate.reg = reg;
    gate.bit_idx = bit_idx;
    gate.flags = clk_gate_flags;
    gate.lock = lock;
    gate.hw.init = &init;
    hw = &gate.hw;
    if (dev || !np)
    ret = clk_hw_register(dev, hw);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: np) -> else {
    else if (np)
    ret = of_clk_hw_register(np, hw);
    if (ret) {
    kfree(gate);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
    EXPORT_SYMBOL_GPL(__clk_hw_register_gate);
    struct clk *clk_register_gate(struct device *dev, const char *name,
    const char *parent_name, unsigned long flags,
    void __iomem *reg, u8 bit_idx,
    u8 clk_gate_flags, spinlock_t *lock)
    {
    struct clk_hw *hw;
    hw = clk_hw_register_gate(dev, name, parent_name, flags, reg,
    bit_idx, clk_gate_flags, lock);
    if (IS_ERR(hw))
    return ERR_CAST(hw);
    return hw.clk;
    }
    EXPORT_SYMBOL_GPL(clk_register_gate);
#[no_mangle]
pub unsafe extern "C" fn clk_unregister_gate(clk: *mut clk) {
    void clk_unregister_gate(struct clk *clk)
    {
    struct clk_gate *gate;
    struct clk_hw *hw;
    hw = __clk_get_hw(clk);
    if (!hw)
    return;
    gate = to_clk_gate(hw);
    clk_unregister(clk);
    kfree(gate);
    }
    EXPORT_SYMBOL_GPL(clk_unregister_gate);
#[no_mangle]
pub unsafe extern "C" fn clk_hw_unregister_gate(hw: *mut clk_hw) {
    void clk_hw_unregister_gate(struct clk_hw *hw)
    {
    struct clk_gate *gate;
    gate = to_clk_gate(hw);
    clk_hw_unregister(hw);
    kfree(gate);
    }
    EXPORT_SYMBOL_GPL(clk_hw_unregister_gate);
#[no_mangle]
unsafe extern "C" fn devm_clk_hw_release_gate(dev: *mut device, res: *mut c_void) {
    static void devm_clk_hw_release_gate(struct device *dev, void *res)
    {
    clk_hw_unregister_gate(*(struct clk_hw **)res);
    }
    struct clk_hw *__devm_clk_hw_register_gate(struct device *dev,
    struct device_node *np, const char *name,
    const char *parent_name, const struct clk_hw *parent_hw,
    const struct clk_parent_data *parent_data,
    unsigned long flags,
    void __iomem *reg, u8 bit_idx,
    u8 clk_gate_flags, spinlock_t *lock)
    {
    struct clk_hw **ptr, *hw;
    ptr = devres_alloc(devm_clk_hw_release_gate, sizeof(*ptr), GFP_KERNEL);
    if (!ptr)
    return ERR_PTR(-ENOMEM);
    hw = __clk_hw_register_gate(dev, np, name, parent_name, parent_hw,
    parent_data, flags, reg, bit_idx,
    clk_gate_flags, lock);
    if (!IS_ERR(hw)) {
// ptr = hw;
    devres_add(dev, ptr);
    } else {
    devres_free(ptr);
    }
    return hw;
    }
    EXPORT_SYMBOL_GPL(__devm_clk_hw_register_gate);
