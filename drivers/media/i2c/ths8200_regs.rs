//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/ths8200_regs.h
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


//
// ths8200 - Texas Instruments THS8200 video encoder driver
//
// Copyright 2013 Cisco Systems, Inc. and/or its affiliates.
//
// This program is free software; you may redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; version 2 of the License.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation version 2.
//
// This program is distributed .as is. WITHOUT ANY WARRANTY of any
// kind, whether express or implied; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// Register offset macros
pub const THS8200_VERSION: c_uint = 0x02;
pub const THS8200_CHIP_CTL: c_uint = 0x03;
pub const THS8200_CSC_R11: c_uint = 0x04;
pub const THS8200_CSC_R12: c_uint = 0x05;
pub const THS8200_CSC_R21: c_uint = 0x06;
pub const THS8200_CSC_R22: c_uint = 0x07;
pub const THS8200_CSC_R31: c_uint = 0x08;
pub const THS8200_CSC_R32: c_uint = 0x09;
pub const THS8200_CSC_G11: c_uint = 0x0a;
pub const THS8200_CSC_G12: c_uint = 0x0b;
pub const THS8200_CSC_G21: c_uint = 0x0c;
pub const THS8200_CSC_G22: c_uint = 0x0d;
pub const THS8200_CSC_G31: c_uint = 0x0e;
pub const THS8200_CSC_G32: c_uint = 0x0f;
pub const THS8200_CSC_B11: c_uint = 0x10;
pub const THS8200_CSC_B12: c_uint = 0x11;
pub const THS8200_CSC_B21: c_uint = 0x12;
pub const THS8200_CSC_B22: c_uint = 0x13;
pub const THS8200_CSC_B31: c_uint = 0x14;
pub const THS8200_CSC_B32: c_uint = 0x15;
pub const THS8200_CSC_OFFS1: c_uint = 0x16;
pub const THS8200_CSC_OFFS12: c_uint = 0x17;
pub const THS8200_CSC_OFFS23: c_uint = 0x18;
pub const THS8200_CSC_OFFS3: c_uint = 0x19;
pub const THS8200_TST_CNTL1: c_uint = 0x1a;
pub const THS8200_TST_CNTL2: c_uint = 0x1b;
pub const THS8200_DATA_CNTL: c_uint = 0x1c;
pub const THS8200_DTG1_Y_SYNC1_LSB: c_uint = 0x1d;
pub const THS8200_DTG1_Y_SYNC2_LSB: c_uint = 0x1e;
pub const THS8200_DTG1_Y_SYNC3_LSB: c_uint = 0x1f;
pub const THS8200_DTG1_CBCR_SYNC1_LSB: c_uint = 0x20;
pub const THS8200_DTG1_CBCR_SYNC2_LSB: c_uint = 0x21;
pub const THS8200_DTG1_CBCR_SYNC3_LSB: c_uint = 0x22;
pub const THS8200_DTG1_Y_SYNC_MSB: c_uint = 0x23;
pub const THS8200_DTG1_CBCR_SYNC_MSB: c_uint = 0x24;
pub const THS8200_DTG1_SPEC_A: c_uint = 0x25;
pub const THS8200_DTG1_SPEC_B: c_uint = 0x26;
pub const THS8200_DTG1_SPEC_C: c_uint = 0x27;
pub const THS8200_DTG1_SPEC_D_LSB: c_uint = 0x28;
pub const THS8200_DTG1_SPEC_D1: c_uint = 0x29;
pub const THS8200_DTG1_SPEC_E_LSB: c_uint = 0x2a;
pub const THS8200_DTG1_SPEC_DEH_MSB: c_uint = 0x2b;
pub const THS8200_DTG1_SPEC_H_LSB: c_uint = 0x2c;
pub const THS8200_DTG1_SPEC_I_MSB: c_uint = 0x2d;
pub const THS8200_DTG1_SPEC_I_LSB: c_uint = 0x2e;
pub const THS8200_DTG1_SPEC_K_LSB: c_uint = 0x2f;
pub const THS8200_DTG1_SPEC_K_MSB: c_uint = 0x30;
pub const THS8200_DTG1_SPEC_K1: c_uint = 0x31;
pub const THS8200_DTG1_SPEC_G_LSB: c_uint = 0x32;
pub const THS8200_DTG1_SPEC_G_MSB: c_uint = 0x33;
pub const THS8200_DTG1_TOT_PIXELS_MSB: c_uint = 0x34;
pub const THS8200_DTG1_TOT_PIXELS_LSB: c_uint = 0x35;
pub const THS8200_DTG1_FLD_FLIP_LINECNT_MSB: c_uint = 0x36;
pub const THS8200_DTG1_LINECNT_LSB: c_uint = 0x37;
pub const THS8200_DTG1_MODE: c_uint = 0x38;
pub const THS8200_DTG1_FRAME_FIELD_SZ_MSB: c_uint = 0x39;
pub const THS8200_DTG1_FRAME_SZ_LSB: c_uint = 0x3a;
pub const THS8200_DTG1_FIELD_SZ_LSB: c_uint = 0x3b;
pub const THS8200_DTG1_VESA_CBAR_SIZE: c_uint = 0x3c;
pub const THS8200_DAC_CNTL_MSB: c_uint = 0x3d;
pub const THS8200_DAC1_CNTL_LSB: c_uint = 0x3e;
pub const THS8200_DAC2_CNTL_LSB: c_uint = 0x3f;
pub const THS8200_DAC3_CNTL_LSB: c_uint = 0x40;
pub const THS8200_CSM_CLIP_GY_LOW: c_uint = 0x41;
pub const THS8200_CSM_CLIP_BCB_LOW: c_uint = 0x42;
pub const THS8200_CSM_CLIP_RCR_LOW: c_uint = 0x43;
pub const THS8200_CSM_CLIP_GY_HIGH: c_uint = 0x44;
pub const THS8200_CSM_CLIP_BCB_HIGH: c_uint = 0x45;
pub const THS8200_CSM_CLIP_RCR_HIGH: c_uint = 0x46;
pub const THS8200_CSM_SHIFT_GY: c_uint = 0x47;
pub const THS8200_CSM_SHIFT_BCB: c_uint = 0x48;
pub const THS8200_CSM_SHIFT_RCR: c_uint = 0x49;
pub const THS8200_CSM_GY_CNTL_MULT_MSB: c_uint = 0x4a;
pub const THS8200_CSM_MULT_BCB_RCR_MSB: c_uint = 0x4b;
pub const THS8200_CSM_MULT_GY_LSB: c_uint = 0x4c;
pub const THS8200_CSM_MULT_BCB_LSB: c_uint = 0x4d;
pub const THS8200_CSM_MULT_RCR_LSB: c_uint = 0x4e;
pub const THS8200_CSM_MULT_RCR_BCB_CNTL: c_uint = 0x4f;
pub const THS8200_CSM_MULT_RCR_LSB: c_uint = 0x4e;
pub const THS8200_DTG2_BP1_2_MSB: c_uint = 0x50;
pub const THS8200_DTG2_BP3_4_MSB: c_uint = 0x51;
pub const THS8200_DTG2_BP5_6_MSB: c_uint = 0x52;
pub const THS8200_DTG2_BP7_8_MSB: c_uint = 0x53;
pub const THS8200_DTG2_BP9_10_MSB: c_uint = 0x54;
pub const THS8200_DTG2_BP11_12_MSB: c_uint = 0x55;
pub const THS8200_DTG2_BP13_14_MSB: c_uint = 0x56;
pub const THS8200_DTG2_BP15_16_MSB: c_uint = 0x57;
pub const THS8200_DTG2_BP1_LSB: c_uint = 0x58;
pub const THS8200_DTG2_BP2_LSB: c_uint = 0x59;
pub const THS8200_DTG2_BP3_LSB: c_uint = 0x5a;
pub const THS8200_DTG2_BP4_LSB: c_uint = 0x5b;
pub const THS8200_DTG2_BP5_LSB: c_uint = 0x5c;
pub const THS8200_DTG2_BP6_LSB: c_uint = 0x5d;
pub const THS8200_DTG2_BP7_LSB: c_uint = 0x5e;
pub const THS8200_DTG2_BP8_LSB: c_uint = 0x5f;
pub const THS8200_DTG2_BP9_LSB: c_uint = 0x60;
pub const THS8200_DTG2_BP10_LSB: c_uint = 0x61;
pub const THS8200_DTG2_BP11_LSB: c_uint = 0x62;
pub const THS8200_DTG2_BP12_LSB: c_uint = 0x63;
pub const THS8200_DTG2_BP13_LSB: c_uint = 0x64;
pub const THS8200_DTG2_BP14_LSB: c_uint = 0x65;
pub const THS8200_DTG2_BP15_LSB: c_uint = 0x66;
pub const THS8200_DTG2_BP16_LSB: c_uint = 0x67;
pub const THS8200_DTG2_LINETYPE1: c_uint = 0x68;
pub const THS8200_DTG2_LINETYPE2: c_uint = 0x69;
pub const THS8200_DTG2_LINETYPE3: c_uint = 0x6a;
pub const THS8200_DTG2_LINETYPE4: c_uint = 0x6b;
pub const THS8200_DTG2_LINETYPE5: c_uint = 0x6c;
pub const THS8200_DTG2_LINETYPE6: c_uint = 0x6d;
pub const THS8200_DTG2_LINETYPE7: c_uint = 0x6e;
pub const THS8200_DTG2_LINETYPE8: c_uint = 0x6f;
pub const THS8200_DTG2_HLENGTH_LSB: c_uint = 0x70;
pub const THS8200_DTG2_HLENGTH_LSB_HDLY_MSB: c_uint = 0x71;
pub const THS8200_DTG2_HLENGTH_HDLY_LSB: c_uint = 0x72;
pub const THS8200_DTG2_VLENGTH1_LSB: c_uint = 0x73;
pub const THS8200_DTG2_VLENGTH1_MSB_VDLY1_MSB: c_uint = 0x74;
pub const THS8200_DTG2_VDLY1_LSB: c_uint = 0x75;
pub const THS8200_DTG2_VLENGTH2_LSB: c_uint = 0x76;
pub const THS8200_DTG2_VLENGTH2_MSB_VDLY2_MSB: c_uint = 0x77;
pub const THS8200_DTG2_VDLY2_LSB: c_uint = 0x78;
pub const THS8200_DTG2_HS_IN_DLY_MSB: c_uint = 0x79;
pub const THS8200_DTG2_HS_IN_DLY_LSB: c_uint = 0x7a;
pub const THS8200_DTG2_VS_IN_DLY_MSB: c_uint = 0x7b;
pub const THS8200_DTG2_VS_IN_DLY_LSB: c_uint = 0x7c;
pub const THS8200_DTG2_PIXEL_CNT_MSB: c_uint = 0x7d;
pub const THS8200_DTG2_PIXEL_CNT_LSB: c_uint = 0x7e;
pub const THS8200_DTG2_LINE_CNT_MSB: c_uint = 0x7f;
pub const THS8200_DTG2_LINE_CNT_LSB: c_uint = 0x80;
pub const THS8200_DTG2_CNTL: c_uint = 0x82;
pub const THS8200_CGMS_CNTL_HEADER: c_uint = 0x83;
pub const THS8200_CGMS_PAYLOAD_MSB: c_uint = 0x84;
pub const THS8200_CGMS_PAYLOAD_LSB: c_uint = 0x85;
pub const THS8200_MISC_PPL_LSB: c_uint = 0x86;
pub const THS8200_MISC_PPL_MSB: c_uint = 0x87;
pub const THS8200_MISC_LPF_MSB: c_uint = 0x88;
pub const THS8200_MISC_LPF_LSB: c_uint = 0x89;
