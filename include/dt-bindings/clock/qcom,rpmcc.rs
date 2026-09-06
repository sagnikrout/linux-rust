//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,rpmcc.h
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
// Copyright 2015 Linaro Limited
//
// RPM clocks
pub const RPM_PXO_CLK: c_int = 0;
pub const RPM_PXO_A_CLK: c_int = 1;
pub const RPM_CXO_CLK: c_int = 2;
pub const RPM_CXO_A_CLK: c_int = 3;
pub const RPM_APPS_FABRIC_CLK: c_int = 4;
pub const RPM_APPS_FABRIC_A_CLK: c_int = 5;
pub const RPM_CFPB_CLK: c_int = 6;
pub const RPM_CFPB_A_CLK: c_int = 7;
pub const RPM_QDSS_CLK: c_int = 8;
pub const RPM_QDSS_A_CLK: c_int = 9;
pub const RPM_DAYTONA_FABRIC_CLK: c_int = 10;
pub const RPM_DAYTONA_FABRIC_A_CLK: c_int = 11;
pub const RPM_EBI1_CLK: c_int = 12;
pub const RPM_EBI1_A_CLK: c_int = 13;
pub const RPM_MM_FABRIC_CLK: c_int = 14;
pub const RPM_MM_FABRIC_A_CLK: c_int = 15;
pub const RPM_MMFPB_CLK: c_int = 16;
pub const RPM_MMFPB_A_CLK: c_int = 17;
pub const RPM_SYS_FABRIC_CLK: c_int = 18;
pub const RPM_SYS_FABRIC_A_CLK: c_int = 19;
pub const RPM_SFPB_CLK: c_int = 20;
pub const RPM_SFPB_A_CLK: c_int = 21;
pub const RPM_SMI_CLK: c_int = 22;
pub const RPM_SMI_A_CLK: c_int = 23;
pub const RPM_PLL4_CLK: c_int = 24;
pub const RPM_XO_D0: c_int = 25;
pub const RPM_XO_D1: c_int = 26;
pub const RPM_XO_A0: c_int = 27;
pub const RPM_XO_A1: c_int = 28;
pub const RPM_XO_A2: c_int = 29;
pub const RPM_NSS_FABRIC_0_CLK: c_int = 30;
pub const RPM_NSS_FABRIC_0_A_CLK: c_int = 31;
pub const RPM_NSS_FABRIC_1_CLK: c_int = 32;
pub const RPM_NSS_FABRIC_1_A_CLK: c_int = 33;
// SMD RPM clocks
pub const RPM_SMD_XO_CLK_SRC: c_int = 0;
pub const RPM_SMD_XO_A_CLK_SRC: c_int = 1;
pub const RPM_SMD_PCNOC_CLK: c_int = 2;
pub const RPM_SMD_PCNOC_A_CLK: c_int = 3;
pub const RPM_SMD_SNOC_CLK: c_int = 4;
pub const RPM_SMD_SNOC_A_CLK: c_int = 5;
pub const RPM_SMD_BIMC_CLK: c_int = 6;
pub const RPM_SMD_BIMC_A_CLK: c_int = 7;
pub const RPM_SMD_QDSS_CLK: c_int = 8;
pub const RPM_SMD_QDSS_A_CLK: c_int = 9;
pub const RPM_SMD_BB_CLK1: c_int = 10;
pub const RPM_SMD_BB_CLK1_A: c_int = 11;
pub const RPM_SMD_BB_CLK2: c_int = 12;
pub const RPM_SMD_BB_CLK2_A: c_int = 13;
pub const RPM_SMD_RF_CLK1: c_int = 14;
pub const RPM_SMD_RF_CLK1_A: c_int = 15;
pub const RPM_SMD_RF_CLK2: c_int = 16;
pub const RPM_SMD_RF_CLK2_A: c_int = 17;
pub const RPM_SMD_BB_CLK1_PIN: c_int = 18;
pub const RPM_SMD_BB_CLK1_A_PIN: c_int = 19;
pub const RPM_SMD_BB_CLK2_PIN: c_int = 20;
pub const RPM_SMD_BB_CLK2_A_PIN: c_int = 21;
pub const RPM_SMD_RF_CLK1_PIN: c_int = 22;
pub const RPM_SMD_RF_CLK1_A_PIN: c_int = 23;
pub const RPM_SMD_RF_CLK2_PIN: c_int = 24;
pub const RPM_SMD_RF_CLK2_A_PIN: c_int = 25;
pub const RPM_SMD_PNOC_CLK: c_int = 26;
pub const RPM_SMD_PNOC_A_CLK: c_int = 27;
pub const RPM_SMD_CNOC_CLK: c_int = 28;
pub const RPM_SMD_CNOC_A_CLK: c_int = 29;
pub const RPM_SMD_MMSSNOC_AHB_CLK: c_int = 30;
pub const RPM_SMD_MMSSNOC_AHB_A_CLK: c_int = 31;
pub const RPM_SMD_GFX3D_CLK_SRC: c_int = 32;
pub const RPM_SMD_GFX3D_A_CLK_SRC: c_int = 33;
pub const RPM_SMD_OCMEMGX_CLK: c_int = 34;
pub const RPM_SMD_OCMEMGX_A_CLK: c_int = 35;
pub const RPM_SMD_CXO_D0: c_int = 36;
pub const RPM_SMD_CXO_D0_A: c_int = 37;
pub const RPM_SMD_CXO_D1: c_int = 38;
pub const RPM_SMD_CXO_D1_A: c_int = 39;
pub const RPM_SMD_CXO_A0: c_int = 40;
pub const RPM_SMD_CXO_A0_A: c_int = 41;
pub const RPM_SMD_CXO_A1: c_int = 42;
pub const RPM_SMD_CXO_A1_A: c_int = 43;
pub const RPM_SMD_CXO_A2: c_int = 44;
pub const RPM_SMD_CXO_A2_A: c_int = 45;
pub const RPM_SMD_DIV_CLK1: c_int = 46;
pub const RPM_SMD_DIV_A_CLK1: c_int = 47;
pub const RPM_SMD_DIV_CLK2: c_int = 48;
pub const RPM_SMD_DIV_A_CLK2: c_int = 49;
pub const RPM_SMD_DIFF_CLK: c_int = 50;
pub const RPM_SMD_DIFF_A_CLK: c_int = 51;
pub const RPM_SMD_CXO_D0_PIN: c_int = 52;
pub const RPM_SMD_CXO_D0_A_PIN: c_int = 53;
pub const RPM_SMD_CXO_D1_PIN: c_int = 54;
pub const RPM_SMD_CXO_D1_A_PIN: c_int = 55;
pub const RPM_SMD_CXO_A0_PIN: c_int = 56;
pub const RPM_SMD_CXO_A0_A_PIN: c_int = 57;
pub const RPM_SMD_CXO_A1_PIN: c_int = 58;
pub const RPM_SMD_CXO_A1_A_PIN: c_int = 59;
pub const RPM_SMD_CXO_A2_PIN: c_int = 60;
pub const RPM_SMD_CXO_A2_A_PIN: c_int = 61;
pub const RPM_SMD_AGGR1_NOC_CLK: c_int = 62;
pub const RPM_SMD_AGGR1_NOC_A_CLK: c_int = 63;
pub const RPM_SMD_AGGR2_NOC_CLK: c_int = 64;
pub const RPM_SMD_AGGR2_NOC_A_CLK: c_int = 65;
pub const RPM_SMD_MMAXI_CLK: c_int = 66;
pub const RPM_SMD_MMAXI_A_CLK: c_int = 67;
pub const RPM_SMD_IPA_CLK: c_int = 68;
pub const RPM_SMD_IPA_A_CLK: c_int = 69;
pub const RPM_SMD_CE1_CLK: c_int = 70;
pub const RPM_SMD_CE1_A_CLK: c_int = 71;
pub const RPM_SMD_DIV_CLK3: c_int = 72;
pub const RPM_SMD_DIV_A_CLK3: c_int = 73;
pub const RPM_SMD_LN_BB_CLK: c_int = 74;
pub const RPM_SMD_LN_BB_A_CLK: c_int = 75;
pub const RPM_SMD_BIMC_GPU_CLK: c_int = 76;
pub const RPM_SMD_BIMC_GPU_A_CLK: c_int = 77;
pub const RPM_SMD_QPIC_CLK: c_int = 78;
pub const RPM_SMD_QPIC_CLK_A: c_int = 79;
pub const RPM_SMD_LN_BB_CLK1: c_int = 80;
pub const RPM_SMD_LN_BB_CLK1_A: c_int = 81;
pub const RPM_SMD_LN_BB_CLK2: c_int = 82;
pub const RPM_SMD_LN_BB_CLK2_A: c_int = 83;
pub const RPM_SMD_LN_BB_CLK3_PIN: c_int = 84;
pub const RPM_SMD_LN_BB_CLK3_A_PIN: c_int = 85;
pub const RPM_SMD_RF_CLK3: c_int = 86;
pub const RPM_SMD_RF_CLK3_A: c_int = 87;
pub const RPM_SMD_RF_CLK3_PIN: c_int = 88;
pub const RPM_SMD_RF_CLK3_A_PIN: c_int = 89;
pub const RPM_SMD_MMSSNOC_AXI_CLK: c_int = 90;
pub const RPM_SMD_MMSSNOC_AXI_CLK_A: c_int = 91;
pub const RPM_SMD_CNOC_PERIPH_CLK: c_int = 92;
pub const RPM_SMD_CNOC_PERIPH_A_CLK: c_int = 93;
pub const RPM_SMD_LN_BB_CLK3: c_int = 94;
pub const RPM_SMD_LN_BB_CLK3_A: c_int = 95;
pub const RPM_SMD_LN_BB_CLK1_PIN: c_int = 96;
pub const RPM_SMD_LN_BB_CLK1_A_PIN: c_int = 97;
pub const RPM_SMD_LN_BB_CLK2_PIN: c_int = 98;
pub const RPM_SMD_LN_BB_CLK2_A_PIN: c_int = 99;
pub const RPM_SMD_SYSMMNOC_CLK: c_int = 100;
pub const RPM_SMD_SYSMMNOC_A_CLK: c_int = 101;
pub const RPM_SMD_CE2_CLK: c_int = 102;
pub const RPM_SMD_CE2_A_CLK: c_int = 103;
pub const RPM_SMD_CE3_CLK: c_int = 104;
pub const RPM_SMD_CE3_A_CLK: c_int = 105;
pub const RPM_SMD_QUP_CLK: c_int = 106;
pub const RPM_SMD_QUP_A_CLK: c_int = 107;
pub const RPM_SMD_MMRT_CLK: c_int = 108;
pub const RPM_SMD_MMRT_A_CLK: c_int = 109;
pub const RPM_SMD_MMNRT_CLK: c_int = 110;
pub const RPM_SMD_MMNRT_A_CLK: c_int = 111;
pub const RPM_SMD_SNOC_PERIPH_CLK: c_int = 112;
pub const RPM_SMD_SNOC_PERIPH_A_CLK: c_int = 113;
pub const RPM_SMD_SNOC_LPASS_CLK: c_int = 114;
pub const RPM_SMD_SNOC_LPASS_A_CLK: c_int = 115;
pub const RPM_SMD_HWKM_CLK: c_int = 116;
pub const RPM_SMD_HWKM_A_CLK: c_int = 117;
pub const RPM_SMD_PKA_CLK: c_int = 118;
pub const RPM_SMD_PKA_A_CLK: c_int = 119;
pub const RPM_SMD_CPUSS_GNOC_CLK: c_int = 120;
pub const RPM_SMD_CPUSS_GNOC_A_CLK: c_int = 121;
pub const RPM_SMD_MSS_CFG_AHB_CLK: c_int = 122;
pub const RPM_SMD_MSS_CFG_AHB_A_CLK: c_int = 123;
pub const RPM_SMD_BIMC_FREQ_LOG: c_int = 124;
pub const RPM_SMD_LN_BB_CLK_PIN: c_int = 125;
pub const RPM_SMD_LN_BB_A_CLK_PIN: c_int = 126;
pub const RPM_SMD_BB_CLK3: c_int = 127;
pub const RPM_SMD_BB_CLK3_A: c_int = 128;
pub const RPM_SMD_BB_CLK3_PIN: c_int = 129;
pub const RPM_SMD_BB_CLK3_A_PIN: c_int = 130;
