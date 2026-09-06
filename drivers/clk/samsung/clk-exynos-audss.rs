//! Automatically rewritten from C to Rust
//! Source: drivers/clk/samsung/clk-exynos-audss.c
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
// Copyright (c) 2013 Samsung Electronics Co., Ltd.
// Author: Padmavathi Venna <padma.v@samsung.com>
//
// Common Clock Framework support for Audio Subsystem Clock Controller.
//

    static DEFINE_SPINLOCK(lock);
    static void __iomem *reg_base;
    static struct clk_hw_onecell_data *clk_data;
//
// On Exynos5420 this will be a clock which has to be enabled before any
// access to audss registers. Typically a child of EPLL.
//
// On other platforms this will be -ENODEV.
//
    static struct clk *epll;
pub const ASS_CLK_SRC: c_uint = 0x0;
pub const ASS_CLK_DIV: c_uint = 0x4;
pub const ASS_CLK_GATE: c_uint = 0x8;
    static unsigned long reg_save[][2] = {
    { ASS_CLK_SRC,  0 },
    { ASS_CLK_DIV,  0 },
    { ASS_CLK_GATE, 0 },
    };
#[no_mangle]
unsafe extern "C" fn exynos_audss_clk_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused exynos_audss_clk_suspend(struct device *dev)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(reg_save); i++)
    reg_save[i][1] = readl(reg_base + reg_save[i][0]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos_audss_clk_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused exynos_audss_clk_resume(struct device *dev)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(reg_save); i++)
    writel(reg_save[i][1], reg_base + reg_save[i][0]);
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_audss_clk_drvdata {
    pub has_adma_clk:1: c_uint,
    pub has_mst_clk:1: c_uint,
    pub enable_epll:1: c_uint,
    pub num_clks: c_uint,
}

    static const struct exynos_audss_clk_drvdata exynos4210_drvdata = {
    .num_clks	= EXYNOS_AUDSS_MAX_CLKS - 1,
    .enable_epll	= 1,
    };
    static const struct exynos_audss_clk_drvdata exynos5410_drvdata = {
    .num_clks	= EXYNOS_AUDSS_MAX_CLKS - 1,
    .has_mst_clk	= 1,
    };
    static const struct exynos_audss_clk_drvdata exynos5420_drvdata = {
    .num_clks	= EXYNOS_AUDSS_MAX_CLKS,
    .has_adma_clk	= 1,
    .enable_epll	= 1,
    };
    static const struct of_device_id exynos_audss_clk_of_match[] = {
    {
    .compatible	= "samsung,exynos4210-audss-clock",
    .data		= &exynos4210_drvdata,
    }, {
    .compatible	= "samsung,exynos5250-audss-clock",
    .data		= &exynos4210_drvdata,
    }, {
    .compatible	= "samsung,exynos5410-audss-clock",
    .data		= &exynos5410_drvdata,
    }, {
    .compatible	= "samsung,exynos5420-audss-clock",
    .data		= &exynos5420_drvdata,
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, exynos_audss_clk_of_match);
#[no_mangle]
unsafe extern "C" fn exynos_audss_clk_teardown() {
    static void exynos_audss_clk_teardown(void)
    {
    int i;
    for (i = EXYNOS_MOUT_AUDSS; i < EXYNOS_DOUT_SRP; i++) {
    if (!IS_ERR(clk_data.hws[i]))
    clk_hw_unregister_mux(clk_data.hws[i]);
    }
    for (; i < EXYNOS_SRP_CLK; i++) {
    if (!IS_ERR(clk_data.hws[i]))
    clk_hw_unregister_divider(clk_data.hws[i]);
    }
    for (; i < clk_data.num; i++) {
    if (!IS_ERR(clk_data.hws[i]))
    clk_hw_unregister_gate(clk_data.hws[i]);
    }
    }
// register exynos_audss clocks
#[no_mangle]
unsafe extern "C" fn exynos_audss_clk_probe(pdev: *mut platform_device) -> c_int {
    static int exynos_audss_clk_probe(struct platform_device *pdev)
    {
    const char *mout_audss_p[] = {"fin_pll", "fout_epll"};
    const char *mout_i2s_p[] = {"mout_audss", "cdclk0", "sclk_audio0"};
    const char *sclk_pcm_p = "sclk_pcm0";
    struct clk *pll_ref, *pll_in, *cdclk, *sclk_audio, *sclk_pcm_in;
    const struct exynos_audss_clk_drvdata *variant;
    struct clk_hw **clk_table;
    struct device *dev = &pdev.dev;
    int i, ret = 0;
    variant = of_device_get_match_data(&pdev.dev);
    if (!variant)
    return -EINVAL;
    reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(reg_base))
    return PTR_ERR(reg_base);
    epll = ERR_PTR(-ENODEV);
    clk_data = devm_kzalloc(dev,
    struct_size(clk_data, hws,
    EXYNOS_AUDSS_MAX_CLKS),
    GFP_KERNEL);
    if (!clk_data)
    return -ENOMEM;
    clk_data.num = variant.num_clks;
    clk_table = clk_data.hws;
    pll_ref = devm_clk_get(dev, "pll_ref");
    pll_in = devm_clk_get(dev, "pll_in");
    if (!IS_ERR(pll_ref))
    mout_audss_p[0] = __clk_get_name(pll_ref);
    if (!IS_ERR(pll_in)) {
    mout_audss_p[1] = __clk_get_name(pll_in);
    if (variant.enable_epll) {
    epll = pll_in;
    ret = clk_prepare_enable(epll);
    if (ret) {
    dev_err(dev,
    "failed to prepare the epll clock\n");
    return ret;
    }
    }
    }
//
// Enable runtime PM here to allow the clock core using runtime PM
// for the registered clocks. Additionally, we increase the runtime
// PM usage count before registering the clocks, to prevent the
// clock core from runtime suspending the device.
//
    pm_runtime_get_noresume(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    clk_table[EXYNOS_MOUT_AUDSS] = clk_hw_register_mux(dev, "mout_audss",
    mout_audss_p, ARRAY_SIZE(mout_audss_p),
    CLK_SET_RATE_NO_REPARENT | CLK_SET_RATE_PARENT,
    reg_base + ASS_CLK_SRC, 0, 1, 0, &lock);
    cdclk = devm_clk_get(dev, "cdclk");
    sclk_audio = devm_clk_get(dev, "sclk_audio");
    if (!IS_ERR(cdclk))
    mout_i2s_p[1] = __clk_get_name(cdclk);
    if (!IS_ERR(sclk_audio))
    mout_i2s_p[2] = __clk_get_name(sclk_audio);
    clk_table[EXYNOS_MOUT_I2S] = clk_hw_register_mux(dev, "mout_i2s",
    mout_i2s_p, ARRAY_SIZE(mout_i2s_p),
    CLK_SET_RATE_NO_REPARENT,
    reg_base + ASS_CLK_SRC, 2, 2, 0, &lock);
    clk_table[EXYNOS_DOUT_SRP] = clk_hw_register_divider(dev, "dout_srp",
    "mout_audss", CLK_SET_RATE_PARENT,
    reg_base + ASS_CLK_DIV, 0, 4, 0, &lock);
    clk_table[EXYNOS_DOUT_AUD_BUS] = clk_hw_register_divider(dev,
    "dout_aud_bus", "dout_srp", CLK_SET_RATE_PARENT,
    reg_base + ASS_CLK_DIV, 4, 4, 0, &lock);
    clk_table[EXYNOS_DOUT_I2S] = clk_hw_register_divider(dev, "dout_i2s",
    "mout_i2s", 0, reg_base + ASS_CLK_DIV, 8, 4, 0,
    &lock);
    clk_table[EXYNOS_SRP_CLK] = clk_hw_register_gate(dev, "srp_clk",
    "dout_srp", CLK_SET_RATE_PARENT,
    reg_base + ASS_CLK_GATE, 0, 0, &lock);
    clk_table[EXYNOS_I2S_BUS] = clk_hw_register_gate(dev, "i2s_bus",
    "dout_aud_bus", CLK_SET_RATE_PARENT,
    reg_base + ASS_CLK_GATE, 2, 0, &lock);
    clk_table[EXYNOS_SCLK_I2S] = clk_hw_register_gate(dev, "sclk_i2s",
    "dout_i2s", CLK_SET_RATE_PARENT,
    reg_base + ASS_CLK_GATE, 3, 0, &lock);
    clk_table[EXYNOS_PCM_BUS] = clk_hw_register_gate(dev, "pcm_bus",
    "sclk_pcm", CLK_SET_RATE_PARENT,
    reg_base + ASS_CLK_GATE, 4, 0, &lock);
    sclk_pcm_in = devm_clk_get(dev, "sclk_pcm_in");
    if (!IS_ERR(sclk_pcm_in))
    sclk_pcm_p = __clk_get_name(sclk_pcm_in);
    clk_table[EXYNOS_SCLK_PCM] = clk_hw_register_gate(dev, "sclk_pcm",
    sclk_pcm_p, CLK_SET_RATE_PARENT,
    reg_base + ASS_CLK_GATE, 5, 0, &lock);
    if (variant.has_adma_clk) {
    clk_table[EXYNOS_ADMA] = clk_hw_register_gate(dev, "adma",
    "dout_srp", CLK_SET_RATE_PARENT,
    reg_base + ASS_CLK_GATE, 9, 0, &lock);
    }
    for (i = 0; i < clk_data.num; i++) {
    if (IS_ERR(clk_table[i])) {
    dev_err(dev, "failed to register clock %d\n", i);
    ret = PTR_ERR(clk_table[i]);
    goto unregister;
    }
    }
    ret = of_clk_add_hw_provider(dev.of_node, of_clk_hw_onecell_get,
    clk_data);
    if (ret) {
    dev_err(dev, "failed to add clock provider\n");
    goto unregister;
    }
    pm_runtime_put_sync(dev);
    return 0;
    unregister:
    exynos_audss_clk_teardown();
    pm_runtime_put_sync(dev);
    pm_runtime_disable(dev);
    if (!IS_ERR(epll))
    clk_disable_unprepare(epll);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exynos_audss_clk_remove(pdev: *mut platform_device) {
    static void exynos_audss_clk_remove(struct platform_device *pdev)
    {
    of_clk_del_provider(pdev.dev.of_node);
    exynos_audss_clk_teardown();
    pm_runtime_disable(&pdev.dev);
    if (!IS_ERR(epll))
    clk_disable_unprepare(epll);
    }
    static const struct dev_pm_ops exynos_audss_clk_pm_ops = {
    SET_RUNTIME_PM_OPS(exynos_audss_clk_suspend, exynos_audss_clk_resume,
    core::ptr::null_mut())
    SET_LATE_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    };
    static struct platform_driver exynos_audss_clk_driver = {
    .driver	= {
    .name = "exynos-audss-clk",
    .of_match_table = exynos_audss_clk_of_match,
    .pm = &exynos_audss_clk_pm_ops,
    },
    .probe = exynos_audss_clk_probe,
    .remove = exynos_audss_clk_remove,
    };
    module_platform_driver(exynos_audss_clk_driver);
    MODULE_AUTHOR("Padmavathi Venna <padma.v@samsung.com>");
    MODULE_DESCRIPTION("Exynos Audio Subsystem Clock Controller");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:exynos-audss-clk");
