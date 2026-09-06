//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/include/dal_asic_id.h
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
// Copyright 2012-15 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//
// ASIC internal revision ID
//
// DCE60 (based on si_id.h in GPUOpen-Tools CodeXL)
pub const SI_TAHITI_P_A0: c_uint = 0x01;
pub const SI_TAHITI_P_B0: c_uint = 0x05;
pub const SI_TAHITI_P_B1: c_uint = 0x06;
pub const SI_PITCAIRN_PM_A0: c_uint = 0x14;
pub const SI_PITCAIRN_PM_A1: c_uint = 0x15;
pub const SI_CAPEVERDE_M_A0: c_uint = 0x28;
pub const SI_CAPEVERDE_M_A1: c_uint = 0x29;
pub const SI_OLAND_M_A0: c_uint = 0x3C;
pub const SI_HAINAN_V_A0: c_uint = 0x46;
pub const SI_UNKNOWN: c_uint = 0xFF;

// DCE80 (based on ci_id.h in Perforce)
pub const CI_BONAIRE_M_A0: c_uint = 0x14;
pub const CI_BONAIRE_M_A1: c_uint = 0x15;
pub const CI_HAWAII_P_A0: c_uint = 0x28;
pub const CI_UNKNOWN: c_uint = 0xFF;

// KV1 with Spectre GFX core, 8-8-1-2 (CU-Pix-Primitive-RB)
pub const KV_SPECTRE_A0: c_uint = 0x01;
// KV2 with Spooky GFX core, including downgraded from Spectre core,
// 3-4-1-1 (CU-Pix-Primitive-RB)
pub const KV_SPOOKY_A0: c_uint = 0x41;
// KB with Kalindi GFX core, 2-4-1-1 (CU-Pix-Primitive-RB)
pub const KB_KALINDI_A0: c_uint = 0x81;
// KB with Kalindi GFX core, 2-4-1-1 (CU-Pix-Primitive-RB)
pub const KB_KALINDI_A1: c_uint = 0x82;
// BV with Kalindi GFX core, 2-4-1-1 (CU-Pix-Primitive-RB)
pub const BV_KALINDI_A2: c_uint = 0x85;
// ML with Godavari GFX core, 2-4-1-1 (CU-Pix-Primitive-RB)
pub const ML_GODAVARI_A0: c_uint = 0xA1;
// ML with Godavari GFX core, 2-4-1-1 (CU-Pix-Primitive-RB)
pub const ML_GODAVARI_A1: c_uint = 0xA2;
pub const KV_UNKNOWN: c_uint = 0xFF;

// VI Family
// DCE10
pub const VI_TONGA_P_A0: c_int = 20;
pub const VI_TONGA_P_A1: c_int = 21;
pub const VI_FIJI_P_A0: c_int = 60;
// DCE112
pub const VI_POLARIS10_P_A0: c_int = 80;
pub const VI_POLARIS11_M_A0: c_int = 90;
pub const VI_POLARIS12_V_A0: c_int = 100;
pub const VI_VEGAM_A0: c_int = 110;
pub const VI_UNKNOWN: c_uint = 0xFF;

// DCE11
pub const CZ_CARRIZO_A0: c_uint = 0x01;
pub const STONEY_A0: c_uint = 0x61;
pub const CZ_UNKNOWN: c_uint = 0xFF;

// DCE12
pub const AI_UNKNOWN: c_uint = 0xFF;
pub const AI_GREENLAND_P_A0: c_int = 1;
pub const AI_GREENLAND_P_A1: c_int = 2;
pub const AI_UNKNOWN: c_uint = 0xFF;
pub const AI_VEGA12_P_A0: c_int = 20;
pub const AI_VEGA20_P_A0: c_int = 40;

// DCN1_0
pub const INTERNAL_REV_RAVEN_A0: c_uint = 0x00    /* First spin of Raven */;
pub const RAVEN_A0: c_uint = 0x01;
pub const RAVEN_B0: c_uint = 0x21;
pub const PICASSO_A0: c_uint = 0x41;
// DCN1_01
pub const RAVEN2_A0: c_uint = 0x81;
pub const RAVEN1_F0: c_uint = 0xF0;
pub const RAVEN_UNKNOWN: c_uint = 0xFF;
pub const RENOIR_A0: c_uint = 0x91;

