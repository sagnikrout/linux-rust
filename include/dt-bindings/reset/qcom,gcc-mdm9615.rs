//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/qcom,gcc-mdm9615.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2013, The Linux Foundation. All rights reserved.
// Copyright (c) BayLibre, SAS.
// Author : Neil Armstrong <narmstrong@baylibre.com>
//
pub const SFAB_MSS_Q6_SW_RESET: c_int = 0;
pub const SFAB_MSS_Q6_FW_RESET: c_int = 1;
pub const QDSS_STM_RESET: c_int = 2;
pub const AFAB_SMPSS_S_RESET: c_int = 3;
pub const AFAB_SMPSS_M1_RESET: c_int = 4;
pub const AFAB_SMPSS_M0_RESET: c_int = 5;
pub const AFAB_EBI1_CH0_RESET: c_int = 6;
pub const AFAB_EBI1_CH1_RESET: c_int = 7;
pub const SFAB_ADM0_M0_RESET: c_int = 8;
pub const SFAB_ADM0_M1_RESET: c_int = 9;
pub const SFAB_ADM0_M2_RESET: c_int = 10;
pub const ADM0_C2_RESET: c_int = 11;
pub const ADM0_C1_RESET: c_int = 12;
pub const ADM0_C0_RESET: c_int = 13;
pub const ADM0_PBUS_RESET: c_int = 14;
pub const ADM0_RESET: c_int = 15;
pub const QDSS_CLKS_SW_RESET: c_int = 16;
pub const QDSS_POR_RESET: c_int = 17;
pub const QDSS_TSCTR_RESET: c_int = 18;
pub const QDSS_HRESET_RESET: c_int = 19;
pub const QDSS_AXI_RESET: c_int = 20;
pub const QDSS_DBG_RESET: c_int = 21;
pub const PCIE_A_RESET: c_int = 22;
pub const PCIE_AUX_RESET: c_int = 23;
pub const PCIE_H_RESET: c_int = 24;
pub const SFAB_PCIE_M_RESET: c_int = 25;
pub const SFAB_PCIE_S_RESET: c_int = 26;
pub const SFAB_MSS_M_RESET: c_int = 27;
pub const SFAB_USB3_M_RESET: c_int = 28;
pub const SFAB_RIVA_M_RESET: c_int = 29;
pub const SFAB_LPASS_RESET: c_int = 30;
pub const SFAB_AFAB_M_RESET: c_int = 31;
pub const AFAB_SFAB_M0_RESET: c_int = 32;
pub const AFAB_SFAB_M1_RESET: c_int = 33;
pub const SFAB_SATA_S_RESET: c_int = 34;
pub const SFAB_DFAB_M_RESET: c_int = 35;
pub const DFAB_SFAB_M_RESET: c_int = 36;
pub const DFAB_SWAY0_RESET: c_int = 37;
pub const DFAB_SWAY1_RESET: c_int = 38;
pub const DFAB_ARB0_RESET: c_int = 39;
pub const DFAB_ARB1_RESET: c_int = 40;
pub const PPSS_PROC_RESET: c_int = 41;
pub const PPSS_RESET: c_int = 42;
pub const DMA_BAM_RESET: c_int = 43;
pub const SPS_TIC_H_RESET: c_int = 44;
pub const SLIMBUS_H_RESET: c_int = 45;
pub const SFAB_CFPB_M_RESET: c_int = 46;
pub const SFAB_CFPB_S_RESET: c_int = 47;
pub const TSIF_H_RESET: c_int = 48;
pub const CE1_H_RESET: c_int = 49;
pub const CE1_CORE_RESET: c_int = 50;
pub const CE1_SLEEP_RESET: c_int = 51;
pub const CE2_H_RESET: c_int = 52;
pub const CE2_CORE_RESET: c_int = 53;
pub const SFAB_SFPB_M_RESET: c_int = 54;
pub const SFAB_SFPB_S_RESET: c_int = 55;
pub const RPM_PROC_RESET: c_int = 56;
pub const PMIC_SSBI2_RESET: c_int = 57;
pub const SDC1_RESET: c_int = 58;
pub const SDC2_RESET: c_int = 59;
pub const SDC3_RESET: c_int = 60;
pub const SDC4_RESET: c_int = 61;
pub const SDC5_RESET: c_int = 62;
pub const DFAB_A2_RESET: c_int = 63;
pub const USB_HS1_RESET: c_int = 64;
pub const USB_HSIC_RESET: c_int = 65;
pub const USB_FS1_XCVR_RESET: c_int = 66;
pub const USB_FS1_RESET: c_int = 67;
pub const USB_FS2_XCVR_RESET: c_int = 68;
pub const USB_FS2_RESET: c_int = 69;
pub const GSBI1_RESET: c_int = 70;
pub const GSBI2_RESET: c_int = 71;
pub const GSBI3_RESET: c_int = 72;
pub const GSBI4_RESET: c_int = 73;
pub const GSBI5_RESET: c_int = 74;
pub const GSBI6_RESET: c_int = 75;
pub const GSBI7_RESET: c_int = 76;
pub const GSBI8_RESET: c_int = 77;
pub const GSBI9_RESET: c_int = 78;
pub const GSBI10_RESET: c_int = 79;
pub const GSBI11_RESET: c_int = 80;
pub const GSBI12_RESET: c_int = 81;
pub const SPDM_RESET: c_int = 82;
pub const TLMM_H_RESET: c_int = 83;
pub const SFAB_MSS_S_RESET: c_int = 84;
pub const MSS_SLP_RESET: c_int = 85;
pub const MSS_Q6SW_JTAG_RESET: c_int = 86;
pub const MSS_Q6FW_JTAG_RESET: c_int = 87;
pub const MSS_RESET: c_int = 88;
pub const SATA_H_RESET: c_int = 89;
pub const SATA_RXOOB_RESE: c_int = 90;
pub const SATA_PMALIVE_RESET: c_int = 91;
pub const SATA_SFAB_M_RESET: c_int = 92;
pub const TSSC_RESET: c_int = 93;
pub const PDM_RESET: c_int = 94;
pub const MPM_H_RESET: c_int = 95;
pub const MPM_RESET: c_int = 96;
pub const SFAB_SMPSS_S_RESET: c_int = 97;
pub const PRNG_RESET: c_int = 98;
pub const RIVA_RESET: c_int = 99;
pub const USB_HS3_RESET: c_int = 100;
pub const USB_HS4_RESET: c_int = 101;
pub const CE3_RESET: c_int = 102;
pub const PCIE_EXT_PCI_RESET: c_int = 103;
pub const PCIE_PHY_RESET: c_int = 104;
pub const PCIE_PCI_RESET: c_int = 105;
pub const PCIE_POR_RESET: c_int = 106;
pub const PCIE_HCLK_RESET: c_int = 107;
pub const PCIE_ACLK_RESET: c_int = 108;
pub const CE3_H_RESET: c_int = 109;
pub const SFAB_CE3_M_RESET: c_int = 110;
pub const SFAB_CE3_S_RESET: c_int = 111;
pub const SATA_RESET: c_int = 112;
pub const CE3_SLEEP_RESET: c_int = 113;
pub const GSS_SLP_RESET: c_int = 114;
pub const GSS_RESET: c_int = 115;
