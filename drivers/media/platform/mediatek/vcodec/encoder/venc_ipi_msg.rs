//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/encoder/venc_ipi_msg.h
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
// Copyright (c) 2016 MediaTek Inc.
// Author: Jungchang Tsao <jungchang.tsao@mediatek.com>
// Daniel Hsiao <daniel.hsiao@mediatek.com>
// Tiffany Lin <tiffany.lin@mediatek.com>
//
pub const AP_IPIMSG_VENC_BASE: c_uint = 0xC000;
pub const VPU_IPIMSG_VENC_BASE: c_uint = 0xD000;
//
// enum venc_ipi_msg_id - message id between AP and VPU
// (ipi stands for inter-processor interrupt)
// @AP_IPIMSG_ENC_XXX:		AP to VPU cmd message id
// @VPU_IPIMSG_ENC_XXX_DONE:	VPU ack AP cmd message id
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum venc_ipi_msg_id {
    AP_IPIMSG_ENC_INIT = AP_IPIMSG_VENC_BASE,
    AP_IPIMSG_ENC_SET_PARAM,
    AP_IPIMSG_ENC_ENCODE,
    AP_IPIMSG_ENC_DEINIT,

    VPU_IPIMSG_ENC_INIT_DONE = VPU_IPIMSG_VENC_BASE,
    VPU_IPIMSG_ENC_SET_PARAM_DONE,
    VPU_IPIMSG_ENC_ENCODE_DONE,
    VPU_IPIMSG_ENC_DEINIT_DONE,
}

//
// struct venc_ap_ipi_msg_init - AP to VPU init cmd structure
// @msg_id:	message id (AP_IPIMSG_XXX_ENC_INIT)
// @reserved:	reserved for future use. vpu is running in 32bit. Without
// this reserved field, if kernel run in 64bit. this struct size
// will be different between kernel and vpu
// @venc_inst:	AP encoder instance
// (struct venc_vp8_inst/venc_h264_inst *)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_ap_ipi_msg_init {
    pub msg_id: u32,
    pub reserved: u32,
    pub venc_inst: u64,
}

//
// struct venc_ap_ipi_msg_set_param - AP to VPU set_param cmd structure
// @msg_id:	message id (AP_IPIMSG_XXX_ENC_SET_PARAM)
// @vpu_inst_addr:	VPU encoder instance addr
// (struct venc_vp8_vsi/venc_h264_vsi *)
// @param_id:	parameter id (venc_set_param_type)
// @data_item:	number of items in the data array
// @data:	data array to store the set parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_ap_ipi_msg_set_param {
    pub msg_id: u32,
    pub vpu_inst_addr: u32,
    pub param_id: u32,
    pub data_item: u32,
    pub data: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_ap_ipi_msg_set_param_ext {
    pub base: venc_ap_ipi_msg_set_param,
    pub data_ext: [u32; 24],
}

//
// struct venc_ap_ipi_msg_enc - AP to VPU enc cmd structure
// @msg_id:	message id (AP_IPIMSG_XXX_ENC_ENCODE)
// @vpu_inst_addr:	VPU encoder instance addr
// (struct venc_vp8_vsi/venc_h264_vsi *)
// @bs_mode:	bitstream mode for h264
// (H264_BS_MODE_SPS/H264_BS_MODE_PPS/H264_BS_MODE_FRAME)
// @input_addr:	pointer to input image buffer plane
// @bs_addr:	pointer to output bit stream buffer
// @bs_size:	bit stream buffer size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_ap_ipi_msg_enc {
    pub msg_id: u32,
    pub vpu_inst_addr: u32,
    pub bs_mode: u32,
    pub input_addr: [u32; 3],
    pub bs_addr: u32,
    pub bs_size: u32,
}

//
// struct venc_ap_ipi_msg_enc_ext - AP to SCP extended enc cmd structure
//
// @base:	base msg structure
// @data_item:	number of items in the data array
// @data:	data array to store the set parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_ap_ipi_msg_enc_ext {
    pub base: venc_ap_ipi_msg_enc,
    pub data_item: u32,
    pub data: [u32; 32],
}

//
// struct venc_ap_ipi_msg_enc_ext_34 - AP to SCP extended enc cmd structure
// @msg_id:		message id (AP_IPIMSG_XXX_ENC_ENCODE)
// @vpu_inst_addr:	VPU encoder instance addr
// @bs_mode:		bitstream mode for h264
// @reserved:		for struct padding
// @input_addr:		input frame buffer 34 bit address
// @bs_addr:		output bitstream buffer 34 bit address
// @bs_size:		bitstream buffer size
// @data_item:		number of items in the data array
// @data:		data array to store the set parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_ap_ipi_msg_enc_ext_34 {
    pub msg_id: u32,
    pub vpu_inst_addr: u32,
    pub bs_mode: u32,
    pub reserved: u32,
    pub input_addr: [u64; 3],
    pub bs_addr: u64,
    pub bs_size: u32,
    pub data_item: u32,
    pub data: [u32; 32],
}

