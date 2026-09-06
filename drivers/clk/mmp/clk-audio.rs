//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mmp/clk-audio.c
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
// MMP Audio Clock Controller driver
//
// Copyright (C) 2020 Lubomir Rintel <lkundrak@v3.sk>
//

// Audio Controller Registers
pub const SSPA_AUD_CTRL: c_uint = 0x04;
pub const SSPA_AUD_PLL_CTRL0: c_uint = 0x08;
pub const SSPA_AUD_PLL_CTRL1: c_uint = 0x0c;
// SSPA Audio Control Register
pub const SSPA_AUD_CTRL_SYSCLK_SHIFT: c_int = 0;
pub const SSPA_AUD_CTRL_SYSCLK_DIV_SHIFT: c_int = 1;
pub const SSPA_AUD_CTRL_SSPA0_MUX_SHIFT: c_int = 7;
pub const SSPA_AUD_CTRL_SSPA0_SHIFT: c_int = 8;
pub const SSPA_AUD_CTRL_SSPA0_DIV_SHIFT: c_int = 9;
pub const SSPA_AUD_CTRL_SSPA1_SHIFT: c_int = 16;
pub const SSPA_AUD_CTRL_SSPA1_DIV_SHIFT: c_int = 17;
pub const SSPA_AUD_CTRL_SSPA1_MUX_SHIFT: c_int = 23;
pub const SSPA_AUD_CTRL_DIV_MASK: c_uint = 0x7e;
// SSPA Audio PLL Control 0 Register

// SSPA Audio PLL Control 1 Register

