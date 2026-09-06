//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp3/mtk-mdp3-cmdq.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_cmdq_param {
    pub config: *mut img_config,
    pub param: *mut img_ipi_frameparam,
    pub composes: [*const v4l2_rect; IMG_MAX_HW_OUTPUTS],
    pub data): *mut *mut void (cmdq_cb)(struct cmdq_cb_data,
    pub cb_data: *mut c_void,
    pub mdp_ctx: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_cmdq_cmd {
    pub auto_release_work: work_struct,
    pub pkt: cmdq_pkt,
    pub event: *mut i32,
    pub mdp: *mut mdp_dev,
    pub data: *mut cmdq_cb_data,
    pub data): *mut *mut void (user_cmdq_cb)(struct cmdq_cb_data,
    pub user_cb_data: *mut c_void,
    pub comps: *mut mdp_comp,
    pub mdp_ctx: *mut c_void,
    pub num_comps: u8,
    pub pp_idx: u8,
}

extern "C" {
    pub fn mdp_cmdq_send(mdp: *mut mdp_dev, param: *mut mdp_cmdq_param) -> c_int;
}
