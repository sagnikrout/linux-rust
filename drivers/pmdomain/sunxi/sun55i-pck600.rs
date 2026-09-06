//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/sunxi/sun55i-pck600.c
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
// Allwinner PCK-600 power domain support
//
// Copyright (c) 2025 Chen-Yu Tsai <wens@csie.org>
//
// The hardware is likely based on the Arm PCK-600 IP, since some of
// the registers match Arm's documents, with additional delay controls
// that are in registers listed as reserved.
//
// Documents include:
// - "Arm CoreLink PCK-600 Power Control Kit" TRM
// - "Arm Power Policy Unit" architecture specification (DEN0051E)
//

pub const PPU_PWPR: c_uint = 0x0;
pub const PPU_PWSR: c_uint = 0x8;
pub const PPU_DCDR0: c_uint = 0x170;
pub const PPU_DCDR1: c_uint = 0x174;
// shared definition for PPU_PWPR and PPU_PWSR

pub const PPU_POWER_MODE_ON: c_uint = 0x8;
pub const PPU_POWER_MODE_OFF: c_uint = 0x0;
pub const PPU_REG_SIZE: c_uint = 0x1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_pck600_pd_desc {
    pub name: *const c_char,
    pub flags: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_pck600_desc {
    pub pd_descs: *const sunxi_pck600_pd_desc,
    pub num_domains: c_uint,
    pub logic_power_switch0_delay_offset: u32,
    pub logic_power_switch1_delay_offset: u32,
    pub off2on_delay_offset: u32,
    pub device_ctrl0_delay: u32,
    pub device_ctrl1_delay: u32,
    pub logic_power_switch0_delay: u32,
    pub logic_power_switch1_delay: u32,
    pub off2on_delay: u32,
    pub has_rst_clk: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_pck600_pd {
    pub genpd: generic_pm_domain,
    pub pck: *mut sunxi_pck600,
    pub base: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_pck600 {
    pub dev: *mut device,
    pub genpd_data: genpd_onecell_data,
    pub pds: [sunxi_pck600_pd; ],
}

#[no_mangle]
unsafe extern "C" fn sunxi_pck600_pd_set_power(pd: *mut sunxi_pck600_pd, on: bool) -> c_int {
    static int sunxi_pck600_pd_set_power(struct sunxi_pck600_pd *pd, bool on)
    {
    struct sunxi_pck600 *pck = pd.pck;
    struct generic_pm_domain *genpd = &pd.genpd;
    int ret;
    u32 val, reg;
    val = on ? PPU_POWER_MODE_ON : PPU_POWER_MODE_OFF;
    reg = readl(pd.base + PPU_PWPR);
    FIELD_MODIFY(PPU_PWR_STATUS, &reg, val);
    writel(reg, pd.base + PPU_PWPR);
// push write out to hardware
    reg = readl(pd.base + PPU_PWPR);
    ret = readl_poll_timeout_atomic(pd.base + PPU_PWSR, reg,
    FIELD_GET(PPU_PWR_STATUS, reg) == val,
    0, 10000);
    if (ret)
    dev_err(pck.dev, "failed to turn domain \"%s\" %s: %d\n",
    genpd.name, str_on_off(on), ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_pck600_power_on(domain: *mut generic_pm_domain) -> c_int {
    static int sunxi_pck600_power_on(struct generic_pm_domain *domain)
    {
    struct sunxi_pck600_pd *pd = to_sunxi_pd(domain);
    return sunxi_pck600_pd_set_power(pd, true);
    }
#[no_mangle]
unsafe extern "C" fn sunxi_pck600_power_off(domain: *mut generic_pm_domain) -> c_int {
    static int sunxi_pck600_power_off(struct generic_pm_domain *domain)
    {
    struct sunxi_pck600_pd *pd = to_sunxi_pd(domain);
    return sunxi_pck600_pd_set_power(pd, false);
    }
    static void sunxi_pck600_pd_setup(struct sunxi_pck600_pd *pd,
    const struct sunxi_pck600_desc *desc)
    {
    writel(desc.device_ctrl0_delay, pd.base + PPU_DCDR0);
    writel(desc.device_ctrl1_delay, pd.base + PPU_DCDR1);
    writel(desc.logic_power_switch0_delay,
    pd.base + desc.logic_power_switch0_delay_offset);
    writel(desc.logic_power_switch1_delay,
    pd.base + desc.logic_power_switch1_delay_offset);
    writel(desc.off2on_delay, pd.base + desc.off2on_delay_offset);
    }
#[no_mangle]
unsafe extern "C" fn sunxi_pck600_probe(pdev: *mut platform_device) -> c_int {
    static int sunxi_pck600_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct sunxi_pck600_desc *desc;
    struct genpd_onecell_data *genpds;
    struct sunxi_pck600 *pck;
    struct reset_control *rst;
    struct clk *clk;
    void __iomem *base;
    int i, ret;
    desc = of_device_get_match_data(dev);
    pck = devm_kzalloc(dev, struct_size(pck, pds, desc.num_domains), GFP_KERNEL);
    if (!pck)
    return -ENOMEM;
    pck.dev = &pdev.dev;
    platform_set_drvdata(pdev, pck);
    genpds = &pck.genpd_data;
    genpds.num_domains = desc.num_domains;
    genpds.domains = devm_kcalloc(dev, desc.num_domains,
    sizeof(*genpds.domains), GFP_KERNEL);
    if (!genpds.domains)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    if (desc.has_rst_clk) {
    rst = devm_reset_control_get_exclusive_released(dev, core::ptr::null_mut());
    if (IS_ERR(rst))
    return dev_err_probe(dev, PTR_ERR(rst), "failed to get reset control\n");
    }
    clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk), "failed to get clock\n");
    for (i = 0; i < desc.num_domains; i++) {
    struct sunxi_pck600_pd *pd = &pck.pds[i];
    const struct sunxi_pck600_pd_desc *pd_desc = &desc.pd_descs[i];
    pd.genpd.name = pd_desc.name;
    pd.genpd.power_off = sunxi_pck600_power_off;
    pd.genpd.power_on = sunxi_pck600_power_on;
    pd.genpd.flags = pd_desc.flags;
    pd.base = base + PPU_REG_SIZE * i;
    sunxi_pck600_pd_setup(pd, desc);
    ret = pm_genpd_init(&pd.genpd, core::ptr::null_mut(), false);
    if (ret) {
    dev_err_probe(dev, ret, "failed to initialize power domain\n");
    goto err_remove_pds;
    }
    genpds.domains[i] = &pd.genpd;
    }
    ret = of_genpd_add_provider_onecell(dev_of_node(dev), genpds);
    if (ret) {
    dev_err_probe(dev, ret, "failed to add PD provider\n");
    goto err_remove_pds;
    }
    return 0;
    err_remove_pds:
    for (i--; i >= 0; i--)
    pm_genpd_remove(genpds.domains[i]);
    return ret;
    }
    static const struct sunxi_pck600_pd_desc sun55i_a523_pck600_pds[] = {
    { "VE", }, { "GPU", }, { "VI", }, { "VO0", }, { "VO1", },
    { "DE", }, { "NAND", }, { "PCIE", },
    };
    static const struct sunxi_pck600_desc sun55i_a523_pck600_desc = {
    .pd_descs = sun55i_a523_pck600_pds,
    .num_domains = ARRAY_SIZE(sun55i_a523_pck600_pds),
    .logic_power_switch0_delay_offset = 0xc00,
    .logic_power_switch1_delay_offset = 0xc04,
    .off2on_delay_offset = 0xc10,
    .device_ctrl0_delay = 0xffffff,
    .device_ctrl1_delay = 0xffff,
    .logic_power_switch0_delay = 0x8080808,
    .logic_power_switch1_delay = 0x808,
    .off2on_delay = 0x8,
    .has_rst_clk = true,
    };
    static const struct sunxi_pck600_pd_desc sun60i_a733_pck600_pds[] = {
    { "VI", }, { "DE_SYS", }, { "VE_DEC", }, { "VE_ENC", }, { "NPU", },
    { "GPU_TOP", }, { "GPU_CORE", GENPD_FLAG_ALWAYS_ON },
    { "PCIE", }, { "USB2", }, { "VO", }, { "VO1", },
    };
    static const struct sunxi_pck600_desc sun60i_a733_pck600_desc = {
    .pd_descs = sun60i_a733_pck600_pds,
    .num_domains = ARRAY_SIZE(sun60i_a733_pck600_pds),
    .logic_power_switch0_delay_offset = 0xc00,
    .logic_power_switch1_delay_offset = 0xc04,
    .off2on_delay_offset = 0xc10,
    .device_ctrl0_delay = 0x1f1f1f,
    .device_ctrl1_delay = 0x1f1f,
    .logic_power_switch0_delay = 0x8080808,
    .logic_power_switch1_delay = 0x808,
    .off2on_delay = 0x8,
    .has_rst_clk = false,
    };
    static const struct of_device_id sunxi_pck600_of_match[] = {
    {
    .compatible	= "allwinner,sun55i-a523-pck-600",
    .data		= &sun55i_a523_pck600_desc,
    },
    {
    .compatible	= "allwinner,sun60i-a733-pck-600",
    .data		= &sun60i_a733_pck600_desc,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, sunxi_pck600_of_match);
    static struct platform_driver sunxi_pck600_driver = {
    .probe = sunxi_pck600_probe,
    .driver = {
    .name   = "sunxi-pck-600",
    .of_match_table = sunxi_pck600_of_match,
// Power domains cannot be removed if in use.
    .suppress_bind_attrs = true,
    },
    };
    module_platform_driver(sunxi_pck600_driver);
    MODULE_DESCRIPTION("Allwinner PCK-600 power domain driver");
    MODULE_AUTHOR("Chen-Yu Tsai <wens@csie.org>");
    MODULE_LICENSE("GPL");
