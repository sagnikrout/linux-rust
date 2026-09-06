//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/amphion/vpu_codec.h
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
// Copyright 2020-2021 NXP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_encode_params {
    pub input_format: u32,
    pub codec_format: u32,
    pub profile: u32,
    pub tier: u32,
    pub level: u32,
    pub frame_rate: v4l2_fract,
    pub src_stride: u32,
    pub src_width: u32,
    pub src_height: u32,
    pub crop: v4l2_rect,
    pub out_width: u32,
    pub out_height: u32,
    pub gop_length: u32,
    pub bframes: u32,
    pub rc_enable: u32,
    pub rc_mode: u32,
    pub bitrate: u32,
    pub bitrate_min: u32,
    pub bitrate_max: u32,
    pub i_frame_qp: u32,
    pub p_frame_qp: u32,
    pub b_frame_qp: u32,
    pub qp_min: u32,
    pub qp_max: u32,
    pub qp_min_i: u32,
    pub qp_max_i: u32,
    pub enable: u32,
    pub idc: u32,
    pub width: u32,
    pub height: u32,
    pub sar: },
    pub primaries: u32,
    pub transfer: u32,
    pub matrix: u32,
    pub full_range: u32,
    pub color: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_decode_params {
    pub codec_format: u32,
    pub output_format: u32,
    pub display_delay_enable: u32,
    pub display_delay: u32,
    pub b_non_frame: u32,
    pub frame_count: u32,
    pub end_flag: u32,
    pub base: u32,
    pub size: u32,
    pub udata: },
}
