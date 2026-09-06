//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/decoder/vdec_ipi_msg.h
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
// Author: PC Chen <pc.chen@mediatek.com>
//
// enum vdec_ipi_msgid - message id between AP and VPU
// @AP_IPIMSG_XXX	: AP to VPU cmd message id
// @VPU_IPIMSG_XXX_ACK	: VPU ack AP cmd message id
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vdec_ipi_msgid {
    AP_IPIMSG_DEC_INIT = 0xA000,
    AP_IPIMSG_DEC_START = 0xA001,
    AP_IPIMSG_DEC_END = 0xA002,
    AP_IPIMSG_DEC_DEINIT = 0xA003,
    AP_IPIMSG_DEC_RESET = 0xA004,
    AP_IPIMSG_DEC_CORE = 0xA005,
    AP_IPIMSG_DEC_CORE_END = 0xA006,
    AP_IPIMSG_DEC_GET_PARAM = 0xA007,

    VPU_IPIMSG_DEC_INIT_ACK = 0xB000,
    VPU_IPIMSG_DEC_START_ACK = 0xB001,
    VPU_IPIMSG_DEC_END_ACK = 0xB002,
    VPU_IPIMSG_DEC_DEINIT_ACK = 0xB003,
    VPU_IPIMSG_DEC_RESET_ACK = 0xB004,
    VPU_IPIMSG_DEC_CORE_ACK = 0xB005,
    VPU_IPIMSG_DEC_CORE_END_ACK = 0xB006,
    VPU_IPIMSG_DEC_GET_PARAM_ACK = 0xB007,
}

//
// struct vdec_ap_ipi_cmd - generic AP to VPU ipi command format
// @msg_id	: vdec_ipi_msgid
// @vpu_inst_addr : VPU decoder instance address. Used if ABI version < 2.
// @inst_id     : instance ID. Used if the ABI version >= 2.
// @codec_type	: codec fourcc
// @reserved	: reserved param
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_ap_ipi_cmd {
    pub msg_id: u32,
    pub vpu_inst_addr: u32,
    pub inst_id: u32,
}

//
// struct vdec_vpu_ipi_ack - generic VPU to AP ipi command format
// @msg_id	: vdec_ipi_msgid
// @status	: VPU exeuction result
// @ap_inst_addr	: AP video decoder instance address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_vpu_ipi_ack {
    pub msg_id: u32,
    pub status: i32,
    pub ap_inst_addr: u64,
}

//
// struct vdec_ap_ipi_init - for AP_IPIMSG_DEC_INIT
// @msg_id	: AP_IPIMSG_DEC_INIT
// @codec_type	: codec fourcc
// @ap_inst_addr	: AP video decoder instance address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_ap_ipi_init {
    pub msg_id: u32,
    pub codec_type: u32,
    pub ap_inst_addr: u64,
}

//
// struct vdec_ap_ipi_dec_start - for AP_IPIMSG_DEC_START
// @msg_id	: AP_IPIMSG_DEC_START
// @vpu_inst_addr : VPU decoder instance address. Used if ABI version < 2.
// @inst_id     : instance ID. Used if the ABI version >= 2.
// @data	: Header info
// H264 decoder [0]:buf_sz [1]:nal_start
// VP8 decoder  [0]:width/height
// VP9 decoder  [0]:profile, [1][2] width/height
// @codec_type	: codec fourcc
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_ap_ipi_dec_start {
    pub msg_id: u32,
    pub vpu_inst_addr: u32,
    pub inst_id: u32,
}

//
// struct vdec_vpu_ipi_init_ack - for VPU_IPIMSG_DEC_INIT_ACK
// @msg_id	: VPU_IPIMSG_DEC_INIT_ACK
// @status	: VPU exeuction result
// @ap_inst_addr	: AP vcodec_vpu_inst instance address
// @vpu_inst_addr	: VPU decoder instance address
// @vdec_abi_version:	ABI version of the firmware. Kernel can use it to
// ensure that it is compatible with the firmware.
// This field is not valid for MT8173 and must not be
// accessed for this chip.
// @inst_id     : instance ID. Valid only if the ABI version >= 2.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_vpu_ipi_init_ack {
    pub msg_id: u32,
    pub status: i32,
    pub ap_inst_addr: u64,
    pub vpu_inst_addr: u32,
    pub vdec_abi_version: u32,
    pub inst_id: u32,
}

//
// struct vdec_ap_ipi_get_param - for AP_IPIMSG_DEC_GET_PARAM
// @msg_id	: AP_IPIMSG_DEC_GET_PARAM
// @inst_id     : instance ID. Used if the ABI version >= 2.
// @data	: picture information
// @param_type	: get param type
// @codec_type	: Codec fourcc
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_ap_ipi_get_param {
    pub msg_id: u32,
    pub inst_id: u32,
    pub data: [u32; 4],
    pub param_type: u32,
    pub codec_type: u32,
}

//
// struct vdec_vpu_ipi_get_param_ack - for VPU_IPIMSG_DEC_GET_PARAM_ACK
// @msg_id	: VPU_IPIMSG_DEC_GET_PARAM_ACK
// @status	: VPU execution result
// @ap_inst_addr	: AP vcodec_vpu_inst instance address
// @data     : picture information from SCP.
// @param_type	: get param type
// @reserved : reserved param
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_vpu_ipi_get_param_ack {
    pub msg_id: u32,
    pub status: i32,
    pub ap_inst_addr: u64,
    pub data: [u32; 4],
    pub param_type: u32,
    pub reserved: u32,
}
