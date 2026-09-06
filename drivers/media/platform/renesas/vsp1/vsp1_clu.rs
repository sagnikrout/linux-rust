//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/renesas/vsp1/vsp1_clu.h
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
// vsp1_clu.h  --  R-Car VSP1 Cubic Look-Up Table
//
// Copyright (C) 2015 Renesas Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

pub const CLU_PAD_SINK: c_int = 0;
pub const CLU_PAD_SOURCE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_clu {
    pub entity: vsp1_entity,
    pub ctrls: v4l2_ctrl_handler,
    pub yuv_mode: bool,
    pub lock: spinlock_t,
    pub mode: c_uint,
    pub clu: *mut vsp1_dl_body,
    pub pool: *mut vsp1_dl_body_pool,
}

extern "C" {
    pub fn container_of(_arg: subdev, vsp1_clu: struct, _arg: entity.subdev) -> return;
}
