//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp3/mtk-mdp3-type.h
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
// Copyright (c) 2023 MediaTek Inc.
// Author: Ping-Hsun Wu <ping-hsun.wu@mediatek.com>
//

pub const IMG_MAX_HW_INPUTS: c_int = 3;
pub const IMG_MAX_HW_OUTPUTS: c_int = 4;
pub const IMG_MAX_PLANES: c_int = 3;
pub const IMG_MAX_COMPONENTS: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_crop {
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
    pub left_subpix: u32,
    pub top_subpix: u32,
    pub width_subpix: u32,
    pub height_subpix: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_region {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_offset {
    pub left: i32,
    pub top: i32,
    pub left_subpix: u32,
    pub top_subpix: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_mux {
    pub reg: u32,
    pub value: u32,
    pub subsys_id: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_mmsys_ctrl {
    pub 2]: *mut *mut img_mux sets[IMG_MAX_COMPONENTS,
    pub num_sets: u32,
    pub __packed: },
