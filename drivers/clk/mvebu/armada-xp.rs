//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mvebu/armada-xp.c
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
// Marvell Armada XP SoC clocks
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
// Armada XP Sample At Reset is a 64 bit bitfield split in two
// registers of 32 bits
//

pub const SARL_AXP_PCLK_FREQ_OPT: c_int = 21;
pub const SARL_AXP_PCLK_FREQ_OPT_MASK: c_uint = 0x7;
pub const SARL_AXP_FAB_FREQ_OPT: c_int = 24;
pub const SARL_AXP_FAB_FREQ_OPT_MASK: c_uint = 0xF;

pub const SARH_AXP_PCLK_FREQ_OPT_MASK: c_uint = 0x1;
pub const SARH_AXP_PCLK_FREQ_OPT_SHIFT: c_int = 3;

pub const SARH_AXP_FAB_FREQ_OPT_MASK: c_uint = 0x1;
pub const SARH_AXP_FAB_FREQ_OPT_SHIFT: c_int = 4;
    enum { AXP_CPU_TO_NBCLK, AXP_CPU_TO_HCLK, AXP_CPU_TO_DRAMCLK };
    static const struct coreclk_ratio axp_coreclk_ratios[] __initconst = {
    { .id = AXP_CPU_TO_NBCLK, .name = "nbclk" },
    { .id = AXP_CPU_TO_HCLK, .name = "hclk" },
    { .id = AXP_CPU_TO_DRAMCLK, .name = "dramclk" },
    };
// Armada XP TCLK frequency is fixed to 250MHz
#[no_mangle]
unsafe extern "C" fn axp_get_tclk_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init axp_get_tclk_freq(void __iomem *sar)
    {
    return 250000000;
    }
    static const u32 axp_cpu_freqs[] __initconst = {
    1000000000,
    1066000000,
    1200000000,
    1333000000,
    1500000000,
    1666000000,
    1800000000,
    2000000000,
    667000000,
    0,
    800000000,
    1600000000,
    };
#[no_mangle]
unsafe extern "C" fn axp_get_cpu_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init axp_get_cpu_freq(void __iomem *sar)
    {
    u32 cpu_freq;
    let mut cpu_freq_select: u8 = 0;
    cpu_freq_select = ((readl(sar + SARL) >> SARL_AXP_PCLK_FREQ_OPT) &
    SARL_AXP_PCLK_FREQ_OPT_MASK);
//
// The upper bit is not contiguous to the other ones and
// located in the high part of the SAR registers
//
    cpu_freq_select |= (((readl(sar + SARH) >> SARH_AXP_PCLK_FREQ_OPT) &
    SARH_AXP_PCLK_FREQ_OPT_MASK) << SARH_AXP_PCLK_FREQ_OPT_SHIFT);
    if (cpu_freq_select >= ARRAY_SIZE(axp_cpu_freqs)) {
    pr_err("CPU freq select unsupported: %d\n", cpu_freq_select);
    cpu_freq = 0;
    } else
    cpu_freq = axp_cpu_freqs[cpu_freq_select];
    return cpu_freq;
    }
    static const int axp_nbclk_ratios[32][2] __initconst = {
    {0, 1}, {1, 2}, {2, 2}, {2, 2},
    {1, 2}, {1, 2}, {1, 1}, {2, 3},
    {0, 1}, {1, 2}, {2, 4}, {0, 1},
    {1, 2}, {0, 1}, {0, 1}, {2, 2},
    {0, 1}, {0, 1}, {0, 1}, {1, 1},
    {2, 3}, {0, 1}, {0, 1}, {0, 1},
    {0, 1}, {0, 1}, {0, 1}, {1, 1},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    };
    static const int axp_hclk_ratios[32][2] __initconst = {
    {0, 1}, {1, 2}, {2, 6}, {2, 3},
    {1, 3}, {1, 4}, {1, 2}, {2, 6},
    {0, 1}, {1, 6}, {2, 10}, {0, 1},
    {1, 4}, {0, 1}, {0, 1}, {2, 5},
    {0, 1}, {0, 1}, {0, 1}, {1, 2},
    {2, 6}, {0, 1}, {0, 1}, {0, 1},
    {0, 1}, {0, 1}, {0, 1}, {1, 1},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    };
    static const int axp_dramclk_ratios[32][2] __initconst = {
    {0, 1}, {1, 2}, {2, 3}, {2, 3},
    {1, 3}, {1, 2}, {1, 2}, {2, 6},
    {0, 1}, {1, 3}, {2, 5}, {0, 1},
    {1, 4}, {0, 1}, {0, 1}, {2, 5},
    {0, 1}, {0, 1}, {0, 1}, {1, 1},
    {2, 3}, {0, 1}, {0, 1}, {0, 1},
    {0, 1}, {0, 1}, {0, 1}, {1, 1},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    };
    static void __init axp_get_clk_ratio(
    void __iomem *sar, int id, int *mult, int *div)
    {
    u32 opt = ((readl(sar + SARL) >> SARL_AXP_FAB_FREQ_OPT) &
    SARL_AXP_FAB_FREQ_OPT_MASK);
//
// The upper bit is not contiguous to the other ones and
// located in the high part of the SAR registers
//
    opt |= (((readl(sar + SARH) >> SARH_AXP_FAB_FREQ_OPT) &
    SARH_AXP_FAB_FREQ_OPT_MASK) << SARH_AXP_FAB_FREQ_OPT_SHIFT);
    switch (id) {
    case AXP_CPU_TO_NBCLK:
// mult = axp_nbclk_ratios[opt][0];
// div = axp_nbclk_ratios[opt][1];
    break;
    case AXP_CPU_TO_HCLK:
// mult = axp_hclk_ratios[opt][0];
// div = axp_hclk_ratios[opt][1];
    break;
    case AXP_CPU_TO_DRAMCLK:
// mult = axp_dramclk_ratios[opt][0];
// div = axp_dramclk_ratios[opt][1];
    break;
    }
    }
    static const struct coreclk_soc_desc axp_coreclks = {
    .get_tclk_freq = axp_get_tclk_freq,
    .get_cpu_freq = axp_get_cpu_freq,
    .get_clk_ratio = axp_get_clk_ratio,
    .ratios = axp_coreclk_ratios,
    .num_ratios = ARRAY_SIZE(axp_coreclk_ratios),
    };
