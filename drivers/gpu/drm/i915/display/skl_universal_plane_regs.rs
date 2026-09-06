//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/skl_universal_plane_regs.h
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
// Copyright © 2024 Intel Corporation
//

pub const _PLANE_CTL_1_A: c_uint = 0x70180;
pub const _PLANE_CTL_2_A: c_uint = 0x70280;
pub const _PLANE_CTL_1_B: c_uint = 0x71180;
pub const _PLANE_CTL_2_B: c_uint = 0x71280;

//
// ICL+ uses the same PLANE_CTL_FORMAT bits, but the field definition
// expanded to include bit 23 as well. However, the shift-24 based values
// correctly map to the same formats in ICL, as long as bit 23 is set to 0
//

pub const _PLANE_STRIDE_1_A: c_uint = 0x70188;
pub const _PLANE_STRIDE_2_A: c_uint = 0x70288;
pub const _PLANE_STRIDE_1_B: c_uint = 0x71188;
pub const _PLANE_STRIDE_2_B: c_uint = 0x71288;

pub const _PLANE_POS_1_A: c_uint = 0x7018c;
pub const _PLANE_POS_2_A: c_uint = 0x7028c;
pub const _PLANE_POS_1_B: c_uint = 0x7118c;
pub const _PLANE_POS_2_B: c_uint = 0x7128c;

pub const _PLANE_SIZE_1_A: c_uint = 0x70190;
pub const _PLANE_SIZE_2_A: c_uint = 0x70290;
pub const _PLANE_SIZE_1_B: c_uint = 0x71190;
pub const _PLANE_SIZE_2_B: c_uint = 0x71290;

pub const _PLANE_KEYVAL_1_A: c_uint = 0x70194;
pub const _PLANE_KEYVAL_2_A: c_uint = 0x70294;
pub const _PLANE_KEYVAL_1_B: c_uint = 0x71194;
pub const _PLANE_KEYVAL_2_B: c_uint = 0x71294;

pub const _PLANE_KEYMSK_1_A: c_uint = 0x70198;
pub const _PLANE_KEYMSK_2_A: c_uint = 0x70298;
pub const _PLANE_KEYMSK_1_B: c_uint = 0x71198;
pub const _PLANE_KEYMSK_2_B: c_uint = 0x71298;

pub const _PLANE_SURF_1_A: c_uint = 0x7019c;
pub const _PLANE_SURF_2_A: c_uint = 0x7029c;
pub const _PLANE_SURF_1_B: c_uint = 0x7119c;
pub const _PLANE_SURF_2_B: c_uint = 0x7129c;

pub const _PLANE_KEYMAX_1_A: c_uint = 0x701a0;
pub const _PLANE_KEYMAX_2_A: c_uint = 0x702a0;
pub const _PLANE_KEYMAX_1_B: c_uint = 0x711a0;
pub const _PLANE_KEYMAX_2_B: c_uint = 0x712a0;

pub const _PLANE_OFFSET_1_A: c_uint = 0x701a4;
pub const _PLANE_OFFSET_2_A: c_uint = 0x702a4;
pub const _PLANE_OFFSET_1_B: c_uint = 0x711a4;
pub const _PLANE_OFFSET_2_B: c_uint = 0x712a4;

pub const _PLANE_SURFLIVE_1_A: c_uint = 0x701ac;
pub const _PLANE_SURFLIVE_2_A: c_uint = 0x702ac;
pub const _PLANE_SURFLIVE_1_B: c_uint = 0x711ac;
pub const _PLANE_SURFLIVE_2_B: c_uint = 0x712ac;

pub const _PLANE_CC_VAL_1_A: c_uint = 0x701b4;
pub const _PLANE_CC_VAL_2_A: c_uint = 0x702b4;
pub const _PLANE_CC_VAL_1_B: c_uint = 0x711b4;
pub const _PLANE_CC_VAL_2_B: c_uint = 0x712b4;

pub const _PLANE_AUX_DIST_1_A: c_uint = 0x701c0;
pub const _PLANE_AUX_DIST_2_A: c_uint = 0x702c0;
pub const _PLANE_AUX_DIST_1_B: c_uint = 0x711c0;
pub const _PLANE_AUX_DIST_2_B: c_uint = 0x712c0;

pub const _PLANE_AUX_OFFSET_1_A: c_uint = 0x701c4;
pub const _PLANE_AUX_OFFSET_2_A: c_uint = 0x702c4;
pub const _PLANE_AUX_OFFSET_1_B: c_uint = 0x711c4;
pub const _PLANE_AUX_OFFSET_2_B: c_uint = 0x712c4;

