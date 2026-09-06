//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_stream.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvr_stream_type {
    PVR_STREAM_TYPE_GEOM = 0,
    PVR_STREAM_TYPE_FRAG,
    PVR_STREAM_TYPE_COMPUTE,
    PVR_STREAM_TYPE_TRANSFER,
    PVR_STREAM_TYPE_STATIC_RENDER_CONTEXT,
    PVR_STREAM_TYPE_STATIC_COMPUTE_CONTEXT,

    PVR_STREAM_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvr_stream_size {
    PVR_STREAM_SIZE_8 = 0,
    PVR_STREAM_SIZE_16,
    PVR_STREAM_SIZE_32,
    PVR_STREAM_SIZE_64,
    PVR_STREAM_SIZE_ARRAY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_stream_def {
    pub offset: u32,
    pub size: pvr_stream_size,
    pub array_size: u32,
    pub feature: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_stream_ext_def {
    pub stream: *const pvr_stream_def,
    pub stream_len: u32,
    pub header_mask: u32,
    pub quirk: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_stream_ext_header {
    pub ext_streams: *const pvr_stream_ext_def,
    pub ext_streams_num: u32,
    pub valid_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_stream_cmd_defs {
    pub type: pvr_stream_type,
    pub main_stream: *const pvr_stream_def,
    pub main_stream_len: u32,
    pub ext_nr_headers: u32,
    pub ext_headers: *const pvr_stream_ext_header,
    pub dest_size: usize,
}
