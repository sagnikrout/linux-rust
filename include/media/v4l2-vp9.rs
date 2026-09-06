//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-vp9.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Helper functions for vp9 codecs.
//
// Copyright (c) 2021 Collabora, Ltd.
//
// Author: Andrzej Pietrasiewicz <andrzej.p@collabora.com>
//

//
// struct v4l2_vp9_frame_mv_context - motion vector-related probabilities
//
// @joint: motion vector joint probabilities.
// @sign: motion vector sign probabilities.
// @classes: motion vector class probabilities.
// @class0_bit: motion vector class0 bit probabilities.
// @bits: motion vector bits probabilities.
// @class0_fr: motion vector class0 fractional bit probabilities.
// @fr: motion vector fractional bit probabilities.
// @class0_hp: motion vector class0 high precision fractional bit probabilities.
// @hp: motion vector high precision fractional bit probabilities.
//
// A member of v4l2_vp9_frame_context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_vp9_frame_mv_context {
    pub joint: [u8; 3],
    pub sign: [u8; 2],
    pub classes: [u8; 2][10],
    pub class0_bit: [u8; 2],
    pub bits: [u8; 2][10],
    pub class0_fr: [u8; 2][2][3],
    pub fr: [u8; 2][3],
    pub class0_hp: [u8; 2],
    pub hp: [u8; 2],
}

//
// struct v4l2_vp9_frame_context - frame probabilities, including motion-vector related
//
// @tx8: TX 8x8 probabilities.
// @tx16: TX 16x16 probabilities.
// @tx32: TX 32x32 probabilities.
// @coef: coefficient probabilities.
// @skip: skip probabilities.
// @inter_mode: inter mode probabilities.
// @interp_filter: interpolation filter probabilities.
// @is_inter: is inter-block probabilities.
// @comp_mode: compound prediction mode probabilities.
// @single_ref: single ref probabilities.
// @comp_ref: compound ref probabilities.
// @y_mode: Y prediction mode probabilities.
// @uv_mode: UV prediction mode probabilities.
// @partition: partition probabilities.
// @mv: motion vector probabilities.
//
// Drivers which need to keep track of frame context(s) can use this struct.
// The members correspond to probability tables, which are specified only implicitly in the
// vp9 spec. Section 10.5 "Default probability tables" contains all the types of involved
// tables, i.e. the actual tables are of the same kind, and when they are reset (which is
// mandated by the spec sometimes) they are overwritten with values from the default tables.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_vp9_frame_context {
    pub tx8: [u8; 2][1],
    pub tx16: [u8; 2][2],
    pub tx32: [u8; 2][3],
    pub coef: [u8; 4][2][2][6][6][3],
    pub skip: [u8; 3],
    pub inter_mode: [u8; 7][3],
    pub interp_filter: [u8; 4][2],
    pub is_inter: [u8; 4],
    pub comp_mode: [u8; 5],
    pub single_ref: [u8; 5][2],
    pub comp_ref: [u8; 5],
    pub y_mode: [u8; 4][9],
    pub uv_mode: [u8; 10][9],
    pub partition: [u8; 16][3],
    pub mv: v4l2_vp9_frame_mv_context,
}

