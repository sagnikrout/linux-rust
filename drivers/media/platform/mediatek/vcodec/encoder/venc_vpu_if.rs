//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/encoder/venc_vpu_if.h
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
// Author: PoChun Lin <pochun.lin@mediatek.com>
//

//
// struct venc_vpu_inst - encoder VPU driver instance
// @wq_hd: wait queue used for vpu cmd trigger then wait vpu interrupt done
// @signaled: flag used for checking vpu interrupt done
// @failure: flag to show vpu cmd succeeds or not
// @state: enum venc_ipi_msg_enc_state
// @bs_size: bitstream size for skip frame case usage
// @is_key_frm: key frame flag
// @inst_addr: VPU instance addr
// @vsi: driver structure allocated by VPU side and shared to AP side for
// control and info share
// @id: the id of inter-processor interrupt
// @ctx: context for v4l2 layer integration
// @dev: device for v4l2 layer integration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_vpu_inst {
    pub wq_hd: wait_queue_head_t,
    pub signaled: c_int,
    pub failure: c_int,
    pub state: c_int,
    pub bs_size: c_int,
    pub is_key_frm: c_int,
    pub inst_addr: c_uint,
    pub vsi: *mut c_void,
    pub id: c_int,
    pub ctx: *mut mtk_vcodec_enc_ctx,
}

extern "C" {
    pub fn vpu_enc_init(vpu: *mut venc_vpu_inst) -> c_int;
}
extern "C" {
    pub fn vpu_enc_deinit(vpu: *mut venc_vpu_inst) -> c_int;
}
