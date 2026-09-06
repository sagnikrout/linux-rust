//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/renesas/vsp1/vsp1_uif.h
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
// vsp1_uif.h  --  R-Car VSP1 User Logic Interface
//
// Copyright (C) 2017-2018 Laurent Pinchart
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

pub const UIF_PAD_SINK: c_int = 0;
pub const UIF_PAD_SOURCE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_uif {
    pub entity: vsp1_entity,
    pub m3w_quirk: bool,
}

extern "C" {
    pub fn container_of(_arg: subdev, vsp1_uif: struct, _arg: entity.subdev) -> return;
}
extern "C" {
    pub fn vsp1_uif_get_crc(uif: *mut vsp1_uif) -> u32;
}
