//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp3/mtk-mdp3-m2m.h
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
// Copyright (c) 2022 MediaTek Inc.
// Author: Ping-Hsun Wu <ping-hsun.wu@mediatek.com>
//

pub const MDP_MAX_CTRLS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_m2m_ctrls {
    pub hflip: *mut v4l2_ctrl,
    pub vflip: *mut v4l2_ctrl,
    pub rotate: *mut v4l2_ctrl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_m2m_ctx {
    pub id: u32,
    pub mdp_dev: *mut mdp_dev,
    pub fh: v4l2_fh,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub ctrls: mdp_m2m_ctrls,
    pub m2m_ctx: *mut v4l2_m2m_ctx,
    pub frame_count: [u32; MDP_M2M_MAX],
    pub curr_param: mdp_frameparam,
// synchronization protect for mdp m2m context
    pub ctx_lock: mutex,
}

extern "C" {
    pub fn mdp_m2m_device_register(mdp: *mut mdp_dev) -> c_int;
}
extern "C" {
    pub fn mdp_m2m_device_unregister(mdp: *mut mdp_dev);
}
extern "C" {
    pub fn mdp_m2m_job_finish(ctx: *mut mdp_m2m_ctx);
}
