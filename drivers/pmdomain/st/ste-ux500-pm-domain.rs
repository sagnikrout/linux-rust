//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/st/ste-ux500-pm-domain.c
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
// Copyright (C) 2014 Linaro Ltd.
//
// Author: Ulf Hansson <ulf.hansson@linaro.org>
//
// Implements PM domains using the generic PM domain for ux500.
//

#[no_mangle]
unsafe extern "C" fn pd_power_off(domain: *mut generic_pm_domain) -> c_int {
    static int pd_power_off(struct generic_pm_domain *domain)
    {
//
// Handle the gating of the PM domain regulator here.
//
// Drivers/subsystems handling devices in the PM domain needs to perform
// register context save/restore from their respective runtime PM
// callbacks, to be able to enable PM domain gating/ungating.
//
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pd_power_on(domain: *mut generic_pm_domain) -> c_int {
    static int pd_power_on(struct generic_pm_domain *domain)
    {
//
// Handle the ungating of the PM domain regulator here.
//
// Drivers/subsystems handling devices in the PM domain needs to perform
// register context save/restore from their respective runtime PM
// callbacks, to be able to enable PM domain gating/ungating.
//
    return 0;
    }
    static struct generic_pm_domain ux500_pm_domain_vape = {
    .name = "VAPE",
    .power_off = pd_power_off,
    .power_on = pd_power_on,
    };
    static struct generic_pm_domain *ux500_pm_domains[NR_DOMAINS] = {
    [DOMAIN_VAPE] = &ux500_pm_domain_vape,
    };
    static const struct of_device_id ux500_pm_domain_matches[] = {
    { .compatible = "stericsson,ux500-pm-domains", },
    { },
    };
#[no_mangle]
unsafe extern "C" fn ux500_pm_domains_probe(pdev: *mut platform_device) -> c_int {
    static int ux500_pm_domains_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct genpd_onecell_data *genpd_data;
    int i;
    if (!np)
    return -ENODEV;
    genpd_data = kzalloc_obj(*genpd_data);
    if (!genpd_data)
    return -ENOMEM;
    genpd_data.domains = ux500_pm_domains;
    genpd_data.num_domains = ARRAY_SIZE(ux500_pm_domains);
    for (i = 0; i < ARRAY_SIZE(ux500_pm_domains); ++i)
    pm_genpd_init(ux500_pm_domains[i], core::ptr::null_mut(), false);
    of_genpd_add_provider_onecell(np, genpd_data);
    return 0;
    }
    static struct platform_driver ux500_pm_domains_driver = {
    .probe  = ux500_pm_domains_probe,
    .driver = {
    .name = "ux500_pm_domains",
    .of_match_table = ux500_pm_domain_matches,
    },
    };
#[no_mangle]
unsafe extern "C" fn ux500_pm_domains_init() -> int __init {
    static int __init ux500_pm_domains_init(void)
    {
    return platform_driver_register(&ux500_pm_domains_driver);
    }
    arch_initcall(ux500_pm_domains_init);
