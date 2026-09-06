//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mvebu/armada-38x.c
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
// Marvell Armada 380/385 SoC clocks
//
// Copyright (C) 2014 Marvell
//
// Gregory CLEMENT <gregory.clement@free-electrons.com>
// Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
// Andrew Lunn <andrew@lunn.ch>
//

//
// SAR[14:10] : Ratios between PCLK0, NBCLK, HCLK and DRAM clocks
//
// SAR[15]    : TCLK frequency
// 0 = 250 MHz
// 1 = 200 MHz
//
pub const SAR_A380_TCLK_FREQ_OPT: c_int = 15;
pub const SAR_A380_TCLK_FREQ_OPT_MASK: c_uint = 0x1;
pub const SAR_A380_CPU_DDR_L2_FREQ_OPT: c_int = 10;
pub const SAR_A380_CPU_DDR_L2_FREQ_OPT_MASK: c_uint = 0x1F;
    static const u32 armada_38x_tclk_frequencies[] __initconst = {
    250000000,
    200000000,
    };
#[no_mangle]
unsafe extern "C" fn armada_38x_get_tclk_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init armada_38x_get_tclk_freq(void __iomem *sar)
    {
    u8 tclk_freq_select;
    tclk_freq_select = ((readl(sar) >> SAR_A380_TCLK_FREQ_OPT) &
    SAR_A380_TCLK_FREQ_OPT_MASK);
    return armada_38x_tclk_frequencies[tclk_freq_select];
    }
    static const u32 armada_38x_cpu_frequencies[] __initconst = {
    666 * 1000 * 1000,  0, 800 * 1000 * 1000, 0,
    1066 * 1000 * 1000, 0, 1200 * 1000 * 1000, 0,
    1332 * 1000 * 1000, 0, 0, 0,
    1600 * 1000 * 1000, 0, 0, 0,
    1866 * 1000 * 1000, 0, 0, 2000 * 1000 * 1000,
    };
#[no_mangle]
unsafe extern "C" fn armada_38x_get_cpu_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init armada_38x_get_cpu_freq(void __iomem *sar)
    {
    u8 cpu_freq_select;
    cpu_freq_select = ((readl(sar) >> SAR_A380_CPU_DDR_L2_FREQ_OPT) &
    SAR_A380_CPU_DDR_L2_FREQ_OPT_MASK);
    if (cpu_freq_select >= ARRAY_SIZE(armada_38x_cpu_frequencies)) {
    pr_err("Selected CPU frequency (%d) unsupported\n",
    cpu_freq_select);
    return 0;
    }
    return armada_38x_cpu_frequencies[cpu_freq_select];
    }
    enum { A380_CPU_TO_DDR, A380_CPU_TO_L2 };
    static const struct coreclk_ratio armada_38x_coreclk_ratios[] __initconst = {
    { .id = A380_CPU_TO_L2,	 .name = "l2clk" },
    { .id = A380_CPU_TO_DDR, .name = "ddrclk" },
    };
    static const int armada_38x_cpu_l2_ratios[32][2] __initconst = {
    {1, 2}, {0, 1}, {1, 2}, {0, 1},
    {1, 2}, {0, 1}, {1, 2}, {0, 1},
    {1, 2}, {0, 1}, {0, 1}, {0, 1},
    {1, 2}, {0, 1}, {0, 1}, {0, 1},
    {1, 2}, {0, 1}, {0, 1}, {1, 2},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    };
    static const int armada_38x_cpu_ddr_ratios[32][2] __initconst = {
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    {1, 2}, {0, 1}, {0, 1}, {0, 1},
    {1, 2}, {0, 1}, {0, 1}, {0, 1},
    {1, 2}, {0, 1}, {0, 1}, {0, 1},
    {1, 2}, {0, 1}, {0, 1}, {7, 15},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    };
    static void __init armada_38x_get_clk_ratio(
    void __iomem *sar, int id, int *mult, int *div)
    {
    u32 opt = ((readl(sar) >> SAR_A380_CPU_DDR_L2_FREQ_OPT) &
    SAR_A380_CPU_DDR_L2_FREQ_OPT_MASK);
    switch (id) {
    case A380_CPU_TO_L2:
// mult = armada_38x_cpu_l2_ratios[opt][0];
// div = armada_38x_cpu_l2_ratios[opt][1];
    break;
    case A380_CPU_TO_DDR:
// mult = armada_38x_cpu_ddr_ratios[opt][0];
// div = armada_38x_cpu_ddr_ratios[opt][1];
    break;
    }
    }
    static const struct coreclk_soc_desc armada_38x_coreclks = {
    .get_tclk_freq = armada_38x_get_tclk_freq,
    .get_cpu_freq = armada_38x_get_cpu_freq,
    .get_clk_ratio = armada_38x_get_clk_ratio,
    .ratios = armada_38x_coreclk_ratios,
    .num_ratios = ARRAY_SIZE(armada_38x_coreclk_ratios),
    };
#[no_mangle]
unsafe extern "C" fn armada_38x_coreclk_init(np: *mut device_node) -> void __init {
    static void __init armada_38x_coreclk_init(struct device_node *np)
    {
    mvebu_coreclk_setup(np, &armada_38x_coreclks);
    }
    CLK_OF_DECLARE(armada_38x_core_clk, "marvell,armada-380-core-clock",
    armada_38x_coreclk_init);
//
// Clock Gating Control
//
    static const struct clk_gating_soc_desc armada_38x_gating_desc[] __initconst = {
    { "audio", core::ptr::null_mut(), 0 },
    { "ge2", core::ptr::null_mut(), 2 },
    { "ge1", core::ptr::null_mut(), 3 },
    { "ge0", core::ptr::null_mut(), 4 },
    { "pex1", core::ptr::null_mut(), 5 },
    { "pex2", core::ptr::null_mut(), 6 },
    { "pex3", core::ptr::null_mut(), 7 },
    { "pex0", core::ptr::null_mut(), 8 },
    { "usb3h0", core::ptr::null_mut(), 9 },
    { "usb3h1", core::ptr::null_mut(), 10 },
    { "usb3d", core::ptr::null_mut(), 11 },
    { "bm", core::ptr::null_mut(), 13 },
    { "crypto0z", core::ptr::null_mut(), 14 },
    { "sata0", core::ptr::null_mut(), 15 },
    { "crypto1z", core::ptr::null_mut(), 16 },
    { "sdio", core::ptr::null_mut(), 17 },
    { "usb2", core::ptr::null_mut(), 18 },
    { "crypto1", core::ptr::null_mut(), 21 },
    { "xor0", core::ptr::null_mut(), 22 },
    { "crypto0", core::ptr::null_mut(), 23 },
    { "tdm", core::ptr::null_mut(), 25 },
    { "xor1", core::ptr::null_mut(), 28 },
    { "sata1", core::ptr::null_mut(), 30 },
    { }
    };
#[no_mangle]
unsafe extern "C" fn armada_38x_clk_gating_init(np: *mut device_node) -> void __init {
    static void __init armada_38x_clk_gating_init(struct device_node *np)
    {
    mvebu_clk_gating_setup(np, armada_38x_gating_desc);
    }
    CLK_OF_DECLARE(armada_38x_clk_gating, "marvell,armada-380-gating-clock",
    armada_38x_clk_gating_init);
