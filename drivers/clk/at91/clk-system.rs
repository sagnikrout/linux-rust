//! Automatically rewritten from C to Rust
//! Source: drivers/clk/at91/clk-system.c
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

pub const SYSTEM_MAX_ID: c_int = 31;
pub const SYSTEM_MAX_NAME_SZ: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_system {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub pms: at91_clk_pms,
    pub id: u8,
}

#[no_mangle]
pub unsafe extern "C" fn is_pck(id: c_int) -> c_int {
    static inline int is_pck(int id)
    {
    return (id >= 8) && (id <= 15);
    }
#[no_mangle]
pub unsafe extern "C" fn clk_system_ready(regmap: *mut regmap, id: c_int) -> bool {
    static inline bool clk_system_ready(struct regmap *regmap, int id)
    {
    unsigned int status;
    regmap_read(regmap, AT91_PMC_SR, &status);
    return !!(status & (1 << id));
    }
#[no_mangle]
unsafe extern "C" fn clk_system_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_system_prepare(struct clk_hw *hw)
    {
    struct clk_system *sys = to_clk_system(hw);
    regmap_write(sys.regmap, AT91_PMC_SCER, 1 << sys.id);
    if (!is_pck(sys.id))
    return 0;
    while (!clk_system_ready(sys.regmap, sys.id))
    cpu_relax();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_system_unprepare(hw: *mut clk_hw) {
    static void clk_system_unprepare(struct clk_hw *hw)
    {
    struct clk_system *sys = to_clk_system(hw);
    regmap_write(sys.regmap, AT91_PMC_SCDR, 1 << sys.id);
    }
#[no_mangle]
unsafe extern "C" fn clk_system_is_prepared(hw: *mut clk_hw) -> c_int {
    static int clk_system_is_prepared(struct clk_hw *hw)
    {
    struct clk_system *sys = to_clk_system(hw);
    unsigned int status;
    regmap_read(sys.regmap, AT91_PMC_SCSR, &status);
    if (!(status & (1 << sys.id)))
    return 0;
    if (!is_pck(sys.id))
    return 1;
    regmap_read(sys.regmap, AT91_PMC_SR, &status);
    return !!(status & (1 << sys.id));
    }
#[no_mangle]
unsafe extern "C" fn clk_system_save_context(hw: *mut clk_hw) -> c_int {
    static int clk_system_save_context(struct clk_hw *hw)
    {
    struct clk_system *sys = to_clk_system(hw);
    sys.pms.status = clk_system_is_prepared(hw);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_system_restore_context(hw: *mut clk_hw) {
    static void clk_system_restore_context(struct clk_hw *hw)
    {
    struct clk_system *sys = to_clk_system(hw);
    if (sys.pms.status)
    clk_system_prepare(&sys.hw);
    }
    static const struct clk_ops system_ops = {
    .prepare = clk_system_prepare,
    .unprepare = clk_system_unprepare,
    .is_prepared = clk_system_is_prepared,
    .save_context = clk_system_save_context,
    .restore_context = clk_system_restore_context,
    };
    struct clk_hw * __init
    at91_clk_register_system(struct regmap *regmap, const char *name,
    const char *parent_name, struct clk_hw *parent_hw, u8 id,
    unsigned long flags)
    {
    struct clk_system *sys;
    struct clk_hw *hw;
    let mut init: clk_init_data = {};
    int ret;
    if (!(parent_name || parent_hw) || id > SYSTEM_MAX_ID)
    return ERR_PTR(-EINVAL);
    sys = kzalloc_obj(*sys);
    if (!sys)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &system_ops;
    if (parent_hw)
    init.parent_hws = (const struct clk_hw **)&parent_hw;
    else
    init.parent_names = &parent_name;
    init.num_parents = 1;
    init.flags = CLK_SET_RATE_PARENT | flags;
    sys.id = id;
    sys.hw.init = &init;
    sys.regmap = regmap;
    hw = &sys.hw;
    ret = clk_hw_register(core::ptr::null_mut(), &sys.hw);
    if (ret) {
    kfree(sys);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
