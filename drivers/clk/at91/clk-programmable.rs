//! Automatically rewritten from C to Rust
//! Source: drivers/clk/at91/clk-programmable.c
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
// Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
//

pub const PROG_ID_MAX: c_int = 7;

pub const PROG_MAX_RM9200_CSS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_programmable {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub mux_table: *mut u32,
    pub id: u8,
    pub layout: *const clk_programmable_layout,
    pub pms: at91_clk_pms,
}

    static unsigned long clk_programmable_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_programmable *prog = to_clk_programmable(hw);
    const struct clk_programmable_layout *layout = prog.layout;
    unsigned int pckr;
    unsigned long rate;
    regmap_read(prog.regmap, AT91_PMC_PCKR(prog.id), &pckr);
    if (layout.is_pres_direct)
    rate = parent_rate / (PROG_PRES(layout, pckr) + 1);
    else
    rate = parent_rate >> PROG_PRES(layout, pckr);
    return rate;
    }
    static int clk_programmable_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_programmable *prog = to_clk_programmable(hw);
    const struct clk_programmable_layout *layout = prog.layout;
    struct clk_hw *parent;
    let mut best_rate: c_long = -EINVAL;
    unsigned long parent_rate;
    let mut tmp_rate: c_ulong = 0;
    int shift;
    int i;
    for (i = 0; i < clk_hw_get_num_parents(hw); i++) {
    parent = clk_hw_get_parent_by_index(hw, i);
    if (!parent)
    continue;
    parent_rate = clk_hw_get_rate(parent);
    if (layout.is_pres_direct) {
    for (shift = 0; shift <= layout.pres_mask; shift++) {
    tmp_rate = parent_rate / (shift + 1);
    if (tmp_rate <= req.rate)
    break;
    }
    } else {
    for (shift = 0; shift < layout.pres_mask; shift++) {
    tmp_rate = parent_rate >> shift;
    if (tmp_rate <= req.rate)
    break;
    }
    }
    if (tmp_rate > req.rate)
    continue;
    if (best_rate < 0 ||
    (req.rate - tmp_rate) < (req.rate - best_rate)) {
    best_rate = tmp_rate;
    req.best_parent_rate = parent_rate;
    req.best_parent_hw = parent;
    }
    if (!best_rate)
    break;
    }
    if (best_rate < 0)
    return best_rate;
    req.rate = best_rate;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_programmable_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int clk_programmable_set_parent(struct clk_hw *hw, u8 index)
    {
    struct clk_programmable *prog = to_clk_programmable(hw);
    const struct clk_programmable_layout *layout = prog.layout;
    let mut mask: c_uint = layout.css_mask;
    let mut pckr: c_uint = index;
    if (layout.have_slck_mck)
    mask |= AT91_PMC_CSSMCK_MCK;
    if (prog.mux_table)
    pckr = clk_mux_index_to_val(prog.mux_table, 0, index);
    if (index > layout.css_mask) {
    if (index > PROG_MAX_RM9200_CSS && !layout.have_slck_mck)
    return -EINVAL;
    pckr |= AT91_PMC_CSSMCK_MCK;
    }
    regmap_update_bits(prog.regmap, AT91_PMC_PCKR(prog.id), mask, pckr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_programmable_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_programmable_get_parent(struct clk_hw *hw)
    {
    struct clk_programmable *prog = to_clk_programmable(hw);
    const struct clk_programmable_layout *layout = prog.layout;
    unsigned int pckr;
    u8 ret;
    regmap_read(prog.regmap, AT91_PMC_PCKR(prog.id), &pckr);
    ret = pckr & layout.css_mask;
    if (layout.have_slck_mck && (pckr & AT91_PMC_CSSMCK_MCK) && !ret)
    ret = PROG_MAX_RM9200_CSS + 1;
    if (prog.mux_table)
    ret = clk_mux_val_to_index(&prog.hw, prog.mux_table, 0, ret);
    return ret;
    }
    static int clk_programmable_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_programmable *prog = to_clk_programmable(hw);
    const struct clk_programmable_layout *layout = prog.layout;
    let mut div: c_ulong = parent_rate / rate;
    let mut shift: c_int = 0;
    if (!div)
    return -EINVAL;
    if (layout.is_pres_direct) {
    shift = div - 1;
    if (shift > layout.pres_mask)
    return -EINVAL;
    } else {
    shift = fls(div) - 1;
    if (div != (1 << shift))
    return -EINVAL;
    if (shift >= layout.pres_mask)
    return -EINVAL;
    }
    regmap_update_bits(prog.regmap, AT91_PMC_PCKR(prog.id),
    layout.pres_mask << layout.pres_shift,
    shift << layout.pres_shift);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_programmable_save_context(hw: *mut clk_hw) -> c_int {
    static int clk_programmable_save_context(struct clk_hw *hw)
    {
    struct clk_programmable *prog = to_clk_programmable(hw);
    struct clk_hw *parent_hw = clk_hw_get_parent(hw);
    prog.pms.parent = clk_programmable_get_parent(hw);
    prog.pms.parent_rate = clk_hw_get_rate(parent_hw);
    prog.pms.rate = clk_programmable_recalc_rate(hw, prog.pms.parent_rate);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_programmable_restore_context(hw: *mut clk_hw) {
    static void clk_programmable_restore_context(struct clk_hw *hw)
    {
    struct clk_programmable *prog = to_clk_programmable(hw);
    int ret;
    ret = clk_programmable_set_parent(hw, prog.pms.parent);
    if (ret)
    return;
    clk_programmable_set_rate(hw, prog.pms.rate, prog.pms.parent_rate);
    }
    static const struct clk_ops programmable_ops = {
    .recalc_rate = clk_programmable_recalc_rate,
    .determine_rate = clk_programmable_determine_rate,
    .get_parent = clk_programmable_get_parent,
    .set_parent = clk_programmable_set_parent,
    .set_rate = clk_programmable_set_rate,
    .save_context = clk_programmable_save_context,
    .restore_context = clk_programmable_restore_context,
    };
    struct clk_hw * __init
    at91_clk_register_programmable(struct regmap *regmap,
    const char *name, const char **parent_names,
    struct clk_hw **parent_hws, u8 num_parents, u8 id,
    const struct clk_programmable_layout *layout,
    u32 *mux_table)
    {
    struct clk_programmable *prog;
    struct clk_hw *hw;
    let mut init: clk_init_data = {};
    int ret;
    if (id > PROG_ID_MAX || !(parent_names || parent_hws))
    return ERR_PTR(-EINVAL);
    prog = kzalloc_obj(*prog);
    if (!prog)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &programmable_ops;
    if (parent_hws)
    init.parent_hws = (const struct clk_hw **)parent_hws;
    else
    init.parent_names = parent_names;
    init.num_parents = num_parents;
    init.flags = CLK_SET_RATE_GATE | CLK_SET_PARENT_GATE;
    prog.id = id;
    prog.layout = layout;
    prog.hw.init = &init;
    prog.regmap = regmap;
    prog.mux_table = mux_table;
    hw = &prog.hw;
    ret = clk_hw_register(core::ptr::null_mut(), &prog.hw);
    if (ret) {
    kfree(prog);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
    const struct clk_programmable_layout at91rm9200_programmable_layout = {
    .pres_mask = 0x7,
    .pres_shift = 2,
    .css_mask = 0x3,
    .have_slck_mck = 0,
    .is_pres_direct = 0,
    };
    const struct clk_programmable_layout at91sam9g45_programmable_layout = {
    .pres_mask = 0x7,
    .pres_shift = 2,
    .css_mask = 0x3,
    .have_slck_mck = 1,
    .is_pres_direct = 0,
    };
    const struct clk_programmable_layout at91sam9x5_programmable_layout = {
    .pres_mask = 0x7,
    .pres_shift = 4,
    .css_mask = 0x7,
    .have_slck_mck = 0,
    .is_pres_direct = 0,
    };
