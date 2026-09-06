//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-gpr-mux.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_clk_gpr {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub mask: u32,
    pub reg: u32,
    pub mux_table: *const u32,
}

    static struct imx_clk_gpr *to_imx_clk_gpr(struct clk_hw *hw)
    {
    return container_of(hw, struct imx_clk_gpr, hw);
    }
#[no_mangle]
unsafe extern "C" fn imx_clk_gpr_mux_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 imx_clk_gpr_mux_get_parent(struct clk_hw *hw)
    {
    struct imx_clk_gpr *priv = to_imx_clk_gpr(hw);
    unsigned int val;
    int ret;
    ret = regmap_read(priv.regmap, priv.reg, &val);
    if (ret)
    goto get_parent_err;
    val &= priv.mask;
    ret = clk_mux_val_to_index(hw, priv.mux_table, 0, val);
    if (ret < 0)
    goto get_parent_err;
    return ret;
    get_parent_err:
    pr_err("%s: failed to get parent (%pe)\n",
    clk_hw_get_name(hw), ERR_PTR(ret));
// return some realistic non negative value. Potentially we could
// give index to some dummy error parent.
//
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_clk_gpr_mux_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int imx_clk_gpr_mux_set_parent(struct clk_hw *hw, u8 index)
    {
    struct imx_clk_gpr *priv = to_imx_clk_gpr(hw);
    let mut val: c_uint = clk_mux_index_to_val(priv.mux_table, 0, index);
    return regmap_update_bits(priv.regmap, priv.reg, priv.mask, val);
    }
    static const struct clk_ops imx_clk_gpr_mux_ops = {
    .get_parent = imx_clk_gpr_mux_get_parent,
    .set_parent = imx_clk_gpr_mux_set_parent,
    .determine_rate = __clk_mux_determine_rate,
    };
    struct clk_hw *imx_clk_gpr_mux(const char *name, const char *compatible,
    u32 reg, const char **parent_names,
    u8 num_parents, const u32 *mux_table, u32 mask)
    {
    let mut init: clk_init_data = { };
    struct imx_clk_gpr *priv;
    struct regmap *regmap;
    struct clk_hw *hw;
    int ret;
    regmap = syscon_regmap_lookup_by_compatible(compatible);
    if (IS_ERR(regmap)) {
    pr_err("failed to find %s regmap\n", compatible);
    return ERR_CAST(regmap);
    }
    priv = kzalloc_obj(*priv);
    if (!priv)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &imx_clk_gpr_mux_ops;
    init.parent_names = parent_names;
    init.num_parents = num_parents;
    init.flags = CLK_SET_RATE_GATE | CLK_SET_PARENT_GATE;
    priv.hw.init = &init;
    priv.regmap = regmap;
    priv.mux_table = mux_table;
    priv.reg = reg;
    priv.mask = mask;
    hw = &priv.hw;
    ret = clk_hw_register(core::ptr::null_mut(), &priv.hw);
    if (ret) {
    kfree(priv);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
