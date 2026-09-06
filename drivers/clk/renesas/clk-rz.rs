//! Automatically rewritten from C to Rust
//! Source: drivers/clk/renesas/clk-rz.c
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
// RZ/A1 Core CPG Clocks
//
// Copyright (C) 2013 Ideas On Board SPRL
// Copyright (C) 2014 Wolfram Sang, Sang Engineering <wsa@sang-engineering.com>
//

pub const CPG_FRQCR: c_uint = 0x10;
pub const CPG_FRQCR2: c_uint = 0x14;
pub const PPR0: c_uint = 0xFCFE3200;
pub const PIBC0: c_uint = 0xFCFE7000;

// -----------------------------------------------------------------------------
// Initialization
//
#[no_mangle]
unsafe extern "C" fn rz_cpg_read_mode_pins() -> u16 __init {
    static u16 __init rz_cpg_read_mode_pins(void)
    {
    void __iomem *ppr0, *pibc0;
    u16 modes;
    ppr0 = ioremap(PPR0, 2);
    pibc0 = ioremap(PIBC0, 2);
    BUG_ON(!ppr0 || !pibc0);
    iowrite16(4, pibc0);	/* enable input buffer */
    modes = ioread16(ppr0);
    iounmap(ppr0);
    iounmap(pibc0);
    return modes;
    }
    static struct clk * __init
    rz_cpg_register_clock(struct device_node *np, void __iomem *base,
    const char *name)
    {
    u32 val;
    unsigned mult;
    static const unsigned frqcr_tab[4] = { 3, 2, 0, 1 };
    if (strcmp(name, "pll") == 0) {
    let mut cpg_mode: c_uint = MD_CLK(rz_cpg_read_mode_pins());
    const char *parent_name = of_clk_get_parent_name(np, cpg_mode);
    mult = cpg_mode ? (32 / 4) : 30;
    return clk_register_fixed_factor(core::ptr::null_mut(), name, parent_name, 0, mult, 1);
    }
// If mapping regs failed, skip non-pll clocks. System will boot anyhow
    if (!base)
    return ERR_PTR(-ENXIO);
// FIXME:"i" and "g" are variable clocks with non-integer dividers (e.g. 2/3)
// and the constraint that always g <= i. To get the rz platform started,
// let them run at fixed current speed and implement the details later.
//
    if (strcmp(name, "i") == 0)
    val = (readl(base + CPG_FRQCR) >> 8) & 3;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(name, 0: "g") ==) -> else {
    else if (strcmp(name, "g") == 0)
    val = readl(base + CPG_FRQCR2) & 3;
    else
    return ERR_PTR(-EINVAL);
    mult = frqcr_tab[val];
    return clk_register_fixed_factor(core::ptr::null_mut(), name, "pll", 0, mult, 3);
    }
#[no_mangle]
unsafe extern "C" fn rz_cpg_clocks_init(np: *mut device_node) -> void __init {
    static void __init rz_cpg_clocks_init(struct device_node *np)
    {
    struct clk_onecell_data *data;
    struct clk **clks;
    void __iomem *base;
    unsigned i;
    int num_clks;
    num_clks = of_property_count_strings(np, "clock-output-names");
    if (WARN(num_clks <= 0, "can't count CPG clocks\n"))
    return;
    data = kzalloc_obj(*data);
    clks = kzalloc_objs(*clks, num_clks);
    BUG_ON(!data || !clks);
    data.clks = clks;
    data.clk_num = num_clks;
    base = of_iomap(np, 0);
    for (i = 0; i < num_clks; ++i) {
    const char *name;
    struct clk *clk;
    of_property_read_string_index(np, "clock-output-names", i, &name);
    clk = rz_cpg_register_clock(np, base, name);
    if (IS_ERR(clk))
    pr_err("%s: failed to register %pOFn %s clock (%ld)\n",
    __func__, np, name, PTR_ERR(clk));
    else
    data.clks[i] = clk;
    }
    of_clk_add_provider(np, of_clk_src_onecell_get, data);
    cpg_mstp_add_clk_domain(np);
    }
    CLK_OF_DECLARE(rz_cpg_clks, "renesas,rz-cpg-clocks", rz_cpg_clocks_init);
