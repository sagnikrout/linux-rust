//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,qdu1000-ecpricc.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (c) 2022-2023, Qualcomm Innovation Center, Inc. All rights reserved.
//
// ECPRI_CC clocks
pub const ECPRI_CC_PLL0: c_int = 0;
pub const ECPRI_CC_PLL1: c_int = 1;
pub const ECPRI_CC_ECPRI_CG_CLK: c_int = 2;
pub const ECPRI_CC_ECPRI_CLK_SRC: c_int = 3;
pub const ECPRI_CC_ECPRI_DMA_CLK: c_int = 4;
pub const ECPRI_CC_ECPRI_DMA_CLK_SRC: c_int = 5;
pub const ECPRI_CC_ECPRI_DMA_NOC_CLK: c_int = 6;
pub const ECPRI_CC_ECPRI_FAST_CLK: c_int = 7;
pub const ECPRI_CC_ECPRI_FAST_CLK_SRC: c_int = 8;
pub const ECPRI_CC_ECPRI_FAST_DIV2_CLK: c_int = 9;
pub const ECPRI_CC_ECPRI_FAST_DIV2_CLK_SRC: c_int = 10;
pub const ECPRI_CC_ECPRI_FAST_DIV2_NOC_CLK: c_int = 11;
pub const ECPRI_CC_ECPRI_FR_CLK: c_int = 12;
pub const ECPRI_CC_ECPRI_ORAN_CLK_SRC: c_int = 13;
pub const ECPRI_CC_ECPRI_ORAN_DIV2_CLK: c_int = 14;
pub const ECPRI_CC_ETH_100G_C2C0_HM_FF_CLK_SRC: c_int = 15;
pub const ECPRI_CC_ETH_100G_C2C0_UDP_FIFO_CLK: c_int = 16;
pub const ECPRI_CC_ETH_100G_C2C1_UDP_FIFO_CLK: c_int = 17;
pub const ECPRI_CC_ETH_100G_C2C_0_HM_FF_0_CLK: c_int = 18;
pub const ECPRI_CC_ETH_100G_C2C_0_HM_FF_1_CLK: c_int = 19;
pub const ECPRI_CC_ETH_100G_C2C_HM_FF_0_DIV_CLK_SRC: c_int = 20;
pub const ECPRI_CC_ETH_100G_C2C_HM_FF_1_DIV_CLK_SRC: c_int = 21;
pub const ECPRI_CC_ETH_100G_C2C_HM_MACSEC_CLK: c_int = 22;
pub const ECPRI_CC_ETH_100G_C2C_HM_MACSEC_CLK_SRC: c_int = 23;
pub const ECPRI_CC_ETH_100G_DBG_C2C_HM_FF_0_CLK: c_int = 24;
pub const ECPRI_CC_ETH_100G_DBG_C2C_HM_FF_0_DIV_CLK_SRC: c_int = 25;
pub const ECPRI_CC_ETH_100G_DBG_C2C_HM_FF_1_CLK: c_int = 26;
pub const ECPRI_CC_ETH_100G_DBG_C2C_HM_FF_1_DIV_CLK_SRC: c_int = 27;
pub const ECPRI_CC_ETH_100G_DBG_C2C_HM_FF_CLK_SRC: c_int = 28;
pub const ECPRI_CC_ETH_100G_DBG_C2C_UDP_FIFO_CLK: c_int = 29;
pub const ECPRI_CC_ETH_100G_FH0_HM_FF_CLK_SRC: c_int = 30;
pub const ECPRI_CC_ETH_100G_FH0_MACSEC_CLK_SRC: c_int = 31;
pub const ECPRI_CC_ETH_100G_FH1_HM_FF_CLK_SRC: c_int = 32;
pub const ECPRI_CC_ETH_100G_FH1_MACSEC_CLK_SRC: c_int = 33;
pub const ECPRI_CC_ETH_100G_FH2_HM_FF_CLK_SRC: c_int = 34;
pub const ECPRI_CC_ETH_100G_FH2_MACSEC_CLK_SRC: c_int = 35;
pub const ECPRI_CC_ETH_100G_FH_0_HM_FF_0_CLK: c_int = 36;
pub const ECPRI_CC_ETH_100G_FH_0_HM_FF_0_DIV_CLK_SRC: c_int = 37;
pub const ECPRI_CC_ETH_100G_FH_0_HM_FF_1_CLK: c_int = 38;
pub const ECPRI_CC_ETH_100G_FH_0_HM_FF_1_DIV_CLK_SRC: c_int = 39;
pub const ECPRI_CC_ETH_100G_FH_0_HM_FF_2_CLK: c_int = 40;
pub const ECPRI_CC_ETH_100G_FH_0_HM_FF_2_DIV_CLK_SRC: c_int = 41;
pub const ECPRI_CC_ETH_100G_FH_0_HM_FF_3_CLK: c_int = 42;
pub const ECPRI_CC_ETH_100G_FH_0_HM_FF_3_DIV_CLK_SRC: c_int = 43;
pub const ECPRI_CC_ETH_100G_FH_0_UDP_FIFO_CLK: c_int = 44;
pub const ECPRI_CC_ETH_100G_FH_1_HM_FF_0_CLK: c_int = 45;
pub const ECPRI_CC_ETH_100G_FH_1_HM_FF_0_DIV_CLK_SRC: c_int = 46;
pub const ECPRI_CC_ETH_100G_FH_1_HM_FF_1_CLK: c_int = 47;
pub const ECPRI_CC_ETH_100G_FH_1_HM_FF_1_DIV_CLK_SRC: c_int = 48;
pub const ECPRI_CC_ETH_100G_FH_1_HM_FF_2_CLK: c_int = 49;
pub const ECPRI_CC_ETH_100G_FH_1_HM_FF_2_DIV_CLK_SRC: c_int = 50;
pub const ECPRI_CC_ETH_100G_FH_1_HM_FF_3_CLK: c_int = 51;
pub const ECPRI_CC_ETH_100G_FH_1_HM_FF_3_DIV_CLK_SRC: c_int = 52;
pub const ECPRI_CC_ETH_100G_FH_1_UDP_FIFO_CLK: c_int = 53;
pub const ECPRI_CC_ETH_100G_FH_2_HM_FF_0_CLK: c_int = 54;
pub const ECPRI_CC_ETH_100G_FH_2_HM_FF_0_DIV_CLK_SRC: c_int = 55;
pub const ECPRI_CC_ETH_100G_FH_2_HM_FF_1_CLK: c_int = 56;
pub const ECPRI_CC_ETH_100G_FH_2_HM_FF_1_DIV_CLK_SRC: c_int = 57;
pub const ECPRI_CC_ETH_100G_FH_2_HM_FF_2_CLK: c_int = 58;
pub const ECPRI_CC_ETH_100G_FH_2_HM_FF_2_DIV_CLK_SRC: c_int = 59;
pub const ECPRI_CC_ETH_100G_FH_2_HM_FF_3_CLK: c_int = 60;
pub const ECPRI_CC_ETH_100G_FH_2_HM_FF_3_DIV_CLK_SRC: c_int = 61;
pub const ECPRI_CC_ETH_100G_FH_2_UDP_FIFO_CLK: c_int = 62;
pub const ECPRI_CC_ETH_100G_FH_MACSEC_0_CLK: c_int = 63;
pub const ECPRI_CC_ETH_100G_FH_MACSEC_1_CLK: c_int = 64;
pub const ECPRI_CC_ETH_100G_FH_MACSEC_2_CLK: c_int = 65;
pub const ECPRI_CC_ETH_100G_MAC_C2C_HM_REF_CLK: c_int = 66;
pub const ECPRI_CC_ETH_100G_MAC_C2C_HM_REF_CLK_SRC: c_int = 67;
pub const ECPRI_CC_ETH_100G_MAC_DBG_C2C_HM_REF_CLK: c_int = 68;
pub const ECPRI_CC_ETH_100G_MAC_DBG_C2C_HM_REF_CLK_SRC: c_int = 69;
pub const ECPRI_CC_ETH_100G_MAC_FH0_HM_REF_CLK: c_int = 70;
pub const ECPRI_CC_ETH_100G_MAC_FH0_HM_REF_CLK_SRC: c_int = 71;
pub const ECPRI_CC_ETH_100G_MAC_FH1_HM_REF_CLK: c_int = 72;
pub const ECPRI_CC_ETH_100G_MAC_FH1_HM_REF_CLK_SRC: c_int = 73;
pub const ECPRI_CC_ETH_100G_MAC_FH2_HM_REF_CLK: c_int = 74;
pub const ECPRI_CC_ETH_100G_MAC_FH2_HM_REF_CLK_SRC: c_int = 75;
pub const ECPRI_CC_ETH_DBG_NFAPI_AXI_CLK: c_int = 76;
pub const ECPRI_CC_ETH_DBG_NOC_AXI_CLK: c_int = 77;
pub const ECPRI_CC_ETH_PHY_0_OCK_SRAM_CLK: c_int = 78;
pub const ECPRI_CC_ETH_PHY_1_OCK_SRAM_CLK: c_int = 79;
pub const ECPRI_CC_ETH_PHY_2_OCK_SRAM_CLK: c_int = 80;
pub const ECPRI_CC_ETH_PHY_3_OCK_SRAM_CLK: c_int = 81;
pub const ECPRI_CC_ETH_PHY_4_OCK_SRAM_CLK: c_int = 82;
pub const ECPRI_CC_MSS_EMAC_CLK: c_int = 83;
pub const ECPRI_CC_MSS_EMAC_CLK_SRC: c_int = 84;
pub const ECPRI_CC_MSS_ORAN_CLK: c_int = 85;
pub const ECPRI_CC_PHY0_LANE0_RX_CLK: c_int = 86;
pub const ECPRI_CC_PHY0_LANE0_TX_CLK: c_int = 87;
pub const ECPRI_CC_PHY0_LANE1_RX_CLK: c_int = 88;
pub const ECPRI_CC_PHY0_LANE1_TX_CLK: c_int = 89;
pub const ECPRI_CC_PHY0_LANE2_RX_CLK: c_int = 90;
pub const ECPRI_CC_PHY0_LANE2_TX_CLK: c_int = 91;
pub const ECPRI_CC_PHY0_LANE3_RX_CLK: c_int = 92;
pub const ECPRI_CC_PHY0_LANE3_TX_CLK: c_int = 93;
pub const ECPRI_CC_PHY1_LANE0_RX_CLK: c_int = 94;
pub const ECPRI_CC_PHY1_LANE0_TX_CLK: c_int = 95;
pub const ECPRI_CC_PHY1_LANE1_RX_CLK: c_int = 96;
pub const ECPRI_CC_PHY1_LANE1_TX_CLK: c_int = 97;
pub const ECPRI_CC_PHY1_LANE2_RX_CLK: c_int = 98;
pub const ECPRI_CC_PHY1_LANE2_TX_CLK: c_int = 99;
pub const ECPRI_CC_PHY1_LANE3_RX_CLK: c_int = 100;
pub const ECPRI_CC_PHY1_LANE3_TX_CLK: c_int = 101;
pub const ECPRI_CC_PHY2_LANE0_RX_CLK: c_int = 102;
pub const ECPRI_CC_PHY2_LANE0_TX_CLK: c_int = 103;
pub const ECPRI_CC_PHY2_LANE1_RX_CLK: c_int = 104;
pub const ECPRI_CC_PHY2_LANE1_TX_CLK: c_int = 105;
pub const ECPRI_CC_PHY2_LANE2_RX_CLK: c_int = 106;
pub const ECPRI_CC_PHY2_LANE2_TX_CLK: c_int = 107;
pub const ECPRI_CC_PHY2_LANE3_RX_CLK: c_int = 108;
pub const ECPRI_CC_PHY2_LANE3_TX_CLK: c_int = 109;
pub const ECPRI_CC_PHY3_LANE0_RX_CLK: c_int = 110;
pub const ECPRI_CC_PHY3_LANE0_TX_CLK: c_int = 111;
pub const ECPRI_CC_PHY3_LANE1_RX_CLK: c_int = 112;
pub const ECPRI_CC_PHY3_LANE1_TX_CLK: c_int = 113;
pub const ECPRI_CC_PHY3_LANE2_RX_CLK: c_int = 114;
pub const ECPRI_CC_PHY3_LANE2_TX_CLK: c_int = 115;
pub const ECPRI_CC_PHY3_LANE3_RX_CLK: c_int = 116;
pub const ECPRI_CC_PHY3_LANE3_TX_CLK: c_int = 117;
pub const ECPRI_CC_PHY4_LANE0_RX_CLK: c_int = 118;
pub const ECPRI_CC_PHY4_LANE0_TX_CLK: c_int = 119;
pub const ECPRI_CC_PHY4_LANE1_RX_CLK: c_int = 120;
pub const ECPRI_CC_PHY4_LANE1_TX_CLK: c_int = 121;
pub const ECPRI_CC_PHY4_LANE2_RX_CLK: c_int = 122;
pub const ECPRI_CC_PHY4_LANE2_TX_CLK: c_int = 123;
pub const ECPRI_CC_PHY4_LANE3_RX_CLK: c_int = 124;
pub const ECPRI_CC_PHY4_LANE3_TX_CLK: c_int = 125;
// ECPRI_CC resets
pub const ECPRI_CC_CLK_CTL_TOP_ECPRI_CC_ECPRI_SS_BCR: c_int = 0;
pub const ECPRI_CC_CLK_CTL_TOP_ECPRI_CC_ETH_C2C_BCR: c_int = 1;
pub const ECPRI_CC_CLK_CTL_TOP_ECPRI_CC_ETH_FH0_BCR: c_int = 2;
pub const ECPRI_CC_CLK_CTL_TOP_ECPRI_CC_ETH_FH1_BCR: c_int = 3;
pub const ECPRI_CC_CLK_CTL_TOP_ECPRI_CC_ETH_FH2_BCR: c_int = 4;
pub const ECPRI_CC_CLK_CTL_TOP_ECPRI_CC_ETH_WRAPPER_TOP_BCR: c_int = 5;
pub const ECPRI_CC_CLK_CTL_TOP_ECPRI_CC_MODEM_BCR: c_int = 6;
pub const ECPRI_CC_CLK_CTL_TOP_ECPRI_CC_NOC_BCR: c_int = 7;
