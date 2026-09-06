//! Automatically rewritten from C to Rust
//! Source: drivers/clk/axs10x/i2s_pll_clock.c
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
// Synopsys AXS10X SDP I2S PLL clock driver
//
// Copyright (C) 2016 Synopsys
//

// PLL registers addresses
pub const PLL_IDIV_REG: c_uint = 0x0;
pub const PLL_FBDIV_REG: c_uint = 0x4;
pub const PLL_ODIV0_REG: c_uint = 0x8;
pub const PLL_ODIV1_REG: c_uint = 0xC;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2s_pll_cfg {
    pub rate: c_uint,
    pub idiv: c_uint,
    pub fbdiv: c_uint,
    pub odiv0: c_uint,
    pub odiv1: c_uint,
}

    static const struct i2s_pll_cfg i2s_pll_cfg_27m[] = {
// 27 Mhz
    { 1024000, 0x104, 0x451, 0x10E38, 0x2000 },
    { 1411200, 0x104, 0x596, 0x10D35, 0x2000 },
    { 1536000, 0x208, 0xA28, 0x10B2C, 0x2000 },
    { 2048000, 0x82, 0x451, 0x10E38, 0x2000 },
    { 2822400, 0x82, 0x596, 0x10D35, 0x2000 },
    { 3072000, 0x104, 0xA28, 0x10B2C, 0x2000 },
    { 2116800, 0x82, 0x3CF, 0x10C30, 0x2000 },
    { 2304000, 0x104, 0x79E, 0x10B2C, 0x2000 },
    { 0, 0, 0, 0, 0 },
    };
    static const struct i2s_pll_cfg i2s_pll_cfg_28m[] = {
// 28.224 Mhz
    { 1024000, 0x82, 0x105, 0x107DF, 0x2000 },
    { 1411200, 0x28A, 0x1, 0x10001, 0x2000 },
    { 1536000, 0xA28, 0x187, 0x10042, 0x2000 },
    { 2048000, 0x41, 0x105, 0x107DF, 0x2000 },
    { 2822400, 0x145, 0x1, 0x10001, 0x2000 },
    { 3072000, 0x514, 0x187, 0x10042, 0x2000 },
    { 2116800, 0x514, 0x42, 0x10001, 0x2000 },
    { 2304000, 0x619, 0x82, 0x10001, 0x2000 },
    { 0, 0, 0, 0, 0 },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2s_pll_clk {
    pub base: *mut void __iomem,
    pub hw: clk_hw,
    pub dev: *mut device,
}

    static inline void i2s_pll_write(struct i2s_pll_clk *clk, unsigned int reg,
    unsigned int val)
    {
    writel_relaxed(val, clk.base + reg);
    }
    static inline unsigned int i2s_pll_read(struct i2s_pll_clk *clk,
    unsigned int reg)
    {
    return readl_relaxed(clk.base + reg);
    }
    static inline struct i2s_pll_clk *to_i2s_pll_clk(struct clk_hw *hw)
    {
    return container_of(hw, struct i2s_pll_clk, hw);
    }
#[no_mangle]
pub unsafe extern "C" fn i2s_pll_get_value(val: c_uint) -> c_uint {
    static inline unsigned int i2s_pll_get_value(unsigned int val)
    {
    return (val & 0x3F) + ((val >> 6) & 0x3F);
    }
    static const struct i2s_pll_cfg *i2s_pll_get_cfg(unsigned long prate)
    {
    switch (prate) {
    case 27000000:
    return i2s_pll_cfg_27m;
    case 28224000:
    return i2s_pll_cfg_28m;
    default:
    return core::ptr::null_mut();
    }
    }
    static unsigned long i2s_pll_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct i2s_pll_clk *clk = to_i2s_pll_clk(hw);
    unsigned int idiv, fbdiv, odiv;
    idiv = i2s_pll_get_value(i2s_pll_read(clk, PLL_IDIV_REG));
    fbdiv = i2s_pll_get_value(i2s_pll_read(clk, PLL_FBDIV_REG));
    odiv = i2s_pll_get_value(i2s_pll_read(clk, PLL_ODIV0_REG));
    return ((parent_rate / idiv) * fbdiv) / odiv;
    }
    static int i2s_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct i2s_pll_clk *clk = to_i2s_pll_clk(hw);
    const struct i2s_pll_cfg *pll_cfg = i2s_pll_get_cfg(req.best_parent_rate);
    int i;
    if (!pll_cfg) {
    dev_err(clk.dev, "invalid parent rate=%ld\n", req.best_parent_rate);
    return -EINVAL;
    }
    for (i = 0; pll_cfg[i].rate != 0; i++)
    if (pll_cfg[i].rate == req.rate)
    return 0;
    return -EINVAL;
    }
    static int i2s_pll_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct i2s_pll_clk *clk = to_i2s_pll_clk(hw);
    const struct i2s_pll_cfg *pll_cfg = i2s_pll_get_cfg(parent_rate);
    int i;
    if (!pll_cfg) {
    dev_err(clk.dev, "invalid parent rate=%ld\n", parent_rate);
    return -EINVAL;
    }
    for (i = 0; pll_cfg[i].rate != 0; i++) {
    if (pll_cfg[i].rate == rate) {
    i2s_pll_write(clk, PLL_IDIV_REG, pll_cfg[i].idiv);
    i2s_pll_write(clk, PLL_FBDIV_REG, pll_cfg[i].fbdiv);
    i2s_pll_write(clk, PLL_ODIV0_REG, pll_cfg[i].odiv0);
    i2s_pll_write(clk, PLL_ODIV1_REG, pll_cfg[i].odiv1);
    return 0;
    }
    }
    dev_err(clk.dev, "invalid rate=%ld, parent_rate=%ld\n", rate,
    parent_rate);
    return -EINVAL;
    }
    static const struct clk_ops i2s_pll_ops = {
    .recalc_rate = i2s_pll_recalc_rate,
    .determine_rate = i2s_pll_determine_rate,
    .set_rate = i2s_pll_set_rate,
    };
