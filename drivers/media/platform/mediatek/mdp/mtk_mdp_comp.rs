//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp/mtk_mdp_comp.h
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
// Copyright (c) 2016 MediaTek Inc.
// Author: Ming Hsiu Tsai <minghsiu.tsai@mediatek.com>
//
// enum mtk_mdp_comp_type - the MDP component
// @MTK_MDP_RDMA:	Read DMA
// @MTK_MDP_RSZ:	Riszer
// @MTK_MDP_WDMA:	Write DMA
// @MTK_MDP_WROT:	Write DMA with rotation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_mdp_comp_type {
    MTK_MDP_RDMA,
    MTK_MDP_RSZ,
    MTK_MDP_WDMA,
    MTK_MDP_WROT,
}

//
// struct mtk_mdp_comp - the MDP's function component data
// @node:	list node to track sibing MDP components
// @dev_node:	component device node
// @clk:	clocks required for component
// @type:	component type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mdp_comp {
    pub node: list_head,
    pub dev_node: *mut device_node,
    pub clk: [*mut clk; 2],
    pub type: mtk_mdp_comp_type,
}

extern "C" {
    pub fn mtk_mdp_comp_deinit(dev: *mut device, comp: *mut mtk_mdp_comp);
}
extern "C" {
    pub fn mtk_mdp_comp_clock_on(dev: *mut device, comp: *mut mtk_mdp_comp);
}
extern "C" {
    pub fn mtk_mdp_comp_clock_off(dev: *mut device, comp: *mut mtk_mdp_comp);
}
