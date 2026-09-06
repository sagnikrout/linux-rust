//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/imx/imx93-pd.c
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
// Copyright 2022 NXP
//

pub const MIX_SLICE_SW_CTRL_OFF: c_uint = 0x20;

pub const MIX_FUNC_STAT_OFF: c_uint = 0xB4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx93_power_domain {
    pub genpd: generic_pm_domain,
    pub dev: *mut device,
    pub addr: *mut void __iomem,
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
}

#[no_mangle]
unsafe extern "C" fn imx93_pd_on(genpd: *mut generic_pm_domain) -> c_int {
    static int imx93_pd_on(struct generic_pm_domain *genpd)
    {
    struct imx93_power_domain *domain = to_imx93_pd(genpd);
    void __iomem *addr = domain.addr;
    u32 val;
    int ret;
    ret = clk_bulk_prepare_enable(domain.num_clks, domain.clks);
    if (ret) {
    dev_err(domain.dev, "failed to enable clocks for domain: %s\n", genpd.name);
    return ret;
    }
    val = readl(addr + MIX_SLICE_SW_CTRL_OFF);
    val &= ~SLICE_SW_CTRL_PDN_SOFT_MASK;
    writel(val, addr + MIX_SLICE_SW_CTRL_OFF);
    ret = readl_poll_timeout(addr + MIX_FUNC_STAT_OFF, val,
    !(val & FUNC_STAT_SSAR_STAT_MASK), 1, 10000);
    if (ret) {
    dev_err(domain.dev, "pd_on timeout: name: %s, stat: %x\n", genpd.name, val);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx93_pd_off(genpd: *mut generic_pm_domain) -> c_int {
    static int imx93_pd_off(struct generic_pm_domain *genpd)
    {
    struct imx93_power_domain *domain = to_imx93_pd(genpd);
    void __iomem *addr = domain.addr;
    int ret;
    u32 val;
// Power off MIX
    val = readl(addr + MIX_SLICE_SW_CTRL_OFF);
    val |= SLICE_SW_CTRL_PDN_SOFT_MASK;
    writel(val, addr + MIX_SLICE_SW_CTRL_OFF);
    ret = readl_poll_timeout(addr + MIX_FUNC_STAT_OFF, val,
    val & FUNC_STAT_PSW_STAT_MASK, 1, 10000);
    if (ret) {
    dev_err(domain.dev, "pd_off timeout: name: %s, stat: %x\n", genpd.name, val);
    return ret;
    }
    clk_bulk_disable_unprepare(domain.num_clks, domain.clks);
    return 0;
    };
#[no_mangle]
unsafe extern "C" fn imx93_pd_remove(pdev: *mut platform_device) {
    static void imx93_pd_remove(struct platform_device *pdev)
    {
    struct imx93_power_domain *domain = platform_get_drvdata(pdev);
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    of_genpd_del_provider(np);
    pm_genpd_remove(&domain.genpd);
    }
#[no_mangle]
unsafe extern "C" fn imx93_pd_probe(pdev: *mut platform_device) -> c_int {
    static int imx93_pd_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct imx93_power_domain *domain;
    bool init_off;
    int ret;
    domain = devm_kzalloc(dev, sizeof(*domain), GFP_KERNEL);
    if (!domain)
    return -ENOMEM;
    domain.addr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(domain.addr))
    return PTR_ERR(domain.addr);
    domain.num_clks = devm_clk_bulk_get_all(dev, &domain.clks);
    if (domain.num_clks < 0)
    return dev_err_probe(dev, domain.num_clks, "Failed to get domain's clocks\n");
    domain.genpd.name = dev_name(dev);
    domain.genpd.power_off = imx93_pd_off;
    domain.genpd.power_on = imx93_pd_on;
    domain.dev = dev;
    init_off = readl(domain.addr + MIX_FUNC_STAT_OFF) & FUNC_STAT_ISO_STAT_MASK;
// Just to sync the status of hardware
    if (!init_off) {
    ret = clk_bulk_prepare_enable(domain.num_clks, domain.clks);
    if (ret)
    return dev_err_probe(domain.dev, ret,
    "failed to enable clocks for domain: %s\n",
    domain.genpd.name);
    }
    ret = pm_genpd_init(&domain.genpd, core::ptr::null_mut(), init_off);
    if (ret)
    goto err_clk_unprepare;
    platform_set_drvdata(pdev, domain);
    ret = of_genpd_add_provider_simple(np, &domain.genpd);
    if (ret)
    goto err_genpd_remove;
    return 0;
    err_genpd_remove:
    pm_genpd_remove(&domain.genpd);
    err_clk_unprepare:
    if (!init_off)
    clk_bulk_disable_unprepare(domain.num_clks, domain.clks);
    return ret;
    }
    static const struct of_device_id imx93_pd_ids[] = {
    { .compatible = "fsl,imx93-src-slice" },
    { }
    };
    MODULE_DEVICE_TABLE(of, imx93_pd_ids);
    static struct platform_driver imx93_power_domain_driver = {
    .driver = {
    .name	= "imx93_power_domain",
    .of_match_table = imx93_pd_ids,
    },
    .probe = imx93_pd_probe,
    .remove = imx93_pd_remove,
    };
    module_platform_driver(imx93_power_domain_driver);
    MODULE_AUTHOR("Peng Fan <peng.fan@nxp.com>");
    MODULE_DESCRIPTION("NXP i.MX93 power domain driver");
    MODULE_LICENSE("GPL");
