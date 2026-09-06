//! Automatically rewritten from C to Rust
//! Source: drivers/phy/cadence/cdns-dphy.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright: 2017-2018 Cadence Design Systems, Inc.
//

pub const REG_WAKEUP_TIME_NS: c_int = 800;
pub const DPHY_PLL_RATE_HZ: c_int = 108000000;
pub const POLL_TIMEOUT_US: c_int = 1000;
// DPHY registers

pub const DPHY_TX_J721E_WIZ_PLL_CTRL: c_uint = 0xF04;
pub const DPHY_TX_J721E_WIZ_STATUS: c_uint = 0xF08;
pub const DPHY_TX_J721E_WIZ_RST_CTRL: c_uint = 0xF0C;
pub const DPHY_TX_J721E_WIZ_PSM_FREQ: c_uint = 0xF10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_dphy_cfg {
    pub pll_ipdiv: u8,
    pub pll_opdiv: u8,
    pub pll_fbdiv: u16,
    pub hs_clk_rate: u32,
    pub nlanes: c_uint,
}

    enum cdns_dphy_clk_lane_cfg {
    DPHY_CLK_CFG_LEFT_DRIVES_ALL = 0,
    DPHY_CLK_CFG_LEFT_DRIVES_RIGHT = 1,
    DPHY_CLK_CFG_LEFT_DRIVES_LEFT = 2,
    DPHY_CLK_CFG_RIGHT_DRIVES_ALL = 3,
    };
    struct cdns_dphy;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_dphy_ops {
    pub dphy): *mut *mut int (probe)(struct cdns_dphy,
    pub dphy): *mut *mut void (remove)(struct cdns_dphy,
    pub div): *mut *mut *mut void (set_psm_div)(struct cdns_dphy dphy, u8,
    void (*set_clk_lane_cfg)(struct cdns_dphy *dphy,
    pub cfg): enum cdns_dphy_clk_lane_cfg,
    void (*set_pll_cfg)(struct cdns_dphy *dphy,
    pub cfg): *const cdns_dphy_cfg,
    pub dphy): *mut *mut unsigned long (get_wakeup_time_ns)(struct cdns_dphy,
    pub dphy): *mut *mut int (wait_for_pll_lock)(struct cdns_dphy,
    pub dphy): *mut *mut int (wait_for_cmn_ready)(struct cdns_dphy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_dphy {
    pub cfg: cdns_dphy_cfg,
    pub regs: *mut void __iomem,
    pub psm_clk: *mut clk,
    pub pll_ref_clk: *mut clk,
    pub ops: *const cdns_dphy_ops,
    pub phy: *mut phy,
    pub is_configured: bool,
    pub is_powered: bool,
}

// Order of bands is important since the index is the band number.
    static const unsigned int tx_bands[] = {
    80, 100, 120, 160, 200, 240, 320, 390, 450, 510, 560, 640, 690, 770,
    870, 950, 1000, 1200, 1400, 1600, 1800, 2000, 2200, 2500
    };
    static int cdns_dphy_get_pll_cfg(struct cdns_dphy *dphy,
    struct cdns_dphy_cfg *cfg,
    struct phy_configure_opts_mipi_dphy *opts)
    {
    let mut pll_ref_hz: c_ulong = clk_get_rate(dphy.pll_ref_clk);
    u64 dlane_bps;
    memset(cfg, 0, sizeof(*cfg));
    if (pll_ref_hz < 9600000 || pll_ref_hz >= 150000000)
    return -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if(19200000: pll_ref_hz <) -> else {
    else if (pll_ref_hz < 19200000)
    cfg.pll_ipdiv = 1;
#[no_mangle]
pub unsafe extern "C" fn if(38400000: pll_ref_hz <) -> else {
    else if (pll_ref_hz < 38400000)
    cfg.pll_ipdiv = 2;
#[no_mangle]
pub unsafe extern "C" fn if(76800000: pll_ref_hz <) -> else {
    else if (pll_ref_hz < 76800000)
    cfg.pll_ipdiv = 4;
    else
    cfg.pll_ipdiv = 8;
    dlane_bps = opts.hs_clk_rate;
    if (dlane_bps > 2500000000UL || dlane_bps < 80000000UL)
    return -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if(1250000000: dlane_bps >=) -> else {
    else if (dlane_bps >= 1250000000)
    cfg.pll_opdiv = 1;
#[no_mangle]
pub unsafe extern "C" fn if(630000000: dlane_bps >=) -> else {
    else if (dlane_bps >= 630000000)
    cfg.pll_opdiv = 2;
#[no_mangle]
pub unsafe extern "C" fn if(320000000: dlane_bps >=) -> else {
    else if (dlane_bps >= 320000000)
    cfg.pll_opdiv = 4;
#[no_mangle]
pub unsafe extern "C" fn if(160000000: dlane_bps >=) -> else {
    else if (dlane_bps >= 160000000)
    cfg.pll_opdiv = 8;
#[no_mangle]
pub unsafe extern "C" fn if(80000000: dlane_bps >=) -> else {
    else if (dlane_bps >= 80000000)
    cfg.pll_opdiv = 16;
    cfg.pll_fbdiv = DIV_ROUND_UP_ULL(dlane_bps * 2 * cfg.pll_opdiv *
    cfg.pll_ipdiv,
    pll_ref_hz);
    cfg.hs_clk_rate = div_u64((u64)pll_ref_hz * cfg.pll_fbdiv,
    2 * cfg.pll_opdiv * cfg.pll_ipdiv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_dphy_setup_psm(dphy: *mut cdns_dphy) -> c_int {
    static int cdns_dphy_setup_psm(struct cdns_dphy *dphy)
    {
    let mut psm_clk_hz: c_ulong = clk_get_rate(dphy.psm_clk);
    unsigned long psm_div;
    if (!psm_clk_hz || psm_clk_hz > 100000000)
    return -EINVAL;
    psm_div = DIV_ROUND_CLOSEST(psm_clk_hz, 1000000);
    if (dphy.ops.set_psm_div)
    dphy.ops.set_psm_div(dphy, psm_div);
    return 0;
    }
    static void cdns_dphy_set_clk_lane_cfg(struct cdns_dphy *dphy,
    enum cdns_dphy_clk_lane_cfg cfg)
    {
    if (dphy.ops.set_clk_lane_cfg)
    dphy.ops.set_clk_lane_cfg(dphy, cfg);
    }
    static void cdns_dphy_set_pll_cfg(struct cdns_dphy *dphy,
    const struct cdns_dphy_cfg *cfg)
    {
    if (dphy.ops.set_pll_cfg)
    dphy.ops.set_pll_cfg(dphy, cfg);
    }
#[no_mangle]
unsafe extern "C" fn cdns_dphy_get_wakeup_time_ns(dphy: *mut cdns_dphy) -> c_ulong {
    static unsigned long cdns_dphy_get_wakeup_time_ns(struct cdns_dphy *dphy)
    {
    return dphy.ops.get_wakeup_time_ns(dphy);
    }
#[no_mangle]
unsafe extern "C" fn cdns_dphy_wait_for_pll_lock(dphy: *mut cdns_dphy) -> c_int {
    static int cdns_dphy_wait_for_pll_lock(struct cdns_dphy *dphy)
    {
    return dphy.ops.wait_for_pll_lock ? dphy.ops.wait_for_pll_lock(dphy) : 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_dphy_wait_for_cmn_ready(dphy: *mut cdns_dphy) -> c_int {
    static int cdns_dphy_wait_for_cmn_ready(struct cdns_dphy *dphy)
    {
    return  dphy.ops.wait_for_cmn_ready ? dphy.ops.wait_for_cmn_ready(dphy) : 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_dphy_ref_get_wakeup_time_ns(dphy: *mut cdns_dphy) -> c_ulong {
    static unsigned long cdns_dphy_ref_get_wakeup_time_ns(struct cdns_dphy *dphy)
    {
// Default wakeup time is 800 ns (in a simulated environment).
    return 800;
    }
    static void cdns_dphy_ref_set_pll_cfg(struct cdns_dphy *dphy,
    const struct cdns_dphy_cfg *cfg)
    {
    u32 fbdiv_low, fbdiv_high;
    fbdiv_low = (cfg.pll_fbdiv / 4) - 2;
    fbdiv_high = cfg.pll_fbdiv - fbdiv_low - 2;
    writel(DPHY_CMN_IPDIV_FROM_REG | DPHY_CMN_OPDIV_FROM_REG |
    DPHY_CMN_IPDIV(cfg.pll_ipdiv) |
    DPHY_CMN_OPDIV(cfg.pll_opdiv),
    dphy.regs + DPHY_CMN_OPIPDIV);
    writel(DPHY_CMN_FBDIV_FROM_REG |
    DPHY_CMN_FBDIV_VAL(fbdiv_low, fbdiv_high),
    dphy.regs + DPHY_CMN_FBDIV);
    writel(DPHY_CMN_PWM_HIGH(6) | DPHY_CMN_PWM_LOW(0x101) |
    DPHY_CMN_PWM_DIV(0x8),
    dphy.regs + DPHY_CMN_PWM);
    }
#[no_mangle]
unsafe extern "C" fn cdns_dphy_ref_set_psm_div(dphy: *mut cdns_dphy, div: u8) {
    static void cdns_dphy_ref_set_psm_div(struct cdns_dphy *dphy, u8 div)
    {
    writel(DPHY_PSM_CFG_FROM_REG | DPHY_PSM_CLK_DIV(div),
    dphy.regs + DPHY_PSM_CFG);
    }
#[no_mangle]
unsafe extern "C" fn cdns_dphy_j721e_get_wakeup_time_ns(dphy: *mut cdns_dphy) -> c_ulong {
    static unsigned long cdns_dphy_j721e_get_wakeup_time_ns(struct cdns_dphy *dphy)
    {
// Minimum wakeup time as per MIPI D-PHY spec v1.2
    return 1000000;
    }
    static void cdns_dphy_j721e_set_pll_cfg(struct cdns_dphy *dphy,
    const struct cdns_dphy_cfg *cfg)
    {
//
// set the PWM and PLL Byteclk divider settings to recommended values
// which is same as that of in ref ops
//
    writel(DPHY_CMN_PWM_HIGH(6) | DPHY_CMN_PWM_LOW(0x101) |
    DPHY_CMN_PWM_DIV(0x8),
    dphy.regs + DPHY_CMN_PWM);
    writel((FIELD_PREP(DPHY_TX_J721E_WIZ_IPDIV, cfg.pll_ipdiv) |
    FIELD_PREP(DPHY_TX_J721E_WIZ_OPDIV, cfg.pll_opdiv) |
    FIELD_PREP(DPHY_TX_J721E_WIZ_FBDIV, cfg.pll_fbdiv)),
    dphy.regs + DPHY_TX_J721E_WIZ_PLL_CTRL);
    writel(DPHY_TX_J721E_WIZ_LANE_RSTB,
    dphy.regs + DPHY_TX_J721E_WIZ_RST_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn cdns_dphy_j721e_set_psm_div(dphy: *mut cdns_dphy, div: u8) {
    static void cdns_dphy_j721e_set_psm_div(struct cdns_dphy *dphy, u8 div)
    {
    writel(div, dphy.regs + DPHY_TX_J721E_WIZ_PSM_FREQ);
    }
#[no_mangle]
unsafe extern "C" fn cdns_dphy_j721e_wait_for_pll_lock(dphy: *mut cdns_dphy) -> c_int {
    static int cdns_dphy_j721e_wait_for_pll_lock(struct cdns_dphy *dphy)
    {
    u32 status;
    return readl_poll_timeout(dphy.regs + DPHY_TX_J721E_WIZ_PLL_CTRL, status,
    status & DPHY_TX_WIZ_PLL_LOCK, 0, POLL_TIMEOUT_US);
    }
#[no_mangle]
unsafe extern "C" fn cdns_dphy_j721e_wait_for_cmn_ready(dphy: *mut cdns_dphy) -> c_int {
    static int cdns_dphy_j721e_wait_for_cmn_ready(struct cdns_dphy *dphy)
    {
    u32 status;
    return readl_poll_timeout(dphy.regs + DPHY_TX_J721E_WIZ_STATUS, status,
    status & DPHY_TX_WIZ_O_CMN_READY, 0,
    POLL_TIMEOUT_US);
    }
//
// This is the reference implementation of DPHY hooks. Specific integration of
// this IP may have to re-implement some of them depending on how they decided
// to wire things in the SoC.
//
    static const struct cdns_dphy_ops ref_dphy_ops = {
    .get_wakeup_time_ns = cdns_dphy_ref_get_wakeup_time_ns,
    .set_pll_cfg = cdns_dphy_ref_set_pll_cfg,
    .set_psm_div = cdns_dphy_ref_set_psm_div,
    };
    static const struct cdns_dphy_ops j721e_dphy_ops = {
    .get_wakeup_time_ns = cdns_dphy_j721e_get_wakeup_time_ns,
    .set_pll_cfg = cdns_dphy_j721e_set_pll_cfg,
    .set_psm_div = cdns_dphy_j721e_set_psm_div,
    .wait_for_pll_lock = cdns_dphy_j721e_wait_for_pll_lock,
    .wait_for_cmn_ready = cdns_dphy_j721e_wait_for_cmn_ready,
    };
    static int cdns_dphy_config_from_opts(struct phy *phy,
    struct phy_configure_opts_mipi_dphy *opts,
    struct cdns_dphy_cfg *cfg)
    {
    struct cdns_dphy *dphy = phy_get_drvdata(phy);
    int ret;
    ret = phy_mipi_dphy_config_validate(opts);
    if (ret)
    return ret;
    ret = cdns_dphy_get_pll_cfg(dphy, cfg, opts);
    if (ret)
    return ret;
    opts.hs_clk_rate = cfg.hs_clk_rate;
    opts.wakeup = cdns_dphy_get_wakeup_time_ns(dphy) / 1000;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_dphy_tx_get_band_ctrl(hs_clk_rate: c_ulong) -> c_int {
    static int cdns_dphy_tx_get_band_ctrl(unsigned long hs_clk_rate)
    {
    unsigned int rate;
    int i;
    rate = hs_clk_rate / 1000000UL;
    if (rate < tx_bands[0])
    return -EOPNOTSUPP;
    for (i = 0; i < ARRAY_SIZE(tx_bands) - 1; i++) {
    if (rate >= tx_bands[i] && rate < tx_bands[i + 1])
    return i;
    }
    return -EOPNOTSUPP;
    }
    static int cdns_dphy_validate(struct phy *phy, enum phy_mode mode, int submode,
    union phy_configure_opts *opts)
    {
    let mut cfg: cdns_dphy_cfg = { 0 };
    if (mode != PHY_MODE_MIPI_DPHY)
    return -EINVAL;
    return cdns_dphy_config_from_opts(phy, &opts.mipi_dphy, &cfg);
    }
#[no_mangle]
unsafe extern "C" fn cdns_dphy_configure(phy: *mut phy, opts: *mut union phy_configure_opts) -> c_int {
    static int cdns_dphy_configure(struct phy *phy, union phy_configure_opts *opts)
    {
    struct cdns_dphy *dphy = phy_get_drvdata(phy);
    int ret;
    ret = cdns_dphy_config_from_opts(phy, &opts.mipi_dphy, &dphy.cfg);
    if (!ret)
    dphy.is_configured = true;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdns_dphy_power_on(phy: *mut phy) -> c_int {
    static int cdns_dphy_power_on(struct phy *phy)
    {
    struct cdns_dphy *dphy = phy_get_drvdata(phy);
    int ret;
    u32 reg;
    if (!dphy.is_configured || dphy.is_powered)
    return -EINVAL;
    clk_prepare_enable(dphy.psm_clk);
    clk_prepare_enable(dphy.pll_ref_clk);
//
// Configure the internal PSM clk divider so that the DPHY has a
// 1MHz clk (or something close).
//
    ret = cdns_dphy_setup_psm(dphy);
    if (ret) {
    dev_err(&dphy.phy.dev, "Failed to setup PSM with error %d\n", ret);
    goto err_power_on;
    }
//
// Configure attach clk lanes to data lanes: the DPHY has 2 clk lanes
// and 8 data lanes, each clk lane can be attache different set of
// data lanes. The 2 groups are named 'left' and 'right', so here we
// just say that we want the 'left' clk lane to drive the 'left' data
// lanes.
//
    cdns_dphy_set_clk_lane_cfg(dphy, DPHY_CLK_CFG_LEFT_DRIVES_LEFT);
//
// Configure the DPHY PLL that will be used to generate the TX byte
// clk.
//
    cdns_dphy_set_pll_cfg(dphy, &dphy.cfg);
    ret = cdns_dphy_tx_get_band_ctrl(dphy.cfg.hs_clk_rate);
    if (ret < 0) {
    dev_err(&dphy.phy.dev, "Failed to get band control value with error %d\n", ret);
    goto err_power_on;
    }
    reg = FIELD_PREP(DPHY_BAND_CFG_LEFT_BAND, ret) |
    FIELD_PREP(DPHY_BAND_CFG_RIGHT_BAND, ret);
    writel(reg, dphy.regs + DPHY_BAND_CFG);
// Start TX state machine.
    reg = readl(dphy.regs + DPHY_CMN_SSM);
    writel((reg & DPHY_CMN_SSM_CAL_WAIT_TIME) | DPHY_CMN_SSM_EN | DPHY_CMN_TX_MODE_EN,
    dphy.regs + DPHY_CMN_SSM);
    ret = cdns_dphy_wait_for_pll_lock(dphy);
    if (ret) {
    dev_err(&dphy.phy.dev, "Failed to lock PLL with error %d\n", ret);
    goto err_power_on;
    }
    ret = cdns_dphy_wait_for_cmn_ready(dphy);
    if (ret) {
    dev_err(&dphy.phy.dev, "O_CMN_READY signal failed to assert with error %d\n",
    ret);
    goto err_power_on;
    }
    dphy.is_powered = true;
    return 0;
    err_power_on:
    clk_disable_unprepare(dphy.pll_ref_clk);
    clk_disable_unprepare(dphy.psm_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdns_dphy_power_off(phy: *mut phy) -> c_int {
    static int cdns_dphy_power_off(struct phy *phy)
    {
    struct cdns_dphy *dphy = phy_get_drvdata(phy);
    u32 reg;
    clk_disable_unprepare(dphy.pll_ref_clk);
    clk_disable_unprepare(dphy.psm_clk);
// Stop TX state machine.
    reg = readl(dphy.regs + DPHY_CMN_SSM);
    writel(reg & ~DPHY_CMN_SSM_EN, dphy.regs + DPHY_CMN_SSM);
    dphy.is_powered = false;
    return 0;
    }
    static const struct phy_ops cdns_dphy_ops = {
    .configure	= cdns_dphy_configure,
    .validate	= cdns_dphy_validate,
    .power_on	= cdns_dphy_power_on,
    .power_off	= cdns_dphy_power_off,
    };
#[no_mangle]
unsafe extern "C" fn cdns_dphy_probe(pdev: *mut platform_device) -> c_int {
    static int cdns_dphy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct cdns_dphy *dphy;
    int ret;
    dphy = devm_kzalloc(&pdev.dev, sizeof(*dphy), GFP_KERNEL);
    if (!dphy)
    return -ENOMEM;
    dev_set_drvdata(&pdev.dev, dphy);
    dphy.ops = of_device_get_match_data(&pdev.dev);
    if (!dphy.ops)
    return -EINVAL;
    dphy.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(dphy.regs))
    return PTR_ERR(dphy.regs);
    dphy.psm_clk = devm_clk_get(&pdev.dev, "psm");
    if (IS_ERR(dphy.psm_clk))
    return PTR_ERR(dphy.psm_clk);
    dphy.pll_ref_clk = devm_clk_get(&pdev.dev, "pll_ref");
    if (IS_ERR(dphy.pll_ref_clk))
    return PTR_ERR(dphy.pll_ref_clk);
    if (dphy.ops.probe) {
    ret = dphy.ops.probe(dphy);
    if (ret)
    return ret;
    }
    dphy.phy = devm_phy_create(&pdev.dev, core::ptr::null_mut(), &cdns_dphy_ops);
    if (IS_ERR(dphy.phy)) {
    dev_err(&pdev.dev, "failed to create PHY\n");
    if (dphy.ops.remove)
    dphy.ops.remove(dphy);
    return PTR_ERR(dphy.phy);
    }
    phy_set_drvdata(dphy.phy, dphy);
    phy_provider = devm_of_phy_provider_register(&pdev.dev,
    of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
#[no_mangle]
unsafe extern "C" fn cdns_dphy_remove(pdev: *mut platform_device) {
    static void cdns_dphy_remove(struct platform_device *pdev)
    {
    struct cdns_dphy *dphy = dev_get_drvdata(&pdev.dev);
    if (dphy.ops.remove)
    dphy.ops.remove(dphy);
    }
    static const struct of_device_id cdns_dphy_of_match[] = {
    { .compatible = "cdns,dphy", .data = &ref_dphy_ops },
    { .compatible = "ti,j721e-dphy", .data = &j721e_dphy_ops },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, cdns_dphy_of_match);
    static struct platform_driver cdns_dphy_platform_driver = {
    .probe		= cdns_dphy_probe,
    .remove		= cdns_dphy_remove,
    .driver		= {
    .name		= "cdns-mipi-dphy",
    .of_match_table	= cdns_dphy_of_match,
    },
    };
    module_platform_driver(cdns_dphy_platform_driver);
    MODULE_AUTHOR("Maxime Ripard <maxime.ripard@bootlin.com>");
    MODULE_DESCRIPTION("Cadence MIPI D-PHY Driver");
    MODULE_LICENSE("GPL");
