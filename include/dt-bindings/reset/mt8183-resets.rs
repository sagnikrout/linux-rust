//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/mt8183-resets.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2019 MediaTek Inc.
// Author: Yong Liang <yong.liang@mediatek.com>
//
// INFRACFG AO resets
pub const MT8183_INFRACFG_AO_THERM_SW_RST: c_int = 0;
pub const MT8183_INFRACFG_AO_USB_TOP_SW_RST: c_int = 1;
pub const MT8183_INFRACFG_AO_MM_IOMMU_SW_RST: c_int = 3;
pub const MT8183_INFRACFG_AO_MSDC3_SW_RST: c_int = 4;
pub const MT8183_INFRACFG_AO_MSDC2_SW_RST: c_int = 5;
pub const MT8183_INFRACFG_AO_MSDC1_SW_RST: c_int = 6;
pub const MT8183_INFRACFG_AO_MSDC0_SW_RST: c_int = 7;
pub const MT8183_INFRACFG_AO_APDMA_SW_RST: c_int = 9;
pub const MT8183_INFRACFG_AO_MIMP_D_SW_RST: c_int = 10;
pub const MT8183_INFRACFG_AO_BTIF_SW_RST: c_int = 12;
pub const MT8183_INFRACFG_AO_DISP_PWM_SW_RST: c_int = 14;
pub const MT8183_INFRACFG_AO_AUXADC_SW_RST: c_int = 15;
pub const MT8183_INFRACFG_AO_IRTX_SW_RST: c_int = 32;
pub const MT8183_INFRACFG_AO_SPI0_SW_RST: c_int = 33;
pub const MT8183_INFRACFG_AO_I2C0_SW_RST: c_int = 34;
pub const MT8183_INFRACFG_AO_I2C1_SW_RST: c_int = 35;
pub const MT8183_INFRACFG_AO_I2C2_SW_RST: c_int = 36;
pub const MT8183_INFRACFG_AO_I2C3_SW_RST: c_int = 37;
pub const MT8183_INFRACFG_AO_UART0_SW_RST: c_int = 38;
pub const MT8183_INFRACFG_AO_UART1_SW_RST: c_int = 39;
pub const MT8183_INFRACFG_AO_UART2_SW_RST: c_int = 40;
pub const MT8183_INFRACFG_AO_PWM_SW_RST: c_int = 41;
pub const MT8183_INFRACFG_AO_SPI1_SW_RST: c_int = 42;
pub const MT8183_INFRACFG_AO_I2C4_SW_RST: c_int = 43;
pub const MT8183_INFRACFG_AO_DVFSP_SW_RST: c_int = 44;
pub const MT8183_INFRACFG_AO_SPI2_SW_RST: c_int = 45;
pub const MT8183_INFRACFG_AO_SPI3_SW_RST: c_int = 46;
pub const MT8183_INFRACFG_AO_UFSHCI_SW_RST: c_int = 47;
pub const MT8183_INFRACFG_AO_PMIC_WRAP_SW_RST: c_int = 64;
pub const MT8183_INFRACFG_AO_SPM_SW_RST: c_int = 65;
pub const MT8183_INFRACFG_AO_USBSIF_SW_RST: c_int = 66;
pub const MT8183_INFRACFG_AO_KP_SW_RST: c_int = 68;
pub const MT8183_INFRACFG_AO_APXGPT_SW_RST: c_int = 69;
pub const MT8183_INFRACFG_AO_CLDMA_AO_SW_RST: c_int = 70;
pub const MT8183_INFRACFG_AO_UNIPRO_UFS_SW_RST: c_int = 71;
pub const MT8183_INFRACFG_AO_DX_CC_SW_RST: c_int = 72;
pub const MT8183_INFRACFG_AO_UFSPHY_SW_RST: c_int = 73;
pub const MT8183_INFRACFG_AO_DX_CC_SEC_SW_RST: c_int = 96;
pub const MT8183_INFRACFG_AO_GCE_SW_RST: c_int = 97;
pub const MT8183_INFRACFG_AO_CLDMA_SW_RST: c_int = 98;
pub const MT8183_INFRACFG_AO_TRNG_SW_RST: c_int = 99;
pub const MT8183_INFRACFG_AO_AP_MD_CCIF_1_SW_RST: c_int = 103;
pub const MT8183_INFRACFG_AO_AP_MD_CCIF_SW_RST: c_int = 104;
pub const MT8183_INFRACFG_AO_I2C1_IMM_SW_RST: c_int = 105;
pub const MT8183_INFRACFG_AO_I2C1_ARB_SW_RST: c_int = 106;
pub const MT8183_INFRACFG_AO_I2C2_IMM_SW_RST: c_int = 107;
pub const MT8183_INFRACFG_AO_I2C2_ARB_SW_RST: c_int = 108;
pub const MT8183_INFRACFG_AO_I2C5_SW_RST: c_int = 109;
pub const MT8183_INFRACFG_AO_I2C5_IMM_SW_RST: c_int = 110;
pub const MT8183_INFRACFG_AO_I2C5_ARB_SW_RST: c_int = 111;
pub const MT8183_INFRACFG_AO_SPI4_SW_RST: c_int = 112;
pub const MT8183_INFRACFG_AO_SPI5_SW_RST: c_int = 113;
pub const MT8183_INFRACFG_AO_INFRA2MFGAXI_CBIP_CLAS_SW_RST: c_int = 114;
pub const MT8183_INFRACFG_AO_MFGAXI2INFRA_M0_CBIP_GLAS_OUT_SW_RST: c_int = 115;
pub const MT8183_INFRACFG_AO_MFGAXI2INFRA_M1_CBIP_GLAS_OUT_SW_RST: c_int = 116;
pub const MT8183_INFRACFG_AO_UFS_AES_SW_RST: c_int = 117;
pub const MT8183_INFRACFG_AO_CCU_I2C_IRQ_SW_RST: c_int = 118;
pub const MT8183_INFRACFG_AO_CCU_I2C_DMA_SW_RST: c_int = 119;
pub const MT8183_INFRACFG_AO_I2C6_SW_RST: c_int = 120;
pub const MT8183_INFRACFG_AO_CCU_GALS_SW_RST: c_int = 121;
pub const MT8183_INFRACFG_AO_IPU_GALS_SW_RST: c_int = 122;
pub const MT8183_INFRACFG_AO_CONN2AP_GALS_SW_RST: c_int = 123;
pub const MT8183_INFRACFG_AO_AP_MD_CCIF2_SW_RST: c_int = 124;
pub const MT8183_INFRACFG_AO_AP_MD_CCIF3_SW_RST: c_int = 125;
pub const MT8183_INFRACFG_AO_I2C7_SW_RST: c_int = 126;
pub const MT8183_INFRACFG_AO_I2C8_SW_RST: c_int = 127;
pub const MT8183_INFRACFG_SW_RST_NUM: c_int = 128;
// MMSYS resets
pub const MT8183_MMSYS_SW0_RST_B_DISP_DSI0: c_int = 25;
pub const MT8183_TOPRGU_MM_SW_RST: c_int = 1;
pub const MT8183_TOPRGU_MFG_SW_RST: c_int = 2;
pub const MT8183_TOPRGU_VENC_SW_RST: c_int = 3;
pub const MT8183_TOPRGU_VDEC_SW_RST: c_int = 4;
pub const MT8183_TOPRGU_IMG_SW_RST: c_int = 5;
pub const MT8183_TOPRGU_MD_SW_RST: c_int = 7;
pub const MT8183_TOPRGU_CONN_SW_RST: c_int = 9;
pub const MT8183_TOPRGU_CONN_MCU_SW_RST: c_int = 12;
pub const MT8183_TOPRGU_IPU0_SW_RST: c_int = 14;
pub const MT8183_TOPRGU_IPU1_SW_RST: c_int = 15;
pub const MT8183_TOPRGU_AUDIO_SW_RST: c_int = 17;
pub const MT8183_TOPRGU_CAMSYS_SW_RST: c_int = 18;
pub const MT8183_TOPRGU_SW_RST_NUM: c_int = 19;
