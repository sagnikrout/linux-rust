//! Automatically rewritten from C to Rust
//! Source: drivers/phy/allwinner/phy-sun6i-mipi-dphy.c
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
// Copyright (c) 2016 Allwinnertech Co., Ltd.
// Copyright (C) 2017-2018 Bootlin
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//

pub const SUN6I_DPHY_GCTL_REG: c_uint = 0x00;

pub const SUN6I_DPHY_TX_CTL_REG: c_uint = 0x04;

pub const SUN6I_DPHY_RX_CTL_REG: c_uint = 0x08;

pub const SUN6I_DPHY_TX_TIME0_REG: c_uint = 0x10;

pub const SUN6I_DPHY_TX_TIME1_REG: c_uint = 0x14;

pub const SUN6I_DPHY_TX_TIME2_REG: c_uint = 0x18;

pub const SUN6I_DPHY_TX_TIME3_REG: c_uint = 0x1c;
pub const SUN6I_DPHY_TX_TIME4_REG: c_uint = 0x20;

pub const SUN6I_DPHY_RX_TIME0_REG: c_uint = 0x30;

pub const SUN6I_DPHY_RX_TIME1_REG: c_uint = 0x34;

pub const SUN6I_DPHY_RX_TIME2_REG: c_uint = 0x38;

pub const SUN6I_DPHY_RX_TIME3_REG: c_uint = 0x40;

pub const SUN6I_DPHY_ANA0_REG: c_uint = 0x4c;

pub const SUN6I_DPHY_ANA1_REG: c_uint = 0x50;

pub const SUN6I_DPHY_ANA2_REG: c_uint = 0x54;

pub const SUN6I_DPHY_ANA3_REG: c_uint = 0x58;

pub const SUN6I_DPHY_ANA4_REG: c_uint = 0x5c;

pub const SUN6I_DPHY_DBG5_REG: c_uint = 0xf4;
pub const SUN50I_DPHY_TX_SLEW_REG0: c_uint = 0xf8;
pub const SUN50I_DPHY_TX_SLEW_REG1: c_uint = 0xfc;
pub const SUN50I_DPHY_TX_SLEW_REG2: c_uint = 0x100;
pub const SUN50I_DPHY_PLL_REG0: c_uint = 0x104;

pub const SUN50I_DPHY_PLL_REG1: c_uint = 0x108;

pub const SUN50I_DPHY_PLL_REG2: c_uint = 0x10c;

pub const SUN50I_COMBO_PHY_REG0: c_uint = 0x110;

pub const SUN50I_COMBO_PHY_REG1: c_uint = 0x114;