//
// struct v4l2_vp9_frame_symbol_counts - pointers to arrays of symbol counts
//
// @partition: partition counts.
// @skip: skip counts.
// @intra_inter: is inter-block counts.
// @tx32p: TX32 counts.
// @tx16p: TX16 counts.
// @tx8p: TX8 counts.
// @y_mode: Y prediction mode counts.
// @uv_mode: UV prediction mode counts.
// @comp: compound prediction mode counts.
// @comp_ref: compound ref counts.
// @single_ref: single ref counts.
// @mv_mode: inter mode counts.
// @filter: interpolation filter counts.
// @mv_joint: motion vector joint counts.
// @sign: motion vector sign counts.
// @classes: motion vector class counts.
// @class0: motion vector class0 bit counts.
// @bits: motion vector bits counts.
// @class0_fp: motion vector class0 fractional bit counts.
// @fp: motion vector fractional bit counts.
// @class0_hp: motion vector class0 high precision fractional bit counts.
// @hp: motion vector high precision fractional bit counts.
// @coeff: coefficient counts.
// @eob: eob counts
//
// The fields correspond to what is specified in section 8.3 "Clear counts process" of the spec.
// Different pieces of hardware can report the counts in different order, so we cannot rely on
// simply overlaying a struct on a relevant block of memory. Instead we provide pointers to
// arrays or array of pointers to arrays in case of coeff, or array of pointers for eob.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_vp9_frame_symbol_counts {
    pub (*partition)[16][4]: *mut u32,
    pub (*skip)[3][2]: *mut u32,
    pub (*intra_inter)[4][2]: *mut u32,
    pub (*tx32p)[2][4]: *mut u32,
    pub (*tx16p)[2][4]: *mut u32,
    pub (*tx8p)[2][2]: *mut u32,
    pub (*y_mode)[4][10]: *mut u32,
    pub (*uv_mode)[10][10]: *mut u32,
    pub (*comp)[5][2]: *mut u32,
    pub (*comp_ref)[5][2]: *mut u32,
    pub (*single_ref)[5][2][2]: *mut u32,
    pub (*mv_mode)[7][4]: *mut u32,
    pub (*filter)[4][3]: *mut u32,
    pub (*mv_joint)[4]: *mut u32,
    pub (*sign)[2][2]: *mut u32,
    pub (*classes)[2][11]: *mut u32,
    pub (*class0)[2][2]: *mut u32,
    pub (*bits)[2][10][2]: *mut u32,
    pub (*class0_fp)[2][2][4]: *mut u32,
    pub (*fp)[2][4]: *mut u32,
    pub (*class0_hp)[2][2]: *mut u32,
    pub (*hp)[2][2]: *mut u32,
    pub (*coeff[4][2][2][6][6])[3]: *mut u32,
    pub eob: [*mut u32; 4][2][2][6][6][2],
}

//
// v4l2_vp9_fw_update_probs() - Perform forward update of vp9 probabilities
//
// @probs: current probabilities values
// @deltas: delta values from compressed header
// @dec_params: vp9 frame decoding parameters
//
// This function performs forward updates of probabilities for the vp9 boolean decoder.
// The frame header can contain a directive to update the probabilities (deltas), if so, then
// the deltas are provided in the header, too. The userspace parses those and passes the said
// deltas struct to the kernel.
//
// v4l2_vp9_reset_frame_ctx() - Reset appropriate frame context
//
// @dec_params: vp9 frame decoding parameters
// @frame_context: array of the 4 frame contexts
//
// This function resets appropriate frame contexts, based on what's in dec_params.
//
// Returns the frame context index after the update, which might be reset to zero if
// mandated by the spec.
//
// v4l2_vp9_adapt_coef_probs() - Perform backward update of vp9 coefficients probabilities
//
// @probs: current probabilities values
// @counts: values of symbol counts after the current frame has been decoded
// @use_128: flag to request that 128 is used as update factor if true, otherwise 112 is used
// @frame_is_intra: flag indicating that FrameIsIntra is true
//
// This function performs backward updates of coefficients probabilities for the vp9 boolean
// decoder. After a frame has been decoded the counts of how many times a given symbol has
// occurred are known and are used to update the probability of each symbol.
//
// v4l2_vp9_adapt_noncoef_probs() - Perform backward update of vp9 non-coefficients probabilities
//
// @probs: current probabilities values
// @counts: values of symbol counts after the current frame has been decoded
// @reference_mode: specifies the type of inter prediction to be used. See
// &v4l2_vp9_reference_mode for more details
// @interpolation_filter: specifies the filter selection used for performing inter prediction.
// See &v4l2_vp9_interpolation_filter for more details
// @tx_mode: specifies the TX mode. See &v4l2_vp9_tx_mode for more details
// @flags: combination of V4L2_VP9_FRAME_FLAG_* flags
//
// This function performs backward updates of non-coefficients probabilities for the vp9 boolean
// decoder. After a frame has been decoded the counts of how many times a given symbol has
// occurred are known and are used to update the probability of each symbol.
//
// v4l2_vp9_seg_feat_enabled() - Check if a segmentation feature is enabled
//
// @feature_enabled: array of 8-bit flags (for all segments)
// @feature: id of the feature to check
// @segid: id of the segment to look up
//
// This function returns true if a given feature is active in a given segment.
//
