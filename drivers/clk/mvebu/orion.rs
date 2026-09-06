//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mvebu/orion.c
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
// Marvell Orion SoC clocks
//
// Copyright (C) 2014 Thomas Petazzoni
//
// Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
//

    static const struct coreclk_ratio orion_coreclk_ratios[] __initconst = {
    { .id = 0, .name = "ddrclk", }
    };
//
// Orion 5181
//
pub const SAR_MV88F5181_TCLK_FREQ: c_int = 8;
pub const SAR_MV88F5181_TCLK_FREQ_MASK: c_uint = 0x3;
#[no_mangle]
unsafe extern "C" fn mv88f5181_get_tclk_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init mv88f5181_get_tclk_freq(void __iomem *sar)
    {
    u32 opt = (readl(sar) >> SAR_MV88F5181_TCLK_FREQ) &
    SAR_MV88F5181_TCLK_FREQ_MASK;
    if (opt == 0)
    return 133333333;
#[no_mangle]
pub unsafe extern "C" fn if(1: opt ==) -> else {
    else if (opt == 1)
    return 150000000;
#[no_mangle]
pub unsafe extern "C" fn if(2: opt ==) -> else {
    else if (opt == 2)
    return 166666667;
    else
    return 0;
    }
pub const SAR_MV88F5181_CPU_FREQ: c_int = 4;
pub const SAR_MV88F5181_CPU_FREQ_MASK: c_uint = 0xf;
#[no_mangle]
unsafe extern "C" fn mv88f5181_get_cpu_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init mv88f5181_get_cpu_freq(void __iomem *sar)
    {
    u32 opt = (readl(sar) >> SAR_MV88F5181_CPU_FREQ) &
    SAR_MV88F5181_CPU_FREQ_MASK;
    if (opt == 0)
    return 333333333;
#[no_mangle]
pub unsafe extern "C" fn if(2: opt == 1 || opt ==) -> else {
    else if (opt == 1 || opt == 2)
    return 400000000;
#[no_mangle]
pub unsafe extern "C" fn if(3: opt ==) -> else {
    else if (opt == 3)
    return 500000000;
    else
    return 0;
    }
    static void __init mv88f5181_get_clk_ratio(void __iomem *sar, int id,
    int *mult, int *div)
    {
    u32 opt = (readl(sar) >> SAR_MV88F5181_CPU_FREQ) &
    SAR_MV88F5181_CPU_FREQ_MASK;
    if (opt == 0 || opt == 1) {
// mult = 1;
// div  = 2;
    } else if (opt == 2 || opt == 3) {
// mult = 1;
// div  = 3;
    } else {
// mult = 0;
// div  = 1;
    }
    }
    static const struct coreclk_soc_desc mv88f5181_coreclks = {
    .get_tclk_freq = mv88f5181_get_tclk_freq,
    .get_cpu_freq = mv88f5181_get_cpu_freq,
    .get_clk_ratio = mv88f5181_get_clk_ratio,
    .ratios = orion_coreclk_ratios,
    .num_ratios = ARRAY_SIZE(orion_coreclk_ratios),
    };
#[no_mangle]
unsafe extern "C" fn mv88f5181_clk_init(np: *mut device_node) -> void __init {
    static void __init mv88f5181_clk_init(struct device_node *np)
    {
    return mvebu_coreclk_setup(np, &mv88f5181_coreclks);
    }
    CLK_OF_DECLARE(mv88f5181_clk, "marvell,mv88f5181-core-clock", mv88f5181_clk_init);
//
// Orion 5182
//
pub const SAR_MV88F5182_TCLK_FREQ: c_int = 8;
pub const SAR_MV88F5182_TCLK_FREQ_MASK: c_uint = 0x3;
#[no_mangle]
unsafe extern "C" fn mv88f5182_get_tclk_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init mv88f5182_get_tclk_freq(void __iomem *sar)
    {
    u32 opt = (readl(sar) >> SAR_MV88F5182_TCLK_FREQ) &
    SAR_MV88F5182_TCLK_FREQ_MASK;
    if (opt == 1)
    return 150000000;
#[no_mangle]
pub unsafe extern "C" fn if(2: opt ==) -> else {
    else if (opt == 2)
    return 166666667;
    else
    return 0;
    }
pub const SAR_MV88F5182_CPU_FREQ: c_int = 4;
pub const SAR_MV88F5182_CPU_FREQ_MASK: c_uint = 0xf;
#[no_mangle]
unsafe extern "C" fn mv88f5182_get_cpu_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init mv88f5182_get_cpu_freq(void __iomem *sar)
    {
    u32 opt = (readl(sar) >> SAR_MV88F5182_CPU_FREQ) &
    SAR_MV88F5182_CPU_FREQ_MASK;
    if (opt == 0)
    return 333333333;
#[no_mangle]
pub unsafe extern "C" fn if(2: opt == 1 || opt ==) -> else {
    else if (opt == 1 || opt == 2)
    return 400000000;
#[no_mangle]
pub unsafe extern "C" fn if(3: opt ==) -> else {
    else if (opt == 3)
    return 500000000;
    else
    return 0;
    }
    static void __init mv88f5182_get_clk_ratio(void __iomem *sar, int id,
    int *mult, int *div)
    {
    u32 opt = (readl(sar) >> SAR_MV88F5182_CPU_FREQ) &
    SAR_MV88F5182_CPU_FREQ_MASK;
    if (opt == 0 || opt == 1) {
// mult = 1;
// div  = 2;
    } else if (opt == 2 || opt == 3) {
// mult = 1;
// div  = 3;
    } else {
// mult = 0;
// div  = 1;
    }
    }
    static const struct coreclk_soc_desc mv88f5182_coreclks = {
    .get_tclk_freq = mv88f5182_get_tclk_freq,
    .get_cpu_freq = mv88f5182_get_cpu_freq,
    .get_clk_ratio = mv88f5182_get_clk_ratio,
    .ratios = orion_coreclk_ratios,
    .num_ratios = ARRAY_SIZE(orion_coreclk_ratios),
    };
#[no_mangle]
unsafe extern "C" fn mv88f5182_clk_init(np: *mut device_node) -> void __init {
    static void __init mv88f5182_clk_init(struct device_node *np)
    {
    return mvebu_coreclk_setup(np, &mv88f5182_coreclks);
    }
    CLK_OF_DECLARE(mv88f5182_clk, "marvell,mv88f5182-core-clock", mv88f5182_clk_init);
//
// Orion 5281
//
#[no_mangle]
unsafe extern "C" fn mv88f5281_get_tclk_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init mv88f5281_get_tclk_freq(void __iomem *sar)
    {
// On 5281, tclk is always 166 Mhz
    return 166666667;
    }
pub const SAR_MV88F5281_CPU_FREQ: c_int = 4;
pub const SAR_MV88F5281_CPU_FREQ_MASK: c_uint = 0xf;
#[no_mangle]
unsafe extern "C" fn mv88f5281_get_cpu_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init mv88f5281_get_cpu_freq(void __iomem *sar)
    {
    u32 opt = (readl(sar) >> SAR_MV88F5281_CPU_FREQ) &
    SAR_MV88F5281_CPU_FREQ_MASK;
    if (opt == 1 || opt == 2)
    return 400000000;
#[no_mangle]
pub unsafe extern "C" fn if(3: opt ==) -> else {
    else if (opt == 3)
    return 500000000;
    else
    return 0;
    }
    static void __init mv88f5281_get_clk_ratio(void __iomem *sar, int id,
    int *mult, int *div)
    {
    u32 opt = (readl(sar) >> SAR_MV88F5281_CPU_FREQ) &
    SAR_MV88F5281_CPU_FREQ_MASK;
    if (opt == 1) {
// mult = 1;
// div = 2;
    } else if (opt == 2 || opt == 3) {
// mult = 1;
// div = 3;
    } else {
// mult = 0;
// div = 1;
    }
    }
    static const struct coreclk_soc_desc mv88f5281_coreclks = {
    .get_tclk_freq = mv88f5281_get_tclk_freq,
    .get_cpu_freq = mv88f5281_get_cpu_freq,
    .get_clk_ratio = mv88f5281_get_clk_ratio,
    .ratios = orion_coreclk_ratios,
    .num_ratios = ARRAY_SIZE(orion_coreclk_ratios),
    };
#[no_mangle]
unsafe extern "C" fn mv88f5281_clk_init(np: *mut device_node) -> void __init {
    static void __init mv88f5281_clk_init(struct device_node *np)
    {
    return mvebu_coreclk_setup(np, &mv88f5281_coreclks);
    }
    CLK_OF_DECLARE(mv88f5281_clk, "marvell,mv88f5281-core-clock", mv88f5281_clk_init);
//
// Orion 6183
//
pub const SAR_MV88F6183_TCLK_FREQ: c_int = 9;
pub const SAR_MV88F6183_TCLK_FREQ_MASK: c_uint = 0x1;
#[no_mangle]
unsafe extern "C" fn mv88f6183_get_tclk_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init mv88f6183_get_tclk_freq(void __iomem *sar)
    {
    u32 opt = (readl(sar) >> SAR_MV88F6183_TCLK_FREQ) &
    SAR_MV88F6183_TCLK_FREQ_MASK;
    if (opt == 0)
    return 133333333;
#[no_mangle]
pub unsafe extern "C" fn if(1: opt ==) -> else {
    else if (opt == 1)
    return 166666667;
    else
    return 0;
    }
pub const SAR_MV88F6183_CPU_FREQ: c_int = 1;
pub const SAR_MV88F6183_CPU_FREQ_MASK: c_uint = 0x3f;
#[no_mangle]
unsafe extern "C" fn mv88f6183_get_cpu_freq(sar: *mut void __iomem) -> u32 __init {
    static u32 __init mv88f6183_get_cpu_freq(void __iomem *sar)
    {
    u32 opt = (readl(sar) >> SAR_MV88F6183_CPU_FREQ) &
    SAR_MV88F6183_CPU_FREQ_MASK;
    if (opt == 9)
    return 333333333;
#[no_mangle]
pub unsafe extern "C" fn if(17: opt ==) -> else {
    else if (opt == 17)
    return 400000000;
    else
    return 0;
    }
    static void __init mv88f6183_get_clk_ratio(void __iomem *sar, int id,
    int *mult, int *div)
    {
    u32 opt = (readl(sar) >> SAR_MV88F6183_CPU_FREQ) &
    SAR_MV88F6183_CPU_FREQ_MASK;
    if (opt == 9 || opt == 17) {
// mult = 1;
// div  = 2;
    } else {
// mult = 0;
// div  = 1;
    }
    }
    static const struct coreclk_soc_desc mv88f6183_coreclks = {
    .get_tclk_freq = mv88f6183_get_tclk_freq,
    .get_cpu_freq = mv88f6183_get_cpu_freq,
    .get_clk_ratio = mv88f6183_get_clk_ratio,
    .ratios = orion_coreclk_ratios,
    .num_ratios = ARRAY_SIZE(orion_coreclk_ratios),
    };
#[no_mangle]
unsafe extern "C" fn mv88f6183_clk_init(np: *mut device_node) -> void __init {
    static void __init mv88f6183_clk_init(struct device_node *np)
    {
    return mvebu_coreclk_setup(np, &mv88f6183_coreclks);
    }
    CLK_OF_DECLARE(mv88f6183_clk, "marvell,mv88f6183-core-clock", mv88f6183_clk_init);