pub const _PLANE_CUS_CTL_1_A: c_uint = 0x701c8;
pub const _PLANE_CUS_CTL_2_A: c_uint = 0x702c8;
pub const _PLANE_CUS_CTL_1_B: c_uint = 0x711c8;
pub const _PLANE_CUS_CTL_2_B: c_uint = 0x712c8;

pub const _PLANE_COLOR_CTL_1_A: c_uint = 0x701cc /* GLK+ */;
pub const _PLANE_COLOR_CTL_2_A: c_uint = 0x702cc;
pub const _PLANE_COLOR_CTL_1_B: c_uint = 0x711cc;
pub const _PLANE_COLOR_CTL_2_B: c_uint = 0x712cc;

pub const _PLANE_INPUT_CSC_RY_GY_1_A: c_uint = 0x701e0;
pub const _PLANE_INPUT_CSC_RY_GY_2_A: c_uint = 0x702e0;
pub const _PLANE_INPUT_CSC_RY_GY_1_B: c_uint = 0x711e0;
pub const _PLANE_INPUT_CSC_RY_GY_2_B: c_uint = 0x712e0;

pub const _PLANE_INPUT_CSC_PREOFF_HI_1_A: c_uint = 0x701f8;
pub const _PLANE_INPUT_CSC_PREOFF_HI_2_A: c_uint = 0x702f8;
pub const _PLANE_INPUT_CSC_PREOFF_HI_1_B: c_uint = 0x711f8;
pub const _PLANE_INPUT_CSC_PREOFF_HI_2_B: c_uint = 0x712f8;

pub const _PLANE_INPUT_CSC_POSTOFF_HI_1_A: c_uint = 0x70204;
pub const _PLANE_INPUT_CSC_POSTOFF_HI_2_A: c_uint = 0x70304;
pub const _PLANE_INPUT_CSC_POSTOFF_HI_1_B: c_uint = 0x71204;
pub const _PLANE_INPUT_CSC_POSTOFF_HI_2_B: c_uint = 0x71304;

pub const _PLANE_POST_CSC_GAMC_SEG0_INDEX_ENH_1_A: c_uint = 0x70160;
pub const _PLANE_POST_CSC_GAMC_SEG0_INDEX_ENH_1_B: c_uint = 0x71160;
pub const _PLANE_POST_CSC_GAMC_SEG0_INDEX_ENH_2_A: c_uint = 0x70260;
pub const _PLANE_POST_CSC_GAMC_SEG0_INDEX_ENH_2_B: c_uint = 0x71260;

pub const _PLANE_POST_CSC_GAMC_SEG0_DATA_ENH_1_A: c_uint = 0x70164;
pub const _PLANE_POST_CSC_GAMC_SEG0_DATA_ENH_1_B: c_uint = 0x71164;
pub const _PLANE_POST_CSC_GAMC_SEG0_DATA_ENH_2_A: c_uint = 0x70264;
pub const _PLANE_POST_CSC_GAMC_SEG0_DATA_ENH_2_B: c_uint = 0x71264;

pub const _PLANE_POST_CSC_GAMC_INDEX_ENH_1_A: c_uint = 0x701d8;
pub const _PLANE_POST_CSC_GAMC_INDEX_ENH_1_B: c_uint = 0x711d8;
pub const _PLANE_POST_CSC_GAMC_INDEX_ENH_2_A: c_uint = 0x702d8;
pub const _PLANE_POST_CSC_GAMC_INDEX_ENH_2_B: c_uint = 0x712d8;

pub const _PLANE_POST_CSC_GAMC_DATA_ENH_1_A: c_uint = 0x701dc;
pub const _PLANE_POST_CSC_GAMC_DATA_ENH_1_B: c_uint = 0x711dc;
pub const _PLANE_POST_CSC_GAMC_DATA_ENH_2_A: c_uint = 0x702dc;
pub const _PLANE_POST_CSC_GAMC_DATA_ENH_2_B: c_uint = 0x712dc;

pub const _PLANE_POST_CSC_GAMC_INDEX_1_A: c_uint = 0x704d8;
pub const _PLANE_POST_CSC_GAMC_INDEX_1_B: c_uint = 0x714d8;
pub const _PLANE_POST_CSC_GAMC_INDEX_2_A: c_uint = 0x705d8;
pub const _PLANE_POST_CSC_GAMC_INDEX_2_B: c_uint = 0x715d8;

pub const _PLANE_POST_CSC_GAMC_DATA_1_A: c_uint = 0x704dc;
pub const _PLANE_POST_CSC_GAMC_DATA_1_B: c_uint = 0x714dc;
pub const _PLANE_POST_CSC_GAMC_DATA_2_A: c_uint = 0x705dc;
pub const _PLANE_POST_CSC_GAMC_DATA_2_B: c_uint = 0x715dc;

