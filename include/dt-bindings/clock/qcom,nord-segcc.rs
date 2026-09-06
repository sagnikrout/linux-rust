//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,nord-segcc.h
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// SE_GCC clocks
pub const SE_GCC_EEE_EMAC0_CLK: c_int = 0;
pub const SE_GCC_EEE_EMAC0_CLK_SRC: c_int = 1;
pub const SE_GCC_EEE_EMAC1_CLK: c_int = 2;
pub const SE_GCC_EEE_EMAC1_CLK_SRC: c_int = 3;
pub const SE_GCC_EMAC0_AXI_CLK: c_int = 4;
pub const SE_GCC_EMAC0_CC_SGMIIPHY_RX_CLK: c_int = 5;
pub const SE_GCC_EMAC0_CC_SGMIIPHY_TX_CLK: c_int = 6;
pub const SE_GCC_EMAC0_PHY_AUX_CLK: c_int = 7;
pub const SE_GCC_EMAC0_PHY_AUX_CLK_SRC: c_int = 8;
pub const SE_GCC_EMAC0_PTP_CLK: c_int = 9;
pub const SE_GCC_EMAC0_PTP_CLK_SRC: c_int = 10;
pub const SE_GCC_EMAC0_RGMII_CLK: c_int = 11;
pub const SE_GCC_EMAC0_RGMII_CLK_SRC: c_int = 12;
pub const SE_GCC_EMAC0_RPCS_RX_CLK: c_int = 13;
pub const SE_GCC_EMAC0_RPCS_TX_CLK: c_int = 14;
pub const SE_GCC_EMAC0_XGXS_RX_CLK: c_int = 15;
pub const SE_GCC_EMAC0_XGXS_TX_CLK: c_int = 16;
pub const SE_GCC_EMAC1_AXI_CLK: c_int = 17;
pub const SE_GCC_EMAC1_CC_SGMIIPHY_RX_CLK: c_int = 18;
pub const SE_GCC_EMAC1_CC_SGMIIPHY_TX_CLK: c_int = 19;
pub const SE_GCC_EMAC1_PHY_AUX_CLK: c_int = 20;
pub const SE_GCC_EMAC1_PHY_AUX_CLK_SRC: c_int = 21;
pub const SE_GCC_EMAC1_PTP_CLK: c_int = 22;
pub const SE_GCC_EMAC1_PTP_CLK_SRC: c_int = 23;
pub const SE_GCC_EMAC1_RGMII_CLK: c_int = 24;
pub const SE_GCC_EMAC1_RGMII_CLK_SRC: c_int = 25;
pub const SE_GCC_EMAC1_RPCS_RX_CLK: c_int = 26;
pub const SE_GCC_EMAC1_RPCS_TX_CLK: c_int = 27;
pub const SE_GCC_EMAC1_XGXS_RX_CLK: c_int = 28;
pub const SE_GCC_EMAC1_XGXS_TX_CLK: c_int = 29;
pub const SE_GCC_FRQ_MEASURE_REF_CLK: c_int = 30;
pub const SE_GCC_GP1_CLK: c_int = 31;
pub const SE_GCC_GP1_CLK_SRC: c_int = 32;
pub const SE_GCC_GP2_CLK: c_int = 33;
pub const SE_GCC_GP2_CLK_SRC: c_int = 34;
pub const SE_GCC_GPLL0: c_int = 35;
pub const SE_GCC_GPLL0_OUT_EVEN: c_int = 36;
pub const SE_GCC_GPLL2: c_int = 37;
pub const SE_GCC_GPLL4: c_int = 38;
pub const SE_GCC_GPLL5: c_int = 39;
pub const SE_GCC_MMU_2_TCU_VOTE_CLK: c_int = 40;
pub const SE_GCC_QUPV3_WRAP0_CORE_2X_CLK: c_int = 41;
pub const SE_GCC_QUPV3_WRAP0_CORE_CLK: c_int = 42;
pub const SE_GCC_QUPV3_WRAP0_M_AHB_CLK: c_int = 43;
pub const SE_GCC_QUPV3_WRAP0_S0_CLK: c_int = 44;
pub const SE_GCC_QUPV3_WRAP0_S0_CLK_SRC: c_int = 45;
pub const SE_GCC_QUPV3_WRAP0_S1_CLK: c_int = 46;
pub const SE_GCC_QUPV3_WRAP0_S1_CLK_SRC: c_int = 47;
pub const SE_GCC_QUPV3_WRAP0_S2_CLK: c_int = 48;
pub const SE_GCC_QUPV3_WRAP0_S2_CLK_SRC: c_int = 49;
pub const SE_GCC_QUPV3_WRAP0_S3_CLK: c_int = 50;
pub const SE_GCC_QUPV3_WRAP0_S3_CLK_SRC: c_int = 51;
pub const SE_GCC_QUPV3_WRAP0_S4_CLK: c_int = 52;
pub const SE_GCC_QUPV3_WRAP0_S4_CLK_SRC: c_int = 53;
pub const SE_GCC_QUPV3_WRAP0_S5_CLK: c_int = 54;
pub const SE_GCC_QUPV3_WRAP0_S5_CLK_SRC: c_int = 55;
pub const SE_GCC_QUPV3_WRAP0_S6_CLK: c_int = 56;
pub const SE_GCC_QUPV3_WRAP0_S6_CLK_SRC: c_int = 57;
pub const SE_GCC_QUPV3_WRAP0_S_AHB_CLK: c_int = 58;
pub const SE_GCC_QUPV3_WRAP1_CORE_2X_CLK: c_int = 59;
pub const SE_GCC_QUPV3_WRAP1_CORE_CLK: c_int = 60;
pub const SE_GCC_QUPV3_WRAP1_M_AHB_CLK: c_int = 61;
pub const SE_GCC_QUPV3_WRAP1_S0_CLK: c_int = 62;
pub const SE_GCC_QUPV3_WRAP1_S0_CLK_SRC: c_int = 63;
pub const SE_GCC_QUPV3_WRAP1_S1_CLK: c_int = 64;
pub const SE_GCC_QUPV3_WRAP1_S1_CLK_SRC: c_int = 65;
pub const SE_GCC_QUPV3_WRAP1_S2_CLK: c_int = 66;
pub const SE_GCC_QUPV3_WRAP1_S2_CLK_SRC: c_int = 67;
pub const SE_GCC_QUPV3_WRAP1_S3_CLK: c_int = 68;
pub const SE_GCC_QUPV3_WRAP1_S3_CLK_SRC: c_int = 69;
pub const SE_GCC_QUPV3_WRAP1_S4_CLK: c_int = 70;
pub const SE_GCC_QUPV3_WRAP1_S4_CLK_SRC: c_int = 71;
pub const SE_GCC_QUPV3_WRAP1_S5_CLK: c_int = 72;
pub const SE_GCC_QUPV3_WRAP1_S5_CLK_SRC: c_int = 73;
pub const SE_GCC_QUPV3_WRAP1_S6_CLK: c_int = 74;
pub const SE_GCC_QUPV3_WRAP1_S6_CLK_SRC: c_int = 75;
pub const SE_GCC_QUPV3_WRAP1_S_AHB_CLK: c_int = 76;
// SE_GCC power domains
pub const SE_GCC_EMAC0_GDSC: c_int = 0;
pub const SE_GCC_EMAC1_GDSC: c_int = 1;
// SE_GCC resets
pub const SE_GCC_EMAC0_BCR: c_int = 0;
pub const SE_GCC_EMAC1_BCR: c_int = 1;
pub const SE_GCC_QUPV3_WRAPPER_0_BCR: c_int = 2;
pub const SE_GCC_QUPV3_WRAPPER_1_BCR: c_int = 3;
