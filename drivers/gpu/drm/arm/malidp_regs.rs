//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/arm/malidp_regs.h
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
// (C) COPYRIGHT 2016 ARM Limited. All rights reserved.
// Author: Liviu Dudau <Liviu.Dudau@arm.com>
//
// ARM Mali DP500/DP550/DP650 registers definition.
//
// abbreviations used:
// - DC - display core (general settings)
// - DE - display engine
// - SE - scaling engine
//
// interrupt bit masks

// bit masks that are common between products

// register offsets for IRQ management
pub const MALIDP_REG_STATUS: c_uint = 0x00000;
pub const MALIDP_REG_SETIRQ: c_uint = 0x00004;
pub const MALIDP_REG_MASKIRQ: c_uint = 0x00008;
pub const MALIDP_REG_CLEARIRQ: c_uint = 0x0000c;
// register offsets
pub const MALIDP_DE_CORE_ID: c_uint = 0x00018;
pub const MALIDP_DE_DISPLAY_FUNC: c_uint = 0x00020;
// these offsets are relative to MALIDP5x0_TIMINGS_BASE
pub const MALIDP_DE_H_TIMINGS: c_uint = 0x0;
pub const MALIDP_DE_V_TIMINGS: c_uint = 0x4;
pub const MALIDP_DE_SYNC_WIDTH: c_uint = 0x8;
pub const MALIDP_DE_HV_ACTIVE: c_uint = 0xc;
// Stride register offsets relative to Lx_BASE
pub const MALIDP_DE_LG_STRIDE: c_uint = 0x18;
pub const MALIDP_DE_LV_STRIDE0: c_uint = 0x18;
pub const MALIDP550_DE_LS_R1_STRIDE: c_uint = 0x28;
// macros to set values into registers

// register offsets relative to MALIDP5x0_COEFFS_BASE
pub const MALIDP_COLOR_ADJ_COEF: c_uint = 0x00000;
pub const MALIDP_COEF_TABLE_ADDR: c_uint = 0x00030;
pub const MALIDP_COEF_TABLE_DATA: c_uint = 0x00034;
// Scaling engine registers and masks.

pub const MALIDP_SE_ENH_MASK: c_int = 3;

pub const MALIDP550_SE_CTL_SEL_MASK: c_int = 7;

// Blocks with offsets from SE_CONTROL register.
pub const MALIDP_SE_LAYER_CONTROL: c_uint = 0x14;
pub const MALIDP_SE_L0_IN_SIZE: c_uint = 0x00;
pub const MALIDP_SE_L0_OUT_SIZE: c_uint = 0x04;

pub const MALIDP_SE_SCALING_CONTROL: c_uint = 0x24;
pub const MALIDP_SE_H_INIT_PH: c_uint = 0x00;
pub const MALIDP_SE_H_DELTA_PH: c_uint = 0x04;
pub const MALIDP_SE_V_INIT_PH: c_uint = 0x08;
pub const MALIDP_SE_V_DELTA_PH: c_uint = 0x0c;
pub const MALIDP_SE_COEFFTAB_ADDR: c_uint = 0x10;
pub const MALIDP_SE_COEFFTAB_ADDR_MASK: c_uint = 0x7f;

pub const MALIDP_SE_COEFFTAB_DATA: c_uint = 0x14;
pub const MALIDP_SE_COEFFTAB_DATA_MASK: c_uint = 0x3fff;

// Enhance coefficients register offset
pub const MALIDP_SE_IMAGE_ENH: c_uint = 0x3C;
// ENH_LIMITS offset 0x0
pub const MALIDP_SE_ENH_LOW_LEVEL: c_int = 24;
pub const MALIDP_SE_ENH_HIGH_LEVEL: c_int = 63;
pub const MALIDP_SE_ENH_LIMIT_MASK: c_uint = 0xfff;

