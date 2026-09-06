//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/mdp5/mdp5_pipe.h
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
// Copyright (C) 2016 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//
// TODO: Add SSPP_MAX in mdp5.xml.h

// represents a hw pipe, which is dynamically assigned to a plane
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_hw_pipe {
    pub idx: c_int,
    pub name: *const c_char,
    pub pipe: mdp5_pipe,
    pub reg_offset: u32,
    pub caps: u32,
    pub /: *mut *mut uint32_t flush_mask; / used to commit pipe registers,
// number of smp blocks per plane, ie:
// nblks_y | (nblks_u << 8) | (nblks_v << 16)
//
    pub blkcfg: u32,
}

// global atomic state of assignment between pipes and planes:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_hw_pipe_state {
    pub hwpipe_to_plane: [*mut drm_plane; SSPP_MAX],
}

extern "C" {
    pub fn mdp5_pipe_release(s: *mut drm_atomic_commit, hwpipe: *mut mdp5_hw_pipe) -> c_int;
}
