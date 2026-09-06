//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_color_regs.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Intel Corporation
//

// GMCH palette
pub const _PALETTE_A: c_uint = 0xa000;
pub const _PALETTE_B: c_uint = 0xa800;
pub const _CHV_PALETTE_C: c_uint = 0xc000;
// 8bit mode / i965+ 10.6 interpolated mode ldw/udw

// pre-i965 10bit interpolated mode ldw

// pre-i965 10bit interpolated mode udw

// i965/g4x/vlv/chv
pub const _PIPEAGCMAX: c_uint = 0x70010;
pub const _PIPEBGCMAX: c_uint = 0x71010;

// ilk+ palette
pub const _LGC_PALETTE_A: c_uint = 0x4a000;
pub const _LGC_PALETTE_B: c_uint = 0x4a800;
// see PALETTE_* for the bits

// ilk/snb precision palette
pub const _PREC_PALETTE_A: c_uint = 0x4b000;
pub const _PREC_PALETTE_B: c_uint = 0x4c000;
// 10bit mode

// 12.4 interpolated mode ldw

// 12.4 interpolated mode udw

pub const _PREC_PIPEAGCMAX: c_uint = 0x4d000;
pub const _PREC_PIPEBGCMAX: c_uint = 0x4d010;

pub const _GAMMA_MODE_A: c_uint = 0x4a480;
pub const _GAMMA_MODE_B: c_uint = 0x4ac80;

// pipe CSC
pub const _PIPE_A_CSC_COEFF_RY_GY: c_uint = 0x49010;
pub const _PIPE_A_CSC_COEFF_BY: c_uint = 0x49014;
pub const _PIPE_A_CSC_COEFF_RU_GU: c_uint = 0x49018;
pub const _PIPE_A_CSC_COEFF_BU: c_uint = 0x4901c;
pub const _PIPE_A_CSC_COEFF_RV_GV: c_uint = 0x49020;
pub const _PIPE_A_CSC_COEFF_BV: c_uint = 0x49024;
pub const _PIPE_A_CSC_MODE: c_uint = 0x49028;

pub const _PIPE_A_CSC_PREOFF_HI: c_uint = 0x49030;
pub const _PIPE_A_CSC_PREOFF_ME: c_uint = 0x49034;
pub const _PIPE_A_CSC_PREOFF_LO: c_uint = 0x49038;
pub const _PIPE_A_CSC_POSTOFF_HI: c_uint = 0x49040;
pub const _PIPE_A_CSC_POSTOFF_ME: c_uint = 0x49044;
pub const _PIPE_A_CSC_POSTOFF_LO: c_uint = 0x49048;
pub const _PIPE_B_CSC_COEFF_RY_GY: c_uint = 0x49110;
pub const _PIPE_B_CSC_COEFF_BY: c_uint = 0x49114;
pub const _PIPE_B_CSC_COEFF_RU_GU: c_uint = 0x49118;
pub const _PIPE_B_CSC_COEFF_BU: c_uint = 0x4911c;
pub const _PIPE_B_CSC_COEFF_RV_GV: c_uint = 0x49120;
pub const _PIPE_B_CSC_COEFF_BV: c_uint = 0x49124;
pub const _PIPE_B_CSC_MODE: c_uint = 0x49128;
pub const _PIPE_B_CSC_PREOFF_HI: c_uint = 0x49130;
pub const _PIPE_B_CSC_PREOFF_ME: c_uint = 0x49134;
pub const _PIPE_B_CSC_PREOFF_LO: c_uint = 0x49138;
pub const _PIPE_B_CSC_POSTOFF_HI: c_uint = 0x49140;
pub const _PIPE_B_CSC_POSTOFF_ME: c_uint = 0x49144;
pub const _PIPE_B_CSC_POSTOFF_LO: c_uint = 0x49148;

// Pipe Output CSC
pub const _PIPE_A_OUTPUT_CSC_COEFF_RY_GY: c_uint = 0x49050;
pub const _PIPE_A_OUTPUT_CSC_COEFF_BY: c_uint = 0x49054;
pub const _PIPE_A_OUTPUT_CSC_COEFF_RU_GU: c_uint = 0x49058;
pub const _PIPE_A_OUTPUT_CSC_COEFF_BU: c_uint = 0x4905c;
pub const _PIPE_A_OUTPUT_CSC_COEFF_RV_GV: c_uint = 0x49060;
pub const _PIPE_A_OUTPUT_CSC_COEFF_BV: c_uint = 0x49064;
pub const _PIPE_A_OUTPUT_CSC_PREOFF_HI: c_uint = 0x49068;
pub const _PIPE_A_OUTPUT_CSC_PREOFF_ME: c_uint = 0x4906c;
pub const _PIPE_A_OUTPUT_CSC_PREOFF_LO: c_uint = 0x49070;
pub const _PIPE_A_OUTPUT_CSC_POSTOFF_HI: c_uint = 0x49074;
pub const _PIPE_A_OUTPUT_CSC_POSTOFF_ME: c_uint = 0x49078;
pub const _PIPE_A_OUTPUT_CSC_POSTOFF_LO: c_uint = 0x4907c;
pub const _PIPE_B_OUTPUT_CSC_COEFF_RY_GY: c_uint = 0x49150;
pub const _PIPE_B_OUTPUT_CSC_COEFF_BY: c_uint = 0x49154;
pub const _PIPE_B_OUTPUT_CSC_COEFF_RU_GU: c_uint = 0x49158;
pub const _PIPE_B_OUTPUT_CSC_COEFF_BU: c_uint = 0x4915c;
pub const _PIPE_B_OUTPUT_CSC_COEFF_RV_GV: c_uint = 0x49160;
pub const _PIPE_B_OUTPUT_CSC_COEFF_BV: c_uint = 0x49164;
pub const _PIPE_B_OUTPUT_CSC_PREOFF_HI: c_uint = 0x49168;
pub const _PIPE_B_OUTPUT_CSC_PREOFF_ME: c_uint = 0x4916c;
pub const _PIPE_B_OUTPUT_CSC_PREOFF_LO: c_uint = 0x49170;
pub const _PIPE_B_OUTPUT_CSC_POSTOFF_HI: c_uint = 0x49174;
pub const _PIPE_B_OUTPUT_CSC_POSTOFF_ME: c_uint = 0x49178;
pub const _PIPE_B_OUTPUT_CSC_POSTOFF_LO: c_uint = 0x4917c;

