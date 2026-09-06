//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/mediatek/clk-fhctl.h
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
// Author: Edward-JW Yang <edward-jw.yang@mediatek.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fhctl_variant {
    FHCTL_PLLFH_V1,
    FHCTL_PLLFH_V2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fhctl_offset {
    pub offset_hp_en: u32,
    pub offset_clk_con: u32,
    pub offset_rst_con: u32,
    pub offset_slope0: u32,
    pub offset_slope1: u32,
    pub offset_cfg: u32,
    pub offset_updnlmt: u32,
    pub offset_dds: u32,
    pub offset_dvfs: u32,
    pub offset_mon: u32,
}

extern "C" {
    pub fn fhctl_hw_init(fh: *mut mtk_fh);
}
