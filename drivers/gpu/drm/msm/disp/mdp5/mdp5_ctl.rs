//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/mdp5/mdp5_ctl.h
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
// Copyright (c) 2014 The Linux Foundation. All rights reserved.
//

//
// CTL Manager prototypes:
// mdp5_ctlm_init() returns a ctlm (CTL Manager) handler,
// which is then used to call the other mdp5_ctlm_*(ctlm, ...) functions.
//
extern "C" {
    pub fn mdp5_ctlm_hw_reset(ctlm: *mut mdp5_ctl_manager);
}
//
// CTL prototypes:
// mdp5_ctl_request(ctlm, ...) returns a ctl (CTL resource) handler,
// which is then used to call the other mdp5_ctl_*(ctl, ...) functions.
//
extern "C" {
    pub fn mdp5_ctl_get_ctl_id(ctl: *mut mdp5_ctl) -> c_int;
}
extern "C" {
    pub fn mdp5_ctl_set_pipeline(ctl: *mut mdp5_ctl, p: *mut mdp5_pipeline) -> c_int;
}
pub const MAX_PIPE_STAGE: c_int = 2;
//
// mdp5_ctl_blend() - Blend multiple layers on a Layer Mixer (LM)
//
// @stage: array to contain the pipe num for each stage
// @stage_cnt: valid stage number in stage array
// @ctl_blend_op_flags: blender operation mode flags
//
// Note:
// CTL registers need to be flushed after calling this function
// (call mdp5_ctl_commit() with mdp_ctl_flush_mask_ctl() mask)
//

//
// mdp_ctl_flush_mask...() - Register FLUSH masks
//
// These masks are used to specify which block(s) need to be flushed
// through @flush_mask parameter in mdp5_ctl_commit(.., flush_mask).
//
extern "C" {
    pub fn mdp_ctl_flush_mask_lm(lm: c_int) -> u32;
}
extern "C" {
    pub fn mdp_ctl_flush_mask_pipe(pipe: mdp5_pipe) -> u32;
}
extern "C" {
    pub fn mdp_ctl_flush_mask_cursor(cursor_id: c_int) -> u32;
}
extern "C" {
    pub fn mdp_ctl_flush_mask_encoder(intf: *mut mdp5_interface) -> u32;
}
// @flush_mask: see CTL flush masks definitions below
extern "C" {
    pub fn mdp5_ctl_get_commit_status(ctl: *mut mdp5_ctl) -> u32;
}
