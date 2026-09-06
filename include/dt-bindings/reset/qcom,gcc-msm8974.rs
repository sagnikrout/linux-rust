//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/qcom,gcc-msm8974.h
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
pub const GCC_SYSTEM_NOC_BCR: c_int = 0;
pub const GCC_CONFIG_NOC_BCR: c_int = 1;
pub const GCC_PERIPH_NOC_BCR: c_int = 2;
pub const GCC_IMEM_BCR: c_int = 3;
pub const GCC_MMSS_BCR: c_int = 4;
pub const GCC_QDSS_BCR: c_int = 5;
pub const GCC_USB_30_BCR: c_int = 6;
pub const GCC_USB3_PHY_BCR: c_int = 7;
pub const GCC_USB_HS_HSIC_BCR: c_int = 8;
pub const GCC_USB_HS_BCR: c_int = 9;
pub const GCC_USB2A_PHY_BCR: c_int = 10;
pub const GCC_USB2B_PHY_BCR: c_int = 11;
pub const GCC_SDCC1_BCR: c_int = 12;
pub const GCC_SDCC2_BCR: c_int = 13;
pub const GCC_SDCC3_BCR: c_int = 14;
pub const GCC_SDCC4_BCR: c_int = 15;
pub const GCC_BLSP1_BCR: c_int = 16;
pub const GCC_BLSP1_QUP1_BCR: c_int = 17;
pub const GCC_BLSP1_UART1_BCR: c_int = 18;
pub const GCC_BLSP1_QUP2_BCR: c_int = 19;
pub const GCC_BLSP1_UART2_BCR: c_int = 20;
pub const GCC_BLSP1_QUP3_BCR: c_int = 21;
pub const GCC_BLSP1_UART3_BCR: c_int = 22;
pub const GCC_BLSP1_QUP4_BCR: c_int = 23;
pub const GCC_BLSP1_UART4_BCR: c_int = 24;
pub const GCC_BLSP1_QUP5_BCR: c_int = 25;
pub const GCC_BLSP1_UART5_BCR: c_int = 26;
pub const GCC_BLSP1_QUP6_BCR: c_int = 27;
pub const GCC_BLSP1_UART6_BCR: c_int = 28;
pub const GCC_BLSP2_BCR: c_int = 29;
pub const GCC_BLSP2_QUP1_BCR: c_int = 30;
pub const GCC_BLSP2_UART1_BCR: c_int = 31;
pub const GCC_BLSP2_QUP2_BCR: c_int = 32;
pub const GCC_BLSP2_UART2_BCR: c_int = 33;
pub const GCC_BLSP2_QUP3_BCR: c_int = 34;
pub const GCC_BLSP2_UART3_BCR: c_int = 35;
pub const GCC_BLSP2_QUP4_BCR: c_int = 36;
pub const GCC_BLSP2_UART4_BCR: c_int = 37;
pub const GCC_BLSP2_QUP5_BCR: c_int = 38;
pub const GCC_BLSP2_UART5_BCR: c_int = 39;
pub const GCC_BLSP2_QUP6_BCR: c_int = 40;
pub const GCC_BLSP2_UART6_BCR: c_int = 41;
pub const GCC_PDM_BCR: c_int = 42;
pub const GCC_BAM_DMA_BCR: c_int = 43;
pub const GCC_TSIF_BCR: c_int = 44;
pub const GCC_TCSR_BCR: c_int = 45;
pub const GCC_BOOT_ROM_BCR: c_int = 46;
pub const GCC_MSG_RAM_BCR: c_int = 47;
pub const GCC_TLMM_BCR: c_int = 48;
pub const GCC_MPM_BCR: c_int = 49;
pub const GCC_SEC_CTRL_BCR: c_int = 50;
pub const GCC_SPMI_BCR: c_int = 51;
pub const GCC_SPDM_BCR: c_int = 52;
pub const GCC_CE1_BCR: c_int = 53;
pub const GCC_CE2_BCR: c_int = 54;
pub const GCC_BIMC_BCR: c_int = 55;
pub const GCC_MPM_NON_AHB_RESET: c_int = 56;
pub const GCC_MPM_AHB_RESET: c_int = 57;
pub const GCC_SNOC_BUS_TIMEOUT0_BCR: c_int = 58;
pub const GCC_SNOC_BUS_TIMEOUT2_BCR: c_int = 59;
pub const GCC_PNOC_BUS_TIMEOUT0_BCR: c_int = 60;
pub const GCC_PNOC_BUS_TIMEOUT1_BCR: c_int = 61;
pub const GCC_PNOC_BUS_TIMEOUT2_BCR: c_int = 62;
pub const GCC_PNOC_BUS_TIMEOUT3_BCR: c_int = 63;
pub const GCC_PNOC_BUS_TIMEOUT4_BCR: c_int = 64;
pub const GCC_CNOC_BUS_TIMEOUT0_BCR: c_int = 65;
pub const GCC_CNOC_BUS_TIMEOUT1_BCR: c_int = 66;
pub const GCC_CNOC_BUS_TIMEOUT2_BCR: c_int = 67;
pub const GCC_CNOC_BUS_TIMEOUT3_BCR: c_int = 68;
pub const GCC_CNOC_BUS_TIMEOUT4_BCR: c_int = 69;
pub const GCC_CNOC_BUS_TIMEOUT5_BCR: c_int = 70;
pub const GCC_CNOC_BUS_TIMEOUT6_BCR: c_int = 71;
pub const GCC_DEHR_BCR: c_int = 72;
pub const GCC_RBCPR_BCR: c_int = 73;
pub const GCC_MSS_RESTART: c_int = 74;
pub const GCC_LPASS_RESTART: c_int = 75;
pub const GCC_WCSS_RESTART: c_int = 76;
pub const GCC_VENUS_RESTART: c_int = 77;
