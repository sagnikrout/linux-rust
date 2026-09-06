//! Automatically rewritten from C to Rust
//! Source: drivers/clk/hisilicon/clk-hi3519.c
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
// Hi3519 Clock Driver
//
// Copyright (c) 2015-2016 HiSilicon Technologies Co., Ltd.
//

pub const HI3519_INNER_CLK_OFFSET: c_int = 64;
pub const HI3519_FIXED_24M: c_int = 65;
pub const HI3519_FIXED_50M: c_int = 66;
pub const HI3519_FIXED_75M: c_int = 67;
pub const HI3519_FIXED_125M: c_int = 68;
pub const HI3519_FIXED_150M: c_int = 69;
pub const HI3519_FIXED_200M: c_int = 70;
pub const HI3519_FIXED_250M: c_int = 71;
pub const HI3519_FIXED_300M: c_int = 72;
pub const HI3519_FIXED_400M: c_int = 73;
pub const HI3519_FMC_MUX: c_int = 74;
pub const HI3519_NR_CLKS: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi3519_crg_data {
    pub clk_data: *mut hisi_clock_data,
    pub rstc: *mut hisi_reset_controller,
}

    static const struct hisi_fixed_rate_clock hi3519_fixed_rate_clks[] = {
    { HI3519_FIXED_24M, "24m", core::ptr::null_mut(), 0, 24000000, },
    { HI3519_FIXED_50M, "50m", core::ptr::null_mut(), 0, 50000000, },
    { HI3519_FIXED_75M, "75m", core::ptr::null_mut(), 0, 75000000, },
    { HI3519_FIXED_125M, "125m", core::ptr::null_mut(), 0, 125000000, },
    { HI3519_FIXED_150M, "150m", core::ptr::null_mut(), 0, 150000000, },
    { HI3519_FIXED_200M, "200m", core::ptr::null_mut(), 0, 200000000, },
    { HI3519_FIXED_250M, "250m", core::ptr::null_mut(), 0, 250000000, },
    { HI3519_FIXED_300M, "300m", core::ptr::null_mut(), 0, 300000000, },
    { HI3519_FIXED_400M, "400m", core::ptr::null_mut(), 0, 400000000, },
    };
    static const char *const fmc_mux_p[] = {
    "24m", "75m", "125m", "150m", "200m", "250m", "300m", "400m", };
    static u32 fmc_mux_table[] = {0, 1, 2, 3, 4, 5, 6, 7};
    static const struct hisi_mux_clock hi3519_mux_clks[] = {
    { HI3519_FMC_MUX, "fmc_mux", fmc_mux_p, ARRAY_SIZE(fmc_mux_p),
    CLK_SET_RATE_PARENT, 0xc0, 2, 3, 0, fmc_mux_table, },
    };
    static const struct hisi_gate_clock hi3519_gate_clks[] = {
    { HI3519_FMC_CLK, "clk_fmc", "fmc_mux",
    CLK_SET_RATE_PARENT, 0xc0, 1, 0, },
    { HI3519_UART0_CLK, "clk_uart0", "24m",
    CLK_SET_RATE_PARENT, 0xe4, 20, 0, },
    { HI3519_UART1_CLK, "clk_uart1", "24m",
    CLK_SET_RATE_PARENT, 0xe4, 21, 0, },
    { HI3519_UART2_CLK, "clk_uart2", "24m",
    CLK_SET_RATE_PARENT, 0xe4, 22, 0, },
    { HI3519_UART3_CLK, "clk_uart3", "24m",
    CLK_SET_RATE_PARENT, 0xe4, 23, 0, },
    { HI3519_UART4_CLK, "clk_uart4", "24m",
    CLK_SET_RATE_PARENT, 0xe4, 24, 0, },
    { HI3519_SPI0_CLK, "clk_spi0", "50m",
    CLK_SET_RATE_PARENT, 0xe4, 16, 0, },
    { HI3519_SPI1_CLK, "clk_spi1", "50m",
    CLK_SET_RATE_PARENT, 0xe4, 17, 0, },
    { HI3519_SPI2_CLK, "clk_spi2", "50m",
    CLK_SET_RATE_PARENT, 0xe4, 18, 0, },
    };
    static struct hisi_clock_data *hi3519_clk_register(struct platform_device *pdev)
    {
    struct hisi_clock_data *clk_data;
    int ret;
    clk_data = hisi_clk_alloc(pdev, HI3519_NR_CLKS);
    if (!clk_data)
    return ERR_PTR(-ENOMEM);
    ret = hisi_clk_register_fixed_rate(hi3519_fixed_rate_clks,
    ARRAY_SIZE(hi3519_fixed_rate_clks),
    clk_data);
    if (ret)
    return ERR_PTR(ret);
    ret = hisi_clk_register_mux(hi3519_mux_clks,
    ARRAY_SIZE(hi3519_mux_clks),
    clk_data);
    if (ret)
    goto unregister_fixed_rate;
    ret = hisi_clk_register_gate(hi3519_gate_clks,
    ARRAY_SIZE(hi3519_gate_clks),
    clk_data);
    if (ret)
    goto unregister_mux;
    ret = of_clk_add_provider(pdev.dev.of_node,
    of_clk_src_onecell_get, &clk_data.clk_data);
    if (ret)
    goto unregister_gate;
    return clk_data;
    unregister_fixed_rate:
    hisi_clk_unregister_fixed_rate(hi3519_fixed_rate_clks,
    ARRAY_SIZE(hi3519_fixed_rate_clks),
    clk_data);
    unregister_mux:
    hisi_clk_unregister_mux(hi3519_mux_clks,
    ARRAY_SIZE(hi3519_mux_clks),
    clk_data);
    unregister_gate:
    hisi_clk_unregister_gate(hi3519_gate_clks,
    ARRAY_SIZE(hi3519_gate_clks),
    clk_data);
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn hi3519_clk_unregister(pdev: *mut platform_device) {
    static void hi3519_clk_unregister(struct platform_device *pdev)
    {
    struct hi3519_crg_data *crg = platform_get_drvdata(pdev);
    of_clk_del_provider(pdev.dev.of_node);
    hisi_clk_unregister_gate(hi3519_gate_clks,
    ARRAY_SIZE(hi3519_gate_clks),
    crg.clk_data);
    hisi_clk_unregister_mux(hi3519_mux_clks,
    ARRAY_SIZE(hi3519_mux_clks),
    crg.clk_data);
    hisi_clk_unregister_fixed_rate(hi3519_fixed_rate_clks,
    ARRAY_SIZE(hi3519_fixed_rate_clks),
    crg.clk_data);
    }
#[no_mangle]
unsafe extern "C" fn hi3519_clk_probe(pdev: *mut platform_device) -> c_int {
    static int hi3519_clk_probe(struct platform_device *pdev)
    {
    struct hi3519_crg_data *crg;
    crg = devm_kmalloc(&pdev.dev, sizeof(*crg), GFP_KERNEL);
    if (!crg)
    return -ENOMEM;
    crg.rstc = hisi_reset_init(pdev);
    if (!crg.rstc)
    return -ENOMEM;
    crg.clk_data = hi3519_clk_register(pdev);
    if (IS_ERR(crg.clk_data)) {
    hisi_reset_exit(crg.rstc);
    return PTR_ERR(crg.clk_data);
    }
    platform_set_drvdata(pdev, crg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hi3519_clk_remove(pdev: *mut platform_device) {
    static void hi3519_clk_remove(struct platform_device *pdev)
    {
    struct hi3519_crg_data *crg = platform_get_drvdata(pdev);
    hisi_reset_exit(crg.rstc);
    hi3519_clk_unregister(pdev);
    }
    static const struct of_device_id hi3519_clk_match_table[] = {
    { .compatible = "hisilicon,hi3519-crg" },
    { }
    };
    MODULE_DEVICE_TABLE(of, hi3519_clk_match_table);
    static struct platform_driver hi3519_clk_driver = {
    .probe          = hi3519_clk_probe,
    .remove		= hi3519_clk_remove,
    .driver         = {
    .name   = "hi3519-clk",
    .of_match_table = hi3519_clk_match_table,
    },
    };
#[no_mangle]
unsafe extern "C" fn hi3519_clk_init() -> int __init {
    static int __init hi3519_clk_init(void)
    {
    return platform_driver_register(&hi3519_clk_driver);
    }
    core_initcall(hi3519_clk_init);
#[no_mangle]
unsafe extern "C" fn hi3519_clk_exit() -> void __exit {
    static void __exit hi3519_clk_exit(void)
    {
    platform_driver_unregister(&hi3519_clk_driver);
    }
    module_exit(hi3519_clk_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("HiSilicon Hi3519 Clock Driver");