pub const _PLANE_PRE_CSC_GAMC_INDEX_ENH_1_A: c_uint = 0x701d0;
pub const _PLANE_PRE_CSC_GAMC_INDEX_ENH_1_B: c_uint = 0x711d0;
pub const _PLANE_PRE_CSC_GAMC_INDEX_ENH_2_A: c_uint = 0x702d0;
pub const _PLANE_PRE_CSC_GAMC_INDEX_ENH_2_B: c_uint = 0x712d0;

pub const _PLANE_PRE_CSC_GAMC_DATA_ENH_1_A: c_uint = 0x701d4;
pub const _PLANE_PRE_CSC_GAMC_DATA_ENH_1_B: c_uint = 0x711d4;
pub const _PLANE_PRE_CSC_GAMC_DATA_ENH_2_A: c_uint = 0x702d4;
pub const _PLANE_PRE_CSC_GAMC_DATA_ENH_2_B: c_uint = 0x712d4;

pub const _PLANE_PRE_CSC_GAMC_INDEX_1_A: c_uint = 0x704d0;
pub const _PLANE_PRE_CSC_GAMC_INDEX_1_B: c_uint = 0x714d0;
pub const _PLANE_PRE_CSC_GAMC_INDEX_2_A: c_uint = 0x705d0;
pub const _PLANE_PRE_CSC_GAMC_INDEX_2_B: c_uint = 0x715d0;

pub const _PLANE_PRE_CSC_GAMC_DATA_1_A: c_uint = 0x704d4;
pub const _PLANE_PRE_CSC_GAMC_DATA_1_B: c_uint = 0x714d4;
pub const _PLANE_PRE_CSC_GAMC_DATA_2_A: c_uint = 0x705d4;
pub const _PLANE_PRE_CSC_GAMC_DATA_2_B: c_uint = 0x715d4;

pub const _PLANE_CSC_RY_GY_1_A: c_uint = 0x70210;
pub const _PLANE_CSC_RY_GY_2_A: c_uint = 0x70310;
pub const _PLANE_CSC_RY_GY_1_B: c_uint = 0x71210;
pub const _PLANE_CSC_RY_GY_2_B: c_uint = 0x71310;

pub const _PLANE_CSC_PREOFF_HI_1_A: c_uint = 0x70228;
pub const _PLANE_CSC_PREOFF_HI_2_A: c_uint = 0x70328;
pub const _PLANE_CSC_PREOFF_HI_1_B: c_uint = 0x71228;
pub const _PLANE_CSC_PREOFF_HI_2_B: c_uint = 0x71328;

pub const _PLANE_CSC_POSTOFF_HI_1_A: c_uint = 0x70234;
pub const _PLANE_CSC_POSTOFF_HI_2_A: c_uint = 0x70334;
pub const _PLANE_CSC_POSTOFF_HI_1_B: c_uint = 0x71234;
pub const _PLANE_CSC_POSTOFF_HI_2_B: c_uint = 0x71334;

pub const _PLANE_WM_1_A_0: c_uint = 0x70240;
pub const _PLANE_WM_1_B_0: c_uint = 0x71240;
pub const _PLANE_WM_2_A_0: c_uint = 0x70340;
pub const _PLANE_WM_2_B_0: c_uint = 0x71340;

pub const _PLANE_WM_SAGV_1_A: c_uint = 0x70258;
pub const _PLANE_WM_SAGV_1_B: c_uint = 0x71258;
pub const _PLANE_WM_SAGV_2_A: c_uint = 0x70358;
pub const _PLANE_WM_SAGV_2_B: c_uint = 0x71358;

pub const _PLANE_WM_SAGV_TRANS_1_A: c_uint = 0x7025c;
pub const _PLANE_WM_SAGV_TRANS_1_B: c_uint = 0x7125c;
pub const _PLANE_WM_SAGV_TRANS_2_A: c_uint = 0x7035c;
pub const _PLANE_WM_SAGV_TRANS_2_B: c_uint = 0x7135c;

pub const _PLANE_WM_TRANS_1_A: c_uint = 0x70268;
pub const _PLANE_WM_TRANS_1_B: c_uint = 0x71268;
pub const _PLANE_WM_TRANS_2_A: c_uint = 0x70368;
pub const _PLANE_WM_TRANS_2_B: c_uint = 0x71368;

pub const _PLANE_CHICKEN_1_A: c_uint = 0x7026c /* tgl+ */;
pub const _PLANE_CHICKEN_2_A: c_uint = 0x7036c;
pub const _PLANE_CHICKEN_1_B: c_uint = 0x7126c;
pub const _PLANE_CHICKEN_2_B: c_uint = 0x7136c;