//
// struct venc_ap_ipi_msg_deinit - AP to VPU deinit cmd structure
// @msg_id:	message id (AP_IPIMSG_XXX_ENC_DEINIT)
// @vpu_inst_addr:	VPU encoder instance addr
// (struct venc_vp8_vsi/venc_h264_vsi *)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_ap_ipi_msg_deinit {
    pub msg_id: u32,
    pub vpu_inst_addr: u32,
}

//
// enum venc_ipi_msg_status - VPU ack AP cmd status
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum venc_ipi_msg_status {
    VENC_IPI_MSG_STATUS_OK,
    VENC_IPI_MSG_STATUS_FAIL,
}

//
// struct venc_vpu_ipi_msg_common - VPU ack AP cmd common structure
// @msg_id:	message id (VPU_IPIMSG_XXX_DONE)
// @status:	cmd status (venc_ipi_msg_status)
// @venc_inst:	AP encoder instance (struct venc_vp8_inst/venc_h264_inst *)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_vpu_ipi_msg_common {
    pub msg_id: u32,
    pub status: u32,
    pub venc_inst: u64,
}

//
// struct venc_vpu_ipi_msg_init - VPU ack AP init cmd structure
// @msg_id:	message id (VPU_IPIMSG_XXX_ENC_SET_PARAM_DONE)
// @status:	cmd status (venc_ipi_msg_status)
// @venc_inst:	AP encoder instance (struct venc_vp8_inst/venc_h264_inst *)
// @vpu_inst_addr:	VPU encoder instance addr
// (struct venc_vp8_vsi/venc_h264_vsi *)
// @venc_abi_version:	ABI version of the firmware. Kernel can use it to
// ensure that it is compatible with the firmware.
// For MT8173 the value of this field is undefined and
// should not be used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_vpu_ipi_msg_init {
    pub msg_id: u32,
    pub status: u32,
    pub venc_inst: u64,
    pub vpu_inst_addr: u32,
    pub venc_abi_version: u32,
}

//
// struct venc_vpu_ipi_msg_set_param - VPU ack AP set_param cmd structure
// @msg_id:	message id (VPU_IPIMSG_XXX_ENC_SET_PARAM_DONE)
// @status:	cmd status (venc_ipi_msg_status)
// @venc_inst:	AP encoder instance (struct venc_vp8_inst/venc_h264_inst *)
// @param_id:	parameter id (venc_set_param_type)
// @data_item:	number of items in the data array
// @data:	data array to store the return result
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_vpu_ipi_msg_set_param {
    pub msg_id: u32,
    pub status: u32,
    pub venc_inst: u64,
    pub param_id: u32,
    pub data_item: u32,
    pub data: [u32; 6],
}

//
// enum venc_ipi_msg_enc_state - Type of encode state
// @VEN_IPI_MSG_ENC_STATE_FRAME:	one frame being encoded
// @VEN_IPI_MSG_ENC_STATE_PART:		bit stream buffer full
// @VEN_IPI_MSG_ENC_STATE_SKIP:		encoded skip frame
// @VEN_IPI_MSG_ENC_STATE_ERROR:	encounter error
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum venc_ipi_msg_enc_state {
    VEN_IPI_MSG_ENC_STATE_FRAME,
    VEN_IPI_MSG_ENC_STATE_PART,
    VEN_IPI_MSG_ENC_STATE_SKIP,
    VEN_IPI_MSG_ENC_STATE_ERROR,
}

//
// struct venc_vpu_ipi_msg_enc - VPU ack AP enc cmd structure
// @msg_id:	message id (VPU_IPIMSG_XXX_ENC_ENCODE_DONE)
// @status:	cmd status (venc_ipi_msg_status)
// @venc_inst:	AP encoder instance (struct venc_vp8_inst/venc_h264_inst *)
// @state:	encode state (venc_ipi_msg_enc_state)
// @is_key_frm:	whether the encoded frame is key frame
// @bs_size:	encoded bitstream size
// @reserved:	reserved for future use. vpu is running in 32bit. Without
// this reserved field, if kernel run in 64bit. this struct size
// will be different between kernel and vpu
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_vpu_ipi_msg_enc {
    pub msg_id: u32,
    pub status: u32,
    pub venc_inst: u64,
    pub state: u32,
    pub is_key_frm: u32,
    pub bs_size: u32,
    pub reserved: u32,
}

//
// struct venc_vpu_ipi_msg_deinit - VPU ack AP deinit cmd structure
// @msg_id:   message id (VPU_IPIMSG_XXX_ENC_DEINIT_DONE)
// @status:   cmd status (venc_ipi_msg_status)
// @venc_inst:	AP encoder instance (struct venc_vp8_inst/venc_h264_inst *)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_vpu_ipi_msg_deinit {
    pub msg_id: u32,
    pub status: u32,
    pub venc_inst: u64,
}
