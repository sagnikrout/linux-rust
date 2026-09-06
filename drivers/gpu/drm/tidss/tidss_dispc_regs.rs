//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tidss/tidss_dispc_regs.h
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
// Copyright (C) 2016-2018 Texas Instruments Incorporated - https://www.ti.com
// Author: Jyri Sarha <jsarha@ti.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dispc_common_regs {
    NOT_APPLICABLE_OFF = 0,
    DSS_REVISION_OFF,
    DSS_SYSCONFIG_OFF,
    DSS_SYSSTATUS_OFF,
    DISPC_IRQ_EOI_OFF,
    DISPC_IRQSTATUS_RAW_OFF,
    DISPC_IRQSTATUS_OFF,
    DISPC_IRQENABLE_SET_OFF,
    DISPC_IRQENABLE_CLR_OFF,
    DISPC_VID_IRQENABLE_OFF,
    DISPC_VID_IRQSTATUS_OFF,
    DISPC_VP_IRQENABLE_OFF,
    DISPC_VP_IRQSTATUS_OFF,
    WB_IRQENABLE_OFF,
    WB_IRQSTATUS_OFF,
    DISPC_GLOBAL_MFLAG_ATTRIBUTE_OFF,
    DISPC_GLOBAL_OUTPUT_ENABLE_OFF,
    DISPC_GLOBAL_BUFFER_OFF,
    DSS_CBA_CFG_OFF,
    DISPC_DBG_CONTROL_OFF,
    DISPC_DBG_STATUS_OFF,
    DISPC_CLKGATING_DISABLE_OFF,
    DISPC_SECURE_DISABLE_OFF,
    FBDC_REVISION_1_OFF,
    FBDC_REVISION_2_OFF,
    FBDC_REVISION_3_OFF,
    FBDC_REVISION_4_OFF,
    FBDC_REVISION_5_OFF,
    FBDC_REVISION_6_OFF,
    FBDC_COMMON_CONTROL_OFF,
    FBDC_CONSTANT_COLOR_0_OFF,
    FBDC_CONSTANT_COLOR_1_OFF,
    DISPC_CONNECTIONS_OFF,
    DISPC_MSS_VP1_OFF,
    DISPC_MSS_VP3_OFF,
    DISPC_COMMON_REG_TABLE_LEN,
}

//
// dispc_common_regmap should be defined as const u16 * and pointing
// to a valid dss common register map for the platform, before the
// macros below can be used.
//

// VID
pub const DISPC_VID_ACCUH_0: c_uint = 0x0;
pub const DISPC_VID_ACCUH_1: c_uint = 0x4;
pub const DISPC_VID_ACCUH2_0: c_uint = 0x8;
pub const DISPC_VID_ACCUH2_1: c_uint = 0xc;
pub const DISPC_VID_ACCUV_0: c_uint = 0x10;
pub const DISPC_VID_ACCUV_1: c_uint = 0x14;
pub const DISPC_VID_ACCUV2_0: c_uint = 0x18;
pub const DISPC_VID_ACCUV2_1: c_uint = 0x1c;
pub const DISPC_VID_ATTRIBUTES: c_uint = 0x20;

pub const DISPC_VID_ATTRIBUTES2: c_uint = 0x24;
pub const DISPC_VID_BA_0: c_uint = 0x28;
pub const DISPC_VID_BA_1: c_uint = 0x2c;
pub const DISPC_VID_BA_UV_0: c_uint = 0x30;
pub const DISPC_VID_BA_UV_1: c_uint = 0x34;
pub const DISPC_VID_BUF_SIZE_STATUS: c_uint = 0x38;

pub const DISPC_VID_BUF_THRESHOLD: c_uint = 0x3c;

pub const DISPC_VID_FIRH: c_uint = 0x5c;
pub const DISPC_VID_FIRH2: c_uint = 0x60;
pub const DISPC_VID_FIRV: c_uint = 0x64;
pub const DISPC_VID_FIRV2: c_uint = 0x68;
pub const DISPC_VID_FIR_COEFS_H0: c_uint = 0x6c;

pub const DISPC_VID_FIR_COEFS_H0_C: c_uint = 0x90;

pub const DISPC_VID_FIR_COEFS_H12: c_uint = 0xb4;

pub const DISPC_VID_FIR_COEFS_H12_C: c_uint = 0xf4;

pub const DISPC_VID_FIR_COEFS_V0: c_uint = 0x134;

