//! Automatically rewritten from C to Rust
//! Source: drivers/net/pcs/pcs-xpcs-nxp.c
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
// Copyright 2021 NXP
//

// LANE_DRIVER1_0 register
pub const SJA1110_LANE_DRIVER1_0: c_uint = 0x8038;

// LANE_DRIVER2_0 register
pub const SJA1110_LANE_DRIVER2_0: c_uint = 0x803a;

// LANE_DRIVER2_1 register
pub const SJA1110_LANE_DRIVER2_1: c_uint = 0x803b;

// LANE_TRIM register
pub const SJA1110_LANE_TRIM: c_uint = 0x8040;

// LANE_DATAPATH_1 register
pub const SJA1110_LANE_DATAPATH_1: c_uint = 0x8037;
// POWERDOWN_ENABLE register
pub const SJA1110_POWERDOWN_ENABLE: c_uint = 0x8041;

// RXPLL_CTRL0 register
pub const SJA1110_RXPLL_CTRL0: c_uint = 0x8065;

// RXPLL_CTRL1 register
pub const SJA1110_RXPLL_CTRL1: c_uint = 0x8066;

// TXPLL_CTRL0 register
pub const SJA1110_TXPLL_CTRL0: c_uint = 0x806d;

// TXPLL_CTRL1 register
pub const SJA1110_TXPLL_CTRL1: c_uint = 0x806e;

// RX_DATA_DETECT register
pub const SJA1110_RX_DATA_DETECT: c_uint = 0x8045;
// RX_CDR_CTLE register
pub const SJA1110_RX_CDR_CTLE: c_uint = 0x8042;
// In NXP SJA1105, the PCS is integrated with a PMA that has the TX lane
// polarity inverted by default (PLUS is MINUS, MINUS is PLUS). To obtain
// normal non-inverted behavior, the TX lane polarity must be inverted in the
// PCS, via the DIGITAL_CONTROL_2 register.
//
#[no_mangle]
pub unsafe extern "C" fn nxp_sja1105_sgmii_pma_config(xpcs: *mut dw_xpcs) -> c_int {
    int nxp_sja1105_sgmii_pma_config(struct dw_xpcs *xpcs)
    {
    return xpcs_write(xpcs, MDIO_MMD_VEND2, DW_VR_MII_DIG_CTRL2,
    DW_VR_MII_DIG_CTRL2_TX_POL_INV);
    }
    static int nxp_sja1110_pma_config(struct dw_xpcs *xpcs,
    u16 txpll_fbdiv, u16 txpll_refdiv,
    u16 rxpll_fbdiv, u16 rxpll_refdiv,
    u16 rx_cdr_ctle)
    {
    u16 val;
    int ret;
// Program TX PLL feedback divider and reference divider settings for
// correct oscillation frequency.
//
    ret = xpcs_write(xpcs, MDIO_MMD_VEND2, SJA1110_TXPLL_CTRL0,
    SJA1110_TXPLL_FBDIV(txpll_fbdiv));
    if (ret < 0)
    return ret;
    ret = xpcs_write(xpcs, MDIO_MMD_VEND2, SJA1110_TXPLL_CTRL1,
    SJA1110_TXPLL_REFDIV(txpll_refdiv));
    if (ret < 0)
    return ret;
// Program transmitter amplitude and disable amplitude trimming
    ret = xpcs_write(xpcs, MDIO_MMD_VEND2, SJA1110_LANE_DRIVER1_0,
    SJA1110_TXDRV(0x5));
    if (ret < 0)
    return ret;
    val = SJA1110_TXDRVTRIM_LSB(0xffffffull);
    ret = xpcs_write(xpcs, MDIO_MMD_VEND2, SJA1110_LANE_DRIVER2_0, val);
    if (ret < 0)
    return ret;
    val = SJA1110_TXDRVTRIM_MSB(0xffffffull) | SJA1110_LANE_DRIVER2_1_RSV;
    ret = xpcs_write(xpcs, MDIO_MMD_VEND2, SJA1110_LANE_DRIVER2_1, val);
    if (ret < 0)
    return ret;
// Enable input and output resistor terminations for low BER.
    val = SJA1110_ACCOUPLE_RXVCM_EN | SJA1110_CDR_GAIN |
    SJA1110_RXRTRIM(4) | SJA1110_RXTEN | SJA1110_TXPLL_BWSEL |
    SJA1110_TXRTRIM(3) | SJA1110_TXTEN;
    ret = xpcs_write(xpcs, MDIO_MMD_VEND2, SJA1110_LANE_TRIM, val);
    if (ret < 0)
    return ret;
// Select PCS as transmitter data source.
    ret = xpcs_write(xpcs, MDIO_MMD_VEND2, SJA1110_LANE_DATAPATH_1, 0);
    if (ret < 0)
    return ret;
// Program RX PLL feedback divider and reference divider for correct
// oscillation frequency.
//
    ret = xpcs_write(xpcs, MDIO_MMD_VEND2, SJA1110_RXPLL_CTRL0,
    SJA1110_RXPLL_FBDIV(rxpll_fbdiv));
    if (ret < 0)
    return ret;
    ret = xpcs_write(xpcs, MDIO_MMD_VEND2, SJA1110_RXPLL_CTRL1,
    SJA1110_RXPLL_REFDIV(rxpll_refdiv));
    if (ret < 0)
    return ret;
// Program threshold for receiver signal detector.
// Enable control of RXPLL by receiver signal detector to disable RXPLL
// when an input signal is not present.
//
    ret = xpcs_write(xpcs, MDIO_MMD_VEND2, SJA1110_RX_DATA_DETECT, 0x0005);
    if (ret < 0)
    return ret;
// Enable TX and RX PLLs and circuits.
// Release reset of PMA to enable data flow to/from PCS.
//
    ret = xpcs_modify(xpcs, MDIO_MMD_VEND2, SJA1110_POWERDOWN_ENABLE,
    SJA1110_TXPLL_PD | SJA1110_TXPD | SJA1110_RXCH_PD |
    SJA1110_RXBIAS_PD | SJA1110_RESET_SER_EN |
    SJA1110_RESET_SER | SJA1110_RESET_DES |
    SJA1110_RXPKDETEN | SJA1110_RCVEN,
    SJA1110_RXPKDETEN | SJA1110_RCVEN);
    if (ret < 0)
    return ret;
// Program continuous-time linear equalizer (CTLE) settings.
    return xpcs_write(xpcs, MDIO_MMD_VEND2, SJA1110_RX_CDR_CTLE,
    rx_cdr_ctle);
    }
#[no_mangle]
pub unsafe extern "C" fn nxp_sja1110_sgmii_pma_config(xpcs: *mut dw_xpcs) -> c_int {
    int nxp_sja1110_sgmii_pma_config(struct dw_xpcs *xpcs)
    {
    return nxp_sja1110_pma_config(xpcs, 0x19, 0x1, 0x19, 0x1, 0x212a);
    }
#[no_mangle]
pub unsafe extern "C" fn nxp_sja1110_2500basex_pma_config(xpcs: *mut dw_xpcs) -> c_int {
    int nxp_sja1110_2500basex_pma_config(struct dw_xpcs *xpcs)
    {
    return nxp_sja1110_pma_config(xpcs, 0x7d, 0x2, 0x7d, 0x2, 0x732a);
    }