pub const _PLANE_NV12_BUF_CFG_1_A: c_uint = 0x70278;
pub const _PLANE_NV12_BUF_CFG_2_A: c_uint = 0x70378;
pub const _PLANE_NV12_BUF_CFG_1_B: c_uint = 0x71278;
pub const _PLANE_NV12_BUF_CFG_2_B: c_uint = 0x71378;

pub const _PLANE_BUF_CFG_1_A: c_uint = 0x7027c;
pub const _PLANE_BUF_CFG_2_A: c_uint = 0x7037c;
pub const _PLANE_BUF_CFG_1_B: c_uint = 0x7127c;
pub const _PLANE_BUF_CFG_2_B: c_uint = 0x7137c;

// skl+: 10 bits, icl+ 11 bits, adlp+ 12 bits, xe3p_lpd 13 bits

pub const _PLANE_MIN_BUF_CFG_1_A: c_uint = 0x70274;
pub const _PLANE_MIN_BUF_CFG_2_A: c_uint = 0x70374;
pub const _PLANE_MIN_BUF_CFG_1_B: c_uint = 0x71274;
pub const _PLANE_MIN_BUF_CFG_2_B: c_uint = 0x71374;

// tgl+
pub const _SEL_FETCH_PLANE_CTL_1_A: c_uint = 0x70890;
pub const _SEL_FETCH_PLANE_CTL_2_A: c_uint = 0x708b0;
pub const _SEL_FETCH_PLANE_CTL_5_A: c_uint = 0x70920;
pub const _SEL_FETCH_PLANE_CTL_6_A: c_uint = 0x70940;
pub const _SEL_FETCH_PLANE_CTL_1_B: c_uint = 0x71890;
pub const _SEL_FETCH_PLANE_CTL_2_B: c_uint = 0x718b0;
pub const _SEL_FETCH_PLANE_CTL_5_B: c_uint = 0x71920;
pub const _SEL_FETCH_PLANE_CTL_6_B: c_uint = 0x71940;

// tgl+
pub const _SEL_FETCH_PLANE_POS_1_A: c_uint = 0x70894;
pub const _SEL_FETCH_PLANE_POS_2_A: c_uint = 0x708b4;
pub const _SEL_FETCH_PLANE_POS_5_A: c_uint = 0x70924;
pub const _SEL_FETCH_PLANE_POS_6_A: c_uint = 0x70944;
pub const _SEL_FETCH_PLANE_POS_1_B: c_uint = 0x71894;
pub const _SEL_FETCH_PLANE_POS_2_B: c_uint = 0x718b4;
pub const _SEL_FETCH_PLANE_POS_5_B: c_uint = 0x71924;
pub const _SEL_FETCH_PLANE_POS_6_B: c_uint = 0x71944;

// tgl+
pub const _SEL_FETCH_PLANE_SIZE_1_A: c_uint = 0x70898;
pub const _SEL_FETCH_PLANE_SIZE_2_A: c_uint = 0x708b8;
pub const _SEL_FETCH_PLANE_SIZE_5_A: c_uint = 0x70928;
pub const _SEL_FETCH_PLANE_SIZE_6_A: c_uint = 0x70948;
pub const _SEL_FETCH_PLANE_SIZE_1_B: c_uint = 0x71898;
pub const _SEL_FETCH_PLANE_SIZE_2_B: c_uint = 0x718b8;
pub const _SEL_FETCH_PLANE_SIZE_5_B: c_uint = 0x71928;
pub const _SEL_FETCH_PLANE_SIZE_6_B: c_uint = 0x71948;

// tgl+
pub const _SEL_FETCH_PLANE_OFFSET_1_A: c_uint = 0x7089c;
pub const _SEL_FETCH_PLANE_OFFSET_2_A: c_uint = 0x708bc;
pub const _SEL_FETCH_PLANE_OFFSET_5_A: c_uint = 0x7092c;
pub const _SEL_FETCH_PLANE_OFFSET_6_A: c_uint = 0x7094c;
pub const _SEL_FETCH_PLANE_OFFSET_1_B: c_uint = 0x7189c;
pub const _SEL_FETCH_PLANE_OFFSET_2_B: c_uint = 0x718bc;
pub const _SEL_FETCH_PLANE_OFFSET_5_B: c_uint = 0x7192c;
pub const _SEL_FETCH_PLANE_OFFSET_6_B: c_uint = 0x7194c;

pub const _PLANE_PIXEL_NORMALIZE_1_A: c_uint = 0x701a8;
pub const _PLANE_PIXEL_NORMALIZE_2_A: c_uint = 0x702a8;
pub const _PLANE_PIXEL_NORMALIZE_1_B: c_uint = 0x711a8;
pub const _PLANE_PIXEL_NORMALIZE_2_B: c_uint = 0x712a8;

pub const PLANE_PIXEL_NORMALIZE_NORM_FACTOR_1_0: c_uint = 0x3c00;