//
// Clock Gating Control
//
    static const struct clk_gating_soc_desc axp_gating_desc[] __initconst = {
    { "audio", core::ptr::null_mut(), 0, 0 },
    { "ge3", core::ptr::null_mut(), 1, 0 },
    { "ge2", core::ptr::null_mut(),  2, 0 },
    { "ge1", core::ptr::null_mut(), 3, 0 },
    { "ge0", core::ptr::null_mut(), 4, 0 },
    { "pex00", core::ptr::null_mut(), 5, 0 },
    { "pex01", core::ptr::null_mut(), 6, 0 },
    { "pex02", core::ptr::null_mut(), 7, 0 },
    { "pex03", core::ptr::null_mut(), 8, 0 },
    { "pex10", core::ptr::null_mut(), 9, 0 },
    { "pex11", core::ptr::null_mut(), 10, 0 },
    { "pex12", core::ptr::null_mut(), 11, 0 },
    { "pex13", core::ptr::null_mut(), 12, 0 },
    { "bp", core::ptr::null_mut(), 13, 0 },
    { "sata0lnk", core::ptr::null_mut(), 14, 0 },
    { "sata0", "sata0lnk", 15, 0 },
    { "lcd", core::ptr::null_mut(), 16, 0 },
    { "sdio", core::ptr::null_mut(), 17, 0 },
    { "usb0", core::ptr::null_mut(), 18, 0 },
    { "usb1", core::ptr::null_mut(), 19, 0 },
    { "usb2", core::ptr::null_mut(), 20, 0 },
    { "xor0", core::ptr::null_mut(), 22, 0 },
    { "crypto", core::ptr::null_mut(), 23, 0 },
    { "tdm", core::ptr::null_mut(), 25, 0 },
    { "pex20", core::ptr::null_mut(), 26, 0 },
    { "pex30", core::ptr::null_mut(), 27, 0 },
    { "xor1", core::ptr::null_mut(), 28, 0 },
    { "sata1lnk", core::ptr::null_mut(), 29, 0 },
    { "sata1", "sata1lnk", 30, 0 },
    { }
    };
#[no_mangle]
unsafe extern "C" fn axp_clk_init(np: *mut device_node) -> void __init {
    static void __init axp_clk_init(struct device_node *np)
    {
    struct device_node *cgnp =
    of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "marvell,armada-xp-gating-clock");
    mvebu_coreclk_setup(np, &axp_coreclks);
    if (cgnp) {
    mvebu_clk_gating_setup(cgnp, axp_gating_desc);
    of_node_put(cgnp);
    }
    }
    CLK_OF_DECLARE(axp_clk, "marvell,armada-xp-core-clock", axp_clk_init);
