//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8173-mfgtop.c
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
// Copyright (c) 2024 Google LLC
// Author: Chen-Yu Tsai <wenst@chromium.org>
//
// Based on driver in downstream ChromeOS v5.15 kernel.
//
// Copyright (c) 2014 MediaTek Inc.
// Author: Chiawen Lee <chiawen.lee@mediatek.com>
//

    static const struct mtk_gate_regs mfg_cg_regs = {
    .sta_ofs = 0x0000,
    .clr_ofs = 0x0008,
    .set_ofs = 0x0004,
    };

    GATE_MTK_FLAGS(_id, _name, _parent, &mfg_cg_regs, _shift, &mtk_clk_gate_ops_setclr, _flags)
// TODO: The block actually has dividers for the core and mem clocks.
    static const struct mtk_gate mfg_clks[] = {
    GATE_MFG(CLK_MFG_AXI, "mfg_axi", "axi_mfg_in_sel", 0, CLK_SET_RATE_PARENT),
    GATE_MFG(CLK_MFG_MEM, "mfg_mem", "mem_mfg_in_sel", 1, CLK_SET_RATE_PARENT),
    GATE_MFG(CLK_MFG_G3D, "mfg_g3d", "mfg_sel", 2, CLK_SET_RATE_PARENT),
    GATE_MFG(CLK_MFG_26M, "mfg_26m", "clk26m", 3, 0),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt8173_mfgtop_data {
    pub clk_data: *mut clk_hw_onecell_data,
    pub regmap: *mut regmap,
    pub genpd: generic_pm_domain,
    pub child_pd: of_phandle_args parent_pd,,
    pub clk_26m: *mut clk,
}

// Delay count in clock cycles
pub const MFG_ACTIVE_POWER_CON0: c_uint = 0x24;

pub const MFG_ACTIVE_POWER_CON1: c_uint = 0x28;

#[no_mangle]
unsafe extern "C" fn clk_mt8173_mfgtop_power_on(domain: *mut generic_pm_domain) -> c_int {
    static int clk_mt8173_mfgtop_power_on(struct generic_pm_domain *domain)
    {
    struct mt8173_mfgtop_data *data = container_of(domain, struct mt8173_mfgtop_data, genpd);
    int ret;
// drives internal power management
    ret = clk_prepare_enable(data.clk_26m);
    if (ret)
    return ret;
// Power on/off delays for various signals
    regmap_write(data.regmap, MFG_ACTIVE_POWER_CON0,
    FIELD_PREP(RST_B_DELAY_CNT, 77) |
    FIELD_PREP(CLK_EN_DELAY_CNT, 61) |
    FIELD_PREP(CLK_DIS_DELAY_CNT, 60) |
    FIELD_PREP(ACTIVE_PWRCTL_EN, 0));
    regmap_write(data.regmap, MFG_ACTIVE_POWER_CON1,
    FIELD_PREP(PWR_ON_S_DELAY_CNT, 11) |
    FIELD_PREP(ISO_DELAY_CNT, 68) |
    FIELD_PREP(ISOOFF_DELAY_CNT, 69) |
    FIELD_PREP(RST_DELAY_CNT, 77));
// Magic numbers related to core switch sequence and delays
    regmap_write(data.regmap, 0xe0, 0x7a710184);
    regmap_write(data.regmap, 0xe4, 0x835f6856);
    regmap_write(data.regmap, 0xe8, 0x002b0234);
    regmap_write(data.regmap, 0xec, 0x80000000);
    regmap_write(data.regmap, 0xa0, 0x08000000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_mt8173_mfgtop_power_off(domain: *mut generic_pm_domain) -> c_int {
    static int clk_mt8173_mfgtop_power_off(struct generic_pm_domain *domain)
    {
    struct mt8173_mfgtop_data *data = container_of(domain, struct mt8173_mfgtop_data, genpd);
// Magic numbers related to core switch sequence and delays
    regmap_write(data.regmap, 0xec, 0);
// drives internal power management
    clk_disable_unprepare(data.clk_26m);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_mt8173_mfgtop_probe(pdev: *mut platform_device) -> c_int {
    static int clk_mt8173_mfgtop_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    struct mt8173_mfgtop_data *data;
    int ret;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    platform_set_drvdata(pdev, data);
    data.clk_data = mtk_devm_alloc_clk_data(dev, ARRAY_SIZE(mfg_clks));
    if (!data.clk_data)
    return -ENOMEM;
// MTK clock gates also uses regmap
    data.regmap = device_node_to_regmap(node);
    if (IS_ERR(data.regmap))
    return dev_err_probe(dev, PTR_ERR(data.regmap), "Failed to get regmap\n");
    data.child_pd.np = node;
    data.child_pd.args_count = 0;
    ret = of_parse_phandle_with_args(node, "power-domains", "#power-domain-cells", 0,
    &data.parent_pd);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to parse power domain\n");
    devm_pm_runtime_enable(dev);
//
// Do a pm_runtime_resume_and_get() to workaround a possible
// deadlock between clk_register() and the genpd framework.
//
    ret = pm_runtime_resume_and_get(dev);
    if (ret) {
    dev_err_probe(dev, ret, "Failed to runtime resume device\n");
    goto put_of_node;
    }
    ret = mtk_clk_register_gates(dev, node, mfg_clks, ARRAY_SIZE(mfg_clks),
    data.clk_data);
    if (ret) {
    dev_err_probe(dev, ret, "Failed to register clock gates\n");
    goto put_pm_runtime;
    }
    data.clk_26m = clk_hw_get_clk(data.clk_data.hws[CLK_MFG_26M], "26m");
    if (IS_ERR(data.clk_26m)) {
    ret = dev_err_probe(dev, PTR_ERR(data.clk_26m), "Failed to get 26 MHz clock\n");
    goto unregister_clks;
    }
    ret = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, data.clk_data);
    if (ret) {
    dev_err_probe(dev, ret, "Failed to add clk OF provider\n");
    goto put_26m_clk;
    }
    data.genpd.name = "mfg-top";
    data.genpd.power_on = clk_mt8173_mfgtop_power_on;
    data.genpd.power_off = clk_mt8173_mfgtop_power_off;
    ret = pm_genpd_init(&data.genpd, core::ptr::null_mut(), true);
    if (ret) {
    dev_err_probe(dev, ret, "Failed to add power domain\n");
    goto del_clk_provider;
    }
    ret = of_genpd_add_provider_simple(node, &data.genpd);
    if (ret) {
    dev_err_probe(dev, ret, "Failed to add power domain OF provider\n");
    goto remove_pd;
    }
    ret = of_genpd_add_subdomain(&data.parent_pd, &data.child_pd);
    if (ret) {
    dev_err_probe(dev, ret, "Failed to link PM domains\n");
    goto del_pd_provider;
    }
    pm_runtime_put(dev);
    return 0;
    del_pd_provider:
    of_genpd_del_provider(node);
    remove_pd:
    pm_genpd_remove(&data.genpd);
    del_clk_provider:
    of_clk_del_provider(node);
    put_26m_clk:
    clk_put(data.clk_26m);
    unregister_clks:
    mtk_clk_unregister_gates(mfg_clks, ARRAY_SIZE(mfg_clks), data.clk_data);
    put_pm_runtime:
    pm_runtime_put_sync(dev);
    put_of_node:
    of_node_put(data.parent_pd.np);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn clk_mt8173_mfgtop_remove(pdev: *mut platform_device) {
    static void clk_mt8173_mfgtop_remove(struct platform_device *pdev)
    {
    struct mt8173_mfgtop_data *data = platform_get_drvdata(pdev);
    struct device_node *node = pdev.dev.of_node;
    of_genpd_remove_subdomain(&data.parent_pd, &data.child_pd);
    of_genpd_del_provider(node);
    pm_genpd_remove(&data.genpd);
    of_clk_del_provider(node);
    clk_put(data.clk_26m);
    mtk_clk_unregister_gates(mfg_clks, ARRAY_SIZE(mfg_clks), data.clk_data);
    of_node_put(data.parent_pd.np);
    }
    static const struct of_device_id of_match_clk_mt8173_mfgtop[] = {
    { .compatible = "mediatek,mt8173-mfgtop" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8173_mfgtop);
    static struct platform_driver clk_mt8173_mfgtop_drv = {
    .probe = clk_mt8173_mfgtop_probe,
    .remove = clk_mt8173_mfgtop_remove,
    .driver = {
    .name = "clk-mt8173-mfgtop",
    .of_match_table = of_match_clk_mt8173_mfgtop,
    },
    };
    module_platform_driver(clk_mt8173_mfgtop_drv);
    MODULE_DESCRIPTION("MediaTek MT8173 mfgtop clock driver");
    MODULE_LICENSE("GPL");