pub const MALIDP_SE_ENH_COEFF0: c_uint = 0x04;
// register offsets relative to MALIDP5x0_SE_MEMWRITE_BASE
pub const MALIDP_MW_FORMAT: c_uint = 0x00000;
pub const MALIDP_MW_P1_STRIDE: c_uint = 0x00004;
pub const MALIDP_MW_P2_STRIDE: c_uint = 0x00008;
pub const MALIDP_MW_P1_PTR_LOW: c_uint = 0x0000c;
pub const MALIDP_MW_P1_PTR_HIGH: c_uint = 0x00010;
pub const MALIDP_MW_P2_PTR_LOW: c_uint = 0x0002c;
pub const MALIDP_MW_P2_PTR_HIGH: c_uint = 0x00030;
// register offsets and bits specific to DP500
pub const MALIDP500_ADDR_SPACE_SIZE: c_uint = 0x01000;
pub const MALIDP500_DC_BASE: c_uint = 0x00000;
pub const MALIDP500_DC_CONTROL: c_uint = 0x0000c;

pub const MALIDP500_DC_CLEAR_MASK: c_uint = 0x300fff;
pub const MALIDP500_DE_LINE_COUNTER: c_uint = 0x00010;
pub const MALIDP500_DE_AXI_CONTROL: c_uint = 0x00014;
pub const MALIDP500_DE_SECURE_CTRL: c_uint = 0x0001c;
pub const MALIDP500_DE_CHROMA_KEY: c_uint = 0x00024;
pub const MALIDP500_TIMINGS_BASE: c_uint = 0x00028;
pub const MALIDP500_CONFIG_3D: c_uint = 0x00038;
pub const MALIDP500_BGND_COLOR: c_uint = 0x0003c;
pub const MALIDP500_OUTPUT_DEPTH: c_uint = 0x00044;
pub const MALIDP500_COEFFS_BASE: c_uint = 0x00078;
//
// The YUV2RGB coefficients on the DP500 are not in the video layer's register
// block. They belong in a separate block above the layer's registers, hence
// the negative offset.
//

pub const MALIDP500_DE_LV_BASE: c_uint = 0x00100;
pub const MALIDP500_DE_LV_PTR_BASE: c_uint = 0x00124;
pub const MALIDP500_DE_LV_AD_CTRL: c_uint = 0x00400;
pub const MALIDP500_DE_LG1_BASE: c_uint = 0x00200;
pub const MALIDP500_DE_LG1_PTR_BASE: c_uint = 0x0021c;
pub const MALIDP500_DE_LG1_AD_CTRL: c_uint = 0x0040c;
pub const MALIDP500_DE_LG2_BASE: c_uint = 0x00300;
pub const MALIDP500_DE_LG2_PTR_BASE: c_uint = 0x0031c;
pub const MALIDP500_DE_LG2_AD_CTRL: c_uint = 0x00418;
pub const MALIDP500_SE_BASE: c_uint = 0x00c00;
pub const MALIDP500_SE_CONTROL: c_uint = 0x00c0c;
pub const MALIDP500_SE_MEMWRITE_OUT_SIZE: c_uint = 0x00c2c;
pub const MALIDP500_SE_RGB_YUV_COEFFS: c_uint = 0x00C74;
pub const MALIDP500_SE_MEMWRITE_BASE: c_uint = 0x00e00;
pub const MALIDP500_DC_IRQ_BASE: c_uint = 0x00f00;
pub const MALIDP500_CONFIG_VALID: c_uint = 0x00f00;
pub const MALIDP500_CONFIG_ID: c_uint = 0x00fd4;
//
// The quality of service (QoS) register on the DP500. RQOS register values
// are driven by the ARQOS signal, using AXI transacations, dependent on the
// FIFO input level.
// The RQOS register can also set QoS levels for:
// - RED_ARQOS   @ A 4-bit signal value for close to underflow conditions
// - GREEN_ARQOS @ A 4-bit signal value for normal conditions
//
pub const MALIDP500_RQOS_QUALITY: c_uint = 0x00500;
// register offsets and bits specific to DP550/DP650
pub const MALIDP550_ADDR_SPACE_SIZE: c_uint = 0x10000;
pub const MALIDP550_DE_CONTROL: c_uint = 0x00010;
pub const MALIDP550_DE_LINE_COUNTER: c_uint = 0x00014;
pub const MALIDP550_DE_AXI_CONTROL: c_uint = 0x00018;
pub const MALIDP550_DE_QOS: c_uint = 0x0001c;
pub const MALIDP550_TIMINGS_BASE: c_uint = 0x00030;

