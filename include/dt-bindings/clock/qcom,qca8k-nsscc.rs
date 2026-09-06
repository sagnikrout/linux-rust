//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,qca8k-nsscc.h
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
// Copyright (c) 2023 Qualcomm Innovation Center, Inc. All rights reserved.
//
pub const NSS_CC_SWITCH_CORE_CLK_SRC: c_int = 0;
pub const NSS_CC_SWITCH_CORE_CLK: c_int = 1;
pub const NSS_CC_APB_BRIDGE_CLK: c_int = 2;
pub const NSS_CC_MAC0_TX_CLK_SRC: c_int = 3;
pub const NSS_CC_MAC0_TX_DIV_CLK_SRC: c_int = 4;
pub const NSS_CC_MAC0_TX_CLK: c_int = 5;
pub const NSS_CC_MAC0_TX_SRDS1_CLK: c_int = 6;
pub const NSS_CC_MAC0_RX_CLK_SRC: c_int = 7;
pub const NSS_CC_MAC0_RX_DIV_CLK_SRC: c_int = 8;
pub const NSS_CC_MAC0_RX_CLK: c_int = 9;
pub const NSS_CC_MAC0_RX_SRDS1_CLK: c_int = 10;
pub const NSS_CC_MAC1_TX_CLK_SRC: c_int = 11;
pub const NSS_CC_MAC1_TX_DIV_CLK_SRC: c_int = 12;
pub const NSS_CC_MAC1_SRDS1_CH0_XGMII_RX_DIV_CLK_SRC: c_int = 13;
pub const NSS_CC_MAC1_SRDS1_CH0_RX_CLK: c_int = 14;
pub const NSS_CC_MAC1_TX_CLK: c_int = 15;
pub const NSS_CC_MAC1_GEPHY0_TX_CLK: c_int = 16;
pub const NSS_CC_MAC1_SRDS1_CH0_XGMII_RX_CLK: c_int = 17;
pub const NSS_CC_MAC1_RX_CLK_SRC: c_int = 18;
pub const NSS_CC_MAC1_RX_DIV_CLK_SRC: c_int = 19;
pub const NSS_CC_MAC1_SRDS1_CH0_XGMII_TX_DIV_CLK_SRC: c_int = 20;
pub const NSS_CC_MAC1_SRDS1_CH0_TX_CLK: c_int = 21;
pub const NSS_CC_MAC1_RX_CLK: c_int = 22;
pub const NSS_CC_MAC1_GEPHY0_RX_CLK: c_int = 23;
pub const NSS_CC_MAC1_SRDS1_CH0_XGMII_TX_CLK: c_int = 24;
pub const NSS_CC_MAC2_TX_CLK_SRC: c_int = 25;
pub const NSS_CC_MAC2_TX_DIV_CLK_SRC: c_int = 26;
pub const NSS_CC_MAC2_SRDS1_CH1_XGMII_RX_DIV_CLK_SRC: c_int = 27;
pub const NSS_CC_MAC2_SRDS1_CH1_RX_CLK: c_int = 28;
pub const NSS_CC_MAC2_TX_CLK: c_int = 29;
pub const NSS_CC_MAC2_GEPHY1_TX_CLK: c_int = 30;
pub const NSS_CC_MAC2_SRDS1_CH1_XGMII_RX_CLK: c_int = 31;
pub const NSS_CC_MAC2_RX_CLK_SRC: c_int = 32;
pub const NSS_CC_MAC2_RX_DIV_CLK_SRC: c_int = 33;
pub const NSS_CC_MAC2_SRDS1_CH1_XGMII_TX_DIV_CLK_SRC: c_int = 34;
pub const NSS_CC_MAC2_SRDS1_CH1_TX_CLK: c_int = 35;
pub const NSS_CC_MAC2_RX_CLK: c_int = 36;
pub const NSS_CC_MAC2_GEPHY1_RX_CLK: c_int = 37;
pub const NSS_CC_MAC2_SRDS1_CH1_XGMII_TX_CLK: c_int = 38;
pub const NSS_CC_MAC3_TX_CLK_SRC: c_int = 39;
pub const NSS_CC_MAC3_TX_DIV_CLK_SRC: c_int = 40;
pub const NSS_CC_MAC3_SRDS1_CH2_XGMII_RX_DIV_CLK_SRC: c_int = 41;
pub const NSS_CC_MAC3_SRDS1_CH2_RX_CLK: c_int = 42;
pub const NSS_CC_MAC3_TX_CLK: c_int = 43;
pub const NSS_CC_MAC3_GEPHY2_TX_CLK: c_int = 44;
pub const NSS_CC_MAC3_SRDS1_CH2_XGMII_RX_CLK: c_int = 45;
pub const NSS_CC_MAC3_RX_CLK_SRC: c_int = 46;
pub const NSS_CC_MAC3_RX_DIV_CLK_SRC: c_int = 47;
pub const NSS_CC_MAC3_SRDS1_CH2_XGMII_TX_DIV_CLK_SRC: c_int = 48;
pub const NSS_CC_MAC3_SRDS1_CH2_TX_CLK: c_int = 49;
pub const NSS_CC_MAC3_RX_CLK: c_int = 50;
pub const NSS_CC_MAC3_GEPHY2_RX_CLK: c_int = 51;
pub const NSS_CC_MAC3_SRDS1_CH2_XGMII_TX_CLK: c_int = 52;
pub const NSS_CC_MAC4_TX_CLK_SRC: c_int = 53;
pub const NSS_CC_MAC4_TX_DIV_CLK_SRC: c_int = 54;
pub const NSS_CC_MAC4_SRDS1_CH3_XGMII_RX_DIV_CLK_SRC: c_int = 55;
pub const NSS_CC_MAC4_SRDS1_CH3_RX_CLK: c_int = 56;
pub const NSS_CC_MAC4_TX_CLK: c_int = 57;
pub const NSS_CC_MAC4_GEPHY3_TX_CLK: c_int = 58;
pub const NSS_CC_MAC4_SRDS1_CH3_XGMII_RX_CLK: c_int = 59;
pub const NSS_CC_MAC4_RX_CLK_SRC: c_int = 60;
pub const NSS_CC_MAC4_RX_DIV_CLK_SRC: c_int = 61;
pub const NSS_CC_MAC4_SRDS1_CH3_XGMII_TX_DIV_CLK_SRC: c_int = 62;
pub const NSS_CC_MAC4_SRDS1_CH3_TX_CLK: c_int = 63;
pub const NSS_CC_MAC4_RX_CLK: c_int = 64;
pub const NSS_CC_MAC4_GEPHY3_RX_CLK: c_int = 65;
pub const NSS_CC_MAC4_SRDS1_CH3_XGMII_TX_CLK: c_int = 66;
pub const NSS_CC_MAC5_TX_CLK_SRC: c_int = 67;
pub const NSS_CC_MAC5_TX_DIV_CLK_SRC: c_int = 68;
pub const NSS_CC_MAC5_TX_SRDS0_CLK: c_int = 69;
pub const NSS_CC_MAC5_TX_CLK: c_int = 70;
pub const NSS_CC_MAC5_RX_CLK_SRC: c_int = 71;
pub const NSS_CC_MAC5_RX_DIV_CLK_SRC: c_int = 72;
pub const NSS_CC_MAC5_RX_SRDS0_CLK: c_int = 73;
pub const NSS_CC_MAC5_RX_CLK: c_int = 74;
pub const NSS_CC_MAC5_TX_SRDS0_CLK_SRC: c_int = 75;
pub const NSS_CC_MAC5_RX_SRDS0_CLK_SRC: c_int = 76;
pub const NSS_CC_AHB_CLK_SRC: c_int = 77;
pub const NSS_CC_AHB_CLK: c_int = 78;
pub const NSS_CC_SEC_CTRL_AHB_CLK: c_int = 79;
pub const NSS_CC_TLMM_CLK: c_int = 80;
pub const NSS_CC_TLMM_AHB_CLK: c_int = 81;
pub const NSS_CC_CNOC_AHB_CLK: c_int = 82;
pub const NSS_CC_MDIO_AHB_CLK: c_int = 83;
pub const NSS_CC_MDIO_MASTER_AHB_CLK: c_int = 84;
pub const NSS_CC_SYS_CLK_SRC: c_int = 85;
pub const NSS_CC_SRDS0_SYS_CLK: c_int = 86;
pub const NSS_CC_SRDS1_SYS_CLK: c_int = 87;
pub const NSS_CC_GEPHY0_SYS_CLK: c_int = 88;
pub const NSS_CC_GEPHY1_SYS_CLK: c_int = 89;
pub const NSS_CC_GEPHY2_SYS_CLK: c_int = 90;
pub const NSS_CC_GEPHY3_SYS_CLK: c_int = 91;
