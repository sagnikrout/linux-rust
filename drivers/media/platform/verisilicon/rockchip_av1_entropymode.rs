//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/verisilicon/rockchip_av1_entropymode.h
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

pub const AV1_INTER_MODE_CONTEXTS: c_int = 15;
pub const AV1_INTRA_MODES: c_int = 13;
pub const AV1_REF_CONTEXTS: c_int = 3;

pub const AV1_TX_SIZE_CONTEXTS: c_int = 3;
pub const BLOCK_SIZE_GROUPS: c_int = 4;
pub const BR_CDF_SIZE: c_int = 4;
pub const BWD_REFS: c_int = 3;
pub const CFL_ALLOWED_TYPES: c_int = 2;
pub const CFL_ALPHA_CONTEXTS: c_int = 6;
pub const CFL_ALPHABET_SIZE: c_int = 16;
pub const CFL_JOINT_SIGNS: c_int = 8;

pub const COMP_GROUP_IDX_CONTEXTS: c_int = 7;
pub const COMP_INDEX_CONTEXTS: c_int = 6;
pub const COMP_INTER_CONTEXTS: c_int = 5;
pub const COMP_REF_TYPE_CONTEXTS: c_int = 5;
pub const COMPOUND_TYPES: c_int = 3;
pub const DC_SIGN_CONTEXTS: c_int = 3;
pub const DELTA_LF_PROBS: c_int = 3;
pub const DELTA_Q_PROBS: c_int = 3;
pub const DIRECTIONAL_MODES: c_int = 8;
pub const DRL_MODE_CONTEXTS: c_int = 3;
pub const EOB_COEF_CONTEXTS: c_int = 9;
pub const EXT_TX_SIZES: c_int = 3;
pub const EXT_TX_TYPES: c_int = 16;
pub const EXTTX_SIZES: c_int = 4;
pub const FRAME_LF_COUNT: c_int = 4;
pub const FWD_REFS: c_int = 4;
pub const GLOBALMV_MODE_CONTEXTS: c_int = 2;

pub const INTER_COMPOUND_MODES: c_int = 8;
pub const INTERINTRA_MODES: c_int = 4;
pub const INTRA_INTER_CONTEXTS: c_int = 4;
pub const KF_MODE_CONTEXTS: c_int = 5;
pub const LEVEL_CONTEXTS: c_int = 21;
pub const MAX_ANGLE_DELTA: c_int = 3;
pub const MAX_MB_SEGMENTS: c_int = 8;
pub const MAX_SEGMENTS: c_int = 8;
pub const MAX_TX_CATS: c_int = 4;
pub const MAX_TX_DEPTH: c_int = 2;
pub const MBSKIP_CONTEXTS: c_int = 3;
pub const MOTION_MODES: c_int = 3;
pub const MOTION_MODE_CONTEXTS: c_int = 10;
pub const NEWMV_MODE_CONTEXTS: c_int = 6;
pub const NUM_BASE_LEVELS: c_int = 2;
pub const NUM_REF_FRAMES: c_int = 8;
pub const PALETTE_BLOCK_SIZES: c_int = 7;
pub const PALETTE_IDX_CONTEXTS: c_int = 18;
pub const PALETTE_SIZES: c_int = 7;
pub const PALETTE_UV_MODE_CONTEXTS: c_int = 2;
pub const PALETTE_Y_MODE_CONTEXTS: c_int = 3;
pub const PARTITION_PLOFFSET: c_int = 4;

pub const PLANE_TYPES: c_int = 2;
pub const PREDICTION_PROBS: c_int = 3;
pub const REF_CONTEXTS: c_int = 5;
pub const REFMV_MODE_CONTEXTS: c_int = 9;
pub const SEG_TEMPORAL_PRED_CTXS: c_int = 3;
pub const SIG_COEF_CONTEXTS: c_int = 42;
pub const SIG_COEF_CONTEXTS_EOB: c_int = 4;
pub const SINGLE_REFS: c_int = 7;
pub const SKIP_CONTEXTS: c_int = 3;
pub const SKIP_MODE_CONTEXTS: c_int = 3;
pub const SPATIAL_PREDICTION_PROBS: c_int = 3;