pub const MALIDP550_DE_DISP_SIDEBAND: c_uint = 0x00040;
pub const MALIDP550_DE_BGND_COLOR: c_uint = 0x00044;
pub const MALIDP550_DE_OUTPUT_DEPTH: c_uint = 0x0004c;
pub const MALIDP550_COEFFS_BASE: c_uint = 0x00050;
pub const MALIDP550_LV_YUV2RGB: c_uint = 0x00084;
pub const MALIDP550_DE_LV1_BASE: c_uint = 0x00100;
pub const MALIDP550_DE_LV1_PTR_BASE: c_uint = 0x00124;
pub const MALIDP550_DE_LV1_AD_CTRL: c_uint = 0x001B8;
pub const MALIDP550_DE_LV2_BASE: c_uint = 0x00200;
pub const MALIDP550_DE_LV2_PTR_BASE: c_uint = 0x00224;
pub const MALIDP550_DE_LV2_AD_CTRL: c_uint = 0x002B8;
pub const MALIDP550_DE_LG_BASE: c_uint = 0x00300;
pub const MALIDP550_DE_LG_PTR_BASE: c_uint = 0x0031c;
pub const MALIDP550_DE_LG_AD_CTRL: c_uint = 0x00330;
pub const MALIDP550_DE_LS_BASE: c_uint = 0x00400;
pub const MALIDP550_DE_LS_PTR_BASE: c_uint = 0x0042c;
pub const MALIDP550_DE_PERF_BASE: c_uint = 0x00500;
pub const MALIDP550_SE_BASE: c_uint = 0x08000;
pub const MALIDP550_SE_CONTROL: c_uint = 0x08010;

pub const MALIDP550_SE_MEMWRITE_OUT_SIZE: c_uint = 0x08030;
pub const MALIDP550_SE_RGB_YUV_COEFFS: c_uint = 0x08078;
pub const MALIDP550_SE_MEMWRITE_BASE: c_uint = 0x08100;
pub const MALIDP550_DC_BASE: c_uint = 0x0c000;
pub const MALIDP550_DC_CONTROL: c_uint = 0x0c010;

pub const MALIDP550_CONFIG_VALID: c_uint = 0x0c014;
pub const MALIDP550_CONFIG_ID: c_uint = 0x0ffd4;
// register offsets specific to DP650
pub const MALIDP650_DE_LV_MMU_CTRL: c_uint = 0x000D0;
pub const MALIDP650_DE_LG_MMU_CTRL: c_uint = 0x00048;
pub const MALIDP650_DE_LS_MMU_CTRL: c_uint = 0x00078;
// bit masks to set the MMU control register

// AFBC register offsets relative to MALIDPXXX_DE_LX_AD_CTRL
// The following register offsets are common for DP500, DP550 and DP650
pub const MALIDP_AD_CROP_H: c_uint = 0x4;
pub const MALIDP_AD_CROP_V: c_uint = 0x8;
pub const MALIDP_AD_END_PTR_LOW: c_uint = 0xc;
pub const MALIDP_AD_END_PTR_HIGH: c_uint = 0x10;
// AFBC decoder Registers

pub const MALIDP_AD_CROP_RIGHT_OFFSET: c_int = 16;
pub const MALIDP_AD_CROP_BOTTOM_OFFSET: c_int = 16;
//
// Starting with DP550 the register map blocks has been standardised to the
// following layout:
//
// Offset            Block registers
// 0x00000            Display Engine
// 0x08000            Scaling Engine
// 0x0c000            Display Core
// 0x10000            Secure control
//
// The old DP500 IP mixes some DC with the DE registers, hence the need
// for a mapping structure.
//
