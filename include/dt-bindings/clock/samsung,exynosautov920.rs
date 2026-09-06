//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/samsung,exynosautov920.h
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
// Copyright (c) 2024 Samsung Electronics Co., Ltd.
// Author: Sunyeal Hong <sunyeal.hong@samsung.com>
//
// Device Tree binding constants for ExynosAuto v920 clock controller.
//
// CMU_TOP
pub const FOUT_SHARED0_PLL: c_int = 1;
pub const FOUT_SHARED1_PLL: c_int = 2;
pub const FOUT_SHARED2_PLL: c_int = 3;
pub const FOUT_SHARED3_PLL: c_int = 4;
pub const FOUT_SHARED4_PLL: c_int = 5;
pub const FOUT_SHARED5_PLL: c_int = 6;
pub const FOUT_MMC_PLL: c_int = 7;
// MUX in CMU_TOP
pub const MOUT_SHARED0_PLL: c_int = 8;
pub const MOUT_SHARED1_PLL: c_int = 9;
pub const MOUT_SHARED2_PLL: c_int = 10;
pub const MOUT_SHARED3_PLL: c_int = 11;
pub const MOUT_SHARED4_PLL: c_int = 12;
pub const MOUT_SHARED5_PLL: c_int = 13;
pub const MOUT_MMC_PLL: c_int = 14;
pub const MOUT_CLKCMU_CMU_BOOST: c_int = 15;
pub const MOUT_CLKCMU_CMU_CMUREF: c_int = 16;
pub const MOUT_CLKCMU_ACC_NOC: c_int = 17;
pub const MOUT_CLKCMU_ACC_ORB: c_int = 18;
pub const MOUT_CLKCMU_APM_NOC: c_int = 19;
pub const MOUT_CLKCMU_AUD_CPU: c_int = 20;
pub const MOUT_CLKCMU_AUD_NOC: c_int = 21;
pub const MOUT_CLKCMU_CPUCL0_SWITCH: c_int = 22;
pub const MOUT_CLKCMU_CPUCL0_CLUSTER: c_int = 23;
pub const MOUT_CLKCMU_CPUCL0_DBG: c_int = 24;
pub const MOUT_CLKCMU_CPUCL1_SWITCH: c_int = 25;
pub const MOUT_CLKCMU_CPUCL1_CLUSTER: c_int = 26;
pub const MOUT_CLKCMU_CPUCL2_SWITCH: c_int = 27;
pub const MOUT_CLKCMU_CPUCL2_CLUSTER: c_int = 28;
pub const MOUT_CLKCMU_DNC_NOC: c_int = 29;
pub const MOUT_CLKCMU_DPTX_NOC: c_int = 30;
pub const MOUT_CLKCMU_DPTX_DPGTC: c_int = 31;
pub const MOUT_CLKCMU_DPTX_DPOSC: c_int = 32;
pub const MOUT_CLKCMU_DPUB_NOC: c_int = 33;
pub const MOUT_CLKCMU_DPUB_DSIM: c_int = 34;
pub const MOUT_CLKCMU_DPUF0_NOC: c_int = 35;
pub const MOUT_CLKCMU_DPUF1_NOC: c_int = 36;
pub const MOUT_CLKCMU_DPUF2_NOC: c_int = 37;
pub const MOUT_CLKCMU_DSP_NOC: c_int = 38;
pub const MOUT_CLKCMU_G3D_SWITCH: c_int = 39;
pub const MOUT_CLKCMU_G3D_NOCP: c_int = 40;
pub const MOUT_CLKCMU_GNPU_NOC: c_int = 41;
pub const MOUT_CLKCMU_HSI0_NOC: c_int = 42;
pub const MOUT_CLKCMU_HSI1_NOC: c_int = 43;
pub const MOUT_CLKCMU_HSI1_USBDRD: c_int = 44;
pub const MOUT_CLKCMU_HSI1_MMC_CARD: c_int = 45;
pub const MOUT_CLKCMU_HSI2_NOC: c_int = 46;
pub const MOUT_CLKCMU_HSI2_NOC_UFS: c_int = 47;
pub const MOUT_CLKCMU_HSI2_UFS_EMBD: c_int = 48;
pub const MOUT_CLKCMU_HSI2_ETHERNET: c_int = 49;
pub const MOUT_CLKCMU_ISP_NOC: c_int = 50;
pub const MOUT_CLKCMU_M2M_NOC: c_int = 51;
pub const MOUT_CLKCMU_M2M_JPEG: c_int = 52;
pub const MOUT_CLKCMU_MFC_MFC: c_int = 53;
pub const MOUT_CLKCMU_MFC_WFD: c_int = 54;
pub const MOUT_CLKCMU_MFD_NOC: c_int = 55;
pub const MOUT_CLKCMU_MIF_SWITCH: c_int = 56;
pub const MOUT_CLKCMU_MIF_NOCP: c_int = 57;
pub const MOUT_CLKCMU_MISC_NOC: c_int = 58;
pub const MOUT_CLKCMU_NOCL0_NOC: c_int = 59;
pub const MOUT_CLKCMU_NOCL1_NOC: c_int = 60;
pub const MOUT_CLKCMU_NOCL2_NOC: c_int = 61;
pub const MOUT_CLKCMU_PERIC0_NOC: c_int = 62;
pub const MOUT_CLKCMU_PERIC0_IP: c_int = 63;
pub const MOUT_CLKCMU_PERIC1_NOC: c_int = 64;
pub const MOUT_CLKCMU_PERIC1_IP: c_int = 65;
pub const MOUT_CLKCMU_SDMA_NOC: c_int = 66;
pub const MOUT_CLKCMU_SNW_NOC: c_int = 67;
pub const MOUT_CLKCMU_SSP_NOC: c_int = 68;
pub const MOUT_CLKCMU_TAA_NOC: c_int = 69;
// DIV in CMU_TOP
pub const DOUT_SHARED0_DIV1: c_int = 70;
pub const DOUT_SHARED0_DIV2: c_int = 71;
pub const DOUT_SHARED0_DIV3: c_int = 72;
pub const DOUT_SHARED0_DIV4: c_int = 73;
pub const DOUT_SHARED1_DIV1: c_int = 74;
pub const DOUT_SHARED1_DIV2: c_int = 75;
pub const DOUT_SHARED1_DIV3: c_int = 76;
pub const DOUT_SHARED1_DIV4: c_int = 77;
pub const DOUT_SHARED2_DIV1: c_int = 78;
pub const DOUT_SHARED2_DIV2: c_int = 79;
pub const DOUT_SHARED2_DIV3: c_int = 80;
pub const DOUT_SHARED2_DIV4: c_int = 81;
pub const DOUT_SHARED3_DIV1: c_int = 82;
pub const DOUT_SHARED3_DIV2: c_int = 83;
pub const DOUT_SHARED3_DIV3: c_int = 84;
pub const DOUT_SHARED3_DIV4: c_int = 85;
pub const DOUT_SHARED4_DIV1: c_int = 86;
pub const DOUT_SHARED4_DIV2: c_int = 87;
pub const DOUT_SHARED4_DIV3: c_int = 88;
pub const DOUT_SHARED4_DIV4: c_int = 89;
pub const DOUT_SHARED5_DIV1: c_int = 90;
pub const DOUT_SHARED5_DIV2: c_int = 91;
pub const DOUT_SHARED5_DIV3: c_int = 92;
pub const DOUT_SHARED5_DIV4: c_int = 93;
pub const DOUT_CLKCMU_CMU_BOOST: c_int = 94;
pub const DOUT_CLKCMU_ACC_NOC: c_int = 95;
pub const DOUT_CLKCMU_ACC_ORB: c_int = 96;
pub const DOUT_CLKCMU_APM_NOC: c_int = 97;
pub const DOUT_CLKCMU_AUD_CPU: c_int = 98;
pub const DOUT_CLKCMU_AUD_NOC: c_int = 99;
pub const DOUT_CLKCMU_CPUCL0_SWITCH: c_int = 100;
pub const DOUT_CLKCMU_CPUCL0_CLUSTER: c_int = 101;
pub const DOUT_CLKCMU_CPUCL0_DBG: c_int = 102;
pub const DOUT_CLKCMU_CPUCL1_SWITCH: c_int = 103;
pub const DOUT_CLKCMU_CPUCL1_CLUSTER: c_int = 104;
pub const DOUT_CLKCMU_CPUCL2_SWITCH: c_int = 105;
pub const DOUT_CLKCMU_CPUCL2_CLUSTER: c_int = 106;
pub const DOUT_CLKCMU_DNC_NOC: c_int = 107;
pub const DOUT_CLKCMU_DPTX_NOC: c_int = 108;
pub const DOUT_CLKCMU_DPTX_DPGTC: c_int = 109;
pub const DOUT_CLKCMU_DPTX_DPOSC: c_int = 110;
pub const DOUT_CLKCMU_DPUB_NOC: c_int = 111;
pub const DOUT_CLKCMU_DPUB_DSIM: c_int = 112;
pub const DOUT_CLKCMU_DPUF0_NOC: c_int = 113;
pub const DOUT_CLKCMU_DPUF1_NOC: c_int = 114;
pub const DOUT_CLKCMU_DPUF2_NOC: c_int = 115;
pub const DOUT_CLKCMU_DSP_NOC: c_int = 116;
pub const DOUT_CLKCMU_G3D_SWITCH: c_int = 117;
pub const DOUT_CLKCMU_G3D_NOCP: c_int = 118;
pub const DOUT_CLKCMU_GNPU_NOC: c_int = 119;
pub const DOUT_CLKCMU_HSI0_NOC: c_int = 120;
pub const DOUT_CLKCMU_HSI1_NOC: c_int = 121;
pub const DOUT_CLKCMU_HSI1_USBDRD: c_int = 122;
pub const DOUT_CLKCMU_HSI1_MMC_CARD: c_int = 123;
pub const DOUT_CLKCMU_HSI2_NOC: c_int = 124;
pub const DOUT_CLKCMU_HSI2_NOC_UFS: c_int = 125;
pub const DOUT_CLKCMU_HSI2_UFS_EMBD: c_int = 126;
pub const DOUT_CLKCMU_HSI2_ETHERNET: c_int = 127;
pub const DOUT_CLKCMU_ISP_NOC: c_int = 128;
pub const DOUT_CLKCMU_M2M_NOC: c_int = 129;
pub const DOUT_CLKCMU_M2M_JPEG: c_int = 130;
pub const DOUT_CLKCMU_MFC_MFC: c_int = 131;
pub const DOUT_CLKCMU_MFC_WFD: c_int = 132;
pub const DOUT_CLKCMU_MFD_NOC: c_int = 133;
pub const DOUT_CLKCMU_MIF_NOCP: c_int = 134;
pub const DOUT_CLKCMU_MISC_NOC: c_int = 135;
pub const DOUT_CLKCMU_NOCL0_NOC: c_int = 136;
pub const DOUT_CLKCMU_NOCL1_NOC: c_int = 137;
pub const DOUT_CLKCMU_NOCL2_NOC: c_int = 138;
pub const DOUT_CLKCMU_PERIC0_NOC: c_int = 139;
pub const DOUT_CLKCMU_PERIC0_IP: c_int = 140;
pub const DOUT_CLKCMU_PERIC1_NOC: c_int = 141;
pub const DOUT_CLKCMU_PERIC1_IP: c_int = 142;
pub const DOUT_CLKCMU_SDMA_NOC: c_int = 143;
pub const DOUT_CLKCMU_SNW_NOC: c_int = 144;
pub const DOUT_CLKCMU_SSP_NOC: c_int = 145;
pub const DOUT_CLKCMU_TAA_NOC: c_int = 146;
pub const DOUT_TCXO_DIV2: c_int = 147;
// CMU_CPUCL0
pub const CLK_FOUT_CPUCL0_PLL: c_int = 1;
pub const CLK_MOUT_PLL_CPUCL0: c_int = 2;
pub const CLK_MOUT_CPUCL0_CLUSTER_USER: c_int = 3;
pub const CLK_MOUT_CPUCL0_DBG_USER: c_int = 4;
pub const CLK_MOUT_CPUCL0_SWITCH_USER: c_int = 5;
pub const CLK_MOUT_CPUCL0_CLUSTER: c_int = 6;
pub const CLK_MOUT_CPUCL0_CORE: c_int = 7;
pub const CLK_DOUT_CLUSTER0_ACLK: c_int = 8;
pub const CLK_DOUT_CLUSTER0_ATCLK: c_int = 9;
pub const CLK_DOUT_CLUSTER0_MPCLK: c_int = 10;
pub const CLK_DOUT_CLUSTER0_PCLK: c_int = 11;
pub const CLK_DOUT_CLUSTER0_PERIPHCLK: c_int = 12;
pub const CLK_DOUT_CPUCL0_DBG_NOC: c_int = 13;
pub const CLK_DOUT_CPUCL0_DBG_PCLKDBG: c_int = 14;
pub const CLK_DOUT_CPUCL0_NOCP: c_int = 15;
// CMU_CPUCL1
pub const CLK_FOUT_CPUCL1_PLL: c_int = 1;
pub const CLK_MOUT_PLL_CPUCL1: c_int = 2;
pub const CLK_MOUT_CPUCL1_CLUSTER_USER: c_int = 3;
pub const CLK_MOUT_CPUCL1_SWITCH_USER: c_int = 4;
pub const CLK_MOUT_CPUCL1_CLUSTER: c_int = 5;
pub const CLK_MOUT_CPUCL1_CORE: c_int = 6;
pub const CLK_DOUT_CLUSTER1_ACLK: c_int = 7;
pub const CLK_DOUT_CLUSTER1_ATCLK: c_int = 8;
pub const CLK_DOUT_CLUSTER1_MPCLK: c_int = 9;
pub const CLK_DOUT_CLUSTER1_PCLK: c_int = 10;
pub const CLK_DOUT_CLUSTER1_PERIPHCLK: c_int = 11;
pub const CLK_DOUT_CPUCL1_NOCP: c_int = 12;
// CMU_CPUCL2
pub const CLK_FOUT_CPUCL2_PLL: c_int = 1;
pub const CLK_MOUT_PLL_CPUCL2: c_int = 2;
pub const CLK_MOUT_CPUCL2_CLUSTER_USER: c_int = 3;
pub const CLK_MOUT_CPUCL2_SWITCH_USER: c_int = 4;
pub const CLK_MOUT_CPUCL2_CLUSTER: c_int = 5;
pub const CLK_MOUT_CPUCL2_CORE: c_int = 6;
pub const CLK_DOUT_CLUSTER2_ACLK: c_int = 7;
pub const CLK_DOUT_CLUSTER2_ATCLK: c_int = 8;
pub const CLK_DOUT_CLUSTER2_MPCLK: c_int = 9;
pub const CLK_DOUT_CLUSTER2_PCLK: c_int = 10;
pub const CLK_DOUT_CLUSTER2_PERIPHCLK: c_int = 11;
pub const CLK_DOUT_CPUCL2_NOCP: c_int = 12;
// CMU_PERIC0
pub const CLK_MOUT_PERIC0_IP_USER: c_int = 1;
pub const CLK_MOUT_PERIC0_NOC_USER: c_int = 2;
pub const CLK_MOUT_PERIC0_USI00_USI: c_int = 3;
pub const CLK_MOUT_PERIC0_USI01_USI: c_int = 4;
pub const CLK_MOUT_PERIC0_USI02_USI: c_int = 5;
pub const CLK_MOUT_PERIC0_USI03_USI: c_int = 6;
pub const CLK_MOUT_PERIC0_USI04_USI: c_int = 7;
pub const CLK_MOUT_PERIC0_USI05_USI: c_int = 8;
pub const CLK_MOUT_PERIC0_USI06_USI: c_int = 9;
pub const CLK_MOUT_PERIC0_USI07_USI: c_int = 10;
pub const CLK_MOUT_PERIC0_USI08_USI: c_int = 11;
pub const CLK_MOUT_PERIC0_USI_I2C: c_int = 12;
pub const CLK_MOUT_PERIC0_I3C: c_int = 13;
pub const CLK_DOUT_PERIC0_USI00_USI: c_int = 14;
pub const CLK_DOUT_PERIC0_USI01_USI: c_int = 15;
pub const CLK_DOUT_PERIC0_USI02_USI: c_int = 16;
pub const CLK_DOUT_PERIC0_USI03_USI: c_int = 17;
pub const CLK_DOUT_PERIC0_USI04_USI: c_int = 18;
pub const CLK_DOUT_PERIC0_USI05_USI: c_int = 19;
pub const CLK_DOUT_PERIC0_USI06_USI: c_int = 20;
pub const CLK_DOUT_PERIC0_USI07_USI: c_int = 21;
pub const CLK_DOUT_PERIC0_USI08_USI: c_int = 22;
pub const CLK_DOUT_PERIC0_USI_I2C: c_int = 23;
pub const CLK_DOUT_PERIC0_I3C: c_int = 24;
// CMU_PERIC1
pub const CLK_MOUT_PERIC1_IP_USER: c_int = 1;
pub const CLK_MOUT_PERIC1_NOC_USER: c_int = 2;
pub const CLK_MOUT_PERIC1_USI09_USI: c_int = 3;
pub const CLK_MOUT_PERIC1_USI10_USI: c_int = 4;
pub const CLK_MOUT_PERIC1_USI11_USI: c_int = 5;
pub const CLK_MOUT_PERIC1_USI12_USI: c_int = 6;
pub const CLK_MOUT_PERIC1_USI13_USI: c_int = 7;
pub const CLK_MOUT_PERIC1_USI14_USI: c_int = 8;
pub const CLK_MOUT_PERIC1_USI15_USI: c_int = 9;
pub const CLK_MOUT_PERIC1_USI16_USI: c_int = 10;
pub const CLK_MOUT_PERIC1_USI17_USI: c_int = 11;
pub const CLK_MOUT_PERIC1_USI_I2C: c_int = 12;
pub const CLK_MOUT_PERIC1_I3C: c_int = 13;
pub const CLK_DOUT_PERIC1_USI09_USI: c_int = 14;
pub const CLK_DOUT_PERIC1_USI10_USI: c_int = 15;
pub const CLK_DOUT_PERIC1_USI11_USI: c_int = 16;
pub const CLK_DOUT_PERIC1_USI12_USI: c_int = 17;
pub const CLK_DOUT_PERIC1_USI13_USI: c_int = 18;
pub const CLK_DOUT_PERIC1_USI14_USI: c_int = 19;
pub const CLK_DOUT_PERIC1_USI15_USI: c_int = 20;
pub const CLK_DOUT_PERIC1_USI16_USI: c_int = 21;
pub const CLK_DOUT_PERIC1_USI17_USI: c_int = 22;
pub const CLK_DOUT_PERIC1_USI_I2C: c_int = 23;
pub const CLK_DOUT_PERIC1_I3C: c_int = 24;
// CMU_MISC
pub const CLK_MOUT_MISC_NOC_USER: c_int = 1;
pub const CLK_MOUT_MISC_GIC: c_int = 2;
pub const CLK_DOUT_MISC_OTP: c_int = 3;
pub const CLK_DOUT_MISC_NOCP: c_int = 4;
pub const CLK_DOUT_MISC_OSC_DIV2: c_int = 5;
// CMU_HSI0
pub const CLK_MOUT_HSI0_NOC_USER: c_int = 1;
pub const CLK_DOUT_HSI0_PCIE_APB: c_int = 2;
// CMU_HSI1
pub const CLK_MOUT_HSI1_MMC_CARD_USER: c_int = 1;
pub const CLK_MOUT_HSI1_NOC_USER: c_int = 2;
pub const CLK_MOUT_HSI1_USBDRD_USER: c_int = 3;
pub const CLK_MOUT_HSI1_USBDRD: c_int = 4;
// CMU_HSI2
pub const FOUT_PLL_ETH: c_int = 1;
pub const CLK_MOUT_HSI2_NOC_UFS_USER: c_int = 2;
pub const CLK_MOUT_HSI2_UFS_EMBD_USER: c_int = 3;
pub const CLK_MOUT_HSI2_ETHERNET: c_int = 4;
pub const CLK_MOUT_HSI2_ETHERNET_USER: c_int = 5;
pub const CLK_DOUT_HSI2_ETHERNET: c_int = 6;
pub const CLK_DOUT_HSI2_ETHERNET_PTP: c_int = 7;
// CMU_M2M
pub const CLK_MOUT_M2M_JPEG_USER: c_int = 1;
pub const CLK_MOUT_M2M_NOC_USER: c_int = 2;
pub const CLK_DOUT_M2M_NOCP: c_int = 3;
// CMU_MFC
pub const CLK_MOUT_MFC_MFC_USER: c_int = 1;
pub const CLK_MOUT_MFC_WFD_USER: c_int = 2;
pub const CLK_DOUT_MFC_NOCP: c_int = 3;
// CMU_MFD
pub const CLK_MOUT_MFD_NOC_USER: c_int = 1;
pub const CLK_DOUT_MFD_NOCP: c_int = 2;
// CMU_G3D
pub const FOUT_PLL_G3D: c_int = 1;
pub const CLK_MOUT_G3D_NOC: c_int = 2;
pub const CLK_MOUT_G3D_SWITCH_USER: c_int = 3;
pub const CLK_MOUT_G3D_NOCP_USER: c_int = 4;