#[no_mangle]
unsafe extern "C" fn i2s_pll_clk_probe(pdev: *mut platform_device) -> c_int {
    static int i2s_pll_clk_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    const char *clk_name;
    const char *parent_name;
    struct clk *clk;
    struct i2s_pll_clk *pll_clk;
    struct clk_init_data init;
    pll_clk = devm_kzalloc(dev, sizeof(*pll_clk), GFP_KERNEL);
    if (!pll_clk)
    return -ENOMEM;
    pll_clk.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pll_clk.base))
    return PTR_ERR(pll_clk.base);
    memset(&init, 0, sizeof(init));
    clk_name = node.name;
    init.name = clk_name;
    init.ops = &i2s_pll_ops;
    parent_name = of_clk_get_parent_name(node, 0);
    init.parent_names = &parent_name;
    init.num_parents = 1;
    pll_clk.hw.init = &init;
    pll_clk.dev = dev;
    clk = devm_clk_register(dev, &pll_clk.hw);
    if (IS_ERR(clk)) {
    dev_err(dev, "failed to register %s clock (%ld)\n",
    clk_name, PTR_ERR(clk));
    return PTR_ERR(clk);
    }
    return of_clk_add_provider(node, of_clk_src_simple_get, clk);
    }
#[no_mangle]
unsafe extern "C" fn i2s_pll_clk_remove(pdev: *mut platform_device) {
    static void i2s_pll_clk_remove(struct platform_device *pdev)
    {
    of_clk_del_provider(pdev.dev.of_node);
    }
    static const struct of_device_id i2s_pll_clk_id[] = {
    { .compatible = "snps,axs10x-i2s-pll-clock", },
    { },
    };
    MODULE_DEVICE_TABLE(of, i2s_pll_clk_id);
    static struct platform_driver i2s_pll_clk_driver = {
    .driver = {
    .name = "axs10x-i2s-pll-clock",
    .of_match_table = i2s_pll_clk_id,
    },
    .probe = i2s_pll_clk_probe,
    .remove = i2s_pll_clk_remove,
    };
    module_platform_driver(i2s_pll_clk_driver);
    MODULE_AUTHOR("Jose Abreu <joabreu@synopsys.com>");
    MODULE_DESCRIPTION("Synopsys AXS10X SDP I2S PLL Clock Driver");
    MODULE_LICENSE("GPL v2");
