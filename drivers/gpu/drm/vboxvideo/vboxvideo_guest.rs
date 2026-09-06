//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vboxvideo/vboxvideo_guest.h
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


// SPDX-License-Identifier: MIT
// Copyright (C) 2006-2016 Oracle Corporation

//
// Structure grouping the context needed for sending graphics acceleration
// information to the host via VBVA.  Each screen has its own VBVA buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbva_buf_ctx {
// Offset of the buffer in the VRAM section for the screen
    pub buffer_offset: u32,
// Length of the buffer in bytes
    pub buffer_length: u32,
// Set if we wrote to the buffer faster than the host could read it
    pub buffer_overflow: bool,
// VBVA record that we are currently preparing for the host, or NULL
    pub record: *mut vbva_record,
//
// Pointer to the VBVA buffer mapped into the current address space.
// Will be NULL if VBVA is not enabled.
//
    pub vbva: *mut vbva_buffer,
}

extern "C" {
    pub fn hgsmi_report_flags_location(ctx: *mut gen_pool, location: u32) -> c_int;
}
extern "C" {
    pub fn hgsmi_send_caps_info(ctx: *mut gen_pool, caps: u32) -> c_int;
}
extern "C" {
    pub fn hgsmi_test_query_conf(ctx: *mut gen_pool) -> c_int;
}
extern "C" {
    pub fn hgsmi_query_conf(ctx: *mut gen_pool, index: u32, value_ret: *mut u32) -> c_int;
}
extern "C" {
    pub fn vbva_buffer_end_update(vbva_ctx: *mut vbva_buf_ctx);
}