pub const CLK_AUDIO_NR_CLKS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp2_audio_clk {
    pub mmio_base: *mut void __iomem,
    pub audio_pll_hw: clk_hw,
    pub sspa_mux: clk_mux,
    pub sspa1_mux: clk_mux,
    pub sysclk_div: clk_divider,
    pub sspa0_div: clk_divider,
    pub sspa1_div: clk_divider,
    pub sysclk_gate: clk_gate,
    pub sspa0_gate: clk_gate,
    pub sspa1_gate: clk_gate,
    pub aud_ctrl: u32,
    pub aud_pll_ctrl0: u32,
    pub aud_pll_ctrl1: u32,
    pub lock: spinlock_t,
// Must be last
    pub clk_data: clk_hw_onecell_data,
}

    static const struct {
    unsigned long parent_rate;
    unsigned long freq_vco;
    unsigned char mclk;
    unsigned char fbcclk;
    unsigned short fract;
    } predivs[] = {
    { 26000000, 135475200, 0, 0, 0x8a18 },
    { 26000000, 147456000, 0, 1, 0x0da1 },
    { 38400000, 135475200, 1, 2, 0x8208 },
    { 38400000, 147456000, 1, 3, 0xaaaa },
    };
    static const struct {
    unsigned char divisor;
    unsigned char modulo;
    unsigned char pattern;
    } postdivs[] = {
    {   1,	3,  0, },
    {   2,	5,  0, },
    {   4,	0,  0, },
    {   6,	1,  1, },
    {   8,	1,  0, },
    {   9,	1,  2, },
    {  12,	2,  1, },
    {  16,	2,  0, },
    {  18,	2,  2, },
    {  24,	4,  1, },
    {  36,	4,  2, },
    {  48,	6,  1, },
    {  72,	6,  2, },
    };
    static unsigned long audio_pll_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct mmp2_audio_clk *priv = container_of(hw, struct mmp2_audio_clk, audio_pll_hw);
    unsigned int prediv;
    unsigned int postdiv;
    u32 aud_pll_ctrl0;
    u32 aud_pll_ctrl1;
    aud_pll_ctrl0 = readl(priv.mmio_base + SSPA_AUD_PLL_CTRL0);
    aud_pll_ctrl0 &= SSPA_AUD_PLL_CTRL0_DIV_OCLK_MODULO_MASK |
    SSPA_AUD_PLL_CTRL0_FRACT_MASK |
    SSPA_AUD_PLL_CTRL0_ENA_DITHER |
    SSPA_AUD_PLL_CTRL0_DIV_FBCCLK_MASK |
    SSPA_AUD_PLL_CTRL0_DIV_MCLK_MASK |
    SSPA_AUD_PLL_CTRL0_PU;
    aud_pll_ctrl1 = readl(priv.mmio_base + SSPA_AUD_PLL_CTRL1);
    aud_pll_ctrl1 &= SSPA_AUD_PLL_CTRL1_CLK_SEL_MASK |
    SSPA_AUD_PLL_CTRL1_DIV_OCLK_PATTERN_MASK;
    for (prediv = 0; prediv < ARRAY_SIZE(predivs); prediv++) {
    if (predivs[prediv].parent_rate != parent_rate)
    continue;
    for (postdiv = 0; postdiv < ARRAY_SIZE(postdivs); postdiv++) {
    unsigned long freq;
    u32 val;
    val = SSPA_AUD_PLL_CTRL0_ENA_DITHER;
    val |= SSPA_AUD_PLL_CTRL0_PU;
    val |= SSPA_AUD_PLL_CTRL0_DIV_OCLK_MODULO(postdivs[postdiv].modulo);
    val |= SSPA_AUD_PLL_CTRL0_FRACT(predivs[prediv].fract);
    val |= SSPA_AUD_PLL_CTRL0_DIV_FBCCLK(predivs[prediv].fbcclk);
    val |= SSPA_AUD_PLL_CTRL0_DIV_MCLK(predivs[prediv].mclk);
    if (val != aud_pll_ctrl0)
    continue;
    val = SSPA_AUD_PLL_CTRL1_CLK_SEL_AUDIO_PLL;
    val |= SSPA_AUD_PLL_CTRL1_DIV_OCLK_PATTERN(postdivs[postdiv].pattern);
    if (val != aud_pll_ctrl1)
    continue;
    freq = predivs[prediv].freq_vco;
    freq /= postdivs[postdiv].divisor;
    return freq;
    }
    }
    return 0;
    }
    static int audio_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    unsigned int prediv;
    unsigned int postdiv;
    let mut rounded: c_long = 0;
    for (prediv = 0; prediv < ARRAY_SIZE(predivs); prediv++) {
    if (predivs[prediv].parent_rate != req.best_parent_rate)
    continue;
    for (postdiv = 0; postdiv < ARRAY_SIZE(postdivs); postdiv++) {
    let mut freq: c_long = predivs[prediv].freq_vco;
    freq /= postdivs[postdiv].divisor;
    if (freq == req.rate)
    return 0;
    if (freq < req.rate)
    continue;
    if (rounded && freq > rounded)
    continue;
    rounded = freq;
    }
    }
    req.rate = rounded;
    return 0;
    }
    static int audio_pll_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct mmp2_audio_clk *priv = container_of(hw, struct mmp2_audio_clk, audio_pll_hw);
    unsigned int prediv;
    unsigned int postdiv;
    unsigned long val;
    for (prediv = 0; prediv < ARRAY_SIZE(predivs); prediv++) {
    if (predivs[prediv].parent_rate != parent_rate)
    continue;
    for (postdiv = 0; postdiv < ARRAY_SIZE(postdivs); postdiv++) {
    if (rate * postdivs[postdiv].divisor != predivs[prediv].freq_vco)
    continue;
    val = SSPA_AUD_PLL_CTRL0_ENA_DITHER;
    val |= SSPA_AUD_PLL_CTRL0_PU;
    val |= SSPA_AUD_PLL_CTRL0_DIV_OCLK_MODULO(postdivs[postdiv].modulo);
    val |= SSPA_AUD_PLL_CTRL0_FRACT(predivs[prediv].fract);
    val |= SSPA_AUD_PLL_CTRL0_DIV_FBCCLK(predivs[prediv].fbcclk);
    val |= SSPA_AUD_PLL_CTRL0_DIV_MCLK(predivs[prediv].mclk);
    writel(val, priv.mmio_base + SSPA_AUD_PLL_CTRL0);
    val = SSPA_AUD_PLL_CTRL1_CLK_SEL_AUDIO_PLL;
    val |= SSPA_AUD_PLL_CTRL1_DIV_OCLK_PATTERN(postdivs[postdiv].pattern);
    writel(val, priv.mmio_base + SSPA_AUD_PLL_CTRL1);
    return 0;
    }
    }
    return -ERANGE;
    }
    static const struct clk_ops audio_pll_ops = {
    .recalc_rate = audio_pll_recalc_rate,
    .determine_rate = audio_pll_determine_rate,
    .set_rate = audio_pll_set_rate,
    };
