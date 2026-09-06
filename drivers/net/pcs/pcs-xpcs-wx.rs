//! Automatically rewritten from C to Rust
//! Source: drivers/net/pcs/pcs-xpcs-wx.c
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
// Copyright (c) 2015 - 2023 Beijing WangXun Technology Co., Ltd.

// VR_XS_PMA_MMD
pub const TXGBE_PMA_MMD: c_uint = 0x8020;
pub const TXGBE_TX_GENCTL1: c_uint = 0x11;

pub const TXGBE_TX_GEN_CTL2: c_uint = 0x12;

pub const TXGBE_TX_RATE_CTL: c_uint = 0x14;

pub const TXGBE_RX_GEN_CTL2: c_uint = 0x32;

pub const TXGBE_RX_GEN_CTL3: c_uint = 0x33;

pub const TXGBE_RX_RATE_CTL: c_uint = 0x34;

pub const TXGBE_RX_EQ_ATTN_CTL: c_uint = 0x37;

pub const TXGBE_RX_EQ_CTL0: c_uint = 0x38;

pub const TXGBE_RX_EQ_CTL4: c_uint = 0x3C;

pub const TXGBE_AFE_DFE_ENABLE: c_uint = 0x3D;

pub const TXGBE_DFE_TAP_CTL0: c_uint = 0x3E;
pub const TXGBE_MPLLA_CTL0: c_uint = 0x51;
pub const TXGBE_MPLLA_CTL2: c_uint = 0x53;

pub const TXGBE_MPLLA_CTL3: c_uint = 0x57;
pub const TXGBE_MISC_CTL0: c_uint = 0x70;

