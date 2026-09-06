//! Automatically rewritten from C to Rust
//! Source: drivers/clk/samsung/clk-s5pv210-audss.c
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
// Copyright (c) 2014 Tomasz Figa <t.figa@samsung.com>
//
// Based on Exynos Audio Subsystem Clock Controller driver:
//
// Copyright (c) 2013 Samsung Electronics Co., Ltd.
// Author: Padmavathi Venna <padma.v@samsung.com>
//
// Driver for Audio Subsystem Clock Controller of S5PV210-compatible SoCs.
//

    static DEFINE_SPINLOCK(lock);
    static void __iomem *reg_base;
    static struct clk_hw_onecell_data *clk_data;
pub const ASS_CLK_SRC: c_uint = 0x0;
pub const ASS_CLK_DIV: c_uint = 0x4;
pub const ASS_CLK_GATE: c_uint = 0x8;

    static unsigned long reg_save[][2] = {
    {ASS_CLK_SRC,  0},
    {ASS_CLK_DIV,  0},
    {ASS_CLK_GATE, 0},
    };
#[no_mangle]
unsafe extern "C" fn s5pv210_audss_clk_suspend(data: *mut c_void) -> c_int {
    static int s5pv210_audss_clk_suspend(void *data)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(reg_save); i++)
    reg_save[i][1] = readl(reg_base + reg_save[i][0]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s5pv210_audss_clk_resume(data: *mut c_void) {
    static void s5pv210_audss_clk_resume(void *data)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(reg_save); i++)
    writel(reg_save[i][1], reg_base + reg_save[i][0]);
    }
    static const struct syscore_ops s5pv210_audss_clk_syscore_ops = {
    .suspend	= s5pv210_audss_clk_suspend,
    .resume		= s5pv210_audss_clk_resume,
    };
    static struct syscore s5pv210_audss_clk_syscore = {
    .ops = &s5pv210_audss_clk_syscore_ops,
    };

