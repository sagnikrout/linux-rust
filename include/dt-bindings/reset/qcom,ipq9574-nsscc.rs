//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/qcom,ipq9574-nsscc.h
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
pub const EDMA_HW_RESET: c_int = 0;
pub const NSS_CC_CE_BCR: c_int = 1;
pub const NSS_CC_CLC_BCR: c_int = 2;
pub const NSS_CC_EIP197_BCR: c_int = 3;
pub const NSS_CC_HAQ_BCR: c_int = 4;
pub const NSS_CC_IMEM_BCR: c_int = 5;
pub const NSS_CC_MAC_BCR: c_int = 6;
pub const NSS_CC_PPE_BCR: c_int = 7;
pub const NSS_CC_UBI_BCR: c_int = 8;
pub const NSS_CC_UNIPHY_BCR: c_int = 9;
pub const UBI3_CLKRST_CLAMP_ENABLE: c_int = 10;
pub const UBI3_CORE_CLAMP_ENABLE: c_int = 11;
pub const UBI2_CLKRST_CLAMP_ENABLE: c_int = 12;
pub const UBI2_CORE_CLAMP_ENABLE: c_int = 13;
pub const UBI1_CLKRST_CLAMP_ENABLE: c_int = 14;
pub const UBI1_CORE_CLAMP_ENABLE: c_int = 15;
pub const UBI0_CLKRST_CLAMP_ENABLE: c_int = 16;
pub const UBI0_CORE_CLAMP_ENABLE: c_int = 17;
pub const NSSNOC_NSS_CSR_ARES: c_int = 18;
pub const NSS_CSR_ARES: c_int = 19;
pub const PPE_BTQ_ARES: c_int = 20;
pub const PPE_IPE_ARES: c_int = 21;
pub const PPE_ARES: c_int = 22;
pub const PPE_CFG_ARES: c_int = 23;
pub const PPE_EDMA_ARES: c_int = 24;
pub const PPE_EDMA_CFG_ARES: c_int = 25;
pub const CRY_PPE_ARES: c_int = 26;
pub const NSSNOC_PPE_ARES: c_int = 27;
pub const NSSNOC_PPE_CFG_ARES: c_int = 28;
pub const PORT1_MAC_ARES: c_int = 29;
pub const PORT2_MAC_ARES: c_int = 30;
pub const PORT3_MAC_ARES: c_int = 31;
pub const PORT4_MAC_ARES: c_int = 32;
pub const PORT5_MAC_ARES: c_int = 33;
pub const PORT6_MAC_ARES: c_int = 34;
pub const XGMAC0_PTP_REF_ARES: c_int = 35;
pub const XGMAC1_PTP_REF_ARES: c_int = 36;
pub const XGMAC2_PTP_REF_ARES: c_int = 37;
pub const XGMAC3_PTP_REF_ARES: c_int = 38;
pub const XGMAC4_PTP_REF_ARES: c_int = 39;
pub const XGMAC5_PTP_REF_ARES: c_int = 40;
pub const HAQ_AHB_ARES: c_int = 41;
pub const HAQ_AXI_ARES: c_int = 42;
pub const NSSNOC_HAQ_AHB_ARES: c_int = 43;
pub const NSSNOC_HAQ_AXI_ARES: c_int = 44;
pub const CE_APB_ARES: c_int = 45;
pub const CE_AXI_ARES: c_int = 46;
pub const NSSNOC_CE_APB_ARES: c_int = 47;
pub const NSSNOC_CE_AXI_ARES: c_int = 48;
pub const CRYPTO_ARES: c_int = 49;
pub const NSSNOC_CRYPTO_ARES: c_int = 50;
pub const NSSNOC_NC_AXI0_1_ARES: c_int = 51;
pub const UBI0_CORE_ARES: c_int = 52;
pub const UBI1_CORE_ARES: c_int = 53;
pub const UBI2_CORE_ARES: c_int = 54;
pub const UBI3_CORE_ARES: c_int = 55;
pub const NC_AXI0_ARES: c_int = 56;
pub const UTCM0_ARES: c_int = 57;
pub const NC_AXI1_ARES: c_int = 58;
pub const UTCM1_ARES: c_int = 59;
pub const NC_AXI2_ARES: c_int = 60;
pub const UTCM2_ARES: c_int = 61;
pub const NC_AXI3_ARES: c_int = 62;
pub const UTCM3_ARES: c_int = 63;
pub const NSSNOC_NC_AXI0_ARES: c_int = 64;
pub const AHB0_ARES: c_int = 65;
pub const INTR0_AHB_ARES: c_int = 66;
pub const AHB1_ARES: c_int = 67;
pub const INTR1_AHB_ARES: c_int = 68;
pub const AHB2_ARES: c_int = 69;
pub const INTR2_AHB_ARES: c_int = 70;
pub const AHB3_ARES: c_int = 71;
pub const INTR3_AHB_ARES: c_int = 72;
pub const NSSNOC_AHB0_ARES: c_int = 73;
pub const NSSNOC_INT0_AHB_ARES: c_int = 74;
pub const AXI0_ARES: c_int = 75;
pub const AXI1_ARES: c_int = 76;
pub const AXI2_ARES: c_int = 77;
pub const AXI3_ARES: c_int = 78;
pub const NSSNOC_AXI0_ARES: c_int = 79;
pub const IMEM_QSB_ARES: c_int = 80;
pub const NSSNOC_IMEM_QSB_ARES: c_int = 81;
pub const IMEM_AHB_ARES: c_int = 82;
pub const NSSNOC_IMEM_AHB_ARES: c_int = 83;
pub const UNIPHY_PORT1_RX_ARES: c_int = 84;
pub const UNIPHY_PORT1_TX_ARES: c_int = 85;
pub const UNIPHY_PORT2_RX_ARES: c_int = 86;
pub const UNIPHY_PORT2_TX_ARES: c_int = 87;
pub const UNIPHY_PORT3_RX_ARES: c_int = 88;
pub const UNIPHY_PORT3_TX_ARES: c_int = 89;
pub const UNIPHY_PORT4_RX_ARES: c_int = 90;
pub const UNIPHY_PORT4_TX_ARES: c_int = 91;
pub const UNIPHY_PORT5_RX_ARES: c_int = 92;
pub const UNIPHY_PORT5_TX_ARES: c_int = 93;
pub const UNIPHY_PORT6_RX_ARES: c_int = 94;
pub const UNIPHY_PORT6_TX_ARES: c_int = 95;
pub const PORT1_RX_ARES: c_int = 96;
pub const PORT1_TX_ARES: c_int = 97;
pub const PORT2_RX_ARES: c_int = 98;
pub const PORT2_TX_ARES: c_int = 99;
pub const PORT3_RX_ARES: c_int = 100;
pub const PORT3_TX_ARES: c_int = 101;
pub const PORT4_RX_ARES: c_int = 102;
pub const PORT4_TX_ARES: c_int = 103;
pub const PORT5_RX_ARES: c_int = 104;
pub const PORT5_TX_ARES: c_int = 105;
pub const PORT6_RX_ARES: c_int = 106;
pub const PORT6_TX_ARES: c_int = 107;
pub const PPE_FULL_RESET: c_int = 108;
pub const UNIPHY0_SOFT_RESET: c_int = 109;
pub const UNIPHY1_SOFT_RESET: c_int = 110;
pub const UNIPHY2_SOFT_RESET: c_int = 111;
pub const UNIPHY_PORT1_ARES: c_int = 112;
pub const UNIPHY_PORT2_ARES: c_int = 113;
pub const UNIPHY_PORT3_ARES: c_int = 114;
pub const UNIPHY_PORT4_ARES: c_int = 115;
pub const UNIPHY_PORT5_ARES: c_int = 116;
pub const UNIPHY_PORT6_ARES: c_int = 117;
pub const NSSPORT1_RESET: c_int = 118;
pub const NSSPORT2_RESET: c_int = 119;
pub const NSSPORT3_RESET: c_int = 120;
pub const NSSPORT4_RESET: c_int = 121;
pub const NSSPORT5_RESET: c_int = 122;
pub const NSSPORT6_RESET: c_int = 123;