pub const TXGBE_VCO_CAL_LD0: c_uint = 0x72;
pub const TXGBE_VCO_CAL_REF0: c_uint = 0x76;
#[no_mangle]
unsafe extern "C" fn txgbe_write_pma(xpcs: *mut dw_xpcs, reg: c_int, val: u16) -> c_int {
    static int txgbe_write_pma(struct dw_xpcs *xpcs, int reg, u16 val)
    {
    return xpcs_write(xpcs, MDIO_MMD_PMAPMD, TXGBE_PMA_MMD + reg, val);
    }
#[no_mangle]
unsafe extern "C" fn txgbe_modify_pma(xpcs: *mut dw_xpcs, reg: c_int, mask: u16, set: u16) -> c_int {
    static int txgbe_modify_pma(struct dw_xpcs *xpcs, int reg, u16 mask, u16 set)
    {
    return xpcs_modify(xpcs, MDIO_MMD_PMAPMD, TXGBE_PMA_MMD + reg, mask,
    set);
    }
#[no_mangle]
unsafe extern "C" fn txgbe_pma_config_10gbaser(xpcs: *mut dw_xpcs) {
    static void txgbe_pma_config_10gbaser(struct dw_xpcs *xpcs)
    {
    txgbe_write_pma(xpcs, TXGBE_MPLLA_CTL0, 0x21);
    txgbe_write_pma(xpcs, TXGBE_MPLLA_CTL3, 0);
    txgbe_modify_pma(xpcs, TXGBE_TX_GENCTL1, TXGBE_TX_GENCTL1_VBOOST_LVL,
    FIELD_PREP(TXGBE_TX_GENCTL1_VBOOST_LVL, 0x5));
    txgbe_write_pma(xpcs, TXGBE_MISC_CTL0, TXGBE_MISC_CTL0_PLL |
    TXGBE_MISC_CTL0_CR_PARA_SEL | TXGBE_MISC_CTL0_RX_VREF(0xF));
    txgbe_write_pma(xpcs, TXGBE_VCO_CAL_LD0, 0x549);
    txgbe_write_pma(xpcs, TXGBE_VCO_CAL_REF0, 0x29);
    txgbe_write_pma(xpcs, TXGBE_TX_RATE_CTL, 0);
    txgbe_write_pma(xpcs, TXGBE_RX_RATE_CTL, 0);
    txgbe_write_pma(xpcs, TXGBE_TX_GEN_CTL2, TXGBE_TX_GEN_CTL2_TX0_WIDTH(3));
    txgbe_write_pma(xpcs, TXGBE_RX_GEN_CTL2, TXGBE_RX_GEN_CTL2_RX0_WIDTH(3));
    txgbe_write_pma(xpcs, TXGBE_MPLLA_CTL2, TXGBE_MPLLA_CTL2_DIV16P5_CLK_EN |
    TXGBE_MPLLA_CTL2_DIV10_CLK_EN);
    txgbe_write_pma(xpcs, TXGBE_RX_EQ_CTL0, TXGBE_RX_EQ_CTL0_CTLE_POLE(2) |
    TXGBE_RX_EQ_CTL0_CTLE_BOOST(5));
    txgbe_modify_pma(xpcs, TXGBE_RX_EQ_ATTN_CTL, TXGBE_RX_EQ_ATTN_LVL0, 0);
    txgbe_write_pma(xpcs, TXGBE_DFE_TAP_CTL0, 0xBE);
    txgbe_modify_pma(xpcs, TXGBE_AFE_DFE_ENABLE,
    TXGBE_DFE_EN_0 | TXGBE_AFE_EN_0, 0);
    txgbe_modify_pma(xpcs, TXGBE_RX_EQ_CTL4, TXGBE_RX_EQ_CTL4_CONT_ADAPT0,
    0);
    }
#[no_mangle]
unsafe extern "C" fn txgbe_pma_config_1g(xpcs: *mut dw_xpcs) {
    static void txgbe_pma_config_1g(struct dw_xpcs *xpcs)
    {
    txgbe_modify_pma(xpcs, TXGBE_TX_GENCTL1,
    TXGBE_TX_GENCTL1_VBOOST_LVL |
    TXGBE_TX_GENCTL1_VBOOST_EN0,
    FIELD_PREP(TXGBE_TX_GENCTL1_VBOOST_LVL, 0x5));
    txgbe_write_pma(xpcs, TXGBE_MISC_CTL0, TXGBE_MISC_CTL0_PLL |
    TXGBE_MISC_CTL0_CR_PARA_SEL | TXGBE_MISC_CTL0_RX_VREF(0xF));
    txgbe_write_pma(xpcs, TXGBE_RX_EQ_CTL0, TXGBE_RX_EQ_CTL0_VGA1_GAIN(7) |
    TXGBE_RX_EQ_CTL0_VGA2_GAIN(7) | TXGBE_RX_EQ_CTL0_CTLE_BOOST(6));
    txgbe_modify_pma(xpcs, TXGBE_RX_EQ_ATTN_CTL, TXGBE_RX_EQ_ATTN_LVL0, 0);
    txgbe_write_pma(xpcs, TXGBE_DFE_TAP_CTL0, 0);
    txgbe_modify_pma(xpcs, TXGBE_RX_GEN_CTL3, TXGBE_RX_GEN_CTL3_LOS_TRSHLD0,
    FIELD_PREP(TXGBE_RX_GEN_CTL3_LOS_TRSHLD0, 0x4));
    txgbe_write_pma(xpcs, TXGBE_MPLLA_CTL0, 0x20);
    txgbe_write_pma(xpcs, TXGBE_MPLLA_CTL3, 0x46);
    txgbe_write_pma(xpcs, TXGBE_VCO_CAL_LD0, 0x540);
    txgbe_write_pma(xpcs, TXGBE_VCO_CAL_REF0, 0x2A);
    txgbe_write_pma(xpcs, TXGBE_AFE_DFE_ENABLE, 0);
    txgbe_write_pma(xpcs, TXGBE_RX_EQ_CTL4, TXGBE_RX_EQ_CTL4_CONT_OFF_CAN0);
    txgbe_write_pma(xpcs, TXGBE_TX_RATE_CTL, TXGBE_TX_RATE_CTL_TX0_RATE(3));
    txgbe_write_pma(xpcs, TXGBE_RX_RATE_CTL, TXGBE_RX_RATE_CTL_RX0_RATE(3));
    txgbe_write_pma(xpcs, TXGBE_TX_GEN_CTL2, TXGBE_TX_GEN_CTL2_TX0_WIDTH(1));
    txgbe_write_pma(xpcs, TXGBE_RX_GEN_CTL2, TXGBE_RX_GEN_CTL2_RX0_WIDTH(1));
    txgbe_write_pma(xpcs, TXGBE_MPLLA_CTL2, TXGBE_MPLLA_CTL2_DIV10_CLK_EN);
    }
#[no_mangle]
unsafe extern "C" fn txgbe_pcs_poll_power_up(xpcs: *mut dw_xpcs) -> c_int {
    static int txgbe_pcs_poll_power_up(struct dw_xpcs *xpcs)
    {
    int val, ret;
// Wait xpcs power-up good
    ret = read_poll_timeout(xpcs_read_vpcs, val,
    (val & DW_PSEQ_ST) == DW_PSEQ_ST_GOOD,
    10000, 1000000, false,
    xpcs, DW_VR_XS_PCS_DIG_STS);
    if (ret < 0)
    dev_err(&xpcs.mdiodev.dev, "xpcs power-up timeout\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn txgbe_pma_init_done(xpcs: *mut dw_xpcs) -> c_int {
    static int txgbe_pma_init_done(struct dw_xpcs *xpcs)
    {
    int val, ret;
    xpcs_write_vpcs(xpcs, DW_VR_XS_PCS_DIG_CTRL1, DW_VR_RST | DW_EN_VSMMD1);
// wait pma initialization done
    ret = read_poll_timeout(xpcs_read_vpcs, val, !(val & DW_VR_RST),
    100000, 10000000, false,
    xpcs, DW_VR_XS_PCS_DIG_CTRL1);
    if (ret < 0)
    dev_err(&xpcs.mdiodev.dev, "xpcs pma initialization timeout\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn txgbe_xpcs_mode_quirk(xpcs: *mut dw_xpcs) -> bool {
    static bool txgbe_xpcs_mode_quirk(struct dw_xpcs *xpcs)
    {
    int ret;
// When txgbe do LAN reset, PCS will change to default 10GBASE-R mode
    ret = xpcs_read(xpcs, MDIO_MMD_PCS, MDIO_CTRL2);
    ret &= MDIO_PCS_CTRL2_TYPE;
    if ((ret == MDIO_PCS_CTRL2_10GBR &&
    xpcs.interface != PHY_INTERFACE_MODE_10GBASER) ||
    xpcs.interface == PHY_INTERFACE_MODE_SGMII)
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn txgbe_xpcs_switch_mode(xpcs: *mut dw_xpcs, interface: phy_interface_t) -> c_int {
    int txgbe_xpcs_switch_mode(struct dw_xpcs *xpcs, phy_interface_t interface)
    {
    int ret;
    switch (interface) {
    case PHY_INTERFACE_MODE_10GBASER:
    case PHY_INTERFACE_MODE_SGMII:
    case PHY_INTERFACE_MODE_1000BASEX:
    break;
    default:
    return 0;
    }
    if (xpcs.interface == interface && !txgbe_xpcs_mode_quirk(xpcs))
    return 0;
    xpcs.interface = interface;
    ret = txgbe_pcs_poll_power_up(xpcs);
    if (ret < 0)
    return ret;
    if (interface == PHY_INTERFACE_MODE_10GBASER) {
    xpcs_write(xpcs, MDIO_MMD_PCS, MDIO_CTRL2, MDIO_PCS_CTRL2_10GBR);
    xpcs_modify(xpcs, MDIO_MMD_PMAPMD, MDIO_CTRL1,
    MDIO_CTRL1_SPEED10G, MDIO_CTRL1_SPEED10G);
    txgbe_pma_config_10gbaser(xpcs);
    } else {
    xpcs_write(xpcs, MDIO_MMD_PCS, MDIO_CTRL2, MDIO_PCS_CTRL2_10GBX);
    xpcs_write(xpcs, MDIO_MMD_PMAPMD, MDIO_CTRL1, 0);
    xpcs_write(xpcs, MDIO_MMD_PCS, MDIO_CTRL1, 0);
    txgbe_pma_config_1g(xpcs);
    }
    return txgbe_pma_init_done(xpcs);
    }
