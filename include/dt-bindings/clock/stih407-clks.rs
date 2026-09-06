//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/stih407-clks.h
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
// This header provides constants clk index STMicroelectronics
// STiH407 SoC.
//
// CLOCKGEN A0
pub const CLK_IC_LMI0: c_int = 0;
pub const CLK_IC_LMI1: c_int = 1;
// CLOCKGEN C0
pub const CLK_ICN_GPU: c_int = 0;
pub const CLK_FDMA: c_int = 1;
pub const CLK_NAND: c_int = 2;
pub const CLK_HVA: c_int = 3;
pub const CLK_PROC_STFE: c_int = 4;
pub const CLK_PROC_TP: c_int = 5;
pub const CLK_RX_ICN_DMU: c_int = 6;
pub const CLK_RX_ICN_DISP_0: c_int = 6;
pub const CLK_RX_ICN_DISP_1: c_int = 6;
pub const CLK_RX_ICN_HVA: c_int = 7;
pub const CLK_RX_ICN_TS: c_int = 7;
pub const CLK_ICN_CPU: c_int = 8;
pub const CLK_TX_ICN_DMU: c_int = 9;
pub const CLK_TX_ICN_HVA: c_int = 9;
pub const CLK_TX_ICN_TS: c_int = 9;
pub const CLK_ICN_COMPO: c_int = 9;
pub const CLK_MMC_0: c_int = 10;
pub const CLK_MMC_1: c_int = 11;
pub const CLK_JPEGDEC: c_int = 12;
pub const CLK_ICN_REG: c_int = 13;
pub const CLK_TRACE_A9: c_int = 13;
pub const CLK_PTI_STM: c_int = 13;
pub const CLK_EXT2F_A9: c_int = 13;
pub const CLK_IC_BDISP_0: c_int = 14;
pub const CLK_IC_BDISP_1: c_int = 15;
pub const CLK_PP_DMU: c_int = 16;
pub const CLK_VID_DMU: c_int = 17;
pub const CLK_DSS_LPC: c_int = 18;
pub const CLK_ST231_AUD_0: c_int = 19;
pub const CLK_ST231_GP_0: c_int = 19;
pub const CLK_ST231_GP_1: c_int = 20;
pub const CLK_ST231_DMU: c_int = 21;
pub const CLK_ICN_LMI: c_int = 22;
pub const CLK_TX_ICN_DISP_0: c_int = 23;
pub const CLK_TX_ICN_DISP_1: c_int = 23;
pub const CLK_ICN_SBC: c_int = 24;
pub const CLK_STFE_FRC2: c_int = 25;
pub const CLK_ETH_PHY: c_int = 26;
pub const CLK_ETH_REF_PHYCLK: c_int = 27;
pub const CLK_FLASH_PROMIP: c_int = 28;
pub const CLK_MAIN_DISP: c_int = 29;
pub const CLK_AUX_DISP: c_int = 30;
pub const CLK_COMPO_DVP: c_int = 31;
// CLOCKGEN D0
pub const CLK_PCM_0: c_int = 0;
pub const CLK_PCM_1: c_int = 1;
pub const CLK_PCM_2: c_int = 2;
pub const CLK_SPDIFF: c_int = 3;
// CLOCKGEN D2
pub const CLK_PIX_MAIN_DISP: c_int = 0;
pub const CLK_PIX_PIP: c_int = 1;
pub const CLK_PIX_GDP1: c_int = 2;
pub const CLK_PIX_GDP2: c_int = 3;
pub const CLK_PIX_GDP3: c_int = 4;
pub const CLK_PIX_GDP4: c_int = 5;
pub const CLK_PIX_AUX_DISP: c_int = 6;
pub const CLK_DENC: c_int = 7;
pub const CLK_PIX_HDDAC: c_int = 8;
pub const CLK_HDDAC: c_int = 9;
pub const CLK_SDDAC: c_int = 10;
pub const CLK_PIX_DVO: c_int = 11;
pub const CLK_DVO: c_int = 12;
pub const CLK_PIX_HDMI: c_int = 13;
pub const CLK_TMDS_HDMI: c_int = 14;
pub const CLK_REF_HDMIPHY: c_int = 15;
// CLOCKGEN D3
pub const CLK_STFE_FRC1: c_int = 0;
pub const CLK_TSOUT_0: c_int = 1;
pub const CLK_TSOUT_1: c_int = 2;
pub const CLK_MCHI: c_int = 3;
pub const CLK_VSENS_COMPO: c_int = 4;
pub const CLK_FRC1_REMOTE: c_int = 5;
pub const CLK_LPC_0: c_int = 6;
pub const CLK_LPC_1: c_int = 7;
