//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mvebu/dove.c
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
// Marvell Dove SoC clocks
//
// Copyright (C) 2012 Marvell
//
// Gregory CLEMENT <gregory.clement@free-electrons.com>
// Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
// Andrew Lunn <andrew@lunn.ch>
//

//
// Core Clocks
//
// Dove PLL sample-at-reset configuration
//
// SAR0[8:5]   : CPU frequency
// 5  = 1000 MHz
// 6  =  933 MHz
// 7  =  933 MHz
// 8  =  800 MHz
// 9  =  800 MHz
// 10 =  800 MHz
// 11 = 1067 MHz
// 12 =  667 MHz
// 13 =  533 MHz
// 14 =  400 MHz
// 15 =  333 MHz
// others reserved.
//
// SAR0[11:9]  : CPU to L2 Clock divider ratio
// 0 = (1/1) * CPU
// 2 = (1/2) * CPU
// 4 = (1/3) * CPU
// 6 = (1/4) * CPU
// others reserved.
//
// SAR0[15:12] : CPU to DDR DRAM Clock divider ratio
// 0  = (1/1) * CPU
// 2  = (1/2) * CPU
// 3  = (2/5) * CPU
// 4  = (1/3) * CPU
// 6  = (1/4) * CPU
// 8  = (1/5) * CPU
// 10 = (1/6) * CPU
// 12 = (1/7) * CPU
// 14 = (1/8) * CPU
// 15 = (1/10) * CPU
// others reserved.
//
// SAR0[24:23] : TCLK frequency
// 0 = 166 MHz
// 1 = 125 MHz
// others reserved.
//
pub const SAR_DOVE_CPU_FREQ: c_int = 5;
pub const SAR_DOVE_CPU_FREQ_MASK: c_uint = 0xf;
pub const SAR_DOVE_L2_RATIO: c_int = 9;
pub const SAR_DOVE_L2_RATIO_MASK: c_uint = 0x7;
pub const SAR_DOVE_DDR_RATIO: c_int = 12;
pub const SAR_DOVE_DDR_RATIO_MASK: c_uint = 0xf;
pub const SAR_DOVE_TCLK_FREQ: c_int = 23;
pub const SAR_DOVE_TCLK_FREQ_MASK: c_uint = 0x3;
    enum { DOVE_CPU_TO_L2, DOVE_CPU_TO_DDR };
    static const struct coreclk_ratio dove_coreclk_ratios[] __initconst = {
    { .id = DOVE_CPU_TO_L2, .name = "l2clk", },
    { .id = DOVE_CPU_TO_DDR, .name = "ddrclk", }
    };
    static const u32 dove_tclk_freqs[] __initconst = {
    166666667,
    125000000,
    0, 0
    };
#[no_mangle]
unsafe extern "C" fn dove_get_tclk_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init dove_get_tclk_freq(void __iomem *sar)
    {
    u32 opt = (readl(sar) >> SAR_DOVE_TCLK_FREQ) &
    SAR_DOVE_TCLK_FREQ_MASK;
    return dove_tclk_freqs[opt];
    }
    static const u32 dove_cpu_freqs[] __initconst = {
    0, 0, 0, 0, 0,
    1000000000,
    933333333, 933333333,
    800000000, 800000000, 800000000,
    1066666667,
    666666667,
    533333333,
    400000000,
    333333333
    };
#[no_mangle]
unsafe extern "C" fn dove_get_cpu_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init dove_get_cpu_freq(void __iomem *sar)
    {
    u32 opt = (readl(sar) >> SAR_DOVE_CPU_FREQ) &
    SAR_DOVE_CPU_FREQ_MASK;
    return dove_cpu_freqs[opt];
    }
    static const int dove_cpu_l2_ratios[8][2] __initconst = {
    { 1, 1 }, { 0, 1 }, { 1, 2 }, { 0, 1 },
    { 1, 3 }, { 0, 1 }, { 1, 4 }, { 0, 1 }
    };
    static const int dove_cpu_ddr_ratios[16][2] __initconst = {
    { 1, 1 }, { 0, 1 }, { 1, 2 }, { 2, 5 },
    { 1, 3 }, { 0, 1 }, { 1, 4 }, { 0, 1 },
    { 1, 5 }, { 0, 1 }, { 1, 6 }, { 0, 1 },
    { 1, 7 }, { 0, 1 }, { 1, 8 }, { 1, 10 }
    };
    static void __init dove_get_clk_ratio(
    void __iomem *sar, int id, int *mult, int *div)
    {
    switch (id) {
    case DOVE_CPU_TO_L2:
    {
    u32 opt = (readl(sar) >> SAR_DOVE_L2_RATIO) &
    SAR_DOVE_L2_RATIO_MASK;
// mult = dove_cpu_l2_ratios[opt][0];
// div = dove_cpu_l2_ratios[opt][1];
    break;
    }
    case DOVE_CPU_TO_DDR:
    {
    u32 opt = (readl(sar) >> SAR_DOVE_DDR_RATIO) &
    SAR_DOVE_DDR_RATIO_MASK;
// mult = dove_cpu_ddr_ratios[opt][0];
// div = dove_cpu_ddr_ratios[opt][1];
    break;
    }
    }
    }
    static const struct coreclk_soc_desc dove_coreclks = {
    .get_tclk_freq = dove_get_tclk_freq,
    .get_cpu_freq = dove_get_cpu_freq,
    .get_clk_ratio = dove_get_clk_ratio,
    .ratios = dove_coreclk_ratios,
    .num_ratios = ARRAY_SIZE(dove_coreclk_ratios),
    };
//
// Clock Gating Control
//
    static const struct clk_gating_soc_desc dove_gating_desc[] __initconst = {
    { "usb0", core::ptr::null_mut(), 0, 0 },
    { "usb1", core::ptr::null_mut(), 1, 0 },
    { "ge",	"gephy", 2, 0 },
    { "sata", core::ptr::null_mut(), 3, 0 },
    { "pex0", core::ptr::null_mut(), 4, 0 },
    { "pex1", core::ptr::null_mut(), 5, 0 },
    { "sdio0", core::ptr::null_mut(), 8, 0 },
    { "sdio1", core::ptr::null_mut(), 9, 0 },
    { "nand", core::ptr::null_mut(), 10, 0 },
    { "camera", core::ptr::null_mut(), 11, 0 },
    { "i2s0", core::ptr::null_mut(), 12, 0 },
    { "i2s1", core::ptr::null_mut(), 13, 0 },
    { "crypto", core::ptr::null_mut(), 15, 0 },
    { "ac97", core::ptr::null_mut(), 21, 0 },
    { "pdma", core::ptr::null_mut(), 22, 0 },
    { "xor0", core::ptr::null_mut(), 23, 0 },
    { "xor1", core::ptr::null_mut(), 24, 0 },
    { "gephy", core::ptr::null_mut(), 30, 0 },
    { }
    };
#[no_mangle]
unsafe extern "C" fn dove_clk_init(np: *mut device_node) -> void __init {
    static void __init dove_clk_init(struct device_node *np)
    {
    struct device_node *cgnp =
    of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "marvell,dove-gating-clock");
    struct device_node *ddnp =
    of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "marvell,dove-divider-clock");
    mvebu_coreclk_setup(np, &dove_coreclks);
    if (ddnp) {
    dove_divider_clk_init(ddnp);
    of_node_put(ddnp);
    }
    if (cgnp) {
    mvebu_clk_gating_setup(cgnp, dove_gating_desc);
    of_node_put(cgnp);
    }
    }
    CLK_OF_DECLARE(dove_clk, "marvell,dove-core-clock", dove_clk_init);
