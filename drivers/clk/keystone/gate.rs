//! Automatically rewritten from C to Rust
//! Source: drivers/clk/keystone/gate.c
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
// Clock driver for Keystone 2 based devices
//
// Copyright (C) 2013 Texas Instruments.
// Murali Karicheri <m-karicheri2@ti.com>
// Santosh Shilimkar <santosh.shilimkar@ti.com>
//

// PSC register offsets
pub const PTCMD: c_uint = 0x120;
pub const PTSTAT: c_uint = 0x128;
pub const PDSTAT: c_uint = 0x200;
pub const PDCTL: c_uint = 0x300;
pub const MDSTAT: c_uint = 0x800;
pub const MDCTL: c_uint = 0xa00;
// PSC module states
pub const PSC_STATE_SWRSTDISABLE: c_int = 0;
pub const PSC_STATE_SYNCRST: c_int = 1;
pub const PSC_STATE_DISABLE: c_int = 2;
pub const PSC_STATE_ENABLE: c_int = 3;
pub const MDSTAT_STATE_MASK: c_uint = 0x3f;

pub const PDSTAT_STATE_MASK: c_uint = 0x1f;

// Maximum timeout to bail out state transition for module
pub const STATE_TRANS_MAX_COUNT: c_uint = 0xffff;
    static void __iomem *domain_transition_base;
//
// struct clk_psc_data - PSC data
// @control_base: Base address for a PSC control
// @domain_base: Base address for a PSC domain
// @domain_id: PSC domain id number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_psc_data {
    pub control_base: *mut void __iomem,
    pub domain_base: *mut void __iomem,
    pub domain_id: u32,
}

//
// struct clk_psc - PSC clock structure
// @hw: clk_hw for the psc
// @psc_data: PSC driver specific data
// @lock: Spinlock used by the driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_psc {
    pub hw: clk_hw,
    pub psc_data: *mut clk_psc_data,
    pub lock: *mut spinlock_t,
}

    static DEFINE_SPINLOCK(psc_lock);

    static void psc_config(void __iomem *control_base, void __iomem *domain_base,
    u32 next_state, u32 domain_id)
    {
    u32 ptcmd, pdstat, pdctl, mdstat, mdctl, ptstat;
    let mut count: u32 = STATE_TRANS_MAX_COUNT;
    mdctl = readl(control_base + MDCTL);
    mdctl &= ~MDSTAT_STATE_MASK;
    mdctl |= next_state;
// For disable, we always put the module in local reset
    if (next_state == PSC_STATE_DISABLE)
    mdctl &= ~MDCTL_LRESET;
    writel(mdctl, control_base + MDCTL);
    pdstat = readl(domain_base + PDSTAT);
    if (!(pdstat & PDSTAT_STATE_MASK)) {
    pdctl = readl(domain_base + PDCTL);
    pdctl |= PDCTL_NEXT;
    writel(pdctl, domain_base + PDCTL);
    }
    ptcmd = 1 << domain_id;
    writel(ptcmd, domain_transition_base + PTCMD);
    do {
    ptstat = readl(domain_transition_base + PTSTAT);
    } while (((ptstat >> domain_id) & 1) && count--);
    count = STATE_TRANS_MAX_COUNT;
    do {
    mdstat = readl(control_base + MDSTAT);
    } while (!((mdstat & MDSTAT_STATE_MASK) == next_state) && count--);
    }