pub const DISPC_VID_FIR_COEFS_V0_C: c_uint = 0x158;

pub const DISPC_VID_FIR_COEFS_V12: c_uint = 0x17c;

pub const DISPC_VID_FIR_COEFS_V12_C: c_uint = 0x1bc;

pub const DISPC_VID_GLOBAL_ALPHA: c_uint = 0x1fc;

pub const DISPC_VID_K2G_IRQENABLE: c_uint = 0x200 /* K2G */;
pub const DISPC_VID_K2G_IRQSTATUS: c_uint = 0x204 /* K2G */;
pub const DISPC_VID_MFLAG_THRESHOLD: c_uint = 0x208;

pub const DISPC_VID_PICTURE_SIZE: c_uint = 0x20c;

pub const DISPC_VID_PIXEL_INC: c_uint = 0x210;
pub const DISPC_VID_K2G_POSITION: c_uint = 0x214 /* K2G */;
pub const DISPC_VID_PRELOAD: c_uint = 0x218;
pub const DISPC_VID_ROW_INC: c_uint = 0x21c;
pub const DISPC_VID_SIZE: c_uint = 0x220;

pub const DISPC_VID_BA_EXT_0: c_uint = 0x22c;
pub const DISPC_VID_BA_EXT_1: c_uint = 0x230;
pub const DISPC_VID_BA_UV_EXT_0: c_uint = 0x234;
pub const DISPC_VID_BA_UV_EXT_1: c_uint = 0x238;
pub const DISPC_VID_CSC_COEF7: c_uint = 0x23c;
pub const DISPC_VID_ROW_INC_UV: c_uint = 0x248;
pub const DISPC_VID_CLUT: c_uint = 0x260;
pub const DISPC_VID_SAFETY_ATTRIBUTES: c_uint = 0x2a0;
pub const DISPC_VID_SAFETY_CAPT_SIGNATURE: c_uint = 0x2a4;
pub const DISPC_VID_SAFETY_POSITION: c_uint = 0x2a8;
pub const DISPC_VID_SAFETY_REF_SIGNATURE: c_uint = 0x2ac;
pub const DISPC_VID_SAFETY_SIZE: c_uint = 0x2b0;
pub const DISPC_VID_SAFETY_LFSR_SEED: c_uint = 0x2b4;
pub const DISPC_VID_LUMAKEY: c_uint = 0x2b8;
pub const DISPC_VID_DMA_BUFSIZE: c_uint = 0x2bc /* J721E */;
// OVR
pub const DISPC_OVR_CONFIG: c_uint = 0x0;
pub const DISPC_OVR_VIRTVP: c_uint = 0x4 /* J721E */;
pub const DISPC_OVR_DEFAULT_COLOR: c_uint = 0x8;
pub const DISPC_OVR_DEFAULT_COLOR2: c_uint = 0xc;
pub const DISPC_OVR_TRANS_COLOR_MAX: c_uint = 0x10;
pub const DISPC_OVR_TRANS_COLOR_MAX2: c_uint = 0x14;
pub const DISPC_OVR_TRANS_COLOR_MIN: c_uint = 0x18;
pub const DISPC_OVR_TRANS_COLOR_MIN2: c_uint = 0x1c;

// VP
pub const DISPC_VP_CONFIG: c_uint = 0x0;

pub const DISPC_VP_CONTROL: c_uint = 0x4;

pub const DISPC_VP_CSC_COEF0: c_uint = 0x8;
pub const DISPC_VP_CSC_COEF1: c_uint = 0xc;
pub const DISPC_VP_CSC_COEF2: c_uint = 0x10;
pub const DISPC_VP_DATA_CYCLE_0: c_uint = 0x14;
pub const DISPC_VP_DATA_CYCLE_1: c_uint = 0x18;
pub const DISPC_VP_K2G_GAMMA_TABLE: c_uint = 0x20 /* K2G */;
pub const DISPC_VP_K2G_IRQENABLE: c_uint = 0x3c /* K2G */;
pub const DISPC_VP_K2G_IRQSTATUS: c_uint = 0x40 /* K2G */;
pub const DISPC_VP_DATA_CYCLE_2: c_uint = 0x1c;
pub const DISPC_VP_LINE_NUMBER: c_uint = 0x44;
pub const DISPC_VP_POL_FREQ: c_uint = 0x4c;