#[no_mangle]
unsafe extern "C" fn register_clocks(priv: *mut mmp2_audio_clk, dev: *mut device) -> c_int {
    static int register_clocks(struct mmp2_audio_clk *priv, struct device *dev)
    {
    const struct clk_parent_data sspa_mux_parents[] = {
    { .hw = &priv.audio_pll_hw },
    { .fw_name = "i2s0" },
    };
    const struct clk_parent_data sspa1_mux_parents[] = {
    { .hw = &priv.audio_pll_hw },
    { .fw_name = "i2s1" },
    };
    int ret;
    priv.audio_pll_hw.init = CLK_HW_INIT_FW_NAME("audio_pll",
    "vctcxo", &audio_pll_ops,
    CLK_SET_RATE_PARENT);
    ret = devm_clk_hw_register(dev, &priv.audio_pll_hw);
    if (ret)
    return ret;
    priv.sspa_mux.hw.init = CLK_HW_INIT_PARENTS_DATA("sspa_mux",
    sspa_mux_parents, &clk_mux_ops,
    CLK_SET_RATE_PARENT);
    priv.sspa_mux.reg = priv.mmio_base + SSPA_AUD_CTRL;
    priv.sspa_mux.mask = 1;
    priv.sspa_mux.shift = SSPA_AUD_CTRL_SSPA0_MUX_SHIFT;
    ret = devm_clk_hw_register(dev, &priv.sspa_mux.hw);
    if (ret)
    return ret;
    priv.sysclk_div.hw.init = CLK_HW_INIT_HW("sys_div",
    &priv.sspa_mux.hw, &clk_divider_ops,
    CLK_SET_RATE_PARENT);
    priv.sysclk_div.reg = priv.mmio_base + SSPA_AUD_CTRL;
    priv.sysclk_div.shift = SSPA_AUD_CTRL_SYSCLK_DIV_SHIFT;
    priv.sysclk_div.width = 6;
    priv.sysclk_div.flags = CLK_DIVIDER_ONE_BASED;
    priv.sysclk_div.flags |= CLK_DIVIDER_ROUND_CLOSEST;
    priv.sysclk_div.flags |= CLK_DIVIDER_ALLOW_ZERO;
    ret = devm_clk_hw_register(dev, &priv.sysclk_div.hw);
    if (ret)
    return ret;
    priv.sysclk_gate.hw.init = CLK_HW_INIT_HW("sys_clk",
    &priv.sysclk_div.hw, &clk_gate_ops,
    CLK_SET_RATE_PARENT);
    priv.sysclk_gate.reg = priv.mmio_base + SSPA_AUD_CTRL;
    priv.sysclk_gate.bit_idx = SSPA_AUD_CTRL_SYSCLK_SHIFT;
    ret = devm_clk_hw_register(dev, &priv.sysclk_gate.hw);
    if (ret)
    return ret;
    priv.sspa0_div.hw.init = CLK_HW_INIT_HW("sspa0_div",
    &priv.sspa_mux.hw, &clk_divider_ops, 0);
    priv.sspa0_div.reg = priv.mmio_base + SSPA_AUD_CTRL;
    priv.sspa0_div.shift = SSPA_AUD_CTRL_SSPA0_DIV_SHIFT;
    priv.sspa0_div.width = 6;
    priv.sspa0_div.flags = CLK_DIVIDER_ONE_BASED;
    priv.sspa0_div.flags |= CLK_DIVIDER_ROUND_CLOSEST;
    priv.sspa0_div.flags |= CLK_DIVIDER_ALLOW_ZERO;
    ret = devm_clk_hw_register(dev, &priv.sspa0_div.hw);
    if (ret)
    return ret;
    priv.sspa0_gate.hw.init = CLK_HW_INIT_HW("sspa0_clk",
    &priv.sspa0_div.hw, &clk_gate_ops,
    CLK_SET_RATE_PARENT);
    priv.sspa0_gate.reg = priv.mmio_base + SSPA_AUD_CTRL;
    priv.sspa0_gate.bit_idx = SSPA_AUD_CTRL_SSPA0_SHIFT;
    ret = devm_clk_hw_register(dev, &priv.sspa0_gate.hw);
    if (ret)
    return ret;
    priv.sspa1_mux.hw.init = CLK_HW_INIT_PARENTS_DATA("sspa1_mux",
    sspa1_mux_parents, &clk_mux_ops,
    CLK_SET_RATE_PARENT);
    priv.sspa1_mux.reg = priv.mmio_base + SSPA_AUD_CTRL;
    priv.sspa1_mux.mask = 1;
    priv.sspa1_mux.shift = SSPA_AUD_CTRL_SSPA1_MUX_SHIFT;
    ret = devm_clk_hw_register(dev, &priv.sspa1_mux.hw);
    if (ret)
    return ret;
    priv.sspa1_div.hw.init = CLK_HW_INIT_HW("sspa1_div",
    &priv.sspa1_mux.hw, &clk_divider_ops, 0);
    priv.sspa1_div.reg = priv.mmio_base + SSPA_AUD_CTRL;
    priv.sspa1_div.shift = SSPA_AUD_CTRL_SSPA1_DIV_SHIFT;
    priv.sspa1_div.width = 6;
    priv.sspa1_div.flags = CLK_DIVIDER_ONE_BASED;
    priv.sspa1_div.flags |= CLK_DIVIDER_ROUND_CLOSEST;
    priv.sspa1_div.flags |= CLK_DIVIDER_ALLOW_ZERO;
    ret = devm_clk_hw_register(dev, &priv.sspa1_div.hw);
    if (ret)
    return ret;
    priv.sspa1_gate.hw.init = CLK_HW_INIT_HW("sspa1_clk",
    &priv.sspa1_div.hw, &clk_gate_ops,
    CLK_SET_RATE_PARENT);
    priv.sspa1_gate.reg = priv.mmio_base + SSPA_AUD_CTRL;
    priv.sspa1_gate.bit_idx = SSPA_AUD_CTRL_SSPA1_SHIFT;
    ret = devm_clk_hw_register(dev, &priv.sspa1_gate.hw);
    if (ret)
    return ret;
    priv.clk_data.hws[MMP2_CLK_AUDIO_SYSCLK] = &priv.sysclk_gate.hw;
    priv.clk_data.hws[MMP2_CLK_AUDIO_SSPA0] = &priv.sspa0_gate.hw;
    priv.clk_data.hws[MMP2_CLK_AUDIO_SSPA1] = &priv.sspa1_gate.hw;
    priv.clk_data.num = CLK_AUDIO_NR_CLKS;
    return of_clk_add_hw_provider(dev.of_node, of_clk_hw_onecell_get,
    &priv.clk_data);
    }
#[no_mangle]
unsafe extern "C" fn mmp2_audio_clk_probe(pdev: *mut platform_device) -> c_int {
    static int mmp2_audio_clk_probe(struct platform_device *pdev)
    {
    struct mmp2_audio_clk *priv;
    int ret;
    priv = devm_kzalloc(&pdev.dev,
    struct_size(priv, clk_data.hws,
    CLK_AUDIO_NR_CLKS),
    GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    spin_lock_init(&priv.lock);
    platform_set_drvdata(pdev, priv);
    priv.mmio_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.mmio_base))
    return PTR_ERR(priv.mmio_base);
    pm_runtime_enable(&pdev.dev);
    ret = pm_clk_create(&pdev.dev);
    if (ret)
    goto disable_pm_runtime;
    ret = pm_clk_add(&pdev.dev, "audio");
    if (ret)
    goto destroy_pm_clk;
    ret = register_clocks(priv, &pdev.dev);
    if (ret)
    goto destroy_pm_clk;
    return 0;
    destroy_pm_clk:
    pm_clk_destroy(&pdev.dev);
    disable_pm_runtime:
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mmp2_audio_clk_remove(pdev: *mut platform_device) {
    static void mmp2_audio_clk_remove(struct platform_device *pdev)
    {
    pm_clk_destroy(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    }

#[no_mangle]
unsafe extern "C" fn mmp2_audio_clk_suspend(dev: *mut device) -> c_int {
    static int mmp2_audio_clk_suspend(struct device *dev)
    {
    struct mmp2_audio_clk *priv = dev_get_drvdata(dev);
    priv.aud_ctrl = readl(priv.mmio_base + SSPA_AUD_CTRL);
    priv.aud_pll_ctrl0 = readl(priv.mmio_base + SSPA_AUD_PLL_CTRL0);
    priv.aud_pll_ctrl1 = readl(priv.mmio_base + SSPA_AUD_PLL_CTRL1);
    pm_clk_suspend(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mmp2_audio_clk_resume(dev: *mut device) -> c_int {
    static int mmp2_audio_clk_resume(struct device *dev)
    {
    struct mmp2_audio_clk *priv = dev_get_drvdata(dev);
    pm_clk_resume(dev);
    writel(priv.aud_ctrl, priv.mmio_base + SSPA_AUD_CTRL);
    writel(priv.aud_pll_ctrl0, priv.mmio_base + SSPA_AUD_PLL_CTRL0);
    writel(priv.aud_pll_ctrl1, priv.mmio_base + SSPA_AUD_PLL_CTRL1);
    return 0;
    }

    static const struct dev_pm_ops mmp2_audio_clk_pm_ops = {
    SET_RUNTIME_PM_OPS(mmp2_audio_clk_suspend, mmp2_audio_clk_resume, core::ptr::null_mut())
    };
    static const struct of_device_id mmp2_audio_clk_of_match[] = {
    { .compatible = "marvell,mmp2-audio-clock" },
    {}
    };
    MODULE_DEVICE_TABLE(of, mmp2_audio_clk_of_match);
    static struct platform_driver mmp2_audio_clk_driver = {
    .driver = {
    .name = "mmp2-audio-clock",
    .of_match_table = of_match_ptr(mmp2_audio_clk_of_match),
    .pm = &mmp2_audio_clk_pm_ops,
    },
    .probe = mmp2_audio_clk_probe,
    .remove = mmp2_audio_clk_remove,
    };
    module_platform_driver(mmp2_audio_clk_driver);
    MODULE_AUTHOR("Lubomir Rintel <lkundrak@v3.sk>");
    MODULE_DESCRIPTION("Clock driver for MMP2 Audio subsystem");
    MODULE_LICENSE("GPL");
