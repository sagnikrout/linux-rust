//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,ipq5424-nsscc.h
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
// NSS_CC clocks
pub const NSS_CC_CE_APB_CLK: c_int = 0;
pub const NSS_CC_CE_AXI_CLK: c_int = 1;
pub const NSS_CC_CE_CLK_SRC: c_int = 2;
pub const NSS_CC_CFG_CLK_SRC: c_int = 3;
pub const NSS_CC_DEBUG_CLK: c_int = 4;
pub const NSS_CC_EIP_BFDCD_CLK_SRC: c_int = 5;
pub const NSS_CC_EIP_CLK: c_int = 6;
pub const NSS_CC_NSS_CSR_CLK: c_int = 7;
pub const NSS_CC_NSSNOC_CE_APB_CLK: c_int = 8;
pub const NSS_CC_NSSNOC_CE_AXI_CLK: c_int = 9;
pub const NSS_CC_NSSNOC_EIP_CLK: c_int = 10;
pub const NSS_CC_NSSNOC_NSS_CSR_CLK: c_int = 11;
pub const NSS_CC_NSSNOC_PPE_CFG_CLK: c_int = 12;
pub const NSS_CC_NSSNOC_PPE_CLK: c_int = 13;
pub const NSS_CC_PORT1_MAC_CLK: c_int = 14;
pub const NSS_CC_PORT1_RX_CLK: c_int = 15;
pub const NSS_CC_PORT1_RX_CLK_SRC: c_int = 16;
pub const NSS_CC_PORT1_RX_DIV_CLK_SRC: c_int = 17;
pub const NSS_CC_PORT1_TX_CLK: c_int = 18;
pub const NSS_CC_PORT1_TX_CLK_SRC: c_int = 19;
pub const NSS_CC_PORT1_TX_DIV_CLK_SRC: c_int = 20;
pub const NSS_CC_PORT2_MAC_CLK: c_int = 21;
pub const NSS_CC_PORT2_RX_CLK: c_int = 22;
pub const NSS_CC_PORT2_RX_CLK_SRC: c_int = 23;
pub const NSS_CC_PORT2_RX_DIV_CLK_SRC: c_int = 24;
pub const NSS_CC_PORT2_TX_CLK: c_int = 25;
pub const NSS_CC_PORT2_TX_CLK_SRC: c_int = 26;
pub const NSS_CC_PORT2_TX_DIV_CLK_SRC: c_int = 27;
pub const NSS_CC_PORT3_MAC_CLK: c_int = 28;
pub const NSS_CC_PORT3_RX_CLK: c_int = 29;
pub const NSS_CC_PORT3_RX_CLK_SRC: c_int = 30;
pub const NSS_CC_PORT3_RX_DIV_CLK_SRC: c_int = 31;
pub const NSS_CC_PORT3_TX_CLK: c_int = 32;
pub const NSS_CC_PORT3_TX_CLK_SRC: c_int = 33;
pub const NSS_CC_PORT3_TX_DIV_CLK_SRC: c_int = 34;
pub const NSS_CC_PPE_CLK_SRC: c_int = 35;
pub const NSS_CC_PPE_EDMA_CFG_CLK: c_int = 36;
pub const NSS_CC_PPE_EDMA_CLK: c_int = 37;
pub const NSS_CC_PPE_SWITCH_BTQ_CLK: c_int = 38;
pub const NSS_CC_PPE_SWITCH_CFG_CLK: c_int = 39;
pub const NSS_CC_PPE_SWITCH_CLK: c_int = 40;
pub const NSS_CC_PPE_SWITCH_IPE_CLK: c_int = 41;
pub const NSS_CC_UNIPHY_PORT1_RX_CLK: c_int = 42;
pub const NSS_CC_UNIPHY_PORT1_TX_CLK: c_int = 43;
pub const NSS_CC_UNIPHY_PORT2_RX_CLK: c_int = 44;
pub const NSS_CC_UNIPHY_PORT2_TX_CLK: c_int = 45;
pub const NSS_CC_UNIPHY_PORT3_RX_CLK: c_int = 46;
pub const NSS_CC_UNIPHY_PORT3_TX_CLK: c_int = 47;
pub const NSS_CC_XGMAC0_PTP_REF_CLK: c_int = 48;
pub const NSS_CC_XGMAC0_PTP_REF_DIV_CLK_SRC: c_int = 49;
pub const NSS_CC_XGMAC1_PTP_REF_CLK: c_int = 50;
pub const NSS_CC_XGMAC1_PTP_REF_DIV_CLK_SRC: c_int = 51;
pub const NSS_CC_XGMAC2_PTP_REF_CLK: c_int = 52;
pub const NSS_CC_XGMAC2_PTP_REF_DIV_CLK_SRC: c_int = 53;