#[no_mangle]
unsafe extern "C" fn keystone_clk_is_enabled(hw: *mut clk_hw) -> c_int {
    static int keystone_clk_is_enabled(struct clk_hw *hw)
    {
    struct clk_psc *psc = to_clk_psc(hw);
    struct clk_psc_data *data = psc.psc_data;
    let mut mdstat: u32 = readl(data.control_base + MDSTAT);
    return (mdstat & MDSTAT_MCKOUT) ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn keystone_clk_enable(hw: *mut clk_hw) -> c_int {
    static int keystone_clk_enable(struct clk_hw *hw)
    {
    struct clk_psc *psc = to_clk_psc(hw);
    struct clk_psc_data *data = psc.psc_data;
    let mut flags: c_ulong = 0;
    if (psc.lock)
    spin_lock_irqsave(psc.lock, flags);
    psc_config(data.control_base, data.domain_base,
    PSC_STATE_ENABLE, data.domain_id);
    if (psc.lock)
    spin_unlock_irqrestore(psc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keystone_clk_disable(hw: *mut clk_hw) {
    static void keystone_clk_disable(struct clk_hw *hw)
    {
    struct clk_psc *psc = to_clk_psc(hw);
    struct clk_psc_data *data = psc.psc_data;
    let mut flags: c_ulong = 0;
    if (psc.lock)
    spin_lock_irqsave(psc.lock, flags);
    psc_config(data.control_base, data.domain_base,
    PSC_STATE_DISABLE, data.domain_id);
    if (psc.lock)
    spin_unlock_irqrestore(psc.lock, flags);
    }
    static const struct clk_ops clk_psc_ops = {
    .enable = keystone_clk_enable,
    .disable = keystone_clk_disable,
    .is_enabled = keystone_clk_is_enabled,
    };
//
// clk_register_psc - register psc clock
// @dev: device that is registering this clock
// @name: name of this clock
// @parent_name: name of clock's parent
// @psc_data: platform data to configure this clock
// @lock: spinlock used by this clock
//
    static struct clk *clk_register_psc(struct device *dev,
    const char *name,
    const char *parent_name,
    struct clk_psc_data *psc_data,
    spinlock_t *lock)
    {
    struct clk_init_data init;
    struct clk_psc *psc;
    struct clk *clk;
    psc = kzalloc_obj(*psc);
    if (!psc)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &clk_psc_ops;
    init.flags = 0;
    init.parent_names = (parent_name ? &parent_name : core::ptr::null_mut());
    init.num_parents = (parent_name ? 1 : 0);
    psc.psc_data = psc_data;
    psc.lock = lock;
    psc.hw.init = &init;
    clk = clk_register(core::ptr::null_mut(), &psc.hw);
    if (IS_ERR(clk))
    kfree(psc);
    return clk;
    }
//
// of_psc_clk_init - initialize psc clock through DT
// @node: device tree node for this clock
// @lock: spinlock used by this clock
//
#[no_mangle]
unsafe extern "C" fn of_psc_clk_init(node: *mut device_node, lock: *mut spinlock_t) -> void __init {
    static void __init of_psc_clk_init(struct device_node *node, spinlock_t *lock)
    {
    const char *clk_name = node.name;
    const char *parent_name;
    struct clk_psc_data *data;
    struct clk *clk;
    int i;
    data = kzalloc_obj(*data);
    if (!data) {
    pr_err("%s: Out of memory\n", __func__);
    return;
    }
    i = of_property_match_string(node, "reg-names", "control");
    data.control_base = of_iomap(node, i);
    if (!data.control_base) {
    pr_err("%s: control ioremap failed\n", __func__);
    goto out;
    }
    i = of_property_match_string(node, "reg-names", "domain");
    data.domain_base = of_iomap(node, i);
    if (!data.domain_base) {
    pr_err("%s: domain ioremap failed\n", __func__);
    goto unmap_ctrl;
    }
    of_property_read_u32(node, "domain-id", &data.domain_id);
// Domain transition registers at fixed address space of domain_id 0
    if (!domain_transition_base && !data.domain_id)
    domain_transition_base = data.domain_base;
    of_property_read_string(node, "clock-output-names", &clk_name);
    parent_name = of_clk_get_parent_name(node, 0);
    if (!parent_name) {
    pr_err("%s: Parent clock not found\n", __func__);
    goto unmap_domain;
    }
    clk = clk_register_psc(core::ptr::null_mut(), clk_name, parent_name, data, lock);
    if (!IS_ERR(clk)) {
    of_clk_add_provider(node, of_clk_src_simple_get, clk);
    return;
    }
    pr_err("%s: error registering clk %pOFn\n", __func__, node);
    unmap_domain:
    iounmap(data.domain_base);
    unmap_ctrl:
    iounmap(data.control_base);
    out:
    kfree(data);
    return;
    }
//
// of_keystone_psc_clk_init - initialize psc clock through DT
// @node: device tree node for this clock
//
#[no_mangle]
unsafe extern "C" fn of_keystone_psc_clk_init(node: *mut device_node) -> void __init {
    static void __init of_keystone_psc_clk_init(struct device_node *node)
    {
    of_psc_clk_init(node, &psc_lock);
    }
    CLK_OF_DECLARE(keystone_gate_clk, "ti,keystone,psc-clock",
    of_keystone_psc_clk_init);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Clock driver for Keystone 2 based devices");
    MODULE_AUTHOR("Murali Karicheri <m-karicheri2@ti.com>");
    MODULE_AUTHOR("Santosh Shilimkar <santosh.shilimkar@ti.com>");
