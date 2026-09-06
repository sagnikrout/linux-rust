//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/common/mtk_vcodec_cmn_drv.h
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
// Copyright (c) 2023 MediaTek Inc.
// Author: Yunfei Dong <yunfei.dong@mediatek.com>
//

pub const MTK_VCODEC_MAX_PLANES: c_int = 3;
pub const WAIT_INTR_TIMEOUT_MS: c_int = 1000;
//
// enum mtk_q_type - Type of queue
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_q_type {
    MTK_Q_DATA_SRC = 0,
    MTK_Q_DATA_DST = 1,
}

//
// enum mtk_hw_reg_idx - MTK hw register base index
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_hw_reg_idx {
    VDEC_SYS,
    VDEC_MISC,
    VDEC_LD,
    VDEC_TOP,
    VDEC_CM,
    VDEC_AD,
    VDEC_AV,
    VDEC_PP,
    VDEC_HWD,
    VDEC_HWQ,
    VDEC_HWB,
    VDEC_HWG,
    NUM_MAX_VDEC_REG_BASE,
// h264 encoder
    VENC_SYS = NUM_MAX_VDEC_REG_BASE,
// vp8 encoder
    VENC_LT_SYS,
    NUM_MAX_VCODEC_REG_BASE
}

//
// struct mtk_vcodec_clk_info - Structure used to store clock name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_vcodec_clk_info {
    pub clk_name: *const c_char,
    pub vcodec_clk: *mut clk,
}

//
// struct mtk_vcodec_clk - Structure used to store vcodec clock information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_vcodec_clk {
    pub clk_info: *mut mtk_vcodec_clk_info,
    pub clk_num: c_int,
}

//
// struct mtk_vcodec_pm - Power management data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_vcodec_pm {
    pub vdec_clk: mtk_vcodec_clk,
    pub venc_clk: mtk_vcodec_clk,
    pub dev: *mut device,
}

//
// enum mtk_vdec_hw_id - Hardware index used to separate
// different hardware
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_vdec_hw_id {
    MTK_VDEC_CORE,
    MTK_VDEC_LAT0,
    MTK_VDEC_LAT1,
    MTK_VDEC_LAT_SOC,
    MTK_VDEC_HW_MAX,
}

//
// enum mtk_instance_state - The state of an MTK Vcodec instance.
// @MTK_STATE_FREE: default state when instance is created
// @MTK_STATE_INIT: vcodec instance is initialized
// @MTK_STATE_HEADER: vdec had sps/pps header parsed or venc
// had sps/pps header encoded
// @MTK_STATE_FLUSH: vdec is flushing. Only used by decoder
// @MTK_STATE_ABORT: vcodec should be aborted
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_instance_state {
    MTK_STATE_FREE = 0,
    MTK_STATE_INIT = 1,
    MTK_STATE_HEADER = 2,
    MTK_STATE_FLUSH = 3,
    MTK_STATE_ABORT = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_fmt_type {
    MTK_FMT_DEC = 0,
    MTK_FMT_ENC = 1,
    MTK_FMT_FRAME = 2,
}

//
// struct mtk_video_fmt - Structure used to store information about pixelformats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_video_fmt {
    pub fourcc: u32,
    pub type: mtk_fmt_type,
    pub num_planes: u32,
    pub flags: u32,
    pub frmsize: v4l2_frmsize_stepwise,
}

//
// struct mtk_q_data - Structure used to store information about queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_q_data {
    pub visible_width: c_uint,
    pub visible_height: c_uint,
    pub coded_width: c_uint,
    pub coded_height: c_uint,
    pub field: v4l2_field,
    pub bytesperline: [c_uint; MTK_VCODEC_MAX_PLANES],
    pub sizeimage: [c_uint; MTK_VCODEC_MAX_PLANES],
    pub fmt: *const mtk_video_fmt,
}

//
// enum mtk_instance_type - The type of an MTK Vcodec instance.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_instance_type {
    MTK_INST_DECODER		= 0,
    MTK_INST_ENCODER		= 1,
}
