//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/renesas/vsp1/vsp1_rwpf.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// vsp1_rwpf.h  --  R-Car VSP1 Read and Write Pixel Formatters
//
// Copyright (C) 2013-2014 Renesas Electronics Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

pub const RWPF_PAD_SINK: c_int = 0;
pub const RWPF_PAD_SOURCE: c_int = 1;
pub const RWPF_MIN_WIDTH: c_int = 1;
pub const RWPF_MIN_HEIGHT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_rwpf_memory {
    pub addr: [dma_addr_t; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_rwpf {
    pub entity: vsp1_entity,
    pub ctrls: v4l2_ctrl_handler,
    pub video: *mut vsp1_video,
    pub format: v4l2_pix_format_mplane,
    pub fmtinfo: *const vsp1_format_info,
    pub brx_input: c_uint,
    pub alpha: c_uint,
    pub mult_alpha: u32,
    pub outfmt: u32,
    pub lock: spinlock_t,
    pub vflip: *mut v4l2_ctrl,
    pub hflip: *mut v4l2_ctrl,
    pub rotate: *mut v4l2_ctrl,
    pub ctrls: },
    pub pending: c_uint,
    pub active: c_uint,
    pub rotate: bool,
    pub flip: },
    pub mem: vsp1_rwpf_memory,
    pub writeback: bool,
    pub dlm: *mut vsp1_dl_manager,
}

extern "C" {
    pub fn container_of(_arg: subdev, vsp1_rwpf: struct, _arg: entity.subdev) -> return;
}
extern "C" {
    pub fn container_of(_arg: entity, vsp1_rwpf: struct, _arg: entity) -> return;
}
extern "C" {
    pub fn vsp1_wpf_stop(wpf: *mut vsp1_rwpf);
}
extern "C" {
    pub fn vsp1_rwpf_init_ctrls(rwpf: *mut vsp1_rwpf, ncontrols: c_uint) -> c_int;
}
