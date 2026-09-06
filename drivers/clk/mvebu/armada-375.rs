//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mvebu/armada-375.c
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
// Marvell Armada 375 SoC clocks
//
// Copyright (C) 2014 Marvell
//
// Gregory CLEMENT <gregory.clement@free-electrons.com>
// Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
// Andrew Lunn <andrew@lunn.ch>
//

//
// Core Clocks
//
// For the Armada 375 SoCs, the CPU, DDR and L2 clocks frequencies are
// all modified at the same time, and not separately as for the Armada
// 370 or the Armada XP SoCs.
//
// SAR1[21:17]   : CPU frequency    DDR frequency   L2 frequency
// 6   =  400 MHz	    400 MHz	    200 MHz
// 15  =  600 MHz	    600 MHz	    300 MHz
// 21  =  800 MHz	    534 MHz	    400 MHz
// 25  = 1000 MHz	    500 MHz	    500 MHz
// others reserved.
//
// SAR1[22]   : TCLK frequency
// 0 = 166 MHz
// 1 = 200 MHz
//
pub const SAR1_A375_TCLK_FREQ_OPT: c_int = 22;
pub const SAR1_A375_TCLK_FREQ_OPT_MASK: c_uint = 0x1;
pub const SAR1_A375_CPU_DDR_L2_FREQ_OPT: c_int = 17;
pub const SAR1_A375_CPU_DDR_L2_FREQ_OPT_MASK: c_uint = 0x1F;
    static const u32 armada_375_tclk_frequencies[] __initconst = {
    166000000,
    200000000,
    };
#[no_mangle]
unsafe extern "C" fn armada_375_get_tclk_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init armada_375_get_tclk_freq(void __iomem *sar)
    {
    u8 tclk_freq_select;
    tclk_freq_select = ((readl(sar) >> SAR1_A375_TCLK_FREQ_OPT) &
    SAR1_A375_TCLK_FREQ_OPT_MASK);
    return armada_375_tclk_frequencies[tclk_freq_select];
    }
    static const u32 armada_375_cpu_frequencies[] __initconst = {
    0, 0, 0, 0, 0, 0,
    400000000,
    0, 0, 0, 0, 0, 0, 0, 0,
    600000000,
    0, 0, 0, 0, 0,
    800000000,
    0, 0, 0,
    1000000000,
    };
#[no_mangle]
unsafe extern "C" fn armada_375_get_cpu_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init armada_375_get_cpu_freq(void __iomem *sar)
    {
    u8 cpu_freq_select;
    cpu_freq_select = ((readl(sar) >> SAR1_A375_CPU_DDR_L2_FREQ_OPT) &
    SAR1_A375_CPU_DDR_L2_FREQ_OPT_MASK);
    if (cpu_freq_select >= ARRAY_SIZE(armada_375_cpu_frequencies)) {
    pr_err("Selected CPU frequency (%d) unsupported\n",
    cpu_freq_select);
    return 0;
    } else
    return armada_375_cpu_frequencies[cpu_freq_select];
    }
    enum { A375_CPU_TO_DDR, A375_CPU_TO_L2 };
    static const struct coreclk_ratio armada_375_coreclk_ratios[] __initconst = {
    { .id = A375_CPU_TO_L2,	 .name = "l2clk" },
    { .id = A375_CPU_TO_DDR, .name = "ddrclk" },
    };
    static const int armada_375_cpu_l2_ratios[32][2] __initconst = {
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    {0, 1}, {0, 1}, {1, 2}, {0, 1},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    {0, 1}, {0, 1}, {0, 1}, {1, 2},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    {0, 1}, {1, 2}, {0, 1}, {0, 1},
    {0, 1}, {1, 2}, {0, 1}, {0, 1},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    };
    static const int armada_375_cpu_ddr_ratios[32][2] __initconst = {
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    {0, 1}, {0, 1}, {1, 1}, {0, 1},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    {0, 1}, {0, 1}, {0, 1}, {2, 3},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    {0, 1}, {2, 3}, {0, 1}, {0, 1},
    {0, 1}, {1, 2}, {0, 1}, {0, 1},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    };
    static void __init armada_375_get_clk_ratio(
    void __iomem *sar, int id, int *mult, int *div)
    {
    u32 opt = ((readl(sar) >> SAR1_A375_CPU_DDR_L2_FREQ_OPT) &
    SAR1_A375_CPU_DDR_L2_FREQ_OPT_MASK);
    switch (id) {
    case A375_CPU_TO_L2:
// mult = armada_375_cpu_l2_ratios[opt][0];
// div = armada_375_cpu_l2_ratios[opt][1];
    break;
    case A375_CPU_TO_DDR:
// mult = armada_375_cpu_ddr_ratios[opt][0];
// div = armada_375_cpu_ddr_ratios[opt][1];
    break;
    }
    }
    static const struct coreclk_soc_desc armada_375_coreclks = {
    .get_tclk_freq = armada_375_get_tclk_freq,
    .get_cpu_freq = armada_375_get_cpu_freq,
    .get_clk_ratio = armada_375_get_clk_ratio,
    .ratios = armada_375_coreclk_ratios,
    .num_ratios = ARRAY_SIZE(armada_375_coreclk_ratios),
    };
#[no_mangle]
unsafe extern "C" fn armada_375_coreclk_init(np: *mut device_node) -> void __init {
    static void __init armada_375_coreclk_init(struct device_node *np)
    {
    mvebu_coreclk_setup(np, &armada_375_coreclks);
    }
    CLK_OF_DECLARE(armada_375_core_clk, "marvell,armada-375-core-clock",
    armada_375_coreclk_init);
//
// Clock Gating Control
//
    static const struct clk_gating_soc_desc armada_375_gating_desc[] __initconst = {
    { "mu", core::ptr::null_mut(), 2 },
    { "pp", core::ptr::null_mut(), 3 },
    { "ptp", core::ptr::null_mut(), 4 },
    { "pex0", core::ptr::null_mut(), 5 },
    { "pex1", core::ptr::null_mut(), 6 },
    { "audio", core::ptr::null_mut(), 8 },
    { "nd_clk", "nand", 11 },
    { "sata0_link", "sata0_core", 14 },
    { "sata0_core", core::ptr::null_mut(), 15 },
    { "usb3", core::ptr::null_mut(), 16 },
    { "sdio", core::ptr::null_mut(), 17 },
    { "usb", core::ptr::null_mut(), 18 },
    { "gop", core::ptr::null_mut(), 19 },
    { "sata1_link", "sata1_core", 20 },
    { "sata1_core", core::ptr::null_mut(), 21 },
    { "xor0", core::ptr::null_mut(), 22 },
    { "xor1", core::ptr::null_mut(), 23 },
    { "copro", core::ptr::null_mut(), 24 },
    { "tdm", core::ptr::null_mut(), 25 },
    { "crypto0_enc", core::ptr::null_mut(), 28 },
    { "crypto0_core", core::ptr::null_mut(), 29 },
    { "crypto1_enc", core::ptr::null_mut(), 30 },
    { "crypto1_core", core::ptr::null_mut(), 31 },
    { }
    };
#[no_mangle]
unsafe extern "C" fn armada_375_clk_gating_init(np: *mut device_node) -> void __init {
    static void __init armada_375_clk_gating_init(struct device_node *np)
    {
    mvebu_clk_gating_setup(np, armada_375_gating_desc);
    }
    CLK_OF_DECLARE(armada_375_clk_gating, "marvell,armada-375-gating-clock",
    armada_375_clk_gating_init);
