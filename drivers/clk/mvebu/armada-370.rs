//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mvebu/armada-370.c
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
// Marvell Armada 370 SoC clocks
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

pub const SARL_A370_PCLK_FREQ_OPT: c_int = 11;
pub const SARL_A370_PCLK_FREQ_OPT_MASK: c_uint = 0xF;
pub const SARL_A370_FAB_FREQ_OPT: c_int = 15;
pub const SARL_A370_FAB_FREQ_OPT_MASK: c_uint = 0x1F;
pub const SARL_A370_TCLK_FREQ_OPT: c_int = 20;
pub const SARL_A370_TCLK_FREQ_OPT_MASK: c_uint = 0x1;
    enum { A370_CPU_TO_NBCLK, A370_CPU_TO_HCLK, A370_CPU_TO_DRAMCLK };
    static const struct coreclk_ratio a370_coreclk_ratios[] __initconst = {
    { .id = A370_CPU_TO_NBCLK, .name = "nbclk" },
    { .id = A370_CPU_TO_HCLK, .name = "hclk" },
    { .id = A370_CPU_TO_DRAMCLK, .name = "dramclk" },
    };
    static const u32 a370_tclk_freqs[] __initconst = {
    166000000,
    200000000,
    };
#[no_mangle]
unsafe extern "C" fn a370_get_tclk_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init a370_get_tclk_freq(void __iomem *sar)
    {
    let mut tclk_freq_select: u8 = 0;
    tclk_freq_select = ((readl(sar) >> SARL_A370_TCLK_FREQ_OPT) &
    SARL_A370_TCLK_FREQ_OPT_MASK);
    return a370_tclk_freqs[tclk_freq_select];
    }
    static const u32 a370_cpu_freqs[] __initconst = {
    400000000,
    533000000,
    667000000,
    800000000,
    1000000000,
    1067000000,
    1200000000,
    };
#[no_mangle]
unsafe extern "C" fn a370_get_cpu_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init a370_get_cpu_freq(void __iomem *sar)
    {
    u32 cpu_freq;
    let mut cpu_freq_select: u8 = 0;
    cpu_freq_select = ((readl(sar) >> SARL_A370_PCLK_FREQ_OPT) &
    SARL_A370_PCLK_FREQ_OPT_MASK);
    if (cpu_freq_select >= ARRAY_SIZE(a370_cpu_freqs)) {
    pr_err("CPU freq select unsupported %d\n", cpu_freq_select);
    cpu_freq = 0;
    } else
    cpu_freq = a370_cpu_freqs[cpu_freq_select];
    return cpu_freq;
    }
    static const int a370_nbclk_ratios[32][2] __initconst = {
    {0, 1}, {1, 2}, {2, 2}, {2, 2},
    {1, 2}, {1, 2}, {1, 1}, {2, 3},
    {0, 1}, {1, 2}, {2, 4}, {0, 1},
    {1, 2}, {0, 1}, {0, 1}, {2, 2},
    {0, 1}, {0, 1}, {0, 1}, {1, 1},
    {2, 3}, {0, 1}, {0, 1}, {0, 1},
    {0, 1}, {0, 1}, {0, 1}, {1, 1},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    };
    static const int a370_hclk_ratios[32][2] __initconst = {
    {0, 1}, {1, 2}, {2, 6}, {2, 3},
    {1, 3}, {1, 4}, {1, 2}, {2, 6},
    {0, 1}, {1, 6}, {2, 10}, {0, 1},
    {1, 4}, {0, 1}, {0, 1}, {2, 5},
    {0, 1}, {0, 1}, {0, 1}, {1, 2},
    {2, 6}, {0, 1}, {0, 1}, {0, 1},
    {0, 1}, {0, 1}, {0, 1}, {1, 1},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    };
    static const int a370_dramclk_ratios[32][2] __initconst = {
    {0, 1}, {1, 2}, {2, 3}, {2, 3},
    {1, 3}, {1, 2}, {1, 2}, {2, 6},
    {0, 1}, {1, 3}, {2, 5}, {0, 1},
    {1, 4}, {0, 1}, {0, 1}, {2, 5},
    {0, 1}, {0, 1}, {0, 1}, {1, 1},
    {2, 3}, {0, 1}, {0, 1}, {0, 1},
    {0, 1}, {0, 1}, {0, 1}, {1, 1},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    };
    static void __init a370_get_clk_ratio(
    void __iomem *sar, int id, int *mult, int *div)
    {
    u32 opt = ((readl(sar) >> SARL_A370_FAB_FREQ_OPT) &
    SARL_A370_FAB_FREQ_OPT_MASK);
    switch (id) {
    case A370_CPU_TO_NBCLK:
// mult = a370_nbclk_ratios[opt][0];
// div = a370_nbclk_ratios[opt][1];
    break;
    case A370_CPU_TO_HCLK:
// mult = a370_hclk_ratios[opt][0];
// div = a370_hclk_ratios[opt][1];
    break;
    case A370_CPU_TO_DRAMCLK:
// mult = a370_dramclk_ratios[opt][0];
// div = a370_dramclk_ratios[opt][1];
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn a370_is_sscg_enabled(sar: *mut void __iomem) -> bool {
    static bool a370_is_sscg_enabled(void __iomem *sar)
    {
    return !(readl(sar) & SARL_A370_SSCG_ENABLE);
    }
    static const struct coreclk_soc_desc a370_coreclks = {
    .get_tclk_freq = a370_get_tclk_freq,
    .get_cpu_freq = a370_get_cpu_freq,
    .get_clk_ratio = a370_get_clk_ratio,
    .is_sscg_enabled = a370_is_sscg_enabled,
    .fix_sscg_deviation = kirkwood_fix_sscg_deviation,
    .ratios = a370_coreclk_ratios,
    .num_ratios = ARRAY_SIZE(a370_coreclk_ratios),
    };
//
// Clock Gating Control
//
    static const struct clk_gating_soc_desc a370_gating_desc[] __initconst = {
    { "audio", core::ptr::null_mut(), 0, 0 },
    { "pex0_en", core::ptr::null_mut(), 1, 0 },
    { "pex1_en", core::ptr::null_mut(),  2, 0 },
    { "ge1", core::ptr::null_mut(), 3, 0 },
    { "ge0", core::ptr::null_mut(), 4, 0 },
    { "pex0", "pex0_en", 5, 0 },
    { "pex1", "pex1_en", 9, 0 },
    { "sata0", core::ptr::null_mut(), 15, 0 },
    { "sdio", core::ptr::null_mut(), 17, 0 },
    { "crypto", core::ptr::null_mut(), 23, CLK_IGNORE_UNUSED },
    { "tdm", core::ptr::null_mut(), 25, 0 },
    { "ddr", core::ptr::null_mut(), 28, CLK_IGNORE_UNUSED },
    { "sata1", core::ptr::null_mut(), 30, 0 },
    { }
    };
#[no_mangle]
unsafe extern "C" fn a370_clk_init(np: *mut device_node) -> void __init {
    static void __init a370_clk_init(struct device_node *np)
    {
    struct device_node *cgnp =
    of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "marvell,armada-370-gating-clock");
    mvebu_coreclk_setup(np, &a370_coreclks);
    if (cgnp) {
    mvebu_clk_gating_setup(cgnp, a370_gating_desc);
    of_node_put(cgnp);
    }
    }
    CLK_OF_DECLARE(a370_clk, "marvell,armada-370-core-clock", a370_clk_init);
