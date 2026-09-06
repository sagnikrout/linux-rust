//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dce/dmub_replay.h
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
//
// Copyright 2024 Advanced Micro Devices, Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_replay {
    pub ctx: *mut dc_context,
    pub funcs: *const dmub_replay_funcs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_replay_funcs {
    pub panel_inst): u8,
    pub panel_inst): u8,
    pub panel_inst): *mut *mut replay_context replay_context, uint8_t,
    pub panel_inst): u8,
    pub cmd_element): *mut replay_FW_Message_type msg, union dmub_replay_cmd_set,
    pub frame_skip_number): uint8_t panel_inst, uint16_t,
    pub mode): *const *const uint8_t panel_inst, uint32_t residency, bool is_start, enum pr_residency_mode,
    pub frame_skip_number): u16,
}

extern "C" {
    pub fn dmub_replay_destroy(dmub: *mut dmub_replay);
}
