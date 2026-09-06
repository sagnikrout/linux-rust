//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/omap2/omapfb/dss/hdmi_pll.c
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
// Copyright (C) 2013 Texas Instruments Incorporated
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
    void hdmi_pll_compute(struct hdmi_pll_data *pll,
    unsigned long target_tmds, struct dss_pll_clock_info *pi)
    {
    unsigned long fint, clkdco, clkout;
    unsigned long target_bitclk, target_clkdco;
    unsigned long min_dco;
    unsigned n, m, mf, m2, sd;
    unsigned long clkin;
    const struct dss_pll_hw *hw = pll.pll.hw;
    clkin = clk_get_rate(pll.pll.clkin);
    DSSDBG("clkin %lu, target tmds %lu\n", clkin, target_tmds);
    target_bitclk = target_tmds * 10;
// Fint
    n = DIV_ROUND_UP(clkin, hw.fint_max);
    fint = clkin / n;
// adjust m2 so that the clkdco will be high enough
    min_dco = roundup(hw.clkdco_min, fint);
    m2 = DIV_ROUND_UP(min_dco, target_bitclk);
    if (m2 == 0)
    m2 = 1;
    target_clkdco = target_bitclk * m2;
    m = target_clkdco / fint;
    clkdco = fint * m;
// adjust clkdco with fractional mf
    if (WARN_ON(target_clkdco - clkdco > fint))
    mf = 0;
    else
    mf = (u32)div_u64(262144ull * (target_clkdco - clkdco), fint);
    if (mf > 0)
    clkdco += (u32)div_u64((u64)mf * fint, 262144);
    clkout = clkdco / m2;
// sigma-delta
    sd = DIV_ROUND_UP(fint * m, 250000000);
    DSSDBG("N = %u, M = %u, M.f = %u, M2 = %u, SD = %u\n",
    n, m, mf, m2, sd);
    DSSDBG("Fint %lu, clkdco %lu, clkout %lu\n", fint, clkdco, clkout);
    pi.n = n;
    pi.m = m;
    pi.mf = mf;
    pi.mX[0] = m2;
    pi.sd = sd;
    pi.fint = fint;
    pi.clkdco = clkdco;
    pi.clkout[0] = clkout;
    }
#[no_mangle]
unsafe extern "C" fn hdmi_pll_enable(dsspll: *mut dss_pll) -> c_int {
    static int hdmi_pll_enable(struct dss_pll *dsspll)
    {
    struct hdmi_pll_data *pll = container_of(dsspll, struct hdmi_pll_data, pll);
    struct hdmi_wp_data *wp = pll.wp;
    dss_ctrl_pll_enable(DSS_PLL_HDMI, true);
    return hdmi_wp_set_pll_pwr(wp, HDMI_PLLPWRCMD_BOTHON_ALLCLKS);
    }
#[no_mangle]
unsafe extern "C" fn hdmi_pll_disable(dsspll: *mut dss_pll) {
    static void hdmi_pll_disable(struct dss_pll *dsspll)
    {
    struct hdmi_pll_data *pll = container_of(dsspll, struct hdmi_pll_data, pll);
    struct hdmi_wp_data *wp = pll.wp;
    hdmi_wp_set_pll_pwr(wp, HDMI_PLLPWRCMD_ALLOFF);
    dss_ctrl_pll_enable(DSS_PLL_HDMI, false);
    }
    static const struct dss_pll_ops dsi_pll_ops = {
    .enable = hdmi_pll_enable,
    .disable = hdmi_pll_disable,
    .set_config = dss_pll_write_config_type_b,
    };
    static const struct dss_pll_hw dss_omap4_hdmi_pll_hw = {
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
#[no_mangle]
unsafe extern "C" fn dsi_init_pll_data(pdev: *mut platform_device, hpll: *mut hdmi_pll_data) -> c_int {
    static int dsi_init_pll_data(struct platform_device *pdev, struct hdmi_pll_data *hpll)
    {
    struct dss_pll *pll = &hpll.pll;
    struct clk *clk;
    clk = devm_clk_get(&pdev.dev, "sys_clk");
    if (IS_ERR(clk)) {
    DSSERR("can't get sys_clk\n");
    return PTR_ERR(clk);
    }
    pll.name = "hdmi";
    pll.id = DSS_PLL_HDMI;
    pll.base = hpll.base;
    pll.clkin = clk;
    switch (omapdss_get_version()) {
    case OMAPDSS_VER_OMAP4430_ES1:
    case OMAPDSS_VER_OMAP4430_ES2:
    case OMAPDSS_VER_OMAP4:
    pll.hw = &dss_omap4_hdmi_pll_hw;
    break;
    case OMAPDSS_VER_OMAP5:
    case OMAPDSS_VER_DRA7xx:
    pll.hw = &dss_omap5_hdmi_pll_hw;
    break;
    default:
    return -ENODEV;
    }
    pll.ops = &dsi_pll_ops;
    return dss_pll_register(pll);
    }
    int hdmi_pll_init(struct platform_device *pdev, struct hdmi_pll_data *pll,
    struct hdmi_wp_data *wp)
    {
    int r;
    pll.wp = wp;
    pll.base = devm_platform_ioremap_resource_byname(pdev, "pll");
    if (IS_ERR(pll.base)) {
    DSSERR("can't ioremap PLLCTRL\n");
    return PTR_ERR(pll.base);
    }
    r = dsi_init_pll_data(pdev, pll);
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
