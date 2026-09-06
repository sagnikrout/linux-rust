//! Automatically rewritten from C to Rust
//! Source: drivers/clk/rockchip/clk-muxgrf.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_muxgrf_clock {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub reg: u32,
    pub shift: u32,
    pub width: u32,
    pub flags: c_int,
}

#[no_mangle]
unsafe extern "C" fn rockchip_muxgrf_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 rockchip_muxgrf_get_parent(struct clk_hw *hw)
    {
    struct rockchip_muxgrf_clock *mux = to_muxgrf_clock(hw);
    let mut mask: c_uint = GENMASK(mux.width - 1, 0);
    unsigned int val;
    regmap_read(mux.regmap, mux.reg, &val);
    val >>= mux.shift;
    val &= mask;
    return val;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_muxgrf_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int rockchip_muxgrf_set_parent(struct clk_hw *hw, u8 index)
    {
    struct rockchip_muxgrf_clock *mux = to_muxgrf_clock(hw);
    let mut mask: c_uint = GENMASK(mux.width + mux.shift - 1, mux.shift);
    unsigned int val;
    val = index;
    val <<= mux.shift;
    if (mux.flags & CLK_MUX_HIWORD_MASK)
    return regmap_write(mux.regmap, mux.reg, val | (mask << 16));
    else
    return regmap_update_bits(mux.regmap, mux.reg, mask, val);
    }
    static const struct clk_ops rockchip_muxgrf_clk_ops = {
    .get_parent = rockchip_muxgrf_get_parent,
    .set_parent = rockchip_muxgrf_set_parent,
    .determine_rate = __clk_mux_determine_rate,
    };
    struct clk *rockchip_clk_register_muxgrf(const char *name,
    const char *const *parent_names, u8 num_parents,
    int flags, struct regmap *regmap, int reg,
    int shift, int width, int mux_flags)
    {
    struct rockchip_muxgrf_clock *muxgrf_clock;
    struct clk_init_data init;
    struct clk *clk;
    if (IS_ERR(regmap)) {
    pr_err("%s: regmap not available\n", __func__);
    return ERR_PTR(-ENOTSUPP);
    }
    muxgrf_clock = kmalloc_obj(*muxgrf_clock);
    if (!muxgrf_clock)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.flags = flags;
    init.num_parents = num_parents;
    init.parent_names = parent_names;
    init.ops = &rockchip_muxgrf_clk_ops;
    muxgrf_clock.hw.init = &init;
    muxgrf_clock.regmap = regmap;
    muxgrf_clock.reg = reg;
    muxgrf_clock.shift = shift;
    muxgrf_clock.width = width;
    muxgrf_clock.flags = mux_flags;
    clk = clk_register(core::ptr::null_mut(), &muxgrf_clock.hw);
    if (IS_ERR(clk))
    kfree(muxgrf_clock);
    return clk;
    }
