//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/qcom,gcc-msm8660.h
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
//
pub const AFAB_CORE_RESET: c_int = 0;
pub const SCSS_SYS_RESET: c_int = 1;
pub const SCSS_SYS_POR_RESET: c_int = 2;
pub const AFAB_SMPSS_S_RESET: c_int = 3;
pub const AFAB_SMPSS_M1_RESET: c_int = 4;
pub const AFAB_SMPSS_M0_RESET: c_int = 5;
pub const AFAB_EBI1_S_RESET: c_int = 6;
pub const SFAB_CORE_RESET: c_int = 7;
pub const SFAB_ADM0_M0_RESET: c_int = 8;
pub const SFAB_ADM0_M1_RESET: c_int = 9;
pub const SFAB_ADM0_M2_RESET: c_int = 10;
pub const ADM0_C2_RESET: c_int = 11;
pub const ADM0_C1_RESET: c_int = 12;
pub const ADM0_C0_RESET: c_int = 13;
pub const ADM0_PBUS_RESET: c_int = 14;
pub const ADM0_RESET: c_int = 15;
pub const SFAB_ADM1_M0_RESET: c_int = 16;
pub const SFAB_ADM1_M1_RESET: c_int = 17;
pub const SFAB_ADM1_M2_RESET: c_int = 18;
pub const MMFAB_ADM1_M3_RESET: c_int = 19;
pub const ADM1_C3_RESET: c_int = 20;
pub const ADM1_C2_RESET: c_int = 21;
pub const ADM1_C1_RESET: c_int = 22;
pub const ADM1_C0_RESET: c_int = 23;
pub const ADM1_PBUS_RESET: c_int = 24;
pub const ADM1_RESET: c_int = 25;
pub const IMEM0_RESET: c_int = 26;
pub const SFAB_LPASS_Q6_RESET: c_int = 27;
pub const SFAB_AFAB_M_RESET: c_int = 28;
pub const AFAB_SFAB_M0_RESET: c_int = 29;
pub const AFAB_SFAB_M1_RESET: c_int = 30;
pub const DFAB_CORE_RESET: c_int = 31;
pub const SFAB_DFAB_M_RESET: c_int = 32;
pub const DFAB_SFAB_M_RESET: c_int = 33;
pub const DFAB_SWAY0_RESET: c_int = 34;
pub const DFAB_SWAY1_RESET: c_int = 35;
pub const DFAB_ARB0_RESET: c_int = 36;
pub const DFAB_ARB1_RESET: c_int = 37;
pub const PPSS_PROC_RESET: c_int = 38;
pub const PPSS_RESET: c_int = 39;
pub const PMEM_RESET: c_int = 40;
pub const DMA_BAM_RESET: c_int = 41;
pub const SIC_RESET: c_int = 42;
pub const SPS_TIC_RESET: c_int = 43;
pub const CFBP0_RESET: c_int = 44;
pub const CFBP1_RESET: c_int = 45;
pub const CFBP2_RESET: c_int = 46;
pub const EBI2_RESET: c_int = 47;
pub const SFAB_CFPB_M_RESET: c_int = 48;
pub const CFPB_MASTER_RESET: c_int = 49;
pub const SFAB_CFPB_S_RESET: c_int = 50;
pub const CFPB_SPLITTER_RESET: c_int = 51;
pub const TSIF_RESET: c_int = 52;
pub const CE1_RESET: c_int = 53;
pub const CE2_RESET: c_int = 54;
pub const SFAB_SFPB_M_RESET: c_int = 55;
pub const SFAB_SFPB_S_RESET: c_int = 56;
pub const RPM_PROC_RESET: c_int = 57;
pub const RPM_BUS_RESET: c_int = 58;
pub const RPM_MSG_RAM_RESET: c_int = 59;
pub const PMIC_ARB0_RESET: c_int = 60;
pub const PMIC_ARB1_RESET: c_int = 61;
pub const PMIC_SSBI2_RESET: c_int = 62;
pub const SDC1_RESET: c_int = 63;
pub const SDC2_RESET: c_int = 64;
pub const SDC3_RESET: c_int = 65;
pub const SDC4_RESET: c_int = 66;
pub const SDC5_RESET: c_int = 67;
pub const USB_HS1_RESET: c_int = 68;
pub const USB_HS2_XCVR_RESET: c_int = 69;
pub const USB_HS2_RESET: c_int = 70;
pub const USB_FS1_XCVR_RESET: c_int = 71;
pub const USB_FS1_RESET: c_int = 72;
pub const USB_FS2_XCVR_RESET: c_int = 73;
pub const USB_FS2_RESET: c_int = 74;
pub const GSBI1_RESET: c_int = 75;
pub const GSBI2_RESET: c_int = 76;
pub const GSBI3_RESET: c_int = 77;
pub const GSBI4_RESET: c_int = 78;
pub const GSBI5_RESET: c_int = 79;
pub const GSBI6_RESET: c_int = 80;
pub const GSBI7_RESET: c_int = 81;
pub const GSBI8_RESET: c_int = 82;
pub const GSBI9_RESET: c_int = 83;
pub const GSBI10_RESET: c_int = 84;
pub const GSBI11_RESET: c_int = 85;
pub const GSBI12_RESET: c_int = 86;
pub const SPDM_RESET: c_int = 87;
pub const SEC_CTRL_RESET: c_int = 88;
pub const TLMM_H_RESET: c_int = 89;
pub const TLMM_RESET: c_int = 90;
pub const MARRM_PWRON_RESET: c_int = 91;
pub const MARM_RESET: c_int = 92;
pub const MAHB1_RESET: c_int = 93;
pub const SFAB_MSS_S_RESET: c_int = 94;
pub const MAHB2_RESET: c_int = 95;
pub const MODEM_SW_AHB_RESET: c_int = 96;
pub const MODEM_RESET: c_int = 97;
pub const SFAB_MSS_MDM1_RESET: c_int = 98;
pub const SFAB_MSS_MDM0_RESET: c_int = 99;
pub const MSS_SLP_RESET: c_int = 100;
pub const MSS_MARM_SAW_RESET: c_int = 101;
pub const MSS_WDOG_RESET: c_int = 102;
pub const TSSC_RESET: c_int = 103;
pub const PDM_RESET: c_int = 104;
pub const SCSS_CORE0_RESET: c_int = 105;
pub const SCSS_CORE0_POR_RESET: c_int = 106;
pub const SCSS_CORE1_RESET: c_int = 107;
pub const SCSS_CORE1_POR_RESET: c_int = 108;
pub const MPM_RESET: c_int = 109;
pub const EBI1_1X_DIV_RESET: c_int = 110;
pub const EBI1_RESET: c_int = 111;
pub const SFAB_SMPSS_S_RESET: c_int = 112;
pub const USB_PHY0_RESET: c_int = 113;
pub const USB_PHY1_RESET: c_int = 114;
pub const PRNG_RESET: c_int = 115;
