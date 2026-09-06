//! Automatically rewritten from C to Rust
//! Source: drivers/clk/sunxi/clk-a20-gmac.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2013 Emilio López
// Emilio López <emilio@elopez.com.ar>
//
// Copyright 2013 Chen-Yu Tsai
// Chen-Yu Tsai <wens@csie.org>
//

    static DEFINE_SPINLOCK(gmac_lock);
pub const SUN7I_A20_GMAC_GPIT: c_int = 2;
pub const SUN7I_A20_GMAC_MASK: c_uint = 0x3;
pub const SUN7I_A20_GMAC_PARENTS: c_int = 2;
    static u32 sun7i_a20_gmac_mux_table[SUN7I_A20_GMAC_PARENTS] = {
    0x00, /* Select mii_phy_tx_clk */
    0x02, /* Select gmac_int_tx_clk */
    };
//
// sun7i_a20_gmac_clk_setup - Setup function for A20/A31 GMAC clock module
// @node: &struct device_node for the clock
//
// This clock looks something like this
// ________________________
// MII TX clock from PHY >-----|___________    _________|----> to GMAC core
// GMAC Int. RGMII TX clk >----|___________\__/__gate---|----> to PHY
// Ext. 125MHz RGMII TX clk >--|__divider__/            |
// |________________________|
//
// The external 125 MHz reference is optional, i.e. GMAC can use its
// internal TX clock just fine. The A31 GMAC clock module does not have
// the divider controls for the external reference.
//
// To keep it simple, let the GMAC use either the MII TX clock for MII mode,
// and its internal TX clock for GMII and RGMII modes. The GMAC driver should
// select the appropriate source and gate/ungate the output to the PHY.
//
// Only the GMAC should use this clock. Altering the clock so that it doesn't
// match the GMAC's operation parameters will result in the GMAC not being
// able to send traffic out. The GMAC driver should set the clock rate and
// enable/disable this clock to configure the required state. The clock
// driver then responds by auto-reparenting the clock.
//
#[no_mangle]
unsafe extern "C" fn sun7i_a20_gmac_clk_setup(node: *mut device_node) -> void __init {
    static void __init sun7i_a20_gmac_clk_setup(struct device_node *node)
    {
    struct clk *clk;
    struct clk_mux *mux;
    struct clk_gate *gate;
    const char *clk_name = node.name;
    const char *parents[SUN7I_A20_GMAC_PARENTS];
    void __iomem *reg;
    if (of_property_read_string(node, "clock-output-names", &clk_name))
    return;
// allocate mux and gate clock structs
    mux = kzalloc_obj(struct clk_mux);
    if (!mux)
    return;
    gate = kzalloc_obj(struct clk_gate);
    if (!gate)
    goto free_mux;
// gmac clock requires exactly 2 parents
    if (of_clk_parent_fill(node, parents, 2) != 2)
    goto free_gate;
    reg = of_iomap(node, 0);
    if (!reg)
    goto free_gate;
// set up gate and fixed rate properties
    gate.reg = reg;
    gate.bit_idx = SUN7I_A20_GMAC_GPIT;
    gate.lock = &gmac_lock;
    mux.reg = reg;
    mux.mask = SUN7I_A20_GMAC_MASK;
    mux.table = sun7i_a20_gmac_mux_table;
    mux.lock = &gmac_lock;
    clk = clk_register_composite(core::ptr::null_mut(), clk_name,
    parents, SUN7I_A20_GMAC_PARENTS,
    &mux.hw, &clk_mux_ops,
    core::ptr::null_mut(), core::ptr::null_mut(),
    &gate.hw, &clk_gate_ops,
    0);
    if (IS_ERR(clk))
    goto iounmap_reg;
    of_clk_add_provider(node, of_clk_src_simple_get, clk);
    return;
    iounmap_reg:
    iounmap(reg);
    free_gate:
    kfree(gate);
    free_mux:
    kfree(mux);
    }
    CLK_OF_DECLARE(sun7i_a20_gmac, "allwinner,sun7i-a20-gmac-clk",
    sun7i_a20_gmac_clk_setup);
