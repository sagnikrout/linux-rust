//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/sunxi/sun50i-h6-prcm-ppu.c
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
// Copyright (C) Arm Ltd. 2024
//
// Allwinner H6/H616 PRCM power domain driver.
// This covers a few registers inside the PRCM (Power Reset Clock Management)
// block that control some power rails, most prominently for the Mali GPU.
//

//
// The PRCM block covers multiple devices, starting with some clocks,
// then followed by the power rails.
// The clocks are covered by a different driver, so this driver's MMIO range
// starts later in the PRCM MMIO frame, not at the beginning of it.
// To keep the register offsets consistent with other PRCM documentation,
// express the registers relative to the beginning of the whole PRCM, and
// subtract the PPU offset this driver is bound to.
//
pub const PD_H6_PPU_OFFSET: c_uint = 0x250;
pub const PD_H6_VDD_SYS_REG: c_uint = 0x250;

pub const PD_H6_GPU_REG: c_uint = 0x254;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun50i_h6_ppu_pd {
    pub genpd: generic_pm_domain,
    pub reg: *mut void __iomem,
    pub gate_mask: u32,
    pub negated: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun50i_h6_ppu_desc {
    pub name: *const c_char,
    pub offset: u32,
    pub mask: u32,
    pub flags: c_uint,
}

    static const struct sun50i_h6_ppu_desc sun50i_h6_ppus[] = {
    { "AVCC", PD_H6_VDD_SYS_REG, PD_H6_AVCC_VDD_GATE },
    { "CPUS", PD_H6_VDD_SYS_REG, PD_H6_CPUS_VDD_GATE },
    { "GPU", PD_H6_GPU_REG, PD_H6_GPU_GATE },
    };
    static const struct sun50i_h6_ppu_desc sun50i_h616_ppus[] = {
    { "PLL", PD_H6_VDD_SYS_REG, PD_H6_AVCC_VDD_GATE,
    FLAG_PPU_ALWAYS_ON | FLAG_PPU_NEGATED },
    { "ANA", PD_H6_VDD_SYS_REG, PD_H616_ANA_VDD_GATE, FLAG_PPU_ALWAYS_ON },
    { "GPU", PD_H6_GPU_REG, PD_H6_GPU_GATE, FLAG_PPU_NEGATED },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun50i_h6_ppu_data {
    pub descs: *const sun50i_h6_ppu_desc,
    pub nr_domains: c_int,
}

    static const struct sun50i_h6_ppu_data sun50i_h6_ppu_data = {
    .descs = sun50i_h6_ppus,
    .nr_domains = ARRAY_SIZE(sun50i_h6_ppus),
    };
    static const struct sun50i_h6_ppu_data sun50i_h616_ppu_data = {
    .descs = sun50i_h616_ppus,
    .nr_domains = ARRAY_SIZE(sun50i_h616_ppus),
    };

    container_of(_genpd, struct sun50i_h6_ppu_pd, genpd)
#[no_mangle]
unsafe extern "C" fn sun50i_h6_ppu_power_status(pd: *const sun50i_h6_ppu_pd) -> bool {
    static bool sun50i_h6_ppu_power_status(const struct sun50i_h6_ppu_pd *pd)
    {
    let mut bit: bool = readl(pd.reg) & pd.gate_mask;
    return bit ^ pd.negated;
    }
    static int sun50i_h6_ppu_pd_set_power(const struct sun50i_h6_ppu_pd *pd,
    bool set_bit)
    {
    let mut reg: u32 = readl(pd.reg);
    if (set_bit)
    writel(reg | pd.gate_mask, pd.reg);
    else
    writel(reg & ~pd.gate_mask, pd.reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun50i_h6_ppu_pd_power_on(genpd: *mut generic_pm_domain) -> c_int {
    static int sun50i_h6_ppu_pd_power_on(struct generic_pm_domain *genpd)
    {
    const struct sun50i_h6_ppu_pd *pd = to_sun50i_h6_ppu_pd(genpd);
    return sun50i_h6_ppu_pd_set_power(pd, !pd.negated);
    }
#[no_mangle]
unsafe extern "C" fn sun50i_h6_ppu_pd_power_off(genpd: *mut generic_pm_domain) -> c_int {
    static int sun50i_h6_ppu_pd_power_off(struct generic_pm_domain *genpd)
    {
    const struct sun50i_h6_ppu_pd *pd = to_sun50i_h6_ppu_pd(genpd);
    return sun50i_h6_ppu_pd_set_power(pd, pd.negated);
    }
#[no_mangle]
unsafe extern "C" fn sun50i_h6_ppu_probe(pdev: *mut platform_device) -> c_int {
    static int sun50i_h6_ppu_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct genpd_onecell_data *ppu;
    struct sun50i_h6_ppu_pd *pds;
    const struct sun50i_h6_ppu_data *data;
    void __iomem *base;
    int ret, i;
    data = of_device_get_match_data(dev);
    if (!data)
    return -EINVAL;
    pds = devm_kcalloc(dev, data.nr_domains, sizeof(*pds), GFP_KERNEL);
    if (!pds)
    return -ENOMEM;
    ppu = devm_kzalloc(dev, sizeof(*ppu), GFP_KERNEL);
    if (!ppu)
    return -ENOMEM;
    ppu.num_domains = data.nr_domains;
    ppu.domains = devm_kcalloc(dev, data.nr_domains,
    sizeof(*ppu.domains), GFP_KERNEL);
    if (!ppu.domains)
    return -ENOMEM;
    platform_set_drvdata(pdev, ppu);
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    for (i = 0; i < data.nr_domains; i++) {
    struct sun50i_h6_ppu_pd *pd = &pds[i];
    const struct sun50i_h6_ppu_desc *desc = &data.descs[i];
    pd.genpd.name		= desc.name;
    pd.genpd.power_off	= sun50i_h6_ppu_pd_power_off;
    pd.genpd.power_on	= sun50i_h6_ppu_pd_power_on;
    if (desc.flags & FLAG_PPU_ALWAYS_ON)
    pd.genpd.flags = GENPD_FLAG_ALWAYS_ON;
    pd.negated		= !!(desc.flags & FLAG_PPU_NEGATED);
    pd.reg			= base + desc.offset - PD_H6_PPU_OFFSET;
    pd.gate_mask		= desc.mask;
    ret = pm_genpd_init(&pd.genpd, core::ptr::null_mut(),
    !sun50i_h6_ppu_power_status(pd));
    if (ret) {
    dev_warn(dev, "Failed to add %s power domain: %d\n",
    desc.name, ret);
    goto out_remove_pds;
    }
    ppu.domains[i] = &pd.genpd;
    }
    ret = of_genpd_add_provider_onecell(dev.of_node, ppu);
    if (!ret)
    return 0;
    dev_warn(dev, "Failed to add provider: %d\n", ret);
    out_remove_pds:
    for (i--; i >= 0; i--)
    pm_genpd_remove(&pds[i].genpd);
    return ret;
    }
    static const struct of_device_id sun50i_h6_ppu_of_match[] = {
    { .compatible	= "allwinner,sun50i-h6-prcm-ppu",
    .data		= &sun50i_h6_ppu_data },
    { .compatible	= "allwinner,sun50i-h616-prcm-ppu",
    .data		= &sun50i_h616_ppu_data },
    { }
    };
    MODULE_DEVICE_TABLE(of, sun50i_h6_ppu_of_match);
    static struct platform_driver sun50i_h6_ppu_driver = {
    .probe	= sun50i_h6_ppu_probe,
    .driver	= {
    .name			= "sun50i-h6-prcm-ppu",
    .of_match_table		= sun50i_h6_ppu_of_match,
// Power domains cannot be removed while they are in use.
    .suppress_bind_attrs	= true,
    },
    };
    module_platform_driver(sun50i_h6_ppu_driver);
    MODULE_AUTHOR("Andre Przywara <andre.przywara@arm.com>");
    MODULE_DESCRIPTION("Allwinner H6 PRCM power domain driver");
    MODULE_LICENSE("GPL");
