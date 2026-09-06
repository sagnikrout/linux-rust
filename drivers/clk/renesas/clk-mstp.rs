//! Automatically rewritten from C to Rust
//! Source: drivers/clk/renesas/clk-mstp.c
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
// R-Car MSTP clocks
//
// Copyright (C) 2013 Ideas On Board SPRL
// Copyright (C) 2015 Glider bvba
//
// Contact: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//

//
// MSTP clocks. We can't use standard gate clocks as we need to poll on the
// status register when enabling the clock.
//
pub const MSTP_MAX_CLOCKS: c_int = 32;
//
// struct mstp_clock_group - MSTP gating clocks group
//
// @data: clock specifier translation for clocks in this group
// @smstpcr: module stop control register
// @mstpsr: module stop status register (optional)
// @lock: protects writes to SMSTPCR
// @width_8bit: registers are 8-bit, not 32-bit
// @clks: clocks in this group
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstp_clock_group {
    pub data: clk_onecell_data,
    pub smstpcr: *mut void __iomem,
    pub mstpsr: *mut void __iomem,
    pub lock: spinlock_t,
    pub width_8bit: bool,
    pub clks: [*mut clk; ],
}

//
// struct mstp_clock - MSTP gating clock
// @hw: handle between common and hardware-specific interfaces
// @bit_index: control bit index
// @group: MSTP clocks group
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstp_clock {
    pub hw: clk_hw,
    pub bit_index: u32,
    pub group: *mut mstp_clock_group,
}

    static inline u32 cpg_mstp_read(struct mstp_clock_group *group,
    u32 __iomem *reg)
    {
    return group.width_8bit ? readb(reg) : readl(reg);
    }
    static inline void cpg_mstp_write(struct mstp_clock_group *group, u32 val,
    u32 __iomem *reg)
    {
    group.width_8bit ? writeb(val, reg) : writel(val, reg);
    }
