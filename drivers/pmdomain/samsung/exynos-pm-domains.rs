//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/samsung/exynos-pm-domains.c
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
// Exynos Generic power domain support.
//
// Copyright (c) 2012 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Implementation of Exynos specific power domain control which is used in
// conjunction with runtime-pm. Support for both device-tree and non-device-tree
// based power domain support is included.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_pm_domain_config {
// Value for LOCAL_PWR_CFG and STATUS fields for each domain
    pub local_pwr_cfg: u32,
}

//
// Exynos specific wrapper around the generic power domain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_pm_domain {
    pub base: *mut void __iomem,
    pub pd: generic_pm_domain,
    pub local_pwr_cfg: u32,
}

#[no_mangle]
unsafe extern "C" fn exynos_pd_power(domain: *mut generic_pm_domain, power_on: bool) -> c_int {
    static int exynos_pd_power(struct generic_pm_domain *domain, bool power_on)
    {
    struct exynos_pm_domain *pd;
    void __iomem *base;
    u32 timeout, pwr;
    char *op;
    pd = container_of(domain, struct exynos_pm_domain, pd);
    base = pd.base;
    pwr = power_on ? pd.local_pwr_cfg : 0;
    writel_relaxed(pwr, base);
// Wait max 1ms
    timeout = 10;
    while ((readl_relaxed(base + 0x4) & pd.local_pwr_cfg) != pwr) {
    if (!timeout) {
    op = (power_on) ? "enable" : "disable";
    pr_err("Power domain %s %s failed\n", domain.name, op);
    return -ETIMEDOUT;
    }
    timeout--;
    cpu_relax();
    usleep_range(80, 100);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos_pd_power_on(domain: *mut generic_pm_domain) -> c_int {
    static int exynos_pd_power_on(struct generic_pm_domain *domain)
    {
    return exynos_pd_power(domain, true);
    }
#[no_mangle]
unsafe extern "C" fn exynos_pd_power_off(domain: *mut generic_pm_domain) -> c_int {
    static int exynos_pd_power_off(struct generic_pm_domain *domain)
    {
    return exynos_pd_power(domain, false);
    }
    static const struct exynos_pm_domain_config exynos4210_cfg = {
    .local_pwr_cfg		= 0x7,
    };
    static const struct exynos_pm_domain_config exynos5433_cfg = {
    .local_pwr_cfg		= 0xf,
    };
    static const struct of_device_id exynos_pm_domain_of_match[] = {
    {
    .compatible = "samsung,exynos4210-pd",
    .data = &exynos4210_cfg,
    }, {
    .compatible = "samsung,exynos5433-pd",
    .data = &exynos5433_cfg,
    },
    { },
    };
    static const char *exynos_get_domain_name(struct device *dev,
    struct device_node *node)
    {
    const char *name;
    if (of_property_read_string(node, "label", &name) < 0)
    name = kbasename(node.full_name);
    return devm_kstrdup_const(dev, name, GFP_KERNEL);
    }
#[no_mangle]
unsafe extern "C" fn exynos_pd_probe(pdev: *mut platform_device) -> c_int {
    static int exynos_pd_probe(struct platform_device *pdev)
    {
    const struct exynos_pm_domain_config *pm_domain_cfg;
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct of_phandle_args child, parent;
    struct exynos_pm_domain *pd;
    int on, ret;
    pm_domain_cfg = of_device_get_match_data(dev);
    pd = devm_kzalloc(dev, sizeof(*pd), GFP_KERNEL);
    if (!pd)
    return -ENOMEM;
    pd.pd.name = exynos_get_domain_name(dev, np);
    if (!pd.pd.name)
    return -ENOMEM;
    pd.base = of_iomap(np, 0);
    if (!pd.base)
    return -ENODEV;
    pd.pd.power_off = exynos_pd_power_off;
    pd.pd.power_on = exynos_pd_power_on;
    pd.local_pwr_cfg = pm_domain_cfg.local_pwr_cfg;
//
// Some Samsung platforms with bootloaders turning on the splash-screen
// and handing it over to the kernel, requires the power-domains to be
// reset during boot.
//
    if (IS_ENABLED(CONFIG_ARM) &&
    of_device_is_compatible(np, "samsung,exynos4210-pd"))
    exynos_pd_power_off(&pd.pd);
    on = readl_relaxed(pd.base + 0x4) & pd.local_pwr_cfg;
    pm_genpd_init(&pd.pd, core::ptr::null_mut(), !on);
    ret = of_genpd_add_provider_simple(np, &pd.pd);
    if (ret == 0 && of_parse_phandle_with_args(np, "power-domains",
    "#power-domain-cells", 0, &parent) == 0) {
    child.np = np;
    child.args_count = 0;
    if (of_genpd_add_subdomain(&parent, &child))
    pr_warn("%pOF failed to add subdomain: %pOF\n",
    parent.np, child.np);
    else
    pr_info("%pOF has as child subdomain: %pOF.\n",
    parent.np, child.np);
    }
    pm_runtime_enable(dev);
    return ret;
    }
    static struct platform_driver exynos_pd_driver = {
    .probe	= exynos_pd_probe,
    .driver	= {
    .name		= "exynos-pd",
    .of_match_table	= exynos_pm_domain_of_match,
    .suppress_bind_attrs = true,
    }
    };
#[no_mangle]
unsafe extern "C" fn exynos4_pm_init_power_domain() -> __init int {
    static __init int exynos4_pm_init_power_domain(void)
    {
    return platform_driver_register(&exynos_pd_driver);
    }
    core_initcall(exynos4_pm_init_power_domain);
