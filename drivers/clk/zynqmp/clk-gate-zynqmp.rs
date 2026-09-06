//! Automatically rewritten from C to Rust
//! Source: drivers/clk/zynqmp/clk-gate-zynqmp.c
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
// Zynq UltraScale+ MPSoC clock controller
//
// Copyright (C) 2016-2018 Xilinx
//
// Gated clock implementation
//

//
// struct zynqmp_clk_gate - gating clock
// @hw:		handle between common and hardware-specific interfaces
// @flags:	hardware-specific flags
// @clk_id:	Id of clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynqmp_clk_gate {
    pub hw: clk_hw,
    pub flags: u8,
    pub clk_id: u32,
}

//
// zynqmp_clk_gate_enable() - Enable clock
// @hw:		handle between common and hardware-specific interfaces
//
// Return: 0 on success else error code
//
#[no_mangle]
unsafe extern "C" fn zynqmp_clk_gate_enable(hw: *mut clk_hw) -> c_int {
    static int zynqmp_clk_gate_enable(struct clk_hw *hw)
    {
    struct zynqmp_clk_gate *gate = to_zynqmp_clk_gate(hw);
    const char *clk_name = clk_hw_get_name(hw);
    let mut clk_id: u32 = gate.clk_id;
    int ret;
    ret = zynqmp_pm_clock_enable(clk_id);
    if (ret)
    pr_debug("%s() clock enable failed for %s (id %d), ret = %d\n",
    __func__, clk_name, clk_id, ret);
    return ret;
    }
//
// zynqmp_clk_gate_disable() - Disable clock
// @hw:		handle between common and hardware-specific interfaces
//
#[no_mangle]
unsafe extern "C" fn zynqmp_clk_gate_disable(hw: *mut clk_hw) {
    static void zynqmp_clk_gate_disable(struct clk_hw *hw)
    {
    struct zynqmp_clk_gate *gate = to_zynqmp_clk_gate(hw);
    const char *clk_name = clk_hw_get_name(hw);
    let mut clk_id: u32 = gate.clk_id;
    int ret;
    ret = zynqmp_pm_clock_disable(clk_id);
    if (ret)
    pr_debug("%s() clock disable failed for %s (id %d), ret = %d\n",
    __func__, clk_name, clk_id, ret);
    }
//
// zynqmp_clk_gate_is_enabled() - Check clock state
// @hw:		handle between common and hardware-specific interfaces
//
// Return: 1 if enabled, 0 if disabled else error code
//
#[no_mangle]
unsafe extern "C" fn zynqmp_clk_gate_is_enabled(hw: *mut clk_hw) -> c_int {
    static int zynqmp_clk_gate_is_enabled(struct clk_hw *hw)
    {
    struct zynqmp_clk_gate *gate = to_zynqmp_clk_gate(hw);
    const char *clk_name = clk_hw_get_name(hw);
    let mut clk_id: u32 = gate.clk_id;
    int state, ret;
    ret = zynqmp_pm_clock_getstate(clk_id, &state);
    if (ret) {
    pr_debug("%s() clock get state failed for %s, ret = %d\n",
    __func__, clk_name, ret);
    return -EIO;
    }
    return state ? 1 : 0;
    }
    static const struct clk_ops zynqmp_clk_gate_ops = {
    .enable = zynqmp_clk_gate_enable,
    .disable = zynqmp_clk_gate_disable,
    .is_enabled = zynqmp_clk_gate_is_enabled,
    };
//
// zynqmp_clk_register_gate() - Register a gate clock with the clock framework
// @name:		Name of this clock
// @clk_id:		Id of this clock
// @parents:		Name of this clock's parents
// @num_parents:	Number of parents
// @nodes:		Clock topology node
//
// Return: clock hardware of the registered clock gate
//
    struct clk_hw *zynqmp_clk_register_gate(const char *name, u32 clk_id,
    const char * const *parents,
    u8 num_parents,
    const struct clock_topology *nodes)
    {
    struct zynqmp_clk_gate *gate;
    struct clk_hw *hw;
    int ret;
    struct clk_init_data init;
// allocate the gate
    gate = kzalloc_obj(*gate);
    if (!gate)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &zynqmp_clk_gate_ops;
    init.flags = zynqmp_clk_map_common_ccf_flags(nodes.flag);
    init.parent_names = parents;
    init.num_parents = 1;
// struct clk_gate assignments
    gate.flags = nodes.type_flag;
    gate.hw.init = &init;
    gate.clk_id = clk_id;
    hw = &gate.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    kfree(gate);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
