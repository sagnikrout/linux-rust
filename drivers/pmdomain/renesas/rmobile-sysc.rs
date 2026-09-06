//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/renesas/rmobile-sysc.c
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
// rmobile power management support
//
// Copyright (C) 2012  Renesas Solutions Corp.
// Copyright (C) 2012  Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
// Copyright (C) 2014  Glider bvba
//
// based on pm-sh7372.c
// Copyright (C) 2011 Magnus Damm
//

// SYSC
pub const SPDCR: c_uint = 0x08	/* SYS Power Down Control Register */;
pub const SWUCR: c_uint = 0x14	/* SYS Wakeup Control Register */;
pub const PSTR: c_uint = 0x80	/* Power Status Register */;
pub const PSTR_RETRIES: c_int = 100;
pub const PSTR_DELAY_US: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmobile_pm_domain {
    pub genpd: generic_pm_domain,
    pub gov: *mut dev_power_governor,
    pub (*suspend)(void): *mut c_int,
    pub base: *mut void __iomem,
    pub bit_shift: c_uint,
}

    static inline
    struct rmobile_pm_domain *to_rmobile_pd(struct generic_pm_domain *d)
    {
    return container_of(d, struct rmobile_pm_domain, genpd);
    }
#[no_mangle]
unsafe extern "C" fn rmobile_pd_power_down(genpd: *mut generic_pm_domain) -> c_int {
    static int rmobile_pd_power_down(struct generic_pm_domain *genpd)
    {
    struct rmobile_pm_domain *rmobile_pd = to_rmobile_pd(genpd);
    let mut mask: c_uint = BIT(rmobile_pd.bit_shift);
    u32 val;
    if (rmobile_pd.suspend) {
    let mut ret: c_int = rmobile_pd.suspend();
    if (ret)
    return ret;
    }
    if (readl(rmobile_pd.base + PSTR) & mask) {
    writel(mask, rmobile_pd.base + SPDCR);
    readl_poll_timeout_atomic(rmobile_pd.base + SPDCR, val,
    !(val & mask), 0, PSTR_RETRIES);
    }
    pr_debug("%s: Power off, 0x%08x . PSTR = 0x%08x\n", genpd.name, mask,
    readl(rmobile_pd.base + PSTR));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __rmobile_pd_power_up(rmobile_pd: *mut rmobile_pm_domain) -> c_int {
    static int __rmobile_pd_power_up(struct rmobile_pm_domain *rmobile_pd)
    {
    unsigned int val, mask = BIT(rmobile_pd.bit_shift);
    let mut ret: c_int = 0;
    if (readl(rmobile_pd.base + PSTR) & mask)
    return ret;
    writel(mask, rmobile_pd.base + SWUCR);
    ret = readl_poll_timeout_atomic(rmobile_pd.base + SWUCR, val,
    (val & mask), PSTR_DELAY_US,
    PSTR_RETRIES * PSTR_DELAY_US);
    pr_debug("%s: Power on, 0x%08x . PSTR = 0x%08x\n",
    rmobile_pd.genpd.name, mask,
    readl(rmobile_pd.base + PSTR));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rmobile_pd_power_up(genpd: *mut generic_pm_domain) -> c_int {
    static int rmobile_pd_power_up(struct generic_pm_domain *genpd)
    {
    return __rmobile_pd_power_up(to_rmobile_pd(genpd));
    }
#[no_mangle]
unsafe extern "C" fn rmobile_init_pm_domain(rmobile_pd: *mut rmobile_pm_domain) {
    static void rmobile_init_pm_domain(struct rmobile_pm_domain *rmobile_pd)
    {
    struct generic_pm_domain *genpd = &rmobile_pd.genpd;
    struct dev_power_governor *gov = rmobile_pd.gov;
    genpd.flags |= GENPD_FLAG_PM_CLK | GENPD_FLAG_ACTIVE_WAKEUP |
    GENPD_FLAG_NO_STAY_ON;
    genpd.attach_dev = cpg_mstp_attach_dev;
    genpd.detach_dev = cpg_mstp_detach_dev;
    if (!(genpd.flags & GENPD_FLAG_ALWAYS_ON)) {
    genpd.power_off = rmobile_pd_power_down;
    genpd.power_on = rmobile_pd_power_up;
    __rmobile_pd_power_up(rmobile_pd);
    }
    pm_genpd_init(genpd, gov ? : &simple_qos_governor, false);
    }
#[no_mangle]
unsafe extern "C" fn rmobile_pd_suspend_console() -> c_int {
    static int rmobile_pd_suspend_console(void)
    {
//
// Serial consoles make use of SCIF hardware located in this domain,
// hence keep the power domain on if "no_console_suspend" is set.
//
    return console_suspend_enabled ? 0 : -EBUSY;
    }
    enum pd_types {
    PD_NORMAL,
    PD_CPU,
    PD_CONSOLE,
    PD_DEBUG,
    PD_MEMCTL,
    };
pub const MAX_NUM_SPECIAL_PDS: c_int = 16;
    static struct special_pd {
    struct device_node *pd;
    enum pd_types type;
    } special_pds[MAX_NUM_SPECIAL_PDS] __initdata;
    static unsigned int num_special_pds __initdata;
    static const struct of_device_id special_ids[] __initconst = {
    { .compatible = "arm,coresight-etm3x", .data = (void *)PD_DEBUG },
    { .compatible = "renesas,dbsc-r8a73a4", .data = (void *)PD_MEMCTL, },
    { .compatible = "renesas,dbsc3-r8a7740", .data = (void *)PD_MEMCTL, },
    { .compatible = "renesas,sbsc-sh73a0", .data = (void *)PD_MEMCTL, },
    { /* sentinel */ },
    };
#[no_mangle]
unsafe extern "C" fn add_special_pd(np: *mut device_node, type: enum pd_types) -> void __init {
    static void __init add_special_pd(struct device_node *np, enum pd_types type)
    {
    unsigned int i;
    struct device_node *pd;
    pd = of_parse_phandle(np, "power-domains", 0);
    if (!pd)
    return;
    for (i = 0; i < num_special_pds; i++)
    if (pd == special_pds[i].pd && type == special_pds[i].type) {
    of_node_put(pd);
    return;
    }
    if (num_special_pds == ARRAY_SIZE(special_pds)) {
    pr_warn("Too many special PM domains\n");
    of_node_put(pd);
    return;
    }
    pr_debug("Special PM domain %pOFn type %d for %pOF\n", pd, type, np);
    special_pds[num_special_pds].pd = pd;
    special_pds[num_special_pds].type = type;
    num_special_pds++;
    }
#[no_mangle]
unsafe extern "C" fn get_special_pds() -> void __init {
    static void __init get_special_pds(void)
    {
    struct device_node *np;
    const struct of_device_id *id;
// PM domains containing CPUs
    for_each_of_cpu_node(np)
    add_special_pd(np, PD_CPU);
// PM domain containing console
    if (of_stdout)
    add_special_pd(of_stdout, PD_CONSOLE);
// PM domains containing other special devices
    for_each_matching_node_and_match(np, special_ids, &id)
    add_special_pd(np, (uintptr_t)id.data);
    }
#[no_mangle]
unsafe extern "C" fn put_special_pds() -> void __init {
    static void __init put_special_pds(void)
    {
    unsigned int i;
    for (i = 0; i < num_special_pds; i++)
    of_node_put(special_pds[i].pd);
    }
#[no_mangle]
unsafe extern "C" fn pd_type(pd: *const device_node) -> enum pd_types __init {
    static enum pd_types __init pd_type(const struct device_node *pd)
    {
    unsigned int i;
    for (i = 0; i < num_special_pds; i++)
    if (pd == special_pds[i].pd)
    return special_pds[i].type;
    return PD_NORMAL;
    }
    static void __init rmobile_setup_pm_domain(struct device_node *np,
    struct rmobile_pm_domain *pd)
    {
    const char *name = pd.genpd.name;
    switch (pd_type(np)) {
    case PD_CPU:
//
// This domain contains the CPU core and therefore it should
// only be turned off if the CPU is not in use.
//
    pr_debug("PM domain %s contains CPU\n", name);
    pd.genpd.flags |= GENPD_FLAG_ALWAYS_ON;
    break;
    case PD_CONSOLE:
    pr_debug("PM domain %s contains serial console\n", name);
    pd.gov = &pm_domain_always_on_gov;
    pd.suspend = rmobile_pd_suspend_console;
    break;
    case PD_DEBUG:
//
// This domain contains the Coresight-ETM hardware block and
// therefore it should only be turned off if the debug module
// is not in use.
//
    pr_debug("PM domain %s contains Coresight-ETM\n", name);
    pd.genpd.flags |= GENPD_FLAG_ALWAYS_ON;
    break;
    case PD_MEMCTL:
//
// This domain contains a memory-controller and therefore it
// should only be turned off if memory is not in use.
//
    pr_debug("PM domain %s contains MEMCTL\n", name);
    pd.genpd.flags |= GENPD_FLAG_ALWAYS_ON;
    break;
    case PD_NORMAL:
    if (pd.bit_shift == ~0) {
// Top-level always-on domain
    pr_debug("PM domain %s is always-on domain\n", name);
    pd.genpd.flags |= GENPD_FLAG_ALWAYS_ON;
    }
    break;
    }
    rmobile_init_pm_domain(pd);
    }
    static int __init rmobile_add_pm_domains(void __iomem *base,
    struct device_node *parent,
    struct generic_pm_domain *genpd_parent)
    {
    for_each_child_of_node_scoped(parent, np) {
    struct rmobile_pm_domain *pd;
    let mut idx: u32 = ~0;
    if (of_property_read_u32(np, "reg", &idx)) {
// always-on domain
    }
    pd = kzalloc_obj(*pd);
    if (!pd)
    return -ENOMEM;
    pd.genpd.name = np.name;
    pd.base = base;
    pd.bit_shift = idx;
    rmobile_setup_pm_domain(np, pd);
    if (genpd_parent)
    pm_genpd_add_subdomain(genpd_parent, &pd.genpd);
    of_genpd_add_provider_simple(np, &pd.genpd);
    rmobile_add_pm_domains(base, np, &pd.genpd);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmobile_init_pm_domains() -> int __init {
    static int __init rmobile_init_pm_domains(void)
    {
    struct device_node *np, *pmd;
    let mut scanned: bool = false;
    void __iomem *base;
    let mut ret: c_int = 0;
    for_each_compatible_node(np, core::ptr::null_mut(), "renesas,sysc-rmobile") {
    base = of_iomap(np, 0);
    if (!base) {
    pr_warn("%pOF cannot map reg 0\n", np);
    continue;
    }
    pmd = of_get_child_by_name(np, "pm-domains");
    if (!pmd) {
    iounmap(base);
    pr_warn("%pOF lacks pm-domains node\n", np);
    continue;
    }
    if (!scanned) {
// Find PM domains containing special blocks
    get_special_pds();
    scanned = true;
    }
    ret = rmobile_add_pm_domains(base, pmd, core::ptr::null_mut());
    of_node_put(pmd);
    if (ret) {
    of_node_put(np);
    break;
    }
    fwnode_dev_initialized(of_fwnode_handle(np), true);
    }
    put_special_pds();
    return ret;
    }
    postcore_initcall(rmobile_init_pm_domains);
