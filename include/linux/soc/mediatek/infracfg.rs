//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/mediatek/infracfg.h
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
pub const MT8365_INFRA_TOPAXI_PROTECTEN_STA1: c_uint = 0x228;
pub const MT8365_INFRA_TOPAXI_PROTECTEN_SET: c_uint = 0x2a0;
pub const MT8365_INFRA_TOPAXI_PROTECTEN_CLR: c_uint = 0x2a4;

pub const MT8365_INFRA_TOPAXI_PROTECTEN_1_STA1: c_uint = 0x258;
pub const MT8365_INFRA_TOPAXI_PROTECTEN_1_SET: c_uint = 0x2a8;
pub const MT8365_INFRA_TOPAXI_PROTECTEN_1_CLR: c_uint = 0x2ac;

pub const MT8365_INFRA_NAO_TOPAXI_SI0_STA: c_uint = 0x0;

pub const MT8365_INFRA_NAO_TOPAXI_SI2_STA: c_uint = 0x28;

pub const MT8365_INFRA_TOPAXI_SI0_CTL: c_uint = 0x200;

pub const MT8365_INFRA_TOPAXI_SI2_CTL: c_uint = 0x234;

pub const MT8365_SMI_COMMON_CLAMP_EN: c_uint = 0x3c0;
pub const MT8365_SMI_COMMON_CLAMP_EN_SET: c_uint = 0x3c4;
pub const MT8365_SMI_COMMON_CLAMP_EN_CLR: c_uint = 0x3c8;
pub const MT8195_TOP_AXI_PROT_EN_STA1: c_uint = 0x228;
pub const MT8195_TOP_AXI_PROT_EN_1_STA1: c_uint = 0x258;
pub const MT8195_TOP_AXI_PROT_EN_SET: c_uint = 0x2a0;
pub const MT8195_TOP_AXI_PROT_EN_CLR: c_uint = 0x2a4;
pub const MT8195_TOP_AXI_PROT_EN_1_SET: c_uint = 0x2a8;
pub const MT8195_TOP_AXI_PROT_EN_1_CLR: c_uint = 0x2ac;
pub const MT8195_TOP_AXI_PROT_EN_MM_SET: c_uint = 0x2d4;
pub const MT8195_TOP_AXI_PROT_EN_MM_CLR: c_uint = 0x2d8;
pub const MT8195_TOP_AXI_PROT_EN_MM_STA1: c_uint = 0x2ec;
pub const MT8195_TOP_AXI_PROT_EN_2_SET: c_uint = 0x714;
pub const MT8195_TOP_AXI_PROT_EN_2_CLR: c_uint = 0x718;
pub const MT8195_TOP_AXI_PROT_EN_2_STA1: c_uint = 0x724;
pub const MT8195_TOP_AXI_PROT_EN_VDNR_SET: c_uint = 0xb84;
pub const MT8195_TOP_AXI_PROT_EN_VDNR_CLR: c_uint = 0xb88;
pub const MT8195_TOP_AXI_PROT_EN_VDNR_STA1: c_uint = 0xb90;
pub const MT8195_TOP_AXI_PROT_EN_VDNR_1_SET: c_uint = 0xba4;
pub const MT8195_TOP_AXI_PROT_EN_VDNR_1_CLR: c_uint = 0xba8;
pub const MT8195_TOP_AXI_PROT_EN_VDNR_1_STA1: c_uint = 0xbb0;
pub const MT8195_TOP_AXI_PROT_EN_VDNR_2_SET: c_uint = 0xbb8;
pub const MT8195_TOP_AXI_PROT_EN_VDNR_2_CLR: c_uint = 0xbbc;
pub const MT8195_TOP_AXI_PROT_EN_VDNR_2_STA1: c_uint = 0xbc4;
pub const MT8195_TOP_AXI_PROT_EN_SUB_INFRA_VDNR_SET: c_uint = 0xbcc;
pub const MT8195_TOP_AXI_PROT_EN_SUB_INFRA_VDNR_CLR: c_uint = 0xbd0;
pub const MT8195_TOP_AXI_PROT_EN_SUB_INFRA_VDNR_STA1: c_uint = 0xbd8;
pub const MT8195_TOP_AXI_PROT_EN_MM_2_SET: c_uint = 0xdcc;
pub const MT8195_TOP_AXI_PROT_EN_MM_2_CLR: c_uint = 0xdd0;
pub const MT8195_TOP_AXI_PROT_EN_MM_2_STA1: c_uint = 0xdd8;

