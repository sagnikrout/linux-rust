//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/decoder/vdec/vdec_h264_req_common.h
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
// Copyright (c) 2022 MediaTek Inc.
// Author: Yunfei Dong <yunfei.dong@mediatek.com>
//

pub const NAL_NON_IDR_SLICE: c_uint = 0x01;
pub const NAL_IDR_SLICE: c_uint = 0x05;

pub const MB_UNIT_LEN: c_int = 16;
// motion vector size (bytes) for every macro block
pub const HW_MB_STORE_SZ: c_int = 64;
pub const H264_MAX_MV_NUM: c_int = 32;
//
// struct mtk_h264_dpb_info  - h264 dpb information
//
// @y_dma_addr:	Y bitstream physical address
// @c_dma_addr:	CbCr bitstream physical address
// @reference_flag:	reference picture flag (short/long term reference picture)
// @field:		field picture flag
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_h264_dpb_info {
    pub y_dma_addr: dma_addr_t,
    pub c_dma_addr: dma_addr_t,
    pub reference_flag: c_int,
    pub field: c_int,
}

//
// struct mtk_h264_sps_param  - parameters for sps
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_h264_sps_param {
    pub chroma_format_idc: c_uchar,
    pub bit_depth_luma_minus8: c_uchar,
    pub bit_depth_chroma_minus8: c_uchar,
    pub log2_max_frame_num_minus4: c_uchar,
    pub pic_order_cnt_type: c_uchar,
    pub log2_max_pic_order_cnt_lsb_minus4: c_uchar,
    pub max_num_ref_frames: c_uchar,
    pub separate_colour_plane_flag: c_uchar,
    pub pic_width_in_mbs_minus1: c_ushort,
    pub pic_height_in_map_units_minus1: c_ushort,
    pub max_frame_nums: c_uint,
    pub qpprime_y_zero_transform_bypass_flag: c_uchar,
    pub delta_pic_order_always_zero_flag: c_uchar,
    pub frame_mbs_only_flag: c_uchar,
    pub mb_adaptive_frame_field_flag: c_uchar,
    pub direct_8x8_inference_flag: c_uchar,
    pub reserved: [c_uchar; 3],
}

//
// struct mtk_h264_pps_param  - parameters for pps
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_h264_pps_param {
    pub num_ref_idx_l0_default_active_minus1: c_uchar,
    pub num_ref_idx_l1_default_active_minus1: c_uchar,
    pub weighted_bipred_idc: c_uchar,
    pub pic_init_qp_minus26: c_char,
    pub chroma_qp_index_offset: c_char,
    pub second_chroma_qp_index_offset: c_char,
    pub entropy_coding_mode_flag: c_uchar,
    pub pic_order_present_flag: c_uchar,
    pub deblocking_filter_control_present_flag: c_uchar,
    pub constrained_intra_pred_flag: c_uchar,
    pub weighted_pred_flag: c_uchar,
    pub redundant_pic_cnt_present_flag: c_uchar,
    pub transform_8x8_mode_flag: c_uchar,
    pub scaling_matrix_present_flag: c_uchar,
    pub reserved: [c_uchar; 2],
}

//
// struct mtk_h264_slice_hd_param  - parameters for slice header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_h264_slice_hd_param {
    pub first_mb_in_slice: c_uint,
    pub field_pic_flag: c_uint,
    pub slice_type: c_uint,
    pub frame_num: c_uint,
    pub pic_order_cnt_lsb: c_int,
    pub delta_pic_order_cnt_bottom: c_int,
    pub bottom_field_flag: c_uint,
    pub direct_spatial_mv_pred_flag: c_uint,
    pub delta_pic_order_cnt0: c_int,
    pub delta_pic_order_cnt1: c_int,
    pub cabac_init_idc: c_uint,
    pub slice_qp_delta: c_int,
    pub disable_deblocking_filter_idc: c_uint,
    pub slice_alpha_c0_offset_div2: c_int,
    pub slice_beta_offset_div2: c_int,
    pub num_ref_idx_l0_active_minus1: c_uint,
    pub num_ref_idx_l1_active_minus1: c_uint,
    pub reserved: c_uint,
}

