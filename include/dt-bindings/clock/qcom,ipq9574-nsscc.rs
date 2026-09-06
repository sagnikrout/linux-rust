//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,ipq9574-nsscc.h
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
// Copyright (c) 2023, 2025 The Linux Foundation. All rights reserved.
//
pub const NSS_CC_CE_APB_CLK: c_int = 0;
pub const NSS_CC_CE_AXI_CLK: c_int = 1;
pub const NSS_CC_CE_CLK_SRC: c_int = 2;
pub const NSS_CC_CFG_CLK_SRC: c_int = 3;
pub const NSS_CC_CLC_AXI_CLK: c_int = 4;
pub const NSS_CC_CLC_CLK_SRC: c_int = 5;
pub const NSS_CC_CRYPTO_CLK: c_int = 6;
pub const NSS_CC_CRYPTO_CLK_SRC: c_int = 7;
pub const NSS_CC_CRYPTO_PPE_CLK: c_int = 8;
pub const NSS_CC_HAQ_AHB_CLK: c_int = 9;
pub const NSS_CC_HAQ_AXI_CLK: c_int = 10;
pub const NSS_CC_HAQ_CLK_SRC: c_int = 11;
pub const NSS_CC_IMEM_AHB_CLK: c_int = 12;
pub const NSS_CC_IMEM_CLK_SRC: c_int = 13;
pub const NSS_CC_IMEM_QSB_CLK: c_int = 14;
pub const NSS_CC_INT_CFG_CLK_SRC: c_int = 15;
pub const NSS_CC_NSS_CSR_CLK: c_int = 16;
pub const NSS_CC_NSSNOC_CE_APB_CLK: c_int = 17;
pub const NSS_CC_NSSNOC_CE_AXI_CLK: c_int = 18;
pub const NSS_CC_NSSNOC_CLC_AXI_CLK: c_int = 19;
pub const NSS_CC_NSSNOC_CRYPTO_CLK: c_int = 20;
pub const NSS_CC_NSSNOC_HAQ_AHB_CLK: c_int = 21;
pub const NSS_CC_NSSNOC_HAQ_AXI_CLK: c_int = 22;
pub const NSS_CC_NSSNOC_IMEM_AHB_CLK: c_int = 23;
pub const NSS_CC_NSSNOC_IMEM_QSB_CLK: c_int = 24;
pub const NSS_CC_NSSNOC_NSS_CSR_CLK: c_int = 25;
pub const NSS_CC_NSSNOC_PPE_CFG_CLK: c_int = 26;
pub const NSS_CC_NSSNOC_PPE_CLK: c_int = 27;
pub const NSS_CC_NSSNOC_UBI32_AHB0_CLK: c_int = 28;
pub const NSS_CC_NSSNOC_UBI32_AXI0_CLK: c_int = 29;
pub const NSS_CC_NSSNOC_UBI32_INT0_AHB_CLK: c_int = 30;
pub const NSS_CC_NSSNOC_UBI32_NC_AXI0_1_CLK: c_int = 31;
pub const NSS_CC_NSSNOC_UBI32_NC_AXI0_CLK: c_int = 32;
pub const NSS_CC_PORT1_MAC_CLK: c_int = 33;
pub const NSS_CC_PORT1_RX_CLK: c_int = 34;
pub const NSS_CC_PORT1_RX_CLK_SRC: c_int = 35;
pub const NSS_CC_PORT1_RX_DIV_CLK_SRC: c_int = 36;
pub const NSS_CC_PORT1_TX_CLK: c_int = 37;
pub const NSS_CC_PORT1_TX_CLK_SRC: c_int = 38;
pub const NSS_CC_PORT1_TX_DIV_CLK_SRC: c_int = 39;
pub const NSS_CC_PORT2_MAC_CLK: c_int = 40;
pub const NSS_CC_PORT2_RX_CLK: c_int = 41;
pub const NSS_CC_PORT2_RX_CLK_SRC: c_int = 42;
pub const NSS_CC_PORT2_RX_DIV_CLK_SRC: c_int = 43;
pub const NSS_CC_PORT2_TX_CLK: c_int = 44;
pub const NSS_CC_PORT2_TX_CLK_SRC: c_int = 45;
pub const NSS_CC_PORT2_TX_DIV_CLK_SRC: c_int = 46;
pub const NSS_CC_PORT3_MAC_CLK: c_int = 47;
pub const NSS_CC_PORT3_RX_CLK: c_int = 48;
pub const NSS_CC_PORT3_RX_CLK_SRC: c_int = 49;
pub const NSS_CC_PORT3_RX_DIV_CLK_SRC: c_int = 50;
pub const NSS_CC_PORT3_TX_CLK: c_int = 51;
pub const NSS_CC_PORT3_TX_CLK_SRC: c_int = 52;
pub const NSS_CC_PORT3_TX_DIV_CLK_SRC: c_int = 53;
pub const NSS_CC_PORT4_MAC_CLK: c_int = 54;
pub const NSS_CC_PORT4_RX_CLK: c_int = 55;
pub const NSS_CC_PORT4_RX_CLK_SRC: c_int = 56;
pub const NSS_CC_PORT4_RX_DIV_CLK_SRC: c_int = 57;
pub const NSS_CC_PORT4_TX_CLK: c_int = 58;
pub const NSS_CC_PORT4_TX_CLK_SRC: c_int = 59;
pub const NSS_CC_PORT4_TX_DIV_CLK_SRC: c_int = 60;
pub const NSS_CC_PORT5_MAC_CLK: c_int = 61;
pub const NSS_CC_PORT5_RX_CLK: c_int = 62;
pub const NSS_CC_PORT5_RX_CLK_SRC: c_int = 63;
pub const NSS_CC_PORT5_RX_DIV_CLK_SRC: c_int = 64;
pub const NSS_CC_PORT5_TX_CLK: c_int = 65;
pub const NSS_CC_PORT5_TX_CLK_SRC: c_int = 66;
pub const NSS_CC_PORT5_TX_DIV_CLK_SRC: c_int = 67;
pub const NSS_CC_PORT6_MAC_CLK: c_int = 68;
pub const NSS_CC_PORT6_RX_CLK: c_int = 69;
pub const NSS_CC_PORT6_RX_CLK_SRC: c_int = 70;
pub const NSS_CC_PORT6_RX_DIV_CLK_SRC: c_int = 71;
pub const NSS_CC_PORT6_TX_CLK: c_int = 72;
pub const NSS_CC_PORT6_TX_CLK_SRC: c_int = 73;
pub const NSS_CC_PORT6_TX_DIV_CLK_SRC: c_int = 74;
pub const NSS_CC_PPE_CLK_SRC: c_int = 75;
pub const NSS_CC_PPE_EDMA_CFG_CLK: c_int = 76;
pub const NSS_CC_PPE_EDMA_CLK: c_int = 77;
pub const NSS_CC_PPE_SWITCH_BTQ_CLK: c_int = 78;
pub const NSS_CC_PPE_SWITCH_CFG_CLK: c_int = 79;
pub const NSS_CC_PPE_SWITCH_CLK: c_int = 80;
pub const NSS_CC_PPE_SWITCH_IPE_CLK: c_int = 81;
pub const NSS_CC_UBI0_CLK_SRC: c_int = 82;
pub const NSS_CC_UBI0_DIV_CLK_SRC: c_int = 83;
pub const NSS_CC_UBI1_CLK_SRC: c_int = 84;
pub const NSS_CC_UBI1_DIV_CLK_SRC: c_int = 85;
pub const NSS_CC_UBI2_CLK_SRC: c_int = 86;
pub const NSS_CC_UBI2_DIV_CLK_SRC: c_int = 87;
pub const NSS_CC_UBI32_AHB0_CLK: c_int = 88;
pub const NSS_CC_UBI32_AHB1_CLK: c_int = 89;
pub const NSS_CC_UBI32_AHB2_CLK: c_int = 90;
pub const NSS_CC_UBI32_AHB3_CLK: c_int = 91;
pub const NSS_CC_UBI32_AXI0_CLK: c_int = 92;
pub const NSS_CC_UBI32_AXI1_CLK: c_int = 93;
pub const NSS_CC_UBI32_AXI2_CLK: c_int = 94;
pub const NSS_CC_UBI32_AXI3_CLK: c_int = 95;
pub const NSS_CC_UBI32_CORE0_CLK: c_int = 96;
pub const NSS_CC_UBI32_CORE1_CLK: c_int = 97;
pub const NSS_CC_UBI32_CORE2_CLK: c_int = 98;
pub const NSS_CC_UBI32_CORE3_CLK: c_int = 99;
pub const NSS_CC_UBI32_INTR0_AHB_CLK: c_int = 100;
pub const NSS_CC_UBI32_INTR1_AHB_CLK: c_int = 101;
pub const NSS_CC_UBI32_INTR2_AHB_CLK: c_int = 102;
pub const NSS_CC_UBI32_INTR3_AHB_CLK: c_int = 103;
pub const NSS_CC_UBI32_NC_AXI0_CLK: c_int = 104;
pub const NSS_CC_UBI32_NC_AXI1_CLK: c_int = 105;
pub const NSS_CC_UBI32_NC_AXI2_CLK: c_int = 106;
pub const NSS_CC_UBI32_NC_AXI3_CLK: c_int = 107;
pub const NSS_CC_UBI32_UTCM0_CLK: c_int = 108;
pub const NSS_CC_UBI32_UTCM1_CLK: c_int = 109;
pub const NSS_CC_UBI32_UTCM2_CLK: c_int = 110;
pub const NSS_CC_UBI32_UTCM3_CLK: c_int = 111;
pub const NSS_CC_UBI3_CLK_SRC: c_int = 112;
pub const NSS_CC_UBI3_DIV_CLK_SRC: c_int = 113;
pub const NSS_CC_UBI_AXI_CLK_SRC: c_int = 114;
pub const NSS_CC_UBI_NC_AXI_BFDCD_CLK_SRC: c_int = 115;
pub const NSS_CC_UNIPHY_PORT1_RX_CLK: c_int = 116;
pub const NSS_CC_UNIPHY_PORT1_TX_CLK: c_int = 117;
pub const NSS_CC_UNIPHY_PORT2_RX_CLK: c_int = 118;
pub const NSS_CC_UNIPHY_PORT2_TX_CLK: c_int = 119;
pub const NSS_CC_UNIPHY_PORT3_RX_CLK: c_int = 120;
pub const NSS_CC_UNIPHY_PORT3_TX_CLK: c_int = 121;
pub const NSS_CC_UNIPHY_PORT4_RX_CLK: c_int = 122;
pub const NSS_CC_UNIPHY_PORT4_TX_CLK: c_int = 123;
pub const NSS_CC_UNIPHY_PORT5_RX_CLK: c_int = 124;
pub const NSS_CC_UNIPHY_PORT5_TX_CLK: c_int = 125;
pub const NSS_CC_UNIPHY_PORT6_RX_CLK: c_int = 126;
pub const NSS_CC_UNIPHY_PORT6_TX_CLK: c_int = 127;
pub const NSS_CC_XGMAC0_PTP_REF_CLK: c_int = 128;
pub const NSS_CC_XGMAC0_PTP_REF_DIV_CLK_SRC: c_int = 129;
pub const NSS_CC_XGMAC1_PTP_REF_CLK: c_int = 130;
pub const NSS_CC_XGMAC1_PTP_REF_DIV_CLK_SRC: c_int = 131;
pub const NSS_CC_XGMAC2_PTP_REF_CLK: c_int = 132;
pub const NSS_CC_XGMAC2_PTP_REF_DIV_CLK_SRC: c_int = 133;
pub const NSS_CC_XGMAC3_PTP_REF_CLK: c_int = 134;
pub const NSS_CC_XGMAC3_PTP_REF_DIV_CLK_SRC: c_int = 135;
pub const NSS_CC_XGMAC4_PTP_REF_CLK: c_int = 136;
pub const NSS_CC_XGMAC4_PTP_REF_DIV_CLK_SRC: c_int = 137;
pub const NSS_CC_XGMAC5_PTP_REF_CLK: c_int = 138;
pub const NSS_CC_XGMAC5_PTP_REF_DIV_CLK_SRC: c_int = 139;
pub const UBI32_PLL: c_int = 140;
pub const UBI32_PLL_MAIN: c_int = 141;
