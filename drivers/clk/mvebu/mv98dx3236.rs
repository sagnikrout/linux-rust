//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mvebu/mv98dx3236.c
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
// Marvell MV98DX3236 SoC clocks
//
// Copyright (C) 2012 Marvell
//
// Gregory CLEMENT <gregory.clement@free-electrons.com>
// Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
// Andrew Lunn <andrew@lunn.ch>
//

//
// For 98DX4251 Sample At Reset the CPU, DDR and Main PLL clocks are all
// defined at the same time
//
// SAR1[20:18]   : CPU frequency    DDR frequency   MPLL frequency
// 0  =  400 MHz	    400 MHz	    800 MHz
// 2  =  667 MHz	    667 MHz	    2000 MHz
// 3  =  800 MHz	    800 MHz	    1600 MHz
// others reserved.
//
// For 98DX3236 Sample At Reset the CPU, DDR and Main PLL clocks are all
// defined at the same time
//
// SAR1[20:18]   : CPU frequency    DDR frequency   MPLL frequency
// 1  =  667 MHz	    667 MHz	    2000 MHz
// 2  =  400 MHz	    400 MHz	    400 MHz
// 3  =  800 MHz	    800 MHz	    800 MHz
// 5  =  800 MHz	    400 MHz	    800 MHz
// others reserved.
//
pub const SAR1_MV98DX3236_CPU_DDR_MPLL_FREQ_OPT: c_int = 18;
pub const SAR1_MV98DX3236_CPU_DDR_MPLL_FREQ_OPT_MASK: c_uint = 0x7;
#[no_mangle]
unsafe extern "C" fn mv98dx3236_get_tclk_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init mv98dx3236_get_tclk_freq(void __iomem *sar)
    {
// Tclk = 200MHz, no SaR dependency
    return 200000000;
    }
    static const u32 mv98dx3236_cpu_frequencies[] __initconst = {
    0,
    667000000,
    400000000,
    800000000,
    0,
    800000000,
    0, 0,
    };
    static const u32 mv98dx4251_cpu_frequencies[] __initconst = {
    400000000,
    0,
    667000000,
    800000000,
    0, 0, 0, 0,
    };
