//! Automatically rewritten from C to Rust
//! Source: drivers/cpuidle/cpuidle-psci-domain.c
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
// PM domains for CPUs via genpd - managed by cpuidle-psci.
//
// Copyright (C) 2019 Linaro Ltd.
// Author: Ulf Hansson <ulf.hansson@linaro.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psci_pd_provider {
    pub link: list_head,
    pub node: *mut device_node,
}

    static LIST_HEAD(psci_pd_providers);
#[no_mangle]
unsafe extern "C" fn psci_pd_power_off(pd: *mut generic_pm_domain) -> c_int {
    static int psci_pd_power_off(struct generic_pm_domain *pd)
    {
    struct genpd_power_state *state = &pd.states[pd.state_idx];
    u32 *pd_state;
    if (!state.data)
    return 0;
// OSI mode is enabled, set the corresponding domain state.
    pd_state = state.data;
    psci_set_domain_state(pd, pd.state_idx, *pd_state);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn psci_pd_init(np: *mut device_node, use_osi: bool) -> c_int {
    static int psci_pd_init(struct device_node *np, bool use_osi)
    {
    struct generic_pm_domain *pd;
    struct psci_pd_provider *pd_provider;
    struct dev_power_governor *pd_gov;
    let mut ret: c_int = -ENOMEM;
    pd = dt_idle_pd_alloc(np, psci_dt_parse_state_node);
    if (!pd)
    goto out;
    pd_provider = kzalloc_obj(*pd_provider);
    if (!pd_provider)
    goto free_pd;
    pd.flags |= GENPD_FLAG_IRQ_SAFE | GENPD_FLAG_CPU_DOMAIN;
//
// Allow power off when OSI has been successfully enabled.
// On a PREEMPT_RT based configuration the domain idle states are
// supported, but only during system-wide suspend.
//
    if (use_osi) {
    pd.power_off = psci_pd_power_off;
    pd.flags |= GENPD_FLAG_ACTIVE_WAKEUP;
    if (IS_ENABLED(CONFIG_PREEMPT_RT))
    pd.flags |= GENPD_FLAG_RPM_ALWAYS_ON;
    } else {
    pd.flags |= GENPD_FLAG_ALWAYS_ON;
    }
// Use governor for CPU PM domains if it has some states to manage.
    pd_gov = pd.states ? &pm_domain_cpu_gov : core::ptr::null_mut();
    ret = pm_genpd_init(pd, pd_gov, false);
    if (ret)
    goto free_pd_prov;
    ret = of_genpd_add_provider_simple(np, pd);
    if (ret)
    goto remove_pd;
    pd_provider.node = of_node_get(np);
    list_add(&pd_provider.link, &psci_pd_providers);
    pr_debug("init PM domain %s\n", pd.name);
    return 0;
    remove_pd:
    pm_genpd_remove(pd);
    free_pd_prov:
    kfree(pd_provider);
    free_pd:
    dt_idle_pd_free(pd);
    out:
    pr_err("failed to init PM domain ret=%d %pOF\n", ret, np);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn psci_pd_remove() {
    static void psci_pd_remove(void)
    {
    struct psci_pd_provider *pd_provider, *it;
    struct generic_pm_domain *genpd;
    list_for_each_entry_safe_reverse(pd_provider, it,
    &psci_pd_providers, link) {
    of_genpd_del_provider(pd_provider.node);
    genpd = of_genpd_remove_last(pd_provider.node);
    if (!IS_ERR(genpd))
    kfree(genpd);
    of_node_put(pd_provider.node);
    list_del(&pd_provider.link);
    kfree(pd_provider);
    }
    }
    static const struct of_device_id psci_of_match[] = {
    { .compatible = "arm,psci-1.0" },
    {}
    };
#[no_mangle]
unsafe extern "C" fn psci_cpuidle_domain_probe(pdev: *mut platform_device) -> c_int {
    static int psci_cpuidle_domain_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    let mut use_osi: bool = psci_has_osi_support();
    let mut ret: c_int = 0, pd_count = 0;
    if (!np)
    return -ENODEV;
//
// Parse child nodes for the "#power-domain-cells" property and
// initialize a genpd/genpd-of-provider pair when it's found.
//
    for_each_child_of_node_scoped(np, node) {
    if (!of_property_present(node, "#power-domain-cells"))
    continue;
    ret = psci_pd_init(node, use_osi);
    if (ret)
    goto exit;
    pd_count++;
    }
// Bail out if not using the hierarchical CPU topology.
    if (!pd_count)
    return 0;
// Link genpd masters/subdomains to model the CPU topology.
    ret = dt_idle_pd_init_topology(np);
    if (ret)
    goto remove_pd;
// let's try to enable OSI.
    ret = psci_set_osi_mode(use_osi);
    if (ret)
    goto remove_pd;
    pr_info("Initialized CPU PM domain topology using %s mode\n",
    use_osi ? "OSI" : "PC");
    return 0;
    remove_pd:
    dt_idle_pd_remove_topology(np);
    psci_pd_remove();
    exit:
    pr_err("failed to create CPU PM domains ret=%d\n", ret);
    return ret;
    }
    static struct platform_driver psci_cpuidle_domain_driver = {
    .probe  = psci_cpuidle_domain_probe,
    .driver = {
    .name = "psci-cpuidle-domain",
    .of_match_table = psci_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn psci_idle_init_domains() -> int __init {
    static int __init psci_idle_init_domains(void)
    {
    return platform_driver_register(&psci_cpuidle_domain_driver);
    }
    core_initcall(psci_idle_init_domains);
