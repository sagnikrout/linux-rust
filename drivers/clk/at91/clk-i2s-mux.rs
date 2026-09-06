//! Automatically rewritten from C to Rust
//! Source: drivers/clk/at91/clk-i2s-mux.c
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
// Copyright (C) 2018 Microchip Technology Inc,
// Codrin Ciubotariu <codrin.ciubotariu@microchip.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_i2s_mux {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub bus_id: u8,
}

#[no_mangle]
unsafe extern "C" fn clk_i2s_mux_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_i2s_mux_get_parent(struct clk_hw *hw)
    {
    struct clk_i2s_mux *mux = to_clk_i2s_mux(hw);
    u32 val;
    regmap_read(mux.regmap, AT91_SFR_I2SCLKSEL, &val);
    return (val & BIT(mux.bus_id)) >> mux.bus_id;
    }
#[no_mangle]
unsafe extern "C" fn clk_i2s_mux_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int clk_i2s_mux_set_parent(struct clk_hw *hw, u8 index)
    {
    struct clk_i2s_mux *mux = to_clk_i2s_mux(hw);
    return regmap_update_bits(mux.regmap, AT91_SFR_I2SCLKSEL,
    BIT(mux.bus_id), index << mux.bus_id);
    }
    static const struct clk_ops clk_i2s_mux_ops = {
    .get_parent = clk_i2s_mux_get_parent,
    .set_parent = clk_i2s_mux_set_parent,
    .determine_rate = __clk_mux_determine_rate,
    };
    struct clk_hw * __init
    at91_clk_i2s_mux_register(struct regmap *regmap, const char *name,
    const char * const *parent_names,
    unsigned int num_parents, u8 bus_id)
    {
    let mut init: clk_init_data = {};
    struct clk_i2s_mux *i2s_ck;
    int ret;
    i2s_ck = kzalloc_obj(*i2s_ck);
    if (!i2s_ck)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &clk_i2s_mux_ops;
    init.parent_names = parent_names;
    init.num_parents = num_parents;
    i2s_ck.hw.init = &init;
    i2s_ck.bus_id = bus_id;
    i2s_ck.regmap = regmap;
    ret = clk_hw_register(core::ptr::null_mut(), &i2s_ck.hw);
    if (ret) {
    kfree(i2s_ck);
    return ERR_PTR(ret);
    }
    return &i2s_ck.hw;
    }
