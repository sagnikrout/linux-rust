//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp/mtk_mdp_ipi.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2015-2016 MediaTek Inc.
// Author: Houlong Wei <houlong.wei@mediatek.com>
// Ming Hsiu Tsai <minghsiu.tsai@mediatek.com>
//
pub const MTK_MDP_MAX_NUM_PLANE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mdp_ipi_msgid {
    AP_MDP_INIT		= 0xd000,
    AP_MDP_DEINIT		= 0xd001,
    AP_MDP_PROCESS		= 0xd002,

    VPU_MDP_INIT_ACK	= 0xe000,
    VPU_MDP_DEINIT_ACK	= 0xe001,
    VPU_MDP_PROCESS_ACK	= 0xe002
}

//
// struct mdp_ipi_init - for AP_MDP_INIT
// @msg_id   : AP_MDP_INIT
// @ipi_id   : IPI_MDP
// @ap_inst  : AP mtk_mdp_vpu address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_ipi_init {
    pub msg_id: u32,
    pub ipi_id: u32,
    pub ap_inst: u64,
}

//
// struct mdp_ipi_comm - for AP_MDP_PROCESS, AP_MDP_DEINIT
// @msg_id        : AP_MDP_PROCESS, AP_MDP_DEINIT
// @ipi_id        : IPI_MDP
// @ap_inst       : AP mtk_mdp_vpu address
// @vpu_inst_addr : VPU MDP instance address
// @padding       : Alignment padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_ipi_comm {
    pub msg_id: u32,
    pub ipi_id: u32,
    pub ap_inst: u64,
    pub vpu_inst_addr: u32,
    pub padding: u32,
}

//
// struct mdp_ipi_comm_ack - for VPU_MDP_DEINIT_ACK, VPU_MDP_PROCESS_ACK
// @msg_id        : VPU_MDP_DEINIT_ACK, VPU_MDP_PROCESS_ACK
// @ipi_id        : IPI_MDP
// @ap_inst       : AP mtk_mdp_vpu address
// @vpu_inst_addr : VPU MDP instance address
// @status        : VPU exeuction result
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_ipi_comm_ack {
    pub msg_id: u32,
    pub ipi_id: u32,
    pub ap_inst: u64,
    pub vpu_inst_addr: u32,
    pub status: i32,
}

//
// struct mdp_config - configured for source/destination image
// @x        : left
// @y        : top
// @w        : width
// @h        : height
// @w_stride : bytes in horizontal
// @h_stride : bytes in vertical
// @crop_x   : cropped left
// @crop_y   : cropped top
// @crop_w   : cropped width
// @crop_h   : cropped height
// @format   : color format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_config {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub w_stride: i32,
    pub h_stride: i32,
    pub crop_x: i32,
    pub crop_y: i32,
    pub crop_w: i32,
    pub crop_h: i32,
    pub format: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_buffer {
    pub addr_mva: [u64; MTK_MDP_MAX_NUM_PLANE],
    pub plane_size: [i32; MTK_MDP_MAX_NUM_PLANE],
    pub plane_num: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_config_misc {
    pub /: *mut *mut int32_t orientation; / 0, 90, 180, 270,
    pub /: *mut *mut int32_t hflip; / 1 will enable the flip,
    pub /: *mut *mut int32_t vflip; / 1 will enable the flip,
    pub /: *mut *mut int32_t alpha; / global alpha,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_process_vsi {
    pub src_config: mdp_config,
    pub src_buffer: mdp_buffer,
    pub dst_config: mdp_config,
    pub dst_buffer: mdp_buffer,
    pub misc: mdp_config_misc,
}

