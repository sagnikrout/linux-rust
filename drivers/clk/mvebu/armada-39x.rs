//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mvebu/armada-39x.c
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
// Marvell Armada 39x SoC clocks
//
// Copyright (C) 2015 Marvell
//
// Gregory CLEMENT <gregory.clement@free-electrons.com>
// Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
// Andrew Lunn <andrew@lunn.ch>
// Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
//

//
// SARL[14:10] : Ratios between CPU, NBCLK, HCLK and DCLK.
//
// SARL[15]    : TCLK frequency
// 0 = 250 MHz
// 1 = 200 MHz
//
// SARH[0]     : Reference clock frequency
// 0 = 25 Mhz
// 1 = 40 Mhz
//
pub const SARL: c_int = 0;
pub const SARL_A390_TCLK_FREQ_OPT: c_int = 15;
pub const SARL_A390_TCLK_FREQ_OPT_MASK: c_uint = 0x1;
pub const SARL_A390_CPU_DDR_L2_FREQ_OPT: c_int = 10;
pub const SARL_A390_CPU_DDR_L2_FREQ_OPT_MASK: c_uint = 0x1F;
pub const SARH: c_int = 4;

    static const u32 armada_39x_tclk_frequencies[] __initconst = {
    250000000,
    200000000,
    };
#[no_mangle]
unsafe extern "C" fn armada_39x_get_tclk_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init armada_39x_get_tclk_freq(void __iomem *sar)
    {
    u8 tclk_freq_select;
    tclk_freq_select = ((readl(sar + SARL) >> SARL_A390_TCLK_FREQ_OPT) &
    SARL_A390_TCLK_FREQ_OPT_MASK);
    return armada_39x_tclk_frequencies[tclk_freq_select];
    }
    static const u32 armada_39x_cpu_frequencies[] __initconst = {
    [0x0] = 666 * 1000 * 1000,
    [0x2] = 800 * 1000 * 1000,
    [0x3] = 800 * 1000 * 1000,
    [0x4] = 1066 * 1000 * 1000,
    [0x5] = 1066 * 1000 * 1000,
    [0x6] = 1200 * 1000 * 1000,
    [0x8] = 1332 * 1000 * 1000,
    [0xB] = 1600 * 1000 * 1000,
    [0xC] = 1600 * 1000 * 1000,
    [0x12] = 1800 * 1000 * 1000,
    [0x1E] = 1800 * 1000 * 1000,
    };
#[no_mangle]
unsafe extern "C" fn armada_39x_get_cpu_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init armada_39x_get_cpu_freq(void __iomem *sar)
    {
    u8 cpu_freq_select;
    cpu_freq_select = ((readl(sar + SARL) >> SARL_A390_CPU_DDR_L2_FREQ_OPT) &
    SARL_A390_CPU_DDR_L2_FREQ_OPT_MASK);
    if (cpu_freq_select >= ARRAY_SIZE(armada_39x_cpu_frequencies)) {
    pr_err("Selected CPU frequency (%d) unsupported\n",
    cpu_freq_select);
    return 0;
    }
    return armada_39x_cpu_frequencies[cpu_freq_select];
    }
    enum { A390_CPU_TO_NBCLK, A390_CPU_TO_HCLK, A390_CPU_TO_DCLK };
    static const struct coreclk_ratio armada_39x_coreclk_ratios[] __initconst = {
    { .id = A390_CPU_TO_NBCLK, .name = "nbclk" },
    { .id = A390_CPU_TO_HCLK, .name = "hclk" },
    { .id = A390_CPU_TO_DCLK, .name = "dclk" },
    };
    static void __init armada_39x_get_clk_ratio(
    void __iomem *sar, int id, int *mult, int *div)
    {
    switch (id) {
    case A390_CPU_TO_NBCLK:
// mult = 1;
// div = 2;
    break;
    case A390_CPU_TO_HCLK:
// mult = 1;
// div = 4;
    break;
    case A390_CPU_TO_DCLK:
// mult = 1;
// div = 2;
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn armada_39x_refclk_ratio(sar: *mut void __iomem) -> u32 __init {
    static u32 __init armada_39x_refclk_ratio(void __iomem *sar)
    {
    if (readl(sar + SARH) & SARH_A390_REFCLK_FREQ)
    return 40 * 1000 * 1000;
    else
    return 25 * 1000 * 1000;
    }
    static const struct coreclk_soc_desc armada_39x_coreclks = {
    .get_tclk_freq = armada_39x_get_tclk_freq,
    .get_cpu_freq = armada_39x_get_cpu_freq,
    .get_clk_ratio = armada_39x_get_clk_ratio,
    .get_refclk_freq = armada_39x_refclk_ratio,
    .ratios = armada_39x_coreclk_ratios,
    .num_ratios = ARRAY_SIZE(armada_39x_coreclk_ratios),
    };
#[no_mangle]
unsafe extern "C" fn armada_39x_coreclk_init(np: *mut device_node) -> void __init {
    static void __init armada_39x_coreclk_init(struct device_node *np)
    {
    mvebu_coreclk_setup(np, &armada_39x_coreclks);
    }
    CLK_OF_DECLARE(armada_39x_core_clk, "marvell,armada-390-core-clock",
    armada_39x_coreclk_init);
//
// Clock Gating Control
//
    static const struct clk_gating_soc_desc armada_39x_gating_desc[] __initconst = {
    { "pex1", core::ptr::null_mut(), 5 },
    { "pex2", core::ptr::null_mut(), 6 },
    { "pex3", core::ptr::null_mut(), 7 },
    { "pex0", core::ptr::null_mut(), 8 },
    { "usb3h0", core::ptr::null_mut(), 9 },
    { "usb3h1", core::ptr::null_mut(), 10 },
    { "sata0", core::ptr::null_mut(), 15 },
    { "sdio", core::ptr::null_mut(), 17 },
    { "xor0", core::ptr::null_mut(), 22 },
    { "xor1", core::ptr::null_mut(), 28 },
    { }
    };
#[no_mangle]
unsafe extern "C" fn armada_39x_clk_gating_init(np: *mut device_node) -> void __init {
    static void __init armada_39x_clk_gating_init(struct device_node *np)
    {
    mvebu_clk_gating_setup(np, armada_39x_gating_desc);
    }
    CLK_OF_DECLARE(armada_39x_clk_gating, "marvell,armada-390-gating-clock",
    armada_39x_clk_gating_init);