//
// struct slice_api_h264_scaling_matrix  - parameters for scaling list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slice_api_h264_scaling_matrix {
    pub scaling_list_4x4: [c_uchar; 6][16],
    pub scaling_list_8x8: [c_uchar; 6][64],
}

//
// struct slice_h264_dpb_entry  - each dpb information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slice_h264_dpb_entry {
    pub reference_ts: c_ulonglong,
    pub frame_num: c_ushort,
    pub pic_num: c_ushort,
// Note that field is indicated by v4l2_buffer.field
    pub top_field_order_cnt: c_int,
    pub bottom_field_order_cnt: c_int,
    pub flags: c_uint,
}

//
// struct slice_api_h264_decode_param - parameters for decode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slice_api_h264_decode_param {
    pub dpb: [slice_h264_dpb_entry; V4L2_H264_NUM_DPB_ENTRIES],
    pub num_slices: c_ushort,
    pub nal_ref_idc: c_ushort,
    pub ref_pic_list_p0: [c_uchar; 32],
    pub ref_pic_list_b0: [c_uchar; 32],
    pub ref_pic_list_b1: [c_uchar; 32],
    pub top_field_order_cnt: c_int,
    pub bottom_field_order_cnt: c_int,
    pub flags: c_uint,
}

//
// struct h264_fb - h264 decode frame buffer information
//
// @vdec_fb_va:	virtual address of struct vdec_fb
// @y_fb_dma:		dma address of Y frame buffer (luma)
// @c_fb_dma:		dma address of C frame buffer (chroma)
// @poc:		picture order count of frame buffer
// @reserved:		for 8 bytes alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct h264_fb {
    pub vdec_fb_va: u64,
    pub y_fb_dma: u64,
    pub c_fb_dma: u64,
    pub poc: i32,
    pub reserved: u32,
}

//
// mtk_vdec_h264_get_ref_list - translate V4L2 reference list
//
// @ref_list:		Mediatek reference picture list
// @v4l2_ref_list:	V4L2 reference picture list
// @num_valid:		used reference number
//
// mtk_vdec_h264_get_ctrl_ptr - get each CID control address.
//
// @ctx:	v4l2 ctx
// @id:	CID control ID
//
// Return: returns CID ctrl address.
//
// mtk_vdec_h264_fill_dpb_info - Fill the decoded picture buffer info
//
// @ctx:		v4l2 ctx
// @decode_params:	slice decode params
// @h264_dpb_info:	dpb buffer information
//
// mtk_vdec_h264_copy_sps_params - get sps params.
//
// @dst_param:	sps params for hw decoder
// @src_param:	sps params from user driver
//
// mtk_vdec_h264_copy_pps_params - get pps params.
//
// @dst_param:	pps params for hw decoder
// @src_param:	pps params from user driver
//
// mtk_vdec_h264_copy_slice_hd_params - get slice header params.
//
// @dst_param:	slice params for hw decoder
// @src_param:	slice params from user driver
// @dec_param:	decode params from user driver
//
// mtk_vdec_h264_copy_scaling_matrix - Copy scaling matrix from a control to the driver
//
// @dst_matrix:	scaling list params for the HW decoder
// @src_matrix:	scaling list params from a V4L2 control
//
// This function is used to copy the scaling matrix from a
// v4l2 control into the slice parameters for a decode.
//
// mtk_vdec_h264_copy_decode_params - get decode params.
//
// @dst_params:	dst params for hw decoder
// @src_params:	decode params from user driver
// @dpb:		dpb information
//
// mtk_vdec_h264_update_dpb - update dpb list.
//
// @dec_param:	v4l2 control decode params
// @dpb:	dpb entry informaton
//
// mtk_vdec_h264_find_start_code - find h264 start code using sofeware.
//
// @data:	input buffer address
// @data_sz:	input buffer size
//
// Return: returns start code position.
//
extern "C" {
    pub fn mtk_vdec_h264_find_start_code(data: *mut c_uchar, data_sz: c_uint) -> c_int;
}
//
// mtk_vdec_h264_get_mv_buf_size - get mv buffer size.
//
// @width:	picture width
// @height:	picture height
//
// Return: returns mv buffer size.
//
extern "C" {
    pub fn mtk_vdec_h264_get_mv_buf_size(width: c_uint, height: c_uint) -> c_uint;
}
