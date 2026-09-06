//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/omapdrm/dss/hdmi_pll.c
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
// HDMI PLL
//
// Copyright (C) 2013 Texas Instruments Incorporated - https://www.ti.com
//

#[no_mangle]
pub unsafe extern "C" fn hdmi_pll_dump(pll: *mut hdmi_pll_data, s: *mut seq_file) {
    void hdmi_pll_dump(struct hdmi_pll_data *pll, struct seq_file *s)
    {

    hdmi_read_reg(pll.base, r))
    DUMPPLL(PLLCTRL_PLL_CONTROL);
    DUMPPLL(PLLCTRL_PLL_STATUS);
    DUMPPLL(PLLCTRL_PLL_GO);
    DUMPPLL(PLLCTRL_CFG1);
    DUMPPLL(PLLCTRL_CFG2);
    DUMPPLL(PLLCTRL_CFG3);
    DUMPPLL(PLLCTRL_SSC_CFG1);
    DUMPPLL(PLLCTRL_SSC_CFG2);
    DUMPPLL(PLLCTRL_CFG4);
    }
#[no_mangle]
unsafe extern "C" fn hdmi_pll_enable(dsspll: *mut dss_pll) -> c_int {
    static int hdmi_pll_enable(struct dss_pll *dsspll)
    {
    struct hdmi_pll_data *pll = container_of(dsspll, struct hdmi_pll_data, pll);
    struct hdmi_wp_data *wp = pll.wp;
    int r;
    r = pm_runtime_get_sync(&pll.pdev.dev);
    WARN_ON(r < 0);
    dss_ctrl_pll_enable(dsspll, true);
    r = hdmi_wp_set_pll_pwr(wp, HDMI_PLLPWRCMD_BOTHON_ALLCLKS);
    if (r)
    return r;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hdmi_pll_disable(dsspll: *mut dss_pll) {
    static void hdmi_pll_disable(struct dss_pll *dsspll)
    {
    struct hdmi_pll_data *pll = container_of(dsspll, struct hdmi_pll_data, pll);
    struct hdmi_wp_data *wp = pll.wp;
    int r;
    hdmi_wp_set_pll_pwr(wp, HDMI_PLLPWRCMD_ALLOFF);
    dss_ctrl_pll_enable(dsspll, false);
    r = pm_runtime_put_sync(&pll.pdev.dev);
    WARN_ON(r < 0 && r != -ENOSYS);
    }
    static const struct dss_pll_ops hdmi_pll_ops = {
    .enable = hdmi_pll_enable,
    .disable = hdmi_pll_disable,
    .set_config = dss_pll_write_config_type_b,
    };
    static const struct dss_pll_hw dss_omap4_hdmi_pll_hw = {
    .type = DSS_PLL_TYPE_B,
    .n_max = 255,
    .m_min = 20,
    .m_max = 4095,
    .mX_max = 127,
    .fint_min = 500000,
    .fint_max = 2500000,
    .clkdco_min = 500000000,
    .clkdco_low = 1000000000,
    .clkdco_max = 2000000000,
    .n_msb = 8,
    .n_lsb = 1,
    .m_msb = 20,
    .m_lsb = 9,
    .mX_msb[0] = 24,
    .mX_lsb[0] = 18,
    .has_selfreqdco = true,
    };
    static const struct dss_pll_hw dss_omap5_hdmi_pll_hw = {
    .type = DSS_PLL_TYPE_B,
    .n_max = 255,
    .m_min = 20,
    .m_max = 2045,
    .mX_max = 127,
    .fint_min = 620000,
    .fint_max = 2500000,
    .clkdco_min = 750000000,
    .clkdco_low = 1500000000,
    .clkdco_max = 2500000000UL,
    .n_msb = 8,
    .n_lsb = 1,
    .m_msb = 20,
    .m_lsb = 9,
    .mX_msb[0] = 24,
    .mX_lsb[0] = 18,
    .has_selfreqdco = true,
    .has_refsel = true,
    };
    static int hdmi_init_pll_data(struct dss_device *dss,
    struct platform_device *pdev,
    struct hdmi_pll_data *hpll)
    {
    struct dss_pll *pll = &hpll.pll;
    struct clk *clk;
    int r;
    clk = devm_clk_get(&pdev.dev, "sys_clk");
    if (IS_ERR(clk)) {
    DSSERR("can't get sys_clk\n");
    return PTR_ERR(clk);
    }
    pll.name = "hdmi";
    pll.id = DSS_PLL_HDMI;
    pll.base = hpll.base;
    pll.clkin = clk;
    if (hpll.wp.version == 4)
    pll.hw = &dss_omap4_hdmi_pll_hw;
    else
    pll.hw = &dss_omap5_hdmi_pll_hw;
    pll.ops = &hdmi_pll_ops;
    r = dss_pll_register(dss, pll);
    if (r)
    return r;
    return 0;
    }
    int hdmi_pll_init(struct dss_device *dss, struct platform_device *pdev,
    struct hdmi_pll_data *pll, struct hdmi_wp_data *wp)
    {
    int r;
    pll.pdev = pdev;
    pll.wp = wp;
    pll.base = devm_platform_ioremap_resource_byname(pdev, "pll");
    if (IS_ERR(pll.base))
    return PTR_ERR(pll.base);
    r = hdmi_init_pll_data(dss, pdev, pll);
    if (r) {
    DSSERR("failed to init HDMI PLL\n");
    return r;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hdmi_pll_uninit(hpll: *mut hdmi_pll_data) {
    void hdmi_pll_uninit(struct hdmi_pll_data *hpll)
    {
    struct dss_pll *pll = &hpll.pll;
    dss_pll_unregister(pll);
    }