pub const SUN50I_COMBO_PHY_REG2: c_uint = 0x118;

    enum sun6i_dphy_direction {
    SUN6I_DPHY_DIRECTION_TX,
    SUN6I_DPHY_DIRECTION_RX,
    };
    struct sun6i_dphy;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_dphy_variant {
    pub dphy): *mut *mut void (tx_power_on)(struct sun6i_dphy,
    pub rx_supported: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_dphy {
    pub bus_clk: *mut clk,
    pub mod_clk: *mut clk,
    pub regs: *mut regmap,
    pub reset: *mut reset_control,
    pub phy: *mut phy,
    pub config: phy_configure_opts_mipi_dphy,
    pub variant: *const sun6i_dphy_variant,
    pub direction: enum sun6i_dphy_direction,
}

#[no_mangle]
unsafe extern "C" fn sun6i_dphy_init(phy: *mut phy) -> c_int {
    static int sun6i_dphy_init(struct phy *phy)
    {
    struct sun6i_dphy *dphy = phy_get_drvdata(phy);
    reset_control_deassert(dphy.reset);
    clk_prepare_enable(dphy.mod_clk);
    clk_set_rate_exclusive(dphy.mod_clk, 150000000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun6i_dphy_configure(phy: *mut phy, opts: *mut union phy_configure_opts) -> c_int {
    static int sun6i_dphy_configure(struct phy *phy, union phy_configure_opts *opts)
    {
    struct sun6i_dphy *dphy = phy_get_drvdata(phy);
    int ret;
    ret = phy_mipi_dphy_config_validate(&opts.mipi_dphy);
    if (ret)
    return ret;
    memcpy(&dphy.config, opts, sizeof(dphy.config));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun6i_a31_mipi_dphy_tx_power_on(dphy: *mut sun6i_dphy) {
    static void sun6i_a31_mipi_dphy_tx_power_on(struct sun6i_dphy *dphy)
    {
    let mut lanes_mask: u8 = GENMASK(dphy.config.lanes - 1, 0);
    regmap_write(dphy.regs, SUN6I_DPHY_ANA0_REG,
    SUN6I_DPHY_ANA0_REG_PWS |
    SUN6I_DPHY_ANA0_REG_DMPC |
    SUN6I_DPHY_ANA0_REG_SLV(7) |
    SUN6I_DPHY_ANA0_REG_DMPD(lanes_mask) |
    SUN6I_DPHY_ANA0_REG_DEN(lanes_mask));
    regmap_write(dphy.regs, SUN6I_DPHY_ANA1_REG,
    SUN6I_DPHY_ANA1_REG_CSMPS(1) |
    SUN6I_DPHY_ANA1_REG_SVTT(7));
    regmap_write(dphy.regs, SUN6I_DPHY_ANA4_REG,
    SUN6I_DPHY_ANA4_REG_CKDV(1) |
    SUN6I_DPHY_ANA4_REG_TMSC(1) |
    SUN6I_DPHY_ANA4_REG_TMSD(1) |
    SUN6I_DPHY_ANA4_REG_TXDNSC(1) |
    SUN6I_DPHY_ANA4_REG_TXDNSD(1) |
    SUN6I_DPHY_ANA4_REG_TXPUSC(1) |
    SUN6I_DPHY_ANA4_REG_TXPUSD(1) |
    SUN6I_DPHY_ANA4_REG_DMPLVC |
    SUN6I_DPHY_ANA4_REG_DMPLVD(lanes_mask));
    regmap_write(dphy.regs, SUN6I_DPHY_ANA2_REG,
    SUN6I_DPHY_ANA2_REG_ENIB);
    udelay(5);
    regmap_write(dphy.regs, SUN6I_DPHY_ANA3_REG,
    SUN6I_DPHY_ANA3_EN_LDOR |
    SUN6I_DPHY_ANA3_EN_LDOC |
    SUN6I_DPHY_ANA3_EN_LDOD);
    udelay(1);
    }
#[no_mangle]
unsafe extern "C" fn sun50i_a100_mipi_dphy_tx_power_on(dphy: *mut sun6i_dphy) {
    static void sun50i_a100_mipi_dphy_tx_power_on(struct sun6i_dphy *dphy)
    {
    let mut mipi_symbol_rate: c_ulong = dphy.config.hs_clk_rate;
    unsigned int div, n;
    regmap_write(dphy.regs, SUN6I_DPHY_ANA4_REG,
    SUN6I_DPHY_ANA4_REG_IB(2) |
    SUN6I_DPHY_ANA4_REG_DMPLVD(4) |
    SUN6I_DPHY_ANA4_REG_VTT_SET(3) |
    SUN6I_DPHY_ANA4_REG_CKDV(3) |
    SUN6I_DPHY_ANA4_REG_TMSD(1) |
    SUN6I_DPHY_ANA4_REG_TMSC(1) |
    SUN6I_DPHY_ANA4_REG_TXPUSD(2) |
    SUN6I_DPHY_ANA4_REG_TXPUSC(3) |
    SUN6I_DPHY_ANA4_REG_TXDNSD(2) |
    SUN6I_DPHY_ANA4_REG_TXDNSC(3));
    regmap_update_bits(dphy.regs, SUN6I_DPHY_ANA2_REG,
    SUN6I_DPHY_ANA2_EN_CK_CPU,
    SUN6I_DPHY_ANA2_EN_CK_CPU);
    regmap_update_bits(dphy.regs, SUN6I_DPHY_ANA2_REG,
    SUN6I_DPHY_ANA2_REG_ENIB,
    SUN6I_DPHY_ANA2_REG_ENIB);
    regmap_write(dphy.regs, SUN6I_DPHY_ANA3_REG,
    SUN6I_DPHY_ANA3_EN_LDOR |
    SUN6I_DPHY_ANA3_EN_LDOC |
    SUN6I_DPHY_ANA3_EN_LDOD);
    regmap_write(dphy.regs, SUN6I_DPHY_ANA0_REG,
    SUN6I_DPHY_ANA0_REG_PLR(4) |
    SUN6I_DPHY_ANA0_REG_SFB(1));
    regmap_write(dphy.regs, SUN50I_COMBO_PHY_REG0,
    SUN50I_COMBO_PHY_REG0_EN_CP);
// Choose a divider to limit the VCO frequency to around 2 GHz.
    div = 16 >> order_base_2(DIV_ROUND_UP(mipi_symbol_rate, 264000000));
    n = mipi_symbol_rate * div / 24000000;
    regmap_write(dphy.regs, SUN50I_DPHY_PLL_REG0,
    SUN50I_DPHY_PLL_REG0_CP36_EN |
    SUN50I_DPHY_PLL_REG0_LDO_EN |
    SUN50I_DPHY_PLL_REG0_EN_LVS |
    SUN50I_DPHY_PLL_REG0_PLL_EN |
    SUN50I_DPHY_PLL_REG0_NDET |
    SUN50I_DPHY_PLL_REG0_P((div - 1) % 8) |
    SUN50I_DPHY_PLL_REG0_N(n) |
    SUN50I_DPHY_PLL_REG0_M0((div - 1) / 8) |
    SUN50I_DPHY_PLL_REG0_M1(2));
// Disable sigma-delta modulation.
    regmap_write(dphy.regs, SUN50I_DPHY_PLL_REG2, 0);
    regmap_update_bits(dphy.regs, SUN6I_DPHY_ANA4_REG,
    SUN6I_DPHY_ANA4_REG_EN_MIPI,
    SUN6I_DPHY_ANA4_REG_EN_MIPI);
    regmap_update_bits(dphy.regs, SUN50I_COMBO_PHY_REG0,
    SUN50I_COMBO_PHY_REG0_EN_MIPI |
    SUN50I_COMBO_PHY_REG0_EN_COMBOLDO,
    SUN50I_COMBO_PHY_REG0_EN_MIPI |
    SUN50I_COMBO_PHY_REG0_EN_COMBOLDO);
    regmap_write(dphy.regs, SUN50I_COMBO_PHY_REG2,
    SUN50I_COMBO_PHY_REG2_HS_STOP_DLY(20));
    udelay(1);
    }
#[no_mangle]
unsafe extern "C" fn sun6i_dphy_tx_power_on(dphy: *mut sun6i_dphy) -> c_int {
    static int sun6i_dphy_tx_power_on(struct sun6i_dphy *dphy)
    {
    let mut lanes_mask: u8 = GENMASK(dphy.config.lanes - 1, 0);
    regmap_write(dphy.regs, SUN6I_DPHY_TX_CTL_REG,
    SUN6I_DPHY_TX_CTL_HS_TX_CLK_CONT);
    regmap_write(dphy.regs, SUN6I_DPHY_TX_TIME0_REG,
    SUN6I_DPHY_TX_TIME0_LP_CLK_DIV(14) |
    SUN6I_DPHY_TX_TIME0_HS_PREPARE(6) |
    SUN6I_DPHY_TX_TIME0_HS_TRAIL(10));
    regmap_write(dphy.regs, SUN6I_DPHY_TX_TIME1_REG,
    SUN6I_DPHY_TX_TIME1_CLK_PREPARE(7) |
    SUN6I_DPHY_TX_TIME1_CLK_ZERO(50) |
    SUN6I_DPHY_TX_TIME1_CLK_PRE(3) |
    SUN6I_DPHY_TX_TIME1_CLK_POST(10));
    regmap_write(dphy.regs, SUN6I_DPHY_TX_TIME2_REG,
    SUN6I_DPHY_TX_TIME2_CLK_TRAIL(30));
    regmap_write(dphy.regs, SUN6I_DPHY_TX_TIME3_REG, 0);
    regmap_write(dphy.regs, SUN6I_DPHY_TX_TIME4_REG,
    SUN6I_DPHY_TX_TIME4_HS_TX_ANA0(3) |
    SUN6I_DPHY_TX_TIME4_HS_TX_ANA1(3));
    dphy.variant.tx_power_on(dphy);
    regmap_update_bits(dphy.regs, SUN6I_DPHY_ANA3_REG,
    SUN6I_DPHY_ANA3_EN_VTTC |
    SUN6I_DPHY_ANA3_EN_VTTD_MASK,
    SUN6I_DPHY_ANA3_EN_VTTC |
    SUN6I_DPHY_ANA3_EN_VTTD(lanes_mask));
    udelay(1);
    regmap_update_bits(dphy.regs, SUN6I_DPHY_ANA3_REG,
    SUN6I_DPHY_ANA3_EN_DIV,
    SUN6I_DPHY_ANA3_EN_DIV);
    udelay(1);
    regmap_update_bits(dphy.regs, SUN6I_DPHY_ANA2_REG,
    SUN6I_DPHY_ANA2_EN_CK_CPU,
    SUN6I_DPHY_ANA2_EN_CK_CPU);
    udelay(1);
    regmap_update_bits(dphy.regs, SUN6I_DPHY_ANA1_REG,
    SUN6I_DPHY_ANA1_REG_VTTMODE,
    SUN6I_DPHY_ANA1_REG_VTTMODE);
    regmap_update_bits(dphy.regs, SUN6I_DPHY_ANA2_REG,
    SUN6I_DPHY_ANA2_EN_P2S_CPU_MASK,
    SUN6I_DPHY_ANA2_EN_P2S_CPU(lanes_mask));
    regmap_write(dphy.regs, SUN6I_DPHY_GCTL_REG,
    SUN6I_DPHY_GCTL_LANE_NUM(dphy.config.lanes) |
    SUN6I_DPHY_GCTL_EN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun6i_dphy_rx_power_on(dphy: *mut sun6i_dphy) -> c_int {
    static int sun6i_dphy_rx_power_on(struct sun6i_dphy *dphy)
    {
// Physical clock rate is actually half of symbol rate with DDR.
    let mut mipi_symbol_rate: c_ulong = dphy.config.hs_clk_rate;
    unsigned long dphy_clk_rate;
    unsigned int rx_dly;
    unsigned int lprst_dly;
    u32 value;
    dphy_clk_rate = clk_get_rate(dphy.mod_clk);
    if (!dphy_clk_rate)
    return -EINVAL;
// Hardcoded timing parameters from the Allwinner BSP.
    regmap_write(dphy.regs, SUN6I_DPHY_RX_TIME0_REG,
    SUN6I_DPHY_RX_TIME0_HS_RX_SYNC(255) |
    SUN6I_DPHY_RX_TIME0_HS_RX_CLK_MISS(255) |
    SUN6I_DPHY_RX_TIME0_LP_RX(255));
//
// Formula from the Allwinner BSP, with hardcoded coefficients
// (probably internal divider/multiplier).
//
    rx_dly = 8 * (unsigned int)(dphy_clk_rate / (mipi_symbol_rate / 8));
//
// The Allwinner BSP has an alternative formula for LP_RX_ULPS_WP:
// lp_ulps_wp_cnt = lp_ulps_wp_ms * lp_clk / 1000
// but does not use it and hardcodes 255 instead.
//
    regmap_write(dphy.regs, SUN6I_DPHY_RX_TIME1_REG,
    SUN6I_DPHY_RX_TIME1_RX_DLY(rx_dly) |
    SUN6I_DPHY_RX_TIME1_LP_RX_ULPS_WP(255));
// HS_RX_ANA0 value is hardcoded in the Allwinner BSP.
    regmap_write(dphy.regs, SUN6I_DPHY_RX_TIME2_REG,
    SUN6I_DPHY_RX_TIME2_HS_RX_ANA0(4));
//
// Formula from the Allwinner BSP, with hardcoded coefficients
// (probably internal divider/multiplier).
//
    lprst_dly = 4 * (unsigned int)(dphy_clk_rate / (mipi_symbol_rate / 2));
    regmap_write(dphy.regs, SUN6I_DPHY_RX_TIME3_REG,
    SUN6I_DPHY_RX_TIME3_LPRST_DLY(lprst_dly));
// Analog parameters are hardcoded in the Allwinner BSP.
    regmap_write(dphy.regs, SUN6I_DPHY_ANA0_REG,
    SUN6I_DPHY_ANA0_REG_PWS |
    SUN6I_DPHY_ANA0_REG_SLV(7) |
    SUN6I_DPHY_ANA0_REG_SFB(2));
    regmap_write(dphy.regs, SUN6I_DPHY_ANA1_REG,
    SUN6I_DPHY_ANA1_REG_SVTT(4));
    regmap_write(dphy.regs, SUN6I_DPHY_ANA4_REG,
    SUN6I_DPHY_ANA4_REG_DMPLVC |
    SUN6I_DPHY_ANA4_REG_DMPLVD(1));
    regmap_write(dphy.regs, SUN6I_DPHY_ANA2_REG,
    SUN6I_DPHY_ANA2_REG_ENIB);
    regmap_write(dphy.regs, SUN6I_DPHY_ANA3_REG,
    SUN6I_DPHY_ANA3_EN_LDOR |
    SUN6I_DPHY_ANA3_EN_LDOC |
    SUN6I_DPHY_ANA3_EN_LDOD);
//
// Delay comes from the Allwinner BSP, likely for internal regulator
// ramp-up.
//
    udelay(3);
    value = SUN6I_DPHY_RX_CTL_EN_DBC | SUN6I_DPHY_RX_CTL_RX_CLK_FORCE;
//
// Rx data lane force-enable bits are used as regular RX enable by the
// Allwinner BSP.
//
    if (dphy.config.lanes >= 1)
    value |= SUN6I_DPHY_RX_CTL_RX_D0_FORCE;
    if (dphy.config.lanes >= 2)
    value |= SUN6I_DPHY_RX_CTL_RX_D1_FORCE;
    if (dphy.config.lanes >= 3)
    value |= SUN6I_DPHY_RX_CTL_RX_D2_FORCE;
    if (dphy.config.lanes == 4)
    value |= SUN6I_DPHY_RX_CTL_RX_D3_FORCE;
    regmap_write(dphy.regs, SUN6I_DPHY_RX_CTL_REG, value);
    regmap_write(dphy.regs, SUN6I_DPHY_GCTL_REG,
    SUN6I_DPHY_GCTL_LANE_NUM(dphy.config.lanes) |
    SUN6I_DPHY_GCTL_EN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun6i_dphy_power_on(phy: *mut phy) -> c_int {
    static int sun6i_dphy_power_on(struct phy *phy)
    {
    struct sun6i_dphy *dphy = phy_get_drvdata(phy);
    switch (dphy.direction) {
    case SUN6I_DPHY_DIRECTION_TX:
    return sun6i_dphy_tx_power_on(dphy);
    case SUN6I_DPHY_DIRECTION_RX:
    return sun6i_dphy_rx_power_on(dphy);
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn sun6i_dphy_power_off(phy: *mut phy) -> c_int {
    static int sun6i_dphy_power_off(struct phy *phy)
    {
    struct sun6i_dphy *dphy = phy_get_drvdata(phy);
    regmap_write(dphy.regs, SUN6I_DPHY_GCTL_REG, 0);
    regmap_write(dphy.regs, SUN6I_DPHY_ANA0_REG, 0);
    regmap_write(dphy.regs, SUN6I_DPHY_ANA1_REG, 0);
    regmap_write(dphy.regs, SUN6I_DPHY_ANA2_REG, 0);
    regmap_write(dphy.regs, SUN6I_DPHY_ANA3_REG, 0);
    regmap_write(dphy.regs, SUN6I_DPHY_ANA4_REG, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun6i_dphy_exit(phy: *mut phy) -> c_int {
    static int sun6i_dphy_exit(struct phy *phy)
    {
    struct sun6i_dphy *dphy = phy_get_drvdata(phy);
    clk_rate_exclusive_put(dphy.mod_clk);
    clk_disable_unprepare(dphy.mod_clk);
    reset_control_assert(dphy.reset);
    return 0;
    }
    static const struct phy_ops sun6i_dphy_ops = {
    .configure	= sun6i_dphy_configure,
    .power_on	= sun6i_dphy_power_on,
    .power_off	= sun6i_dphy_power_off,
    .init		= sun6i_dphy_init,
    .exit		= sun6i_dphy_exit,
    };
    static const struct regmap_config sun6i_dphy_regmap_config = {
    .reg_bits	= 32,
    .val_bits	= 32,
    .reg_stride	= 4,
    .max_register	= SUN50I_COMBO_PHY_REG2,
    .name		= "mipi-dphy",
    };
#[no_mangle]
unsafe extern "C" fn sun6i_dphy_probe(pdev: *mut platform_device) -> c_int {
    static int sun6i_dphy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct sun6i_dphy *dphy;
    const char *direction;
    void __iomem *regs;
    int ret;
    dphy = devm_kzalloc(&pdev.dev, sizeof(*dphy), GFP_KERNEL);
    if (!dphy)
    return -ENOMEM;
    dphy.variant = device_get_match_data(&pdev.dev);
    if (!dphy.variant)
    return -EINVAL;
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs)) {
    dev_err(&pdev.dev, "Couldn't map the DPHY encoder registers\n");
    return PTR_ERR(regs);
    }
    dphy.regs = devm_regmap_init_mmio_clk(&pdev.dev, "bus",
    regs, &sun6i_dphy_regmap_config);
    if (IS_ERR(dphy.regs)) {
    dev_err(&pdev.dev, "Couldn't create the DPHY encoder regmap\n");
    return PTR_ERR(dphy.regs);
    }
    dphy.reset = devm_reset_control_get_shared(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(dphy.reset)) {
    dev_err(&pdev.dev, "Couldn't get our reset line\n");
    return PTR_ERR(dphy.reset);
    }
    dphy.mod_clk = devm_clk_get(&pdev.dev, "mod");
    if (IS_ERR(dphy.mod_clk)) {
    dev_err(&pdev.dev, "Couldn't get the DPHY mod clock\n");
    return PTR_ERR(dphy.mod_clk);
    }
    dphy.phy = devm_phy_create(&pdev.dev, core::ptr::null_mut(), &sun6i_dphy_ops);
    if (IS_ERR(dphy.phy)) {
    dev_err(&pdev.dev, "failed to create PHY\n");
    return PTR_ERR(dphy.phy);
    }
    dphy.direction = SUN6I_DPHY_DIRECTION_TX;
    ret = of_property_read_string(pdev.dev.of_node, "allwinner,direction",
    &direction);
    if (!ret && !strncmp(direction, "rx", 2)) {
    if (!dphy.variant.rx_supported) {
    dev_err(&pdev.dev, "RX not supported on this variant\n");
    return -EOPNOTSUPP;
    }
    dphy.direction = SUN6I_DPHY_DIRECTION_RX;
    }
    phy_set_drvdata(dphy.phy, dphy);
    phy_provider = devm_of_phy_provider_register(&pdev.dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct sun6i_dphy_variant sun6i_a31_mipi_dphy_variant = {
    .tx_power_on	= sun6i_a31_mipi_dphy_tx_power_on,
    .rx_supported	= true,
    };
    static const struct sun6i_dphy_variant sun50i_a100_mipi_dphy_variant = {
    .tx_power_on	= sun50i_a100_mipi_dphy_tx_power_on,
    };
    static const struct of_device_id sun6i_dphy_of_table[] = {
    {
    .compatible	= "allwinner,sun6i-a31-mipi-dphy",
    .data		= &sun6i_a31_mipi_dphy_variant,
    },
    {
    .compatible	= "allwinner,sun50i-a100-mipi-dphy",
    .data		= &sun50i_a100_mipi_dphy_variant,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, sun6i_dphy_of_table);
    static struct platform_driver sun6i_dphy_platform_driver = {
    .probe		= sun6i_dphy_probe,
    .driver		= {
    .name		= "sun6i-mipi-dphy",
    .of_match_table	= sun6i_dphy_of_table,
    },
    };
    module_platform_driver(sun6i_dphy_platform_driver);
    MODULE_AUTHOR("Maxime Ripard <maxime.ripard@bootlin>");
    MODULE_DESCRIPTION("Allwinner A31 MIPI D-PHY Driver");
    MODULE_LICENSE("GPL");