// pipe degamma/gamma LUTs on IVB+
pub const _PAL_PREC_INDEX_A: c_uint = 0x4A400;
pub const _PAL_PREC_INDEX_B: c_uint = 0x4AC00;
pub const _PAL_PREC_INDEX_C: c_uint = 0x4B400;

pub const _PAL_PREC_DATA_A: c_uint = 0x4A404;
pub const _PAL_PREC_DATA_B: c_uint = 0x4AC04;
pub const _PAL_PREC_DATA_C: c_uint = 0x4B404;
// see PREC_PALETTE_* for the bits
pub const _PAL_PREC_GC_MAX_A: c_uint = 0x4A410;
pub const _PAL_PREC_GC_MAX_B: c_uint = 0x4AC10;
pub const _PAL_PREC_GC_MAX_C: c_uint = 0x4B410;
pub const _PAL_PREC_EXT_GC_MAX_A: c_uint = 0x4A420;
pub const _PAL_PREC_EXT_GC_MAX_B: c_uint = 0x4AC20;
pub const _PAL_PREC_EXT_GC_MAX_C: c_uint = 0x4B420;
pub const _PAL_PREC_EXT2_GC_MAX_A: c_uint = 0x4A430;
pub const _PAL_PREC_EXT2_GC_MAX_B: c_uint = 0x4AC30;
pub const _PAL_PREC_EXT2_GC_MAX_C: c_uint = 0x4B430;

pub const _PRE_CSC_GAMC_INDEX_A: c_uint = 0x4A484;
pub const _PRE_CSC_GAMC_INDEX_B: c_uint = 0x4AC84;
pub const _PRE_CSC_GAMC_INDEX_C: c_uint = 0x4B484;

pub const _PRE_CSC_GAMC_DATA_A: c_uint = 0x4A488;
pub const _PRE_CSC_GAMC_DATA_B: c_uint = 0x4AC88;
pub const _PRE_CSC_GAMC_DATA_C: c_uint = 0x4B488;

// ICL Multi segmented gamma
pub const _PAL_PREC_MULTI_SEG_INDEX_A: c_uint = 0x4A408;
pub const _PAL_PREC_MULTI_SEG_INDEX_B: c_uint = 0x4AC08;

pub const _PAL_PREC_MULTI_SEG_DATA_A: c_uint = 0x4A40C;
pub const _PAL_PREC_MULTI_SEG_DATA_B: c_uint = 0x4AC0C;
// see PREC_PALETTE_12P4_* for the bits

pub const _PIPE_A_WGC_C01_C00: c_uint = 0x600B0 /* s2.10 */;
pub const _PIPE_A_WGC_C02: c_uint = 0x600B4 /* s2.10 */;
pub const _PIPE_A_WGC_C11_C10: c_uint = 0x600B8 /* s2.10 */;
pub const _PIPE_A_WGC_C12: c_uint = 0x600BC /* s2.10 */;
pub const _PIPE_A_WGC_C21_C20: c_uint = 0x600C0 /* s2.10 */;
pub const _PIPE_A_WGC_C22: c_uint = 0x600C4 /* s2.10 */;

// pipe CSC & degamma/gamma LUTs on CHV

// cgm degamma ldw

// cgm degamma udw

// cgm gamma ldw

// cgm gamma udw

// Skylake+ pipe bottom (background) color
pub const _SKL_BOTTOM_COLOR_A: c_uint = 0x70034;
pub const _SKL_BOTTOM_COLOR_B: c_uint = 0x71034;

// 3D LUT
pub const _LUT_3D_CTL_A: c_uint = 0x490A4;
pub const _LUT_3D_CTL_B: c_uint = 0x491A4;

pub const _LUT_3D_INDEX_A: c_uint = 0x490A8;
pub const _LUT_3D_INDEX_B: c_uint = 0x491A8;

pub const _LUT_3D_DATA_A: c_uint = 0x490AC;
pub const _LUT_3D_DATA_B: c_uint = 0x491AC;

