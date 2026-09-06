//! Automatically rewritten from C to Rust
//! Source: drivers/clk/zynqmp/clk-mux-zynqmp.c
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
// Zynq UltraScale+ MPSoC mux
//
// Copyright (C) 2016-2018 Xilinx
//

//
// DOC: basic adjustable multiplexer clock that cannot gate
//
// Traits of this clock:
// prepare - clk_prepare only ensures that parents are prepared
// enable - clk_enable only ensures that parents are enabled
// rate - rate is only affected by parent switching.  No clk_set_rate support
// parent - parent is adjustable through clk_set_parent
//
// struct zynqmp_clk_mux - multiplexer clock
//
// @hw:		handle between common and hardware-specific interfaces
// @flags:	hardware-specific flags
// @clk_id:	Id of clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynqmp_clk_mux {
    pub hw: clk_hw,
    pub flags: u8,
    pub clk_id: u32,
}

//
// zynqmp_clk_mux_get_parent() - Get parent of clock
// @hw:		handle between common and hardware-specific interfaces
//
// Return: Parent index on success or number of parents in case of error
//
#[no_mangle]
unsafe extern "C" fn zynqmp_clk_mux_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 zynqmp_clk_mux_get_parent(struct clk_hw *hw)
    {
    struct zynqmp_clk_mux *mux = to_zynqmp_clk_mux(hw);
    const char *clk_name = clk_hw_get_name(hw);
    let mut clk_id: u32 = mux.clk_id;
    u32 val;
    int ret;
    ret = zynqmp_pm_clock_getparent(clk_id, &val);
    if (ret) {
    pr_debug("%s() getparent failed for clock: %s, ret = %d\n",
    __func__, clk_name, ret);
//
// clk_core_get_parent_by_index() takes num_parents as incorrect
// index which is exactly what I want to return here
//
    return clk_hw_get_num_parents(hw);
    }
    return val;
    }
//
// zynqmp_clk_mux_set_parent() - Set parent of clock
// @hw:		handle between common and hardware-specific interfaces
// @index:	Parent index
//
// Return: 0 on success else error+reason
//
#[no_mangle]
unsafe extern "C" fn zynqmp_clk_mux_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int zynqmp_clk_mux_set_parent(struct clk_hw *hw, u8 index)
    {
    struct zynqmp_clk_mux *mux = to_zynqmp_clk_mux(hw);
    const char *clk_name = clk_hw_get_name(hw);
    let mut clk_id: u32 = mux.clk_id;
    int ret;
    ret = zynqmp_pm_clock_setparent(clk_id, index);
    if (ret)
    pr_debug("%s() set parent failed for clock: %s, ret = %d\n",
    __func__, clk_name, ret);
    return ret;
    }
    static const struct clk_ops zynqmp_clk_mux_ops = {
    .get_parent = zynqmp_clk_mux_get_parent,
    .set_parent = zynqmp_clk_mux_set_parent,
    .determine_rate = __clk_mux_determine_rate_closest,
    };
    static const struct clk_ops zynqmp_clk_mux_ro_ops = {
    .get_parent = zynqmp_clk_mux_get_parent,
    };
    static inline unsigned long zynqmp_clk_map_mux_ccf_flags(
    const u32 zynqmp_type_flag)
    {
    let mut ccf_flag: c_ulong = 0;
    if (zynqmp_type_flag & ZYNQMP_CLK_MUX_INDEX_ONE)
    ccf_flag |= CLK_MUX_INDEX_ONE;
    if (zynqmp_type_flag & ZYNQMP_CLK_MUX_INDEX_BIT)
    ccf_flag |= CLK_MUX_INDEX_BIT;
    if (zynqmp_type_flag & ZYNQMP_CLK_MUX_HIWORD_MASK)
    ccf_flag |= CLK_MUX_HIWORD_MASK;
    if (zynqmp_type_flag & ZYNQMP_CLK_MUX_READ_ONLY)
    ccf_flag |= CLK_MUX_READ_ONLY;
    if (zynqmp_type_flag & ZYNQMP_CLK_MUX_ROUND_CLOSEST)
    ccf_flag |= CLK_MUX_ROUND_CLOSEST;
    if (zynqmp_type_flag & ZYNQMP_CLK_MUX_BIG_ENDIAN)
    ccf_flag |= CLK_MUX_BIG_ENDIAN;
    return ccf_flag;
    }
//
// zynqmp_clk_register_mux() - Register a mux table with the clock
// framework
// @name:		Name of this clock
// @clk_id:		Id of this clock
// @parents:		Name of this clock's parents
// @num_parents:	Number of parents
// @nodes:		Clock topology node
//
// Return: clock hardware of the registered clock mux
//
    struct clk_hw *zynqmp_clk_register_mux(const char *name, u32 clk_id,
    const char * const *parents,
    u8 num_parents,
    const struct clock_topology *nodes)
    {
    struct zynqmp_clk_mux *mux;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    mux = kzalloc_obj(*mux);
    if (!mux)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    if (nodes.type_flag & CLK_MUX_READ_ONLY)
    init.ops = &zynqmp_clk_mux_ro_ops;
    else
    init.ops = &zynqmp_clk_mux_ops;
    init.flags = zynqmp_clk_map_common_ccf_flags(nodes.flag);
    init.parent_names = parents;
    init.num_parents = num_parents;
    mux.flags = zynqmp_clk_map_mux_ccf_flags(nodes.type_flag);
    mux.hw.init = &init;
    mux.clk_id = clk_id;
    hw = &mux.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    kfree(mux);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
