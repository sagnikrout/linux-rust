//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/amphion/vpu_defs.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MSG_TYPE {
    INIT_DONE = 1,
    PRC_BUF_OFFSET,
    BOOT_ADDRESS,
    COMMAND,
    EVENT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VPU_ENC_MEMORY_RESOURSE {
    MEM_RES_ENC,
    MEM_RES_REF,
    MEM_RES_ACT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VPU_DEC_MEMORY_RESOURCE {
    MEM_RES_FRAME,
    MEM_RES_MBI,
    MEM_RES_DCP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VPU_SCODE_TYPE {
    SCODE_PADDING_EOS = 1,
    SCODE_PADDING_BUFFLUSH = 2,
    SCODE_PADDING_ABORT = 3,
    SCODE_SEQUENCE = 0x31,
    SCODE_PICTURE = 0x32,
    SCODE_SLICE = 0x33
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_pkt_mem_req_data {
    pub enc_frame_size: u32,
    pub enc_frame_num: u32,
    pub ref_frame_size: u32,
    pub ref_frame_num: u32,
    pub act_buf_size: u32,
    pub act_buf_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_enc_pic_info {
    pub frame_id: u32,
    pub pic_type: u32,
    pub skipped_frame: u32,
    pub error_flag: u32,
    pub psnr: u32,
    pub frame_size: u32,
    pub wptr: u32,
    pub crc: u32,
    pub timestamp: i64,
    pub average_qp: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_dec_codec_info {
    pub pixfmt: u32,
    pub num_ref_frms: u32,
    pub num_dpb_frms: u32,
    pub num_dfe_area: u32,
    pub color_primaries: u32,
    pub transfer_chars: u32,
    pub matrix_coeffs: u32,
    pub full_range: u32,
    pub vui_present: u32,
    pub progressive: u32,
    pub width: u32,
    pub height: u32,
    pub decoded_width: u32,
    pub decoded_height: u32,
    pub frame_rate: v4l2_fract,
    pub dsp_asp_ratio: u32,
    pub profile_idc: u32,
    pub level_idc: u32,
    pub bit_depth_luma: u32,
    pub bit_depth_chroma: u32,
    pub chroma_fmt: u32,
    pub mvc_num_views: u32,
    pub offset_x: u32,
    pub offset_y: u32,
    pub tag: u32,
    pub sizeimage: [u32; VIDEO_MAX_PLANES],
    pub bytesperline: [u32; VIDEO_MAX_PLANES],
    pub mbi_size: u32,
    pub dcp_size: u32,
    pub stride: u32,
    pub 1: u32 constraint_set5_flag :,
    pub 1: u32 constraint_set4_flag :,
    pub 1: u32 constraint_set3_flag :,
    pub 1: u32 constraint_set2_flag :,
    pub 1: u32 constraint_set1_flag :,
    pub 1: u32 constraint_set0_flag :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_dec_pic_info {
    pub id: u32,
    pub luma: u32,
    pub start: u32,
    pub end: u32,
    pub pic_size: u32,
    pub stride: u32,
    pub skipped: u32,
    pub timestamp: i64,
    pub consumed_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_fs_info {
    pub id: u32,
    pub type: u32,
    pub tag: u32,
    pub luma_addr: u32,
    pub luma_size: u32,
    pub chroma_addr: u32,
    pub chromau_size: u32,
    pub chromav_addr: u32,
    pub chromav_size: u32,
    pub bytesperline: u32,
    pub not_displayed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_ts_info {
    pub timestamp: i64,
    pub size: u32,
}

