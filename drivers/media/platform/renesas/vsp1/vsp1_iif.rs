//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/renesas/vsp1/vsp1_iif.h
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
// vsp1_iif.h  --  R-Car VSP1 IIF (ISP Interface)
//
// Copyright (C) 2025 Ideas On Board Oy
// Copyright (C) 2025 Renesas Corporation
//

pub const VSPX_IIF_SINK_PAD_IMG: c_int = 0;
pub const VSPX_IIF_SINK_PAD_CONFIG: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_iif {
    pub entity: vsp1_entity,
}

extern "C" {
    pub fn container_of(_arg: subdev, vsp1_iif: struct, _arg: entity.subdev) -> return;
}
