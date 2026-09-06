//! Automatically rewritten from C to Rust
//! Source: drivers/clk/st/clkgen-mux.c
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
// clkgen-mux.c: ST GEN-MUX Clock driver
//
// Copyright (C) 2014 STMicroelectronics (R&D) Limited
//
// Authors: Stephen Gallimore <stephen.gallimore@st.com>
// Pankaj Dev <pankaj.dev@st.com>
//

    static const char ** __init clkgen_mux_get_parents(struct device_node *np,
    int *num_parents)
    {
    const char **parents;
    unsigned int nparents;
    nparents = of_clk_get_parent_count(np);
    if (WARN_ON(!nparents))
    return ERR_PTR(-EINVAL);
    parents = kcalloc(nparents, sizeof(const char *), GFP_KERNEL);
    if (!parents)
    return ERR_PTR(-ENOMEM);
// num_parents = of_clk_parent_fill(np, parents, nparents);
    return parents;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clkgen_mux_data {
    pub offset: u32,
    pub shift: u8,
    pub width: u8,
    pub lock: *mut spinlock_t,
    pub clk_flags: c_ulong,
    pub mux_flags: u8,
}

    static struct clkgen_mux_data stih407_a9_mux_data = {
    .offset = 0x1a4,
    .shift = 0,
    .width = 2,
    .lock = &clkgen_a9_lock,
    };
    static void __init st_of_clkgen_mux_setup(struct device_node *np,
    struct clkgen_mux_data *data)
    {
    struct clk *clk;
    void __iomem *reg;
    const char **parents;
    let mut num_parents: c_int = 0;
    struct device_node *parent_np;
//
// First check for reg property within the node to keep backward
// compatibility, then if reg doesn't exist look at the parent node
//
    reg = of_iomap(np, 0);
    if (!reg) {
    parent_np = of_get_parent(np);
    reg = of_iomap(parent_np, 0);
    of_node_put(parent_np);
    if (!reg) {
    pr_err("%s: Failed to get base address\n", __func__);
    return;
    }
    }
    parents = clkgen_mux_get_parents(np, &num_parents);
    if (IS_ERR(parents)) {
    pr_err("%s: Failed to get parents (%ld)\n",
    __func__, PTR_ERR(parents));
    goto err_parents;
    }
    clk = clk_register_mux(core::ptr::null_mut(), np.name, parents, num_parents,
    data.clk_flags | CLK_SET_RATE_PARENT,
    reg + data.offset,
    data.shift, data.width, data.mux_flags,
    data.lock);
    if (IS_ERR(clk))
    goto err;
    pr_debug("%s: parent %s rate %u\n",
    __clk_get_name(clk),
    __clk_get_name(clk_get_parent(clk)),
    (unsigned int)clk_get_rate(clk));
    kfree(parents);
    of_clk_add_provider(np, of_clk_src_simple_get, clk);
    return;
    err:
    kfree(parents);
    err_parents:
    iounmap(reg);
    }
#[no_mangle]
unsafe extern "C" fn st_of_clkgen_a9_mux_setup(np: *mut device_node) -> void __init {
    static void __init st_of_clkgen_a9_mux_setup(struct device_node *np)
    {
    st_of_clkgen_mux_setup(np, &stih407_a9_mux_data);
    }
    CLK_OF_DECLARE(clkgen_a9mux, "st,stih407-clkgen-a9-mux",
    st_of_clkgen_a9_mux_setup);