#[no_mangle]
unsafe extern "C" fn cpg_mstp_clock_endisable(hw: *mut clk_hw, enable: bool) -> c_int {
    static int cpg_mstp_clock_endisable(struct clk_hw *hw, bool enable)
    {
    struct mstp_clock *clock = to_mstp_clock(hw);
    struct mstp_clock_group *group = clock.group;
    let mut bitmask: u32 = BIT(clock.bit_index);
    unsigned long flags;
    u32 value;
    int ret;
    spin_lock_irqsave(&group.lock, flags);
    value = cpg_mstp_read(group, group.smstpcr);
    if (enable)
    value &= ~bitmask;
    else
    value |= bitmask;
    cpg_mstp_write(group, value, group.smstpcr);
    if (!group.mstpsr) {
// dummy read to ensure write has completed
    cpg_mstp_read(group, group.smstpcr);
    barrier_data(group.smstpcr);
    }
    spin_unlock_irqrestore(&group.lock, flags);
    if (!enable || !group.mstpsr)
    return 0;
// group->width_8bit is always false if group->mstpsr is present
    ret = readl_poll_timeout_atomic(group.mstpsr, value,
    !(value & bitmask), 0, 10);
    if (ret)
    pr_err("%s: failed to enable %p[%d]\n", __func__,
    group.smstpcr, clock.bit_index);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cpg_mstp_clock_enable(hw: *mut clk_hw) -> c_int {
    static int cpg_mstp_clock_enable(struct clk_hw *hw)
    {
    return cpg_mstp_clock_endisable(hw, true);
    }
#[no_mangle]
unsafe extern "C" fn cpg_mstp_clock_disable(hw: *mut clk_hw) {
    static void cpg_mstp_clock_disable(struct clk_hw *hw)
    {
    cpg_mstp_clock_endisable(hw, false);
    }
#[no_mangle]
unsafe extern "C" fn cpg_mstp_clock_is_enabled(hw: *mut clk_hw) -> c_int {
    static int cpg_mstp_clock_is_enabled(struct clk_hw *hw)
    {
    struct mstp_clock *clock = to_mstp_clock(hw);
    struct mstp_clock_group *group = clock.group;
    u32 value;
    if (group.mstpsr)
    value = cpg_mstp_read(group, group.mstpsr);
    else
    value = cpg_mstp_read(group, group.smstpcr);
    return !(value & BIT(clock.bit_index));
    }
    static const struct clk_ops cpg_mstp_clock_ops = {
    .enable = cpg_mstp_clock_enable,
    .disable = cpg_mstp_clock_disable,
    .is_enabled = cpg_mstp_clock_is_enabled,
    };
    static struct clk * __init cpg_mstp_clock_register(const char *name,
    const char *parent_name, unsigned int index,
    struct mstp_clock_group *group)
    {
    let mut init: clk_init_data = {};
    struct mstp_clock *clock;
    struct clk *clk;
    clock = kzalloc_obj(*clock);
    if (!clock)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &cpg_mstp_clock_ops;
    init.flags = CLK_SET_RATE_PARENT;
// INTC-SYS is the module clock of the GIC, and must not be disabled
    if (!strcmp(name, "intc-sys")) {
    pr_debug("MSTP %s setting CLK_IS_CRITICAL\n", name);
    init.flags |= CLK_IS_CRITICAL;
    }
    init.parent_names = &parent_name;
    init.num_parents = 1;
    clock.bit_index = index;
    clock.group = group;
    clock.hw.init = &init;
    clk = clk_register(core::ptr::null_mut(), &clock.hw);
    if (IS_ERR(clk))
    kfree(clock);
    return clk;
    }
#[no_mangle]
unsafe extern "C" fn cpg_mstp_clocks_init(np: *mut device_node) -> void __init {
    static void __init cpg_mstp_clocks_init(struct device_node *np)
    {
    struct mstp_clock_group *group;
    const char *idxname;
    struct clk **clks;
    unsigned int i;
    group = kzalloc_flex(*group, clks, MSTP_MAX_CLOCKS);
    if (!group)
    return;
    clks = group.clks;
    spin_lock_init(&group.lock);
    group.data.clks = clks;
    group.smstpcr = of_iomap(np, 0);
    group.mstpsr = of_iomap(np, 1);
    if (group.smstpcr == core::ptr::null_mut()) {
    pr_err("%s: failed to remap SMSTPCR\n", __func__);
    kfree(group);
    return;
    }
    if (of_device_is_compatible(np, "renesas,r7s72100-mstp-clocks"))
    group.width_8bit = true;
    for (i = 0; i < MSTP_MAX_CLOCKS; ++i)
    clks[i] = ERR_PTR(-ENOENT);
    if (of_property_present(np, "clock-indices"))
    idxname = "clock-indices";
    else
    idxname = "renesas,clock-indices";
    for (i = 0; i < MSTP_MAX_CLOCKS; ++i) {
    const char *parent_name;
    const char *name;
    u32 clkidx;
    int ret;
// Skip clocks with no name.
    ret = of_property_read_string_index(np, "clock-output-names",
    i, &name);
    if (ret < 0 || strlen(name) == 0)
    continue;
    parent_name = of_clk_get_parent_name(np, i);
    ret = of_property_read_u32_index(np, idxname, i, &clkidx);
    if (parent_name == core::ptr::null_mut() || ret < 0)
    break;
    if (clkidx >= MSTP_MAX_CLOCKS) {
    pr_err("%s: invalid clock %pOFn %s index %u\n",
    __func__, np, name, clkidx);
    continue;
    }
    clks[clkidx] = cpg_mstp_clock_register(name, parent_name,
    clkidx, group);
    if (!IS_ERR(clks[clkidx]))
    group.data.clk_num = max(group.data.clk_num,
    clkidx + 1);
    else
    pr_err("%s: failed to register %pOFn %s clock (%ld)\n",
    __func__, np, name, PTR_ERR(clks[clkidx]));
    }
    of_clk_add_provider(np, of_clk_src_onecell_get, &group.data);
    }
    CLK_OF_DECLARE(cpg_mstp_clks, "renesas,cpg-mstp-clocks", cpg_mstp_clocks_init);
#[no_mangle]
pub unsafe extern "C" fn cpg_mstp_attach_dev(unused: *mut generic_pm_domain, dev: *mut device) -> c_int {
    int cpg_mstp_attach_dev(struct generic_pm_domain *unused, struct device *dev)
    {
    struct device_node *np = dev.of_node;
    struct of_phandle_args clkspec;
    struct clk *clk;
    let mut i: c_int = 0;
    int error;
    while (!of_parse_phandle_with_args(np, "clocks", "#clock-cells", i,
    &clkspec)) {
    if (of_device_is_compatible(clkspec.np,
    "renesas,cpg-mstp-clocks"))
    goto found;
// BSC on r8a73a4/sh73a0 uses zb_clk instead of an mstp clock
    if (of_node_name_eq(clkspec.np, "zb_clk"))
    goto found;
    of_node_put(clkspec.np);
    i++;
    }
    return 0;
    found:
    clk = of_clk_get_from_provider(&clkspec);
    of_node_put(clkspec.np);
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    error = pm_clk_create(dev);
    if (error)
    goto fail_put;
    error = pm_clk_add_clk(dev, clk);
    if (error)
    goto fail_destroy;
    return 0;
    fail_destroy:
    pm_clk_destroy(dev);
    fail_put:
    clk_put(clk);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn cpg_mstp_detach_dev(unused: *mut generic_pm_domain, dev: *mut device) {
    void cpg_mstp_detach_dev(struct generic_pm_domain *unused, struct device *dev)
    {
    if (!pm_clk_no_clocks(dev))
    pm_clk_destroy(dev);
    }
    let mut __initdata: *mut static struct device_node cpg_mstp_pd_np = core::ptr::null_mut();
    let mut __initdata: *mut static struct generic_pm_domain cpg_mstp_pd_genpd = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn cpg_mstp_add_clk_domain(np: *mut device_node) -> void __init {
    void __init cpg_mstp_add_clk_domain(struct device_node *np)
    {
    struct generic_pm_domain *pd;
    u32 ncells;
    if (of_property_read_u32(np, "#power-domain-cells", &ncells)) {
    pr_warn("%pOF lacks #power-domain-cells\n", np);
    return;
    }
    pd = kzalloc_obj(*pd);
    if (!pd)
    return;
    pd.name = np.name;
    pd.flags = GENPD_FLAG_PM_CLK | GENPD_FLAG_ALWAYS_ON |
    GENPD_FLAG_ACTIVE_WAKEUP;
    pd.attach_dev = cpg_mstp_attach_dev;
    pd.detach_dev = cpg_mstp_detach_dev;
    pm_genpd_init(pd, &pm_domain_always_on_gov, false);
    cpg_mstp_pd_np = of_node_get(np);
    cpg_mstp_pd_genpd = pd;
    }
#[no_mangle]
unsafe extern "C" fn cpg_mstp_pd_init_provider() -> int __init {
    static int __init cpg_mstp_pd_init_provider(void)
    {
    int error;
    if (!cpg_mstp_pd_np)
    return -ENODEV;
    error = of_genpd_add_provider_simple(cpg_mstp_pd_np, cpg_mstp_pd_genpd);
    of_node_put(cpg_mstp_pd_np);
    return error;
    }
    postcore_initcall(cpg_mstp_pd_init_provider);
