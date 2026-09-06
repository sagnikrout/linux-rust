//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/visl/visl.h
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
// A virtual stateless device for stateless uAPI development purposes.
//
// This tool's objective is to help the development and testing of userspace
// applications that use the V4L2 stateless API to decode media.
//
// A userspace implementation can use visl to run a decoding loop even when no
// hardware is available or when the kernel uAPI for the codec has not been
// upstreamed yet. This can reveal bugs at an early stage.
//
// This driver can also trace the contents of the V4L2 controls submitted to it.
// It can also dump the contents of the vb2 buffers through a debugfs
// interface. This is in many ways similar to the tracing infrastructure
// available for other popular encode/decode APIs out there and can help develop
// a userspace application by using another (working) one as a reference.
//
// Note that no actual decoding of video frames is performed by visl. The V4L2
// test pattern generator is used to write various debug information to the
// capture buffers instead.
//
// Copyright (C) 2022 Collabora, Ltd.
//
// Based on the vim2m driver, that is:
//
// Copyright (c) 2009-2010 Samsung Electronics Co., Ltd.
// Pawel Osciak, <pawel@osciak.com>
// Marek Szyprowski, <m.szyprowski@samsung.com>
//
// Based on the vicodec driver, that is:
//
// Copyright 2018 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//
// Based on the Cedrus VPU driver, that is:
//
// Copyright (C) 2016 Florent Revest <florent.revest@free-electrons.com>
// Copyright (C) 2018 Paul Kocialkowski <paul.kocialkowski@bootlin.com>
// Copyright (C) 2018 Bootlin
//

pub const VISL_M2M_NQUEUES: c_int = 2;
pub const TPG_STR_BUF_SZ: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct visl_ctrls {
    pub ctrls: *const visl_ctrl_desc,
    pub num_ctrls: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visl_coded_format_desc {
    pub pixelformat: u32,
    pub frmsize: v4l2_frmsize_stepwise,
    pub ctrls: *const visl_ctrls,
    pub num_decoded_fmts: c_uint,
    pub decoded_fmts: *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visl_q_data {
    pub sequence: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visl_dev {
    pub v4l2_dev: v4l2_device,
    pub vfd: video_device,

    pub mdev: media_device,

    pub dev_mutex: mutex,
    pub m2m_dev: *mut v4l2_m2m_dev,

    pub debugfs_root: *mut dentry,
    pub bitstream_debugfs: *mut dentry,
    pub bitstream_blobs: list_head,
// Protects the "blob" list
    pub bitstream_lock: mutex,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum visl_codec {
    VISL_CODEC_NONE,
    VISL_CODEC_FWHT,
    VISL_CODEC_MPEG2,
    VISL_CODEC_VP8,
    VISL_CODEC_VP9,
    VISL_CODEC_H264,
    VISL_CODEC_HEVC,
    VISL_CODEC_AV1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visl_blob {
    pub list: list_head,
    pub dentry: *mut dentry,
    pub blob: debugfs_blob_wrapper,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visl_ctx {
    pub fh: v4l2_fh,
    pub dev: *mut visl_dev,
    pub hdl: v4l2_ctrl_handler,
    pub vb_mutex: mutex,
    pub q_data: [visl_q_data; VISL_M2M_NQUEUES],
    pub current_codec: visl_codec,
    pub coded_format_desc: *const visl_coded_format_desc,
    pub coded_fmt: v4l2_format,
    pub decoded_fmt: v4l2_format,
    pub tpg: tpg_data,
    pub capture_streamon_jiffies: u64,
    pub tpg_str_buf: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visl_ctrl_desc {
    pub cfg: v4l2_ctrl_config,
}

extern "C" {
    pub fn container_of(_arg: file_to_v4l2_fh(file), visl_ctx: struct, _arg: fh) -> return;
}
extern "C" {
    pub fn visl_control_num_elems(ctx: *mut visl_ctx, id: u32) -> u32;
}