pub const TOKEN_CDF_Q_CTXS: c_int = 4;
pub const TX_SIZES: c_int = 5;
pub const TX_SIZE_CONTEXTS: c_int = 2;
pub const TX_TYPES: c_int = 4;
pub const TXB_SKIP_CONTEXTS: c_int = 13;
pub const TXFM_PARTITION_CONTEXTS: c_int = 22;
pub const UNI_COMP_REF_CONTEXTS: c_int = 3;
pub const UNIDIR_COMP_REFS: c_int = 4;
pub const UV_INTRA_MODES: c_int = 14;
pub const VARTX_PART_CONTEXTS: c_int = 22;
pub const ZEROMV_MODE_CONTEXTS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blocksizetype {
    BLOCK_SIZE_AB4X4,
    BLOCK_SIZE_SB4X8,
    BLOCK_SIZE_SB8X4,
    BLOCK_SIZE_SB8X8,
    BLOCK_SIZE_SB8X16,
    BLOCK_SIZE_SB16X8,
    BLOCK_SIZE_MB16X16,
    BLOCK_SIZE_SB16X32,
    BLOCK_SIZE_SB32X16,
    BLOCK_SIZE_SB32X32,
    BLOCK_SIZE_SB32X64,
    BLOCK_SIZE_SB64X32,
    BLOCK_SIZE_SB64X64,
    BLOCK_SIZE_SB64X128,
    BLOCK_SIZE_SB128X64,
    BLOCK_SIZE_SB128X128,
    BLOCK_SIZE_SB4X16,
    BLOCK_SIZE_SB16X4,
    BLOCK_SIZE_SB8X32,
    BLOCK_SIZE_SB32X8,
    BLOCK_SIZE_SB16X64,
    BLOCK_SIZE_SB64X16,
    BLOCK_SIZE_TYPES,
    BLOCK_SIZES_ALL = BLOCK_SIZE_TYPES
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum filterintramodetype {
    FILTER_DC_PRED,
    FILTER_V_PRED,
    FILTER_H_PRED,
    FILTER_D153_PRED,
    FILTER_PAETH_PRED,
    FILTER_INTRA_MODES,
    FILTER_INTRA_UNUSED = 7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum frametype {
    KEY_FRAME = 0,
    INTER_FRAME = 1,
    NUM_FRAME_TYPES,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum txsize {
    TX_4X4 = 0,
    TX_8X8 = 1,
    TX_16X16 = 2,
    TX_32X32 = 3,
    TX_SIZE_MAX_SB,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mb_prediction_mode {
    DC_PRED,		/* average of above and left pixels */
    V_PRED,			/* vertical prediction */
    H_PRED,			/* horizontal prediction */
    D45_PRED,		/* Directional 45 deg prediction  [anti-clockwise from 0 deg hor] */
    D135_PRED,		/* Directional 135 deg prediction [anti-clockwise from 0 deg hor] */
    D117_PRED,		/* Directional 112 deg prediction [anti-clockwise from 0 deg hor] */
    D153_PRED,		/* Directional 157 deg prediction [anti-clockwise from 0 deg hor] */
    D27_PRED,		/* Directional 22 deg prediction  [anti-clockwise from 0 deg hor] */
    D63_PRED,		/* Directional 67 deg prediction  [anti-clockwise from 0 deg hor] */
    SMOOTH_PRED,
    TM_PRED_AV1 = SMOOTH_PRED,
    SMOOTH_V_PRED,		// Vertical interpolation
    SMOOTH_H_PRED,		// Horizontal interpolation
    TM_PRED,		/* Truemotion prediction */
    PAETH_PRED = TM_PRED,
    NEARESTMV,
    NEARMV,
    ZEROMV,
    NEWMV,
    NEAREST_NEARESTMV,
    NEAR_NEARMV,
    NEAREST_NEWMV,
    NEW_NEARESTMV,
    NEAR_NEWMV,
    NEW_NEARMV,
    ZERO_ZEROMV,
    NEW_NEWMV,
    SPLITMV,
    MB_MODE_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum partitiontype {
    PARTITION_NONE,
    PARTITION_HORZ,
    PARTITION_VERT,
    PARTITION_SPLIT,
    PARTITION_TYPES
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvcdfs {
    pub joint_cdf: [u16; 3],
    pub sign_cdf: [u16; 2],
    pub clsss_cdf: [u16; 2][10],
    pub clsss0_fp_cdf: [u16; 2][2][3],
    pub fp_cdf: [u16; 2][3],
    pub class0_hp_cdf: [u16; 2],
    pub hp_cdf: [u16; 2],
    pub class0_cdf: [u16; 2],
    pub bits_cdf: [u16; 2][10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct av1cdfs {
    pub partition_cdf: [u16; 13][16],
    pub 1]: u16 kf_ymode_cdf[KF_MODE_CONTEXTS][KF_MODE_CONTEXTS][AV1_INTRA_MODES -,
    pub segment_pred_cdf: [u16; PREDICTION_PROBS],
    pub 1]: u16 spatial_pred_seg_tree_cdf[SPATIAL_PREDICTION_PROBS][MAX_MB_SEGMENTS -,
    pub mbskip_cdf: [u16; MBSKIP_CONTEXTS],
    pub delta_q_cdf: [u16; DELTA_Q_PROBS],
    pub delta_lf_multi_cdf: [u16; FRAME_LF_COUNT][DELTA_LF_PROBS],
    pub delta_lf_cdf: [u16; DELTA_LF_PROBS],
    pub skip_mode_cdf: [u16; SKIP_MODE_CONTEXTS],
    pub vartx_part_cdf: [u16; VARTX_PART_CONTEXTS][1],
    pub tx_size_cdf: [u16; MAX_TX_CATS][AV1_TX_SIZE_CONTEXTS][MAX_TX_DEPTH],
    pub 1]: u16 if_ymode_cdf[BLOCK_SIZE_GROUPS][AV1_INTRA_MODES -,
    pub 1]: u16 uv_mode_cdf[2][AV1_INTRA_MODES][AV1_INTRA_MODES - 1 +,
    pub intra_inter_cdf: [u16; INTRA_INTER_CONTEXTS],
    pub comp_inter_cdf: [u16; COMP_INTER_CONTEXTS],
    pub 1]: u16 single_ref_cdf[AV1_REF_CONTEXTS][SINGLE_REFS -,
    pub comp_ref_type_cdf: [u16; COMP_REF_TYPE_CONTEXTS][1],
    pub 1][1]: u16 uni_comp_ref_cdf[UNI_COMP_REF_CONTEXTS][UNIDIR_COMP_REFS -,
    pub 1]: u16 comp_ref_cdf[AV1_REF_CONTEXTS][FWD_REFS -,
    pub 1]: u16 comp_bwdref_cdf[AV1_REF_CONTEXTS][BWD_REFS -,
    pub newmv_cdf: [u16; NEWMV_MODE_CONTEXTS],
    pub zeromv_cdf: [u16; ZEROMV_MODE_CONTEXTS],
    pub refmv_cdf: [u16; REFMV_MODE_CONTEXTS],
    pub drl_cdf: [u16; DRL_MODE_CONTEXTS],
    pub 1]: u16 interp_filter_cdf[SWITCHABLE_FILTER_CONTEXTS][AV1_SWITCHABLE_FILTERS -,
    pub mv_cdf: mvcdfs,
    pub obmc_cdf: [u16; BLOCK_SIZE_TYPES],
    pub motion_mode_cdf: [u16; BLOCK_SIZE_TYPES][2],
    pub 1]: u16 inter_compound_mode_cdf[AV1_INTER_MODE_CONTEXTS][INTER_COMPOUND_MODES -,
    pub 1)]: u16 compound_type_cdf[BLOCK_SIZE_TYPES][CDF_SIZE(COMPOUND_TYPES -,
    pub interintra_cdf: [u16; BLOCK_SIZE_GROUPS],
    pub 1]: u16 interintra_mode_cdf[BLOCK_SIZE_GROUPS][INTERINTRA_MODES -,
    pub wedge_interintra_cdf: [u16; BLOCK_SIZE_TYPES],
    pub wedge_idx_cdf: [u16; BLOCK_SIZE_TYPES][CDF_SIZE(16)],
    pub palette_y_mode_cdf: [u16; PALETTE_BLOCK_SIZES][PALETTE_Y_MODE_CONTEXTS][1],
    pub palette_uv_mode_cdf: [u16; PALETTE_UV_MODE_CONTEXTS][1],
    pub 1]: u16 palette_y_size_cdf[PALETTE_BLOCK_SIZES][PALETTE_SIZES -,
    pub 1]: u16 palette_uv_size_cdf[PALETTE_BLOCK_SIZES][PALETTE_SIZES -,
    pub 1]: u16 cfl_sign_cdf[CFL_JOINT_SIGNS -,
    pub 1]: u16 cfl_alpha_cdf[CFL_ALPHA_CONTEXTS][CFL_ALPHABET_SIZE -,
    pub intrabc_cdf: [u16; 1],
    pub angle_delta_cdf: [u16; DIRECTIONAL_MODES][6],
    pub 1]: u16 filter_intra_mode_cdf[FILTER_INTRA_MODES -,
    pub filter_intra_cdf: [u16; BLOCK_SIZES_ALL],
    pub comp_group_idx_cdf: [u16; COMP_GROUP_IDX_CONTEXTS][CDF_SIZE(2)],
    pub compound_idx_cdf: [u16; COMP_INDEX_CONTEXTS][CDF_SIZE(2)],
    pub dummy0: [u16; 14],
// Palette index contexts; sizes 1/7, 2/6, 3/5 packed together
    pub palette_y_color_index_cdf: [u16; PALETTE_IDX_CONTEXTS][8],
    pub palette_uv_color_index_cdf: [u16; PALETTE_IDX_CONTEXTS][8],
    pub tx_type_intra0_cdf: [u16; EXTTX_SIZES][AV1_INTRA_MODES][8],
    pub tx_type_intra1_cdf: [u16; EXTTX_SIZES][AV1_INTRA_MODES][4],
    pub tx_type_inter_cdf: [u16; 2][EXTTX_SIZES][EXT_TX_TYPES],
    pub txb_skip_cdf: [u16; TX_SIZES][TXB_SKIP_CONTEXTS][CDF_SIZE(2)],
    pub eob_extra_cdf: [u16; TX_SIZES][PLANE_TYPES][EOB_COEF_CONTEXTS][CDF_SIZE(2)],
    pub dummy1: [u16; 5],
    pub eob_flag_cdf16: [u16; PLANE_TYPES][2][4],
    pub eob_flag_cdf32: [u16; PLANE_TYPES][2][8],
    pub eob_flag_cdf64: [u16; PLANE_TYPES][2][8],
    pub eob_flag_cdf128: [u16; PLANE_TYPES][2][8],
    pub eob_flag_cdf256: [u16; PLANE_TYPES][2][8],
    pub eob_flag_cdf512: [u16; PLANE_TYPES][2][16],
    pub eob_flag_cdf1024: [u16; PLANE_TYPES][2][16],
    pub coeff_base_eob_cdf: [u16; TX_SIZES][PLANE_TYPES][SIG_COEF_CONTEXTS_EOB][CDF_SIZE(3)],
    pub 1]: u16 coeff_base_cdf[TX_SIZES][PLANE_TYPES][SIG_COEF_CONTEXTS][CDF_SIZE(4) +,
    pub dc_sign_cdf: [u16; PLANE_TYPES][DC_SIGN_CONTEXTS][CDF_SIZE(2)],
    pub dummy2: [u16; 2],
    pub 1]: u16 coeff_br_cdf[TX_SIZES][PLANE_TYPES][LEVEL_CONTEXTS][CDF_SIZE(BR_CDF_SIZE) +,
    pub dummy3: [u16; 16],
}

extern "C" {
    pub fn rockchip_av1_get_cdfs(ctx: *mut hantro_ctx, ref_idx: u32);
}
extern "C" {
    pub fn rockchip_av1_default_coeff_probs(base_qindex: u32, ptr: *mut c_void);
}