pub const PRID_DALI_DE: c_uint = 0xDE;
pub const PRID_DALI_DF: c_uint = 0xDF;
pub const PRID_DALI_E3: c_uint = 0xE3;
pub const PRID_DALI_E4: c_uint = 0xE4;
pub const PRID_POLLOCK_94: c_uint = 0x94;
pub const PRID_POLLOCK_95: c_uint = 0x95;
pub const PRID_POLLOCK_E9: c_uint = 0xE9;
pub const PRID_POLLOCK_EA: c_uint = 0xEA;
pub const PRID_POLLOCK_EB: c_uint = 0xEB;

pub const GREEN_SARDINE_A0: c_uint = 0xA1;

pub const DEVICE_ID_NV_13FE: c_uint = 0x13FE  // CYAN_SKILLFISH;
pub const DEVICE_ID_NV_143F: c_uint = 0x143F;
pub const DEVICE_ID_NV_13F9: c_uint = 0x13F9;
pub const DEVICE_ID_NV_13FA: c_uint = 0x13FA;
pub const DEVICE_ID_NV_13FB: c_uint = 0x13FB;
pub const DEVICE_ID_NV_13FC: c_uint = 0x13FC;
pub const DEVICE_ID_NV_13DB: c_uint = 0x13DB;
pub const FAMILY_VGH: c_int = 144;
pub const DEVICE_ID_VGH_163F: c_uint = 0x163F;
pub const DEVICE_ID_VGH_1435: c_uint = 0x1435;
pub const VANGOGH_A0: c_uint = 0x01;
pub const VANGOGH_UNKNOWN: c_uint = 0xFF;

pub const FAMILY_YELLOW_CARP: c_int = 146;
pub const YELLOW_CARP_A0: c_uint = 0x01;
pub const YELLOW_CARP_B0: c_uint = 0x20;
pub const YELLOW_CARP_UNKNOWN: c_uint = 0xFF;

pub const AMDGPU_FAMILY_GC_10_3_6: c_int = 149;
pub const GC_10_3_6_A0: c_uint = 0x01;
pub const GC_10_3_6_UNKNOWN: c_uint = 0xFF;

pub const AMDGPU_FAMILY_GC_10_3_7: c_int = 151;
pub const GC_10_3_7_A0: c_uint = 0x01;
pub const GC_10_3_7_UNKNOWN: c_uint = 0xFF;

pub const AMDGPU_FAMILY_GC_11_0_0: c_int = 145;
pub const AMDGPU_FAMILY_GC_11_0_1: c_int = 148;
pub const AMDGPU_FAMILY_GC_11_5_0: c_int = 150;
pub const AMDGPU_FAMILY_GC_11_5_4: c_int = 154;
pub const GC_11_0_0_A0: c_uint = 0x1;
pub const GC_11_0_2_A0: c_uint = 0x10;
pub const GC_11_0_3_A0: c_uint = 0x20;
pub const GC_11_0_4_A0: c_uint = 0xC0;
pub const GC_11_UNKNOWN: c_uint = 0xFF;

pub const DCN4A_SOC_VAR_B_A0: c_uint = 0xD0;

//
// ASIC chip ID
//
// DCE60
pub const DEVICE_ID_SI_TAHITI_P_6780: c_uint = 0x6780;
pub const DEVICE_ID_SI_PITCAIRN_PM_6800: c_uint = 0x6800;
pub const DEVICE_ID_SI_PITCAIRN_PM_6808: c_uint = 0x6808;
pub const DEVICE_ID_SI_CAPEVERDE_M_6820: c_uint = 0x6820;
pub const DEVICE_ID_SI_CAPEVERDE_M_6828: c_uint = 0x6828;
pub const DEVICE_ID_SI_OLAND_M_6600: c_uint = 0x6600;
pub const DEVICE_ID_SI_OLAND_M_6608: c_uint = 0x6608;
pub const DEVICE_ID_SI_HAINAN_V_6660: c_uint = 0x6660;
// DCE80
pub const DEVICE_ID_KALINDI_9834: c_uint = 0x9834;
pub const DEVICE_ID_TEMASH_9839: c_uint = 0x9839;
pub const DEVICE_ID_TEMASH_983D: c_uint = 0x983D;
// RENOIR
pub const DEVICE_ID_RENOIR_1636: c_uint = 0x1636;
// Asic Family IDs for different asic family.

pub const FAMILY_AI: c_int = 141;
pub const FAMILY_UNKNOWN: c_uint = 0xFF;
