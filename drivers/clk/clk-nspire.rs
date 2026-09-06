//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-nspire.c
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
// Copyright (C) 2013 Daniel Tang <tangrs@tangrs.id.au>
//

pub const BASE_CPU_SHIFT: c_int = 1;
pub const BASE_CPU_MASK: c_uint = 0x7F;
pub const CPU_AHB_SHIFT: c_int = 12;
pub const CPU_AHB_MASK: c_uint = 0x07;
pub const FIXED_BASE_SHIFT: c_int = 8;
pub const FIXED_BASE_MASK: c_uint = 0x01;
pub const CLASSIC_BASE_SHIFT: c_int = 16;
pub const CLASSIC_BASE_MASK: c_uint = 0x1F;
pub const CX_BASE_SHIFT: c_int = 15;
pub const CX_BASE_MASK: c_uint = 0x3F;
pub const CX_UNKNOWN_SHIFT: c_int = 21;
pub const CX_UNKNOWN_MASK: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nspire_clk_info {
    pub base_clock: u32,
    pub base_cpu_ratio: u16,
    pub base_ahb_ratio: u16,
}

#[no_mangle]
unsafe extern "C" fn nspire_clkinfo_cx(val: u32, clk: *mut nspire_clk_info) {
    static void nspire_clkinfo_cx(u32 val, struct nspire_clk_info *clk)
    {
    if (EXTRACT(val, FIXED_BASE))
    clk.base_clock = 48 * MHZ;
    else
    clk.base_clock = 6 * EXTRACT(val, CX_BASE) * MHZ;
    clk.base_cpu_ratio = EXTRACT(val, BASE_CPU) * EXTRACT(val, CX_UNKNOWN);
    clk.base_ahb_ratio = clk.base_cpu_ratio * (EXTRACT(val, CPU_AHB) + 1);
    }
#[no_mangle]
unsafe extern "C" fn nspire_clkinfo_classic(val: u32, clk: *mut nspire_clk_info) {
    static void nspire_clkinfo_classic(u32 val, struct nspire_clk_info *clk)
    {
    if (EXTRACT(val, FIXED_BASE))
    clk.base_clock = 27 * MHZ;
    else
    clk.base_clock = (300 - 6 * EXTRACT(val, CLASSIC_BASE)) * MHZ;
    clk.base_cpu_ratio = EXTRACT(val, BASE_CPU) * 2;
    clk.base_ahb_ratio = clk.base_cpu_ratio * (EXTRACT(val, CPU_AHB) + 1);
    }
    static void __init nspire_ahbdiv_setup(struct device_node *node,
    void (*get_clkinfo)(u32, struct nspire_clk_info *))
    {
    u32 val;
    void __iomem *io;
    struct clk_hw *hw;
    const char *clk_name = node.name;
    const char *parent_name;
    struct nspire_clk_info info;
    io = of_iomap(node, 0);
    if (!io)
    return;
    val = readl(io);
    iounmap(io);
    get_clkinfo(val, &info);
    of_property_read_string(node, "clock-output-names", &clk_name);
    parent_name = of_clk_get_parent_name(node, 0);
    hw = clk_hw_register_fixed_factor(core::ptr::null_mut(), clk_name, parent_name, 0,
    1, info.base_ahb_ratio);
    if (!IS_ERR(hw))
    of_clk_add_hw_provider(node, of_clk_hw_simple_get, hw);
    }
#[no_mangle]
unsafe extern "C" fn nspire_ahbdiv_setup_cx(node: *mut device_node) -> void __init {
    static void __init nspire_ahbdiv_setup_cx(struct device_node *node)
    {
    nspire_ahbdiv_setup(node, nspire_clkinfo_cx);
    }
#[no_mangle]
unsafe extern "C" fn nspire_ahbdiv_setup_classic(node: *mut device_node) -> void __init {
    static void __init nspire_ahbdiv_setup_classic(struct device_node *node)
    {
    nspire_ahbdiv_setup(node, nspire_clkinfo_classic);
    }
    CLK_OF_DECLARE(nspire_ahbdiv_cx, "lsi,nspire-cx-ahb-divider",
    nspire_ahbdiv_setup_cx);
    CLK_OF_DECLARE(nspire_ahbdiv_classic, "lsi,nspire-classic-ahb-divider",
    nspire_ahbdiv_setup_classic);
    static void __init nspire_clk_setup(struct device_node *node,
    void (*get_clkinfo)(u32, struct nspire_clk_info *))
    {
    u32 val;
    void __iomem *io;
    struct clk_hw *hw;
    const char *clk_name = node.name;
    struct nspire_clk_info info;
    io = of_iomap(node, 0);
    if (!io)
    return;
    val = readl(io);
    iounmap(io);
    get_clkinfo(val, &info);
    of_property_read_string(node, "clock-output-names", &clk_name);
    hw = clk_hw_register_fixed_rate(core::ptr::null_mut(), clk_name, core::ptr::null_mut(), 0,
    info.base_clock);
    if (!IS_ERR(hw))
    of_clk_add_hw_provider(node, of_clk_hw_simple_get, hw);
    else
    return;
    pr_info("TI-NSPIRE Base: %uMHz CPU: %uMHz AHB: %uMHz\n",
    info.base_clock / MHZ,
    info.base_clock / info.base_cpu_ratio / MHZ,
    info.base_clock / info.base_ahb_ratio / MHZ);
    }
#[no_mangle]
unsafe extern "C" fn nspire_clk_setup_cx(node: *mut device_node) -> void __init {
    static void __init nspire_clk_setup_cx(struct device_node *node)
    {
    nspire_clk_setup(node, nspire_clkinfo_cx);
    }
#[no_mangle]
unsafe extern "C" fn nspire_clk_setup_classic(node: *mut device_node) -> void __init {
    static void __init nspire_clk_setup_classic(struct device_node *node)
    {
    nspire_clk_setup(node, nspire_clkinfo_classic);
    }
    CLK_OF_DECLARE(nspire_clk_cx, "lsi,nspire-cx-clock", nspire_clk_setup_cx);
    CLK_OF_DECLARE(nspire_clk_classic, "lsi,nspire-classic-clock",
    nspire_clk_setup_classic);
