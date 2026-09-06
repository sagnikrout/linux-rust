//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/meson-mx-sdhc-clkc.c
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
// Amlogic Meson SDHC clock controller
//
// Copyright (C) 2020 Martin Blumenstingl <martin.blumenstingl@googlemail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_mx_sdhc_clkc {
    pub src_sel: clk_mux,
    pub div: clk_divider,
    pub mod_clk_en: clk_gate,
    pub tx_clk_en: clk_gate,
    pub rx_clk_en: clk_gate,
    pub sd_clk_en: clk_gate,
}

    static const struct clk_parent_data meson_mx_sdhc_src_sel_parents[4] = {
    { .fw_name = "clkin0" },
    { .fw_name = "clkin1" },
    { .fw_name = "clkin2" },
    { .fw_name = "clkin3" },
    };
    static const struct clk_div_table meson_mx_sdhc_div_table[] = {
    { .div = 6, .val = 5, },
    { .div = 8, .val = 7, },
    { .div = 9, .val = 8, },
    { .div = 10, .val = 9, },
    { .div = 12, .val = 11, },
    { .div = 16, .val = 15, },
    { .div = 18, .val = 17, },
    { .div = 34, .val = 33, },
    { .div = 142, .val = 141, },
    { .div = 850, .val = 849, },
    { .div = 2126, .val = 2125, },
    { .div = 4096, .val = 4095, },
    { /* sentinel */ }
    };
    static int meson_mx_sdhc_clk_hw_register(struct device *dev,
    const char *name_suffix,
    const struct clk_parent_data *parents,
    unsigned int num_parents,
    const struct clk_ops *ops,
    struct clk_hw *hw)
    {
    let mut init: clk_init_data = { };
    char clk_name[32];
    snprintf(clk_name, sizeof(clk_name), "%s#%s", dev_name(dev),
    name_suffix);
    init.name = clk_name;
    init.ops = ops;
    init.flags = CLK_SET_RATE_PARENT;
    init.parent_data = parents;
    init.num_parents = num_parents;
    hw.init = &init;
    return devm_clk_hw_register(dev, hw);
    }
    static int meson_mx_sdhc_gate_clk_hw_register(struct device *dev,
    const char *name_suffix,
    struct clk_hw *parent,
    struct clk_hw *hw,
    struct clk_bulk_data *clk_bulk_data,
    u8 bulk_index)
    {
    let mut parent_data: clk_parent_data = { .hw = parent };
    int ret;
    ret = meson_mx_sdhc_clk_hw_register(dev, name_suffix, &parent_data, 1,
    &clk_gate_ops, hw);
    if (ret)
    return ret;
    clk_bulk_data[bulk_index].clk = devm_clk_hw_get_clk(dev, hw, name_suffix);
    return PTR_ERR_OR_ZERO(clk_bulk_data[bulk_index].clk);
    }
    int meson_mx_sdhc_register_clkc(struct device *dev, void __iomem *base,
    struct clk_bulk_data *clk_bulk_data)
    {
    let mut div_parent: clk_parent_data = { };
    struct meson_mx_sdhc_clkc *clkc_data;
    int ret;
    clkc_data = devm_kzalloc(dev, sizeof(*clkc_data), GFP_KERNEL);
    if (!clkc_data)
    return -ENOMEM;
    clkc_data.src_sel.reg = base + MESON_SDHC_CLKC;
    clkc_data.src_sel.mask = 0x3;
    clkc_data.src_sel.shift = 16;
    ret = meson_mx_sdhc_clk_hw_register(dev, "src_sel",
    meson_mx_sdhc_src_sel_parents, 4,
    &clk_mux_ops,
    &clkc_data.src_sel.hw);
    if (ret)
    return ret;
    clkc_data.div.reg = base + MESON_SDHC_CLKC;
    clkc_data.div.shift = 0;
    clkc_data.div.width = 12;
    clkc_data.div.table = meson_mx_sdhc_div_table;
    div_parent.hw = &clkc_data.src_sel.hw;
    ret = meson_mx_sdhc_clk_hw_register(dev, "div", &div_parent, 1,
    &clk_divider_ops,
    &clkc_data.div.hw);
    if (ret)
    return ret;
    clkc_data.mod_clk_en.reg = base + MESON_SDHC_CLKC;
    clkc_data.mod_clk_en.bit_idx = 15;
    ret = meson_mx_sdhc_gate_clk_hw_register(dev, "mod_clk_on",
    &clkc_data.div.hw,
    &clkc_data.mod_clk_en.hw,
    clk_bulk_data, 0);
    if (ret)
    return ret;
    clkc_data.tx_clk_en.reg = base + MESON_SDHC_CLKC;
    clkc_data.tx_clk_en.bit_idx = 14;
    ret = meson_mx_sdhc_gate_clk_hw_register(dev, "tx_clk_on",
    &clkc_data.div.hw,
    &clkc_data.tx_clk_en.hw,
    clk_bulk_data, 1);
    if (ret)
    return ret;
    clkc_data.rx_clk_en.reg = base + MESON_SDHC_CLKC;
    clkc_data.rx_clk_en.bit_idx = 13;
    ret = meson_mx_sdhc_gate_clk_hw_register(dev, "rx_clk_on",
    &clkc_data.div.hw,
    &clkc_data.rx_clk_en.hw,
    clk_bulk_data, 2);
    if (ret)
    return ret;
    clkc_data.sd_clk_en.reg = base + MESON_SDHC_CLKC;
    clkc_data.sd_clk_en.bit_idx = 12;
    ret = meson_mx_sdhc_gate_clk_hw_register(dev, "sd_clk_on",
    &clkc_data.div.hw,
    &clkc_data.sd_clk_en.hw,
    clk_bulk_data, 3);
    return ret;
    }
