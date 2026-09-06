//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/microchip/lan966x_serdes_regs.h
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


// SPDX-License-Identifier: GPL-2.0-or-later

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lan966x_target {
    TARGET_HSIO = 32,
    NUM_TARGETS = 66
}

// HSIO:SD:SD_CFG

// Macro flag: #define HSIO_SD_CFG_PHY_RESET_SET(x)\
// Macro flag: #define HSIO_SD_CFG_PHY_RESET_GET(x)\

// Macro flag: #define HSIO_SD_CFG_TX_RESET_SET(x)\
// Macro flag: #define HSIO_SD_CFG_TX_RESET_GET(x)\

// Macro flag: #define HSIO_SD_CFG_TX_RATE_SET(x)\
// Macro flag: #define HSIO_SD_CFG_TX_RATE_GET(x)\

// Macro flag: #define HSIO_SD_CFG_TX_INVERT_SET(x)\
// Macro flag: #define HSIO_SD_CFG_TX_INVERT_GET(x)\

// Macro flag: #define HSIO_SD_CFG_TX_EN_SET(x)\
// Macro flag: #define HSIO_SD_CFG_TX_EN_GET(x)\

// Macro flag: #define HSIO_SD_CFG_TX_DATA_EN_SET(x)\
// Macro flag: #define HSIO_SD_CFG_TX_DATA_EN_GET(x)\

// Macro flag: #define HSIO_SD_CFG_TX_CM_EN_SET(x)\
// Macro flag: #define HSIO_SD_CFG_TX_CM_EN_GET(x)\

// Macro flag: #define HSIO_SD_CFG_LANE_10BIT_SEL_SET(x)\
// Macro flag: #define HSIO_SD_CFG_LANE_10BIT_SEL_GET(x)\

// Macro flag: #define HSIO_SD_CFG_RX_TERM_EN_SET(x)\
// Macro flag: #define HSIO_SD_CFG_RX_TERM_EN_GET(x)\

// Macro flag: #define HSIO_SD_CFG_RX_RESET_SET(x)\
// Macro flag: #define HSIO_SD_CFG_RX_RESET_GET(x)\

// Macro flag: #define HSIO_SD_CFG_RX_RATE_SET(x)\
// Macro flag: #define HSIO_SD_CFG_RX_RATE_GET(x)\

// Macro flag: #define HSIO_SD_CFG_RX_PLL_EN_SET(x)\
// Macro flag: #define HSIO_SD_CFG_RX_PLL_EN_GET(x)\

// Macro flag: #define HSIO_SD_CFG_RX_INVERT_SET(x)\
// Macro flag: #define HSIO_SD_CFG_RX_INVERT_GET(x)\

// Macro flag: #define HSIO_SD_CFG_RX_DATA_EN_SET(x)\
// Macro flag: #define HSIO_SD_CFG_RX_DATA_EN_GET(x)\

// Macro flag: #define HSIO_SD_CFG_LANE_LOOPBK_EN_SET(x)\
// Macro flag: #define HSIO_SD_CFG_LANE_LOOPBK_EN_GET(x)\
// HSIO:SD:MPLL_CFG

// Macro flag: #define HSIO_MPLL_CFG_REF_SSP_EN_SET(x)\
// Macro flag: #define HSIO_MPLL_CFG_REF_SSP_EN_GET(x)\

// Macro flag: #define HSIO_MPLL_CFG_REF_CLKDIV2_SET(x)\
// Macro flag: #define HSIO_MPLL_CFG_REF_CLKDIV2_GET(x)\

// Macro flag: #define HSIO_MPLL_CFG_MPLL_EN_SET(x)\
// Macro flag: #define HSIO_MPLL_CFG_MPLL_EN_GET(x)\

// Macro flag: #define HSIO_MPLL_CFG_MPLL_MULTIPLIER_SET(x)\
// Macro flag: #define HSIO_MPLL_CFG_MPLL_MULTIPLIER_GET(x)\
// HSIO:SD:SD_STAT

// Macro flag: #define HSIO_SD_STAT_MPLL_STATE_SET(x)\
// Macro flag: #define HSIO_SD_STAT_MPLL_STATE_GET(x)\

// Macro flag: #define HSIO_SD_STAT_TX_STATE_SET(x)\
// Macro flag: #define HSIO_SD_STAT_TX_STATE_GET(x)\

// Macro flag: #define HSIO_SD_STAT_TX_CM_STATE_SET(x)\
// Macro flag: #define HSIO_SD_STAT_TX_CM_STATE_GET(x)\

// Macro flag: #define HSIO_SD_STAT_RX_PLL_STATE_SET(x)\
// Macro flag: #define HSIO_SD_STAT_RX_PLL_STATE_GET(x)\
// HSIO:HW_CFGSTAT:HW_CFG

// Macro flag: #define HSIO_HW_CFG_RGMII_1_CFG_SET(x)\
// Macro flag: #define HSIO_HW_CFG_RGMII_1_CFG_GET(x)\

// Macro flag: #define HSIO_HW_CFG_RGMII_0_CFG_SET(x)\
// Macro flag: #define HSIO_HW_CFG_RGMII_0_CFG_GET(x)\

// Macro flag: #define HSIO_HW_CFG_RGMII_ENA_SET(x)\
// Macro flag: #define HSIO_HW_CFG_RGMII_ENA_GET(x)\

// Macro flag: #define HSIO_HW_CFG_SD6G_0_CFG_SET(x)\
// Macro flag: #define HSIO_HW_CFG_SD6G_0_CFG_GET(x)\

// Macro flag: #define HSIO_HW_CFG_SD6G_1_CFG_SET(x)\
// Macro flag: #define HSIO_HW_CFG_SD6G_1_CFG_GET(x)\

// Macro flag: #define HSIO_HW_CFG_GMII_ENA_SET(x)\
// Macro flag: #define HSIO_HW_CFG_GMII_ENA_GET(x)\

// Macro flag: #define HSIO_HW_CFG_QSGMII_ENA_SET(x)\
// Macro flag: #define HSIO_HW_CFG_QSGMII_ENA_GET(x)\
// HSIO:HW_CFGSTAT:RGMII_CFG

// Macro flag: #define HSIO_RGMII_CFG_TX_CLK_CFG_SET(x)\
// Macro flag: #define HSIO_RGMII_CFG_TX_CLK_CFG_GET(x)\

// Macro flag: #define HSIO_RGMII_CFG_RGMII_TX_RST_SET(x)\
// Macro flag: #define HSIO_RGMII_CFG_RGMII_TX_RST_GET(x)\

// Macro flag: #define HSIO_RGMII_CFG_RGMII_RX_RST_SET(x)\
// Macro flag: #define HSIO_RGMII_CFG_RGMII_RX_RST_GET(x)\
// HSIO:HW_CFGSTAT:DLL_CFG

// Macro flag: #define HSIO_DLL_CFG_DELAY_ENA_SET(x)\
// Macro flag: #define HSIO_DLL_CFG_DELAY_ENA_GET(x)\

// Macro flag: #define HSIO_DLL_CFG_DLL_ENA_SET(x)\
// Macro flag: #define HSIO_DLL_CFG_DLL_ENA_GET(x)\

// Macro flag: #define HSIO_DLL_CFG_DLL_RST_SET(x)\
// Macro flag: #define HSIO_DLL_CFG_DLL_RST_GET(x)\
