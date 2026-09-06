//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp/mtk_mdp_vpu.h
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
// Copyright (c) 2015-2016 MediaTek Inc.
// Author: Houlong Wei <houlong.wei@mediatek.com>
// Ming Hsiu Tsai <minghsiu.tsai@mediatek.com>
//

//
// struct mtk_mdp_vpu - VPU instance for MDP
// @pdev	: pointer to the VPU platform device
// @inst_addr	: VPU MDP instance address
// @failure	: VPU execution result status
// @vsi		: VPU shared information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mdp_vpu {
    pub pdev: *mut platform_device,
    pub inst_addr: u32,
    pub failure: i32,
    pub vsi: *mut mdp_process_vsi,
}

extern "C" {
    pub fn mtk_mdp_vpu_register(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn mtk_mdp_vpu_init(vpu: *mut mtk_mdp_vpu) -> c_int;
}
extern "C" {
    pub fn mtk_mdp_vpu_deinit(vpu: *mut mtk_mdp_vpu) -> c_int;
}
extern "C" {
    pub fn mtk_mdp_vpu_process(vpu: *mut mtk_mdp_vpu) -> c_int;
}