pub const MT8192_TOP_AXI_PROT_EN_STA1: c_uint = 0x228;
pub const MT8192_TOP_AXI_PROT_EN_1_STA1: c_uint = 0x258;
pub const MT8192_TOP_AXI_PROT_EN_SET: c_uint = 0x2a0;
pub const MT8192_TOP_AXI_PROT_EN_CLR: c_uint = 0x2a4;
pub const MT8192_TOP_AXI_PROT_EN_1_SET: c_uint = 0x2a8;
pub const MT8192_TOP_AXI_PROT_EN_1_CLR: c_uint = 0x2ac;
pub const MT8192_TOP_AXI_PROT_EN_MM_SET: c_uint = 0x2d4;
pub const MT8192_TOP_AXI_PROT_EN_MM_CLR: c_uint = 0x2d8;
pub const MT8192_TOP_AXI_PROT_EN_MM_STA1: c_uint = 0x2ec;
pub const MT8192_TOP_AXI_PROT_EN_2_SET: c_uint = 0x714;
pub const MT8192_TOP_AXI_PROT_EN_2_CLR: c_uint = 0x718;
pub const MT8192_TOP_AXI_PROT_EN_2_STA1: c_uint = 0x724;
pub const MT8192_TOP_AXI_PROT_EN_VDNR_SET: c_uint = 0xb84;
pub const MT8192_TOP_AXI_PROT_EN_VDNR_CLR: c_uint = 0xb88;
pub const MT8192_TOP_AXI_PROT_EN_VDNR_STA1: c_uint = 0xb90;
pub const MT8192_TOP_AXI_PROT_EN_MM_2_SET: c_uint = 0xdcc;
pub const MT8192_TOP_AXI_PROT_EN_MM_2_CLR: c_uint = 0xdd0;
pub const MT8192_TOP_AXI_PROT_EN_MM_2_STA1: c_uint = 0xdd8;

pub const MT8188_TOP_AXI_PROT_EN_SET: c_uint = 0x2A0;
pub const MT8188_TOP_AXI_PROT_EN_CLR: c_uint = 0x2A4;
pub const MT8188_TOP_AXI_PROT_EN_STA: c_uint = 0x228;
pub const MT8188_TOP_AXI_PROT_EN_1_SET: c_uint = 0x2A8;
pub const MT8188_TOP_AXI_PROT_EN_1_CLR: c_uint = 0x2AC;
pub const MT8188_TOP_AXI_PROT_EN_1_STA: c_uint = 0x258;
pub const MT8188_TOP_AXI_PROT_EN_2_SET: c_uint = 0x714;
pub const MT8188_TOP_AXI_PROT_EN_2_CLR: c_uint = 0x718;
pub const MT8188_TOP_AXI_PROT_EN_2_STA: c_uint = 0x724;
pub const MT8188_TOP_AXI_PROT_EN_MM_SET: c_uint = 0x2D4;
pub const MT8188_TOP_AXI_PROT_EN_MM_CLR: c_uint = 0x2D8;
pub const MT8188_TOP_AXI_PROT_EN_MM_STA: c_uint = 0x2EC;
pub const MT8188_TOP_AXI_PROT_EN_MM_2_SET: c_uint = 0xDCC;
pub const MT8188_TOP_AXI_PROT_EN_MM_2_CLR: c_uint = 0xDD0;
pub const MT8188_TOP_AXI_PROT_EN_MM_2_STA: c_uint = 0xDD8;
pub const MT8188_TOP_AXI_PROT_EN_INFRA_VDNR_SET: c_uint = 0xB84;
pub const MT8188_TOP_AXI_PROT_EN_INFRA_VDNR_CLR: c_uint = 0xB88;
pub const MT8188_TOP_AXI_PROT_EN_INFRA_VDNR_STA: c_uint = 0xB90;
pub const MT8188_TOP_AXI_PROT_EN_SUB_INFRA_VDNR_SET: c_uint = 0xBCC;
pub const MT8188_TOP_AXI_PROT_EN_SUB_INFRA_VDNR_CLR: c_uint = 0xBD0;
pub const MT8188_TOP_AXI_PROT_EN_SUB_INFRA_VDNR_STA: c_uint = 0xBD8;