#[no_mangle]
unsafe extern "C" fn mv98dx3236_get_cpu_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init mv98dx3236_get_cpu_freq(void __iomem *sar)
    {
    let mut cpu_freq: u32 = 0;
    let mut cpu_freq_select: u8 = 0;
    cpu_freq_select = ((readl(sar) >> SAR1_MV98DX3236_CPU_DDR_MPLL_FREQ_OPT) &
    SAR1_MV98DX3236_CPU_DDR_MPLL_FREQ_OPT_MASK);
    if (of_machine_is_compatible("marvell,armadaxp-98dx4251"))
    cpu_freq = mv98dx4251_cpu_frequencies[cpu_freq_select];
#[no_mangle]
pub unsafe extern "C" fn if(_arg: of_machine_is_compatible("marvell, _arg: armadaxp-98dx3236")) -> else {
    else if (of_machine_is_compatible("marvell,armadaxp-98dx3236"))
    cpu_freq = mv98dx3236_cpu_frequencies[cpu_freq_select];
    if (!cpu_freq)
    pr_err("CPU freq select unsupported %d\n", cpu_freq_select);
    return cpu_freq;
    }
    enum {
    MV98DX3236_CPU_TO_DDR,
    MV98DX3236_CPU_TO_MPLL
    };
    static const struct coreclk_ratio mv98dx3236_core_ratios[] __initconst = {
    { .id = MV98DX3236_CPU_TO_DDR, .name = "ddrclk" },
    { .id = MV98DX3236_CPU_TO_MPLL, .name = "mpll" },
    };
    static const int __initconst mv98dx3236_cpu_mpll_ratios[8][2] = {
    {0, 1}, {3, 1}, {1, 1}, {1, 1},
    {0, 1}, {1, 1}, {0, 1}, {0, 1},
    };
    static const int __initconst mv98dx3236_cpu_ddr_ratios[8][2] = {
    {0, 1}, {1, 1}, {1, 1}, {1, 1},
    {0, 1}, {1, 2}, {0, 1}, {0, 1},
    };
    static const int __initconst mv98dx4251_cpu_mpll_ratios[8][2] = {
    {2, 1}, {0, 1}, {3, 1}, {2, 1},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    };
    static const int __initconst mv98dx4251_cpu_ddr_ratios[8][2] = {
    {1, 1}, {0, 1}, {1, 1}, {1, 1},
    {0, 1}, {0, 1}, {0, 1}, {0, 1},
    };
    static void __init mv98dx3236_get_clk_ratio(
    void __iomem *sar, int id, int *mult, int *div)
    {
    u32 opt = ((readl(sar) >> SAR1_MV98DX3236_CPU_DDR_MPLL_FREQ_OPT) &
    SAR1_MV98DX3236_CPU_DDR_MPLL_FREQ_OPT_MASK);
    switch (id) {
    case MV98DX3236_CPU_TO_DDR:
    if (of_machine_is_compatible("marvell,armadaxp-98dx4251")) {
// mult = mv98dx4251_cpu_ddr_ratios[opt][0];
// div = mv98dx4251_cpu_ddr_ratios[opt][1];
    } else if (of_machine_is_compatible("marvell,armadaxp-98dx3236")) {
// mult = mv98dx3236_cpu_ddr_ratios[opt][0];
// div = mv98dx3236_cpu_ddr_ratios[opt][1];
    }
    break;
    case MV98DX3236_CPU_TO_MPLL:
    if (of_machine_is_compatible("marvell,armadaxp-98dx4251")) {
// mult = mv98dx4251_cpu_mpll_ratios[opt][0];
// div = mv98dx4251_cpu_mpll_ratios[opt][1];
    } else if (of_machine_is_compatible("marvell,armadaxp-98dx3236")) {
// mult = mv98dx3236_cpu_mpll_ratios[opt][0];
// div = mv98dx3236_cpu_mpll_ratios[opt][1];
    }
    break;
    }
    }
    static const struct coreclk_soc_desc mv98dx3236_core_clocks = {
    .get_tclk_freq = mv98dx3236_get_tclk_freq,
    .get_cpu_freq = mv98dx3236_get_cpu_freq,
    .get_clk_ratio = mv98dx3236_get_clk_ratio,
    .ratios = mv98dx3236_core_ratios,
    .num_ratios = ARRAY_SIZE(mv98dx3236_core_ratios),
    };
//
// Clock Gating Control
//
    static const struct clk_gating_soc_desc mv98dx3236_gating_desc[] __initconst = {
    { "ge1", core::ptr::null_mut(), 3, 0 },
    { "ge0", core::ptr::null_mut(), 4, 0 },
    { "pex00", core::ptr::null_mut(), 5, 0 },
    { "sdio", core::ptr::null_mut(), 17, 0 },
    { "usb0", core::ptr::null_mut(), 18, 0 },
    { "xor0", core::ptr::null_mut(), 22, 0 },
    { }
    };
#[no_mangle]
unsafe extern "C" fn mv98dx3236_clk_init(np: *mut device_node) -> void __init {
    static void __init mv98dx3236_clk_init(struct device_node *np)
    {
    struct device_node *cgnp =
    of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "marvell,mv98dx3236-gating-clock");
    mvebu_coreclk_setup(np, &mv98dx3236_core_clocks);
    if (cgnp) {
    mvebu_clk_gating_setup(cgnp, mv98dx3236_gating_desc);
    of_node_put(cgnp);
    }
    }
    CLK_OF_DECLARE(mv98dx3236_clk, "marvell,mv98dx3236-core-clock", mv98dx3236_clk_init);