pub const DISPC_VP_SIZE_SCREEN: c_uint = 0x50;

pub const DISPC_VP_TIMING_H: c_uint = 0x54;

pub const DISPC_VP_TIMING_V: c_uint = 0x58;

pub const DISPC_VP_CSC_COEF3: c_uint = 0x5c;
pub const DISPC_VP_CSC_COEF4: c_uint = 0x60;
pub const DISPC_VP_CSC_COEF5: c_uint = 0x64;
pub const DISPC_VP_CSC_COEF6: c_uint = 0x68;
pub const DISPC_VP_CSC_COEF7: c_uint = 0x6c;
pub const DISPC_VP_SAFETY_ATTRIBUTES_0: c_uint = 0x70;
pub const DISPC_VP_SAFETY_ATTRIBUTES_1: c_uint = 0x74;
pub const DISPC_VP_SAFETY_ATTRIBUTES_2: c_uint = 0x78;
pub const DISPC_VP_SAFETY_ATTRIBUTES_3: c_uint = 0x7c;
pub const DISPC_VP_SAFETY_CAPT_SIGNATURE_0: c_uint = 0x90;
pub const DISPC_VP_SAFETY_CAPT_SIGNATURE_1: c_uint = 0x94;
pub const DISPC_VP_SAFETY_CAPT_SIGNATURE_2: c_uint = 0x98;
pub const DISPC_VP_SAFETY_CAPT_SIGNATURE_3: c_uint = 0x9c;
pub const DISPC_VP_SAFETY_POSITION_0: c_uint = 0xb0;
pub const DISPC_VP_SAFETY_POSITION_1: c_uint = 0xb4;
pub const DISPC_VP_SAFETY_POSITION_2: c_uint = 0xb8;
pub const DISPC_VP_SAFETY_POSITION_3: c_uint = 0xbc;
pub const DISPC_VP_SAFETY_REF_SIGNATURE_0: c_uint = 0xd0;
pub const DISPC_VP_SAFETY_REF_SIGNATURE_1: c_uint = 0xd4;
pub const DISPC_VP_SAFETY_REF_SIGNATURE_2: c_uint = 0xd8;
pub const DISPC_VP_SAFETY_REF_SIGNATURE_3: c_uint = 0xdc;
pub const DISPC_VP_SAFETY_SIZE_0: c_uint = 0xf0;
pub const DISPC_VP_SAFETY_SIZE_1: c_uint = 0xf4;
pub const DISPC_VP_SAFETY_SIZE_2: c_uint = 0xf8;
pub const DISPC_VP_SAFETY_SIZE_3: c_uint = 0xfc;
pub const DISPC_VP_SAFETY_LFSR_SEED: c_uint = 0x110;
pub const DISPC_VP_GAMMA_TABLE: c_uint = 0x120;
pub const DISPC_VP_DSS_OLDI_CFG: c_uint = 0x160;

pub const DISPC_VP_DSS_OLDI_STATUS: c_uint = 0x164;
pub const DISPC_VP_DSS_OLDI_LB: c_uint = 0x168;
pub const DISPC_VP_DSS_MERGE_SPLIT: c_uint = 0x16c /* J721E */;
pub const DISPC_VP_DSS_DMA_THREADSIZE: c_uint = 0x170 /* J721E */;
pub const DISPC_VP_DSS_DMA_THREADSIZE_STATUS: c_uint = 0x174 /* J721E */;
// OLDI Config Bits (DISPC_VP_DSS_OLDI_CFG)

// LVDS Format values for OLDI_MAP field in DISPC_VP_OLDI_CFG register
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum oldi_mode_reg_val {

//
// OLDI IO_CTRL register offsets. On AM654 the registers are found
// from CTRL_MMR0, there the syscon regmap should map 0x14 bytes from
// CTRLMMR0P1_OLDI_DAT0_IO_CTRL to CTRLMMR0P1_OLDI_CLK_IO_CTRL
// register range.
//
pub const AM65X_OLDI_DAT0_IO_CTRL: c_uint = 0x00;
pub const AM65X_OLDI_DAT1_IO_CTRL: c_uint = 0x04;
pub const AM65X_OLDI_DAT2_IO_CTRL: c_uint = 0x08;
pub const AM65X_OLDI_DAT3_IO_CTRL: c_uint = 0x0C;
pub const AM65X_OLDI_CLK_IO_CTRL: c_uint = 0x10;

