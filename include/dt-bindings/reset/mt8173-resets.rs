//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/mt8173-resets.h
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
// Copyright (c) 2014 MediaTek Inc.
// Author: Flora Fu, MediaTek
//
// INFRACFG resets
pub const MT8173_INFRA_EMI_REG_RST: c_int = 0;
pub const MT8173_INFRA_DRAMC0_A0_RST: c_int = 1;
pub const MT8173_INFRA_APCIRQ_EINT_RST: c_int = 3;
pub const MT8173_INFRA_APXGPT_RST: c_int = 4;
pub const MT8173_INFRA_SCPSYS_RST: c_int = 5;
pub const MT8173_INFRA_KP_RST: c_int = 6;
pub const MT8173_INFRA_PMIC_WRAP_RST: c_int = 7;
pub const MT8173_INFRA_MPIP_RST: c_int = 8;
pub const MT8173_INFRA_CEC_RST: c_int = 9;
pub const MT8173_INFRA_EMI_RST: c_int = 32;
pub const MT8173_INFRA_DRAMC0_RST: c_int = 34;
pub const MT8173_INFRA_APMIXEDSYS_RST: c_int = 35;
pub const MT8173_INFRA_MIPI_DSI_RST: c_int = 36;
pub const MT8173_INFRA_TRNG_RST: c_int = 37;
pub const MT8173_INFRA_SYSIRQ_RST: c_int = 38;
pub const MT8173_INFRA_MIPI_CSI_RST: c_int = 39;
pub const MT8173_INFRA_GCE_FAXI_RST: c_int = 40;
pub const MT8173_INFRA_MMIOMMURST: c_int = 47;
// MMSYS resets
pub const MT8173_MMSYS_SW0_RST_B_DISP_DSI0: c_int = 25;
// PERICFG resets
pub const MT8173_PERI_UART0_SW_RST: c_int = 0;
pub const MT8173_PERI_UART1_SW_RST: c_int = 1;
pub const MT8173_PERI_UART2_SW_RST: c_int = 2;
pub const MT8173_PERI_UART3_SW_RST: c_int = 3;
pub const MT8173_PERI_IRRX_SW_RST: c_int = 4;
pub const MT8173_PERI_PWM_SW_RST: c_int = 8;
pub const MT8173_PERI_AUXADC_SW_RST: c_int = 10;
pub const MT8173_PERI_DMA_SW_RST: c_int = 11;
pub const MT8173_PERI_I2C6_SW_RST: c_int = 13;
pub const MT8173_PERI_NFI_SW_RST: c_int = 14;
pub const MT8173_PERI_THERM_SW_RST: c_int = 16;
pub const MT8173_PERI_MSDC2_SW_RST: c_int = 17;
pub const MT8173_PERI_MSDC3_SW_RST: c_int = 18;
pub const MT8173_PERI_MSDC0_SW_RST: c_int = 19;
pub const MT8173_PERI_MSDC1_SW_RST: c_int = 20;
pub const MT8173_PERI_I2C0_SW_RST: c_int = 22;
pub const MT8173_PERI_I2C1_SW_RST: c_int = 23;
pub const MT8173_PERI_I2C2_SW_RST: c_int = 24;
pub const MT8173_PERI_I2C3_SW_RST: c_int = 25;
pub const MT8173_PERI_I2C4_SW_RST: c_int = 26;
pub const MT8173_PERI_HDMI_SW_RST: c_int = 29;
pub const MT8173_PERI_SPI0_SW_RST: c_int = 33;
