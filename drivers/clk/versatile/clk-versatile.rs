//! Automatically rewritten from C to Rust
//! Source: drivers/clk/versatile/clk-versatile.c
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
// Clock driver for the ARM Integrator/AP, Integrator/CP, Versatile AB and
// Versatile PB boards.
// Copyright (C) 2012 Linus Walleij
//

pub const INTEGRATOR_HDR_LOCK_OFFSET: c_uint = 0x14;
pub const VERSATILE_SYS_OSCCLCD_OFFSET: c_uint = 0x1c;
pub const VERSATILE_SYS_LOCK_OFFSET: c_uint = 0x20;
// Base offset for the core module
    static void __iomem *cm_base;
    static const struct icst_params cp_auxosc_params = {
    .vco_max	= ICST525_VCO_MAX_5V,
    .vco_min	= ICST525_VCO_MIN,
    .vd_min 	= 8,
    .vd_max 	= 263,
    .rd_min 	= 3,
    .rd_max 	= 65,
    .s2div		= icst525_s2div,
    .idx2s		= icst525_idx2s,
    };
    static const struct clk_icst_desc cm_auxosc_desc __initconst = {
    .params = &cp_auxosc_params,
    .vco_offset = 0x1c,
    .lock_offset = INTEGRATOR_HDR_LOCK_OFFSET,
    };
    static const struct icst_params versatile_auxosc_params = {
    .vco_max	= ICST307_VCO_MAX,
    .vco_min	= ICST307_VCO_MIN,
    .vd_min		= 4 + 8,
    .vd_max		= 511 + 8,
    .rd_min		= 1 + 2,
    .rd_max		= 127 + 2,
    .s2div		= icst307_s2div,
    .idx2s		= icst307_idx2s,
    };
    static const struct clk_icst_desc versatile_auxosc_desc __initconst = {
    .params = &versatile_auxosc_params,
    .vco_offset = VERSATILE_SYS_OSCCLCD_OFFSET,
    .lock_offset = VERSATILE_SYS_LOCK_OFFSET,
    };
    static void __init cm_osc_setup(struct device_node *np,
    const struct clk_icst_desc *desc)
    {
    struct clk *clk;
    const char *clk_name = np.name;
    const char *parent_name;
    if (!cm_base) {
// Remap the core module base if not done yet
    struct device_node *parent;
    parent = of_get_parent(np);
    if (!parent) {
    pr_err("no parent on core module clock\n");
    return;
    }
    cm_base = of_iomap(parent, 0);
    of_node_put(parent);
    if (!cm_base) {
    pr_err("could not remap core module base\n");
    return;
    }
    }
    parent_name = of_clk_get_parent_name(np, 0);
    clk = icst_clk_register(core::ptr::null_mut(), desc, clk_name, parent_name, cm_base);
    if (!IS_ERR(clk))
    of_clk_add_provider(np, of_clk_src_simple_get, clk);
    }
#[no_mangle]
unsafe extern "C" fn of_integrator_cm_osc_setup(np: *mut device_node) -> void __init {
    static void __init of_integrator_cm_osc_setup(struct device_node *np)
    {
    cm_osc_setup(np, &cm_auxosc_desc);
    }
    CLK_OF_DECLARE(integrator_cm_auxosc_clk,
    "arm,integrator-cm-auxosc", of_integrator_cm_osc_setup);
#[no_mangle]
unsafe extern "C" fn of_versatile_cm_osc_setup(np: *mut device_node) -> void __init {
    static void __init of_versatile_cm_osc_setup(struct device_node *np)
    {
    cm_osc_setup(np, &versatile_auxosc_desc);
    }
    CLK_OF_DECLARE(versatile_cm_auxosc_clk,
    "arm,versatile-cm-auxosc", of_versatile_cm_osc_setup);
