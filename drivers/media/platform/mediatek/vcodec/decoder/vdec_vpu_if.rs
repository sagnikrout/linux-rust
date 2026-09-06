//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/decoder/vdec_vpu_if.h
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
// struct vdec_vpu_inst - VPU instance for video codec
// @id          : ipi msg id for each decoder
// @core_id     : core id used to separate different hardware
// @vsi         : driver structure allocated by VPU side and shared to AP side
// for control and info share
// @failure     : VPU execution result status, 0: success, others: fail
// @inst_addr	: VPU decoder instance address
// @fw_abi_version : ABI version of the firmware.
// @inst_id	: if fw_abi_version >= 2, contains the instance ID to be given
// in place of inst_addr in messages.
// @signaled    : 1 - Host has received ack message from VPU, 0 - not received
// @ctx         : context for v4l2 layer integration
// @wq          : wait queue to wait VPU message ack
// @handler     : ipi handler for each decoder
// @codec_type     : use codec type to separate different codecs
// @capture_type:	used capture type to separate different capture format
// @fb_sz  : frame buffer size of each plane
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_vpu_inst {
    pub id: c_int,
    pub core_id: c_int,
    pub vsi: *mut c_void,
    pub failure: i32,
    pub inst_addr: u32,
    pub fw_abi_version: u32,
    pub inst_id: u32,
    pub signaled: c_uint,
    pub ctx: *mut mtk_vcodec_dec_ctx,
    pub wq: wait_queue_head_t,
    pub handler: mtk_vcodec_ipi_handler,
    pub codec_type: c_uint,
    pub capture_type: c_uint,
    pub fb_sz: [c_uint; 2],
}

//
// vpu_dec_init - init decoder instance and allocate required resource in VPU.
//
// @vpu: instance for vdec_vpu_inst
//
extern "C" {
    pub fn vpu_dec_init(vpu: *mut vdec_vpu_inst) -> c_int;
}
//
// vpu_dec_start - start decoding, basically the function will be invoked once
// every frame.
//
// @vpu : instance for vdec_vpu_inst
// @data: meta data to pass bitstream info to VPU decoder
// @len : meta data length
//
extern "C" {
    pub fn vpu_dec_start(vpu: *mut vdec_vpu_inst, data: *mut u32, len: c_uint) -> c_int;
}
//
// vpu_dec_end - end decoding, basically the function will be invoked once
// when HW decoding done interrupt received successfully. The
// decoder in VPU will continue to do reference frame management
// and check if there is a new decoded frame available to display.
//
// @vpu : instance for vdec_vpu_inst
//
extern "C" {
    pub fn vpu_dec_end(vpu: *mut vdec_vpu_inst) -> c_int;
}
//
// vpu_dec_deinit - deinit decoder instance and resource freed in VPU.
//
// @vpu: instance for vdec_vpu_inst
//
extern "C" {
    pub fn vpu_dec_deinit(vpu: *mut vdec_vpu_inst) -> c_int;
}
//
// vpu_dec_reset - reset decoder, use for flush decoder when end of stream or
// seek. Remaining non displayed frame will be pushed to display.
//
// @vpu: instance for vdec_vpu_inst
//
extern "C" {
    pub fn vpu_dec_reset(vpu: *mut vdec_vpu_inst) -> c_int;
}
//
// vpu_dec_core - core start decoding, basically the function will be invoked once
// every frame.
//
// @vpu : instance for vdec_vpu_inst
//
extern "C" {
    pub fn vpu_dec_core(vpu: *mut vdec_vpu_inst) -> c_int;
}
//
// vpu_dec_core_end - core end decoding, basically the function will be invoked once
// when core HW decoding done and receive interrupt successfully. The
// decoder in VPU will update hardware information and deinit hardware
// and check if there is a new decoded frame available to display.
//
// @vpu : instance for vdec_vpu_inst
//
extern "C" {
    pub fn vpu_dec_core_end(vpu: *mut vdec_vpu_inst) -> c_int;
}
//
// vpu_dec_get_param - get param from scp
//
// @vpu : instance for vdec_vpu_inst
// @data: meta data to pass bitstream info to VPU decoder
// @len : meta data length
// @param_type : get param type
//
