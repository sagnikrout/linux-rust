//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-h264.h
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
// Helper functions for H264 codecs.
//
// Copyright (c) 2019 Collabora, Ltd.
//
// Author: Boris Brezillon <boris.brezillon@collabora.com>
//

//
// struct v4l2_h264_reflist_builder - Reference list builder object
//
// @refs.top_field_order_cnt: top field order count
// @refs.bottom_field_order_cnt: bottom field order count
// @refs.frame_num: reference frame number
// @refs.longterm: set to true for a long term reference
// @refs: array of references
// @cur_pic_order_count: picture order count of the frame being decoded
// @cur_pic_fields: fields present in the frame being decoded
// @unordered_reflist: unordered list of references. Will be used to generate
// ordered P/B0/B1 lists
// @num_valid: number of valid references in the refs array
//
// This object stores the context of the P/B0/B1 reference list builder.
// This procedure is described in section '8.2.4 Decoding process for reference
// picture lists construction' of the H264 spec.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_h264_reflist_builder {
    pub top_field_order_cnt: i32,
    pub bottom_field_order_cnt: i32,
    pub frame_num: c_int,
    pub 1: u16 longterm :,
    pub refs: [}; V4L2_H264_NUM_DPB_ENTRIES],
    pub cur_pic_order_count: i32,
    pub cur_pic_fields: u8,
    pub unordered_reflist: [v4l2_h264_reference; V4L2_H264_REF_LIST_LEN],
    pub num_valid: u8,
}

//
// v4l2_h264_build_b_ref_lists() - Build the B0/B1 reference lists
//
// @builder: reference list builder context
// @b0_reflist: 32 sized array used to store the B0 reference list. Each entry
// is a v4l2_h264_reference structure
// @b1_reflist: 32 sized array used to store the B1 reference list. Each entry
// is a v4l2_h264_reference structure
//
// This functions builds the B0/B1 reference lists. This procedure is described
// in section '8.2.4 Decoding process for reference picture lists construction'
// of the H264 spec. This function can be used by H264 decoder drivers that
// need to pass B0/B1 reference lists to the hardware.
//
// v4l2_h264_build_p_ref_list() - Build the P reference list
//
// @builder: reference list builder context
// @reflist: 32 sized array used to store the P reference list. Each entry
// is a v4l2_h264_reference structure
//
// This functions builds the P reference lists. This procedure is describe in
// section '8.2.4 Decoding process for reference picture lists construction'
// of the H264 spec. This function can be used by H264 decoder drivers that
// need to pass a P reference list to the hardware.
//
