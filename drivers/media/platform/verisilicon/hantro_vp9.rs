//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/verisilicon/hantro_vp9.h
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
// Hantro VP9 codec driver
//
// Copyright (C) 2021 Collabora Ltd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_g2_mv_probs {
    pub joint: [u8; 3],
    pub sign: [u8; 2],
    pub class0_bit: [u8; 2][1],
    pub fr: [u8; 2][3],
    pub class0_hp: [u8; 2],
    pub hp: [u8; 2],
    pub classes: [u8; 2][10],
    pub class0_fr: [u8; 2][2][3],
    pub bits: [u8; 2][10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_g2_probs {
    pub inter_mode: [u8; 7][4],
    pub is_inter: [u8; 4],
    pub uv_mode: [u8; 10][8],
    pub tx8: [u8; 2][1],
    pub tx16: [u8; 2][2],
    pub tx32: [u8; 2][3],
    pub y_mode_tail: [u8; 4][1],
    pub y_mode: [u8; 4][8],
    pub /: *mut *mut u8 partition[2][16][4]; / [keyframe][][], [inter][][],
    pub uv_mode_tail: [u8; 10][1],
    pub interp_filter: [u8; 4][2],
    pub comp_mode: [u8; 5],
    pub skip: [u8; 3],
    pub pad1: [u8; 1],
    pub mv: hantro_g2_mv_probs,
    pub single_ref: [u8; 5][2],
    pub comp_ref: [u8; 5],
    pub pad2: [u8; 17],
    pub coef: [u8; 4][2][2][6][6][4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_g2_all_probs {
    pub kf_y_mode_prob: [u8; 10][10][8],
    pub kf_y_mode_prob_tail: [u8; 10][10][1],
    pub ref_pred_probs: [u8; 3],
    pub mb_segment_tree_probs: [u8; 7],
    pub segment_pred_probs: [u8; 3],
    pub ref_scores: [u8; 4],
    pub prob_comppred: [u8; 2],
    pub pad1: [u8; 9],
    pub kf_uv_mode_prob: [u8; 10][8],
    pub kf_uv_mode_prob_tail: [u8; 10][1],
    pub pad2: [u8; 6],
    pub probs: hantro_g2_probs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_counts {
    pub joints: [u32; 4],
    pub sign: [u32; 2][2],
    pub classes: [u32; 2][11],
    pub class0: [u32; 2][2],
    pub bits: [u32; 2][10][2],
    pub class0_fp: [u32; 2][2][4],
    pub fp: [u32; 2][4],
    pub class0_hp: [u32; 2][2],
    pub hp: [u32; 2][2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct symbol_counts {
    pub inter_mode_counts: [u32; 7][3][2],
    pub sb_ymode_counts: [u32; 4][10],
    pub uv_mode_counts: [u32; 10][10],
    pub partition_counts: [u32; 16][4],
    pub switchable_interp_counts: [u32; 4][3],
    pub intra_inter_count: [u32; 4][2],
    pub comp_inter_count: [u32; 5][2],
    pub single_ref_count: [u32; 5][2][2],
    pub comp_ref_count: [u32; 5][2],
    pub tx32x32_count: [u32; 2][4],
    pub tx16x16_count: [u32; 2][3],
    pub tx8x8_count: [u32; 2][2],
    pub mbskip_count: [u32; 3][2],
    pub mv_counts: mv_counts,
    pub count_coeffs: [u32; 2][2][6][6][4],
    pub count_coeffs8x8: [u32; 2][2][6][6][4],
    pub count_coeffs16x16: [u32; 2][2][6][6][4],
    pub count_coeffs32x32: [u32; 2][2][6][6][4],
    pub count_eobs: [u32; 4][2][2][6][6],
}