pub const MT8188_SMI_COMMON_CLAMP_EN_STA: c_uint = 0x3C0;
pub const MT8188_SMI_COMMON_CLAMP_EN_SET: c_uint = 0x3C4;
pub const MT8188_SMI_COMMON_CLAMP_EN_CLR: c_uint = 0x3C8;

pub const MT8188_SMI_LARB10_RESET_ADDR: c_uint = 0xC;
pub const MT8188_SMI_LARB11A_RESET_ADDR: c_uint = 0xC;
pub const MT8188_SMI_LARB11C_RESET_ADDR: c_uint = 0xC;
pub const MT8188_SMI_LARB12_RESET_ADDR: c_uint = 0xC;
pub const MT8188_SMI_LARB11B_RESET_ADDR: c_uint = 0xC;
pub const MT8188_SMI_LARB15_RESET_ADDR: c_uint = 0xC;
pub const MT8188_SMI_LARB16B_RESET_ADDR: c_uint = 0xA0;
pub const MT8188_SMI_LARB17B_RESET_ADDR: c_uint = 0xA0;
pub const MT8188_SMI_LARB16A_RESET_ADDR: c_uint = 0xA0;
pub const MT8188_SMI_LARB17A_RESET_ADDR: c_uint = 0xA0;

// MFG1

// DIS

// IMG

// IPE

// CAM

// VENC

// VDEC

// WPE

// CONN_ON

// ADSP_TOP

pub const MT8183_TOP_AXI_PROT_EN_STA1: c_uint = 0x228;
pub const MT8183_TOP_AXI_PROT_EN_STA1_1: c_uint = 0x258;
pub const MT8183_TOP_AXI_PROT_EN_SET: c_uint = 0x2a0;
pub const MT8183_TOP_AXI_PROT_EN_CLR: c_uint = 0x2a4;
pub const MT8183_TOP_AXI_PROT_EN_1_SET: c_uint = 0x2a8;
pub const MT8183_TOP_AXI_PROT_EN_1_CLR: c_uint = 0x2ac;
pub const MT8183_TOP_AXI_PROT_EN_MCU_SET: c_uint = 0x2c4;
pub const MT8183_TOP_AXI_PROT_EN_MCU_CLR: c_uint = 0x2c8;
pub const MT8183_TOP_AXI_PROT_EN_MCU_STA1: c_uint = 0x2e4;
pub const MT8183_TOP_AXI_PROT_EN_MM_SET: c_uint = 0x2d4;
pub const MT8183_TOP_AXI_PROT_EN_MM_CLR: c_uint = 0x2d8;
pub const MT8183_TOP_AXI_PROT_EN_MM_STA1: c_uint = 0x2ec;

pub const MT8183_SMI_COMMON_CLAMP_EN: c_uint = 0x3c0;
pub const MT8183_SMI_COMMON_CLAMP_EN_SET: c_uint = 0x3c4;
pub const MT8183_SMI_COMMON_CLAMP_EN_CLR: c_uint = 0x3c8;

pub const INFRA_TOPAXI_PROTECTEN: c_uint = 0x0220;
pub const INFRA_TOPAXI_PROTECTSTA1: c_uint = 0x0228;
pub const INFRA_TOPAXI_PROTECTEN_SET: c_uint = 0x0260;
pub const INFRA_TOPAXI_PROTECTEN_CLR: c_uint = 0x0264;
pub const MT8192_INFRA_CTRL: c_uint = 0x290;

pub const REG_INFRA_MISC: c_uint = 0xf00;