// register s5pv210_audss clocks
#[no_mangle]
unsafe extern "C" fn s5pv210_audss_clk_probe(pdev: *mut platform_device) -> c_int {
    static int s5pv210_audss_clk_probe(struct platform_device *pdev)
    {
    int i, ret = 0;
    const char *mout_audss_p[2];
    const char *mout_i2s_p[3];
    const char *hclk_p;
    struct clk_hw **clk_table;
    struct clk *hclk, *pll_ref, *pll_in, *cdclk, *sclk_audio;
    reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(reg_base))
    return PTR_ERR(reg_base);
    clk_data = devm_kzalloc(&pdev.dev,
    struct_size(clk_data, hws, AUDSS_MAX_CLKS),
    GFP_KERNEL);
    if (!clk_data)
    return -ENOMEM;
    clk_data.num = AUDSS_MAX_CLKS;
    clk_table = clk_data.hws;
    hclk = devm_clk_get(&pdev.dev, "hclk");
    if (IS_ERR(hclk)) {
    dev_err(&pdev.dev, "failed to get hclk clock\n");
    return PTR_ERR(hclk);
    }
    pll_in = devm_clk_get(&pdev.dev, "fout_epll");
    if (IS_ERR(pll_in)) {
    dev_err(&pdev.dev, "failed to get fout_epll clock\n");
    return PTR_ERR(pll_in);
    }
    sclk_audio = devm_clk_get(&pdev.dev, "sclk_audio0");
    if (IS_ERR(sclk_audio)) {
    dev_err(&pdev.dev, "failed to get sclk_audio0 clock\n");
    return PTR_ERR(sclk_audio);
    }
// iiscdclk0 is an optional external I2S codec clock
    cdclk = devm_clk_get(&pdev.dev, "iiscdclk0");
    pll_ref = devm_clk_get(&pdev.dev, "xxti");
    if (!IS_ERR(pll_ref))
    mout_audss_p[0] = __clk_get_name(pll_ref);
    else
    mout_audss_p[0] = "xxti";
    mout_audss_p[1] = __clk_get_name(pll_in);
    clk_table[CLK_MOUT_AUDSS] = clk_hw_register_mux(core::ptr::null_mut(), "mout_audss",
    mout_audss_p, ARRAY_SIZE(mout_audss_p),
    CLK_SET_RATE_NO_REPARENT,
    reg_base + ASS_CLK_SRC, 0, 1, 0, &lock);
    mout_i2s_p[0] = "mout_audss";
    if (!IS_ERR(cdclk))
    mout_i2s_p[1] = __clk_get_name(cdclk);
    else
    mout_i2s_p[1] = "iiscdclk0";
    mout_i2s_p[2] = __clk_get_name(sclk_audio);
    clk_table[CLK_MOUT_I2S_A] = clk_hw_register_mux(core::ptr::null_mut(), "mout_i2s_audss",
    mout_i2s_p, ARRAY_SIZE(mout_i2s_p),
    CLK_SET_RATE_NO_REPARENT,
    reg_base + ASS_CLK_SRC, 2, 2, 0, &lock);
    clk_table[CLK_DOUT_AUD_BUS] = clk_hw_register_divider(core::ptr::null_mut(),
    "dout_aud_bus", "mout_audss", 0,
    reg_base + ASS_CLK_DIV, 0, 4, 0, &lock);
    clk_table[CLK_DOUT_I2S_A] = clk_hw_register_divider(core::ptr::null_mut(),
    "dout_i2s_audss", "mout_i2s_audss", 0,
    reg_base + ASS_CLK_DIV, 4, 4, 0, &lock);
    clk_table[CLK_I2S] = clk_hw_register_gate(core::ptr::null_mut(), "i2s_audss",
    "dout_i2s_audss", CLK_SET_RATE_PARENT,
    reg_base + ASS_CLK_GATE, 6, 0, &lock);
    hclk_p = __clk_get_name(hclk);
    clk_table[CLK_HCLK_I2S] = clk_hw_register_gate(core::ptr::null_mut(), "hclk_i2s_audss",
    hclk_p, CLK_IGNORE_UNUSED,
    reg_base + ASS_CLK_GATE, 5, 0, &lock);
    clk_table[CLK_HCLK_UART] = clk_hw_register_gate(core::ptr::null_mut(), "hclk_uart_audss",
    hclk_p, CLK_IGNORE_UNUSED,
    reg_base + ASS_CLK_GATE, 4, 0, &lock);
    clk_table[CLK_HCLK_HWA] = clk_hw_register_gate(core::ptr::null_mut(), "hclk_hwa_audss",
    hclk_p, CLK_IGNORE_UNUSED,
    reg_base + ASS_CLK_GATE, 3, 0, &lock);
    clk_table[CLK_HCLK_DMA] = clk_hw_register_gate(core::ptr::null_mut(), "hclk_dma_audss",
    hclk_p, CLK_IGNORE_UNUSED,
    reg_base + ASS_CLK_GATE, 2, 0, &lock);
    clk_table[CLK_HCLK_BUF] = clk_hw_register_gate(core::ptr::null_mut(), "hclk_buf_audss",
    hclk_p, CLK_IGNORE_UNUSED,
    reg_base + ASS_CLK_GATE, 1, 0, &lock);
    clk_table[CLK_HCLK_RP] = clk_hw_register_gate(core::ptr::null_mut(), "hclk_rp_audss",
    hclk_p, CLK_IGNORE_UNUSED,
    reg_base + ASS_CLK_GATE, 0, 0, &lock);
    for (i = 0; i < clk_data.num; i++) {
    if (IS_ERR(clk_table[i])) {
    dev_err(&pdev.dev, "failed to register clock %d\n", i);
    ret = PTR_ERR(clk_table[i]);
    goto unregister;
    }
    }
    ret = of_clk_add_hw_provider(pdev.dev.of_node, of_clk_hw_onecell_get,
    clk_data);
    if (ret) {
    dev_err(&pdev.dev, "failed to add clock provider\n");
    goto unregister;
    }

    register_syscore(&s5pv210_audss_clk_syscore);

    return 0;
    unregister:
    for (i = 0; i < clk_data.num; i++) {
    if (!IS_ERR(clk_table[i]))
    clk_hw_unregister(clk_table[i]);
    }
    return ret;
    }
    static const struct of_device_id s5pv210_audss_clk_of_match[] = {
    { .compatible = "samsung,s5pv210-audss-clock", },
    {},
    };
    static struct platform_driver s5pv210_audss_clk_driver = {
    .driver	= {
    .name = "s5pv210-audss-clk",
    .suppress_bind_attrs = true,
    .of_match_table = s5pv210_audss_clk_of_match,
    },
    .probe = s5pv210_audss_clk_probe,
    };
#[no_mangle]
unsafe extern "C" fn s5pv210_audss_clk_init() -> int __init {
    static int __init s5pv210_audss_clk_init(void)
    {
    return platform_driver_register(&s5pv210_audss_clk_driver);
    }
    core_initcall(s5pv210_audss_clk_init);
