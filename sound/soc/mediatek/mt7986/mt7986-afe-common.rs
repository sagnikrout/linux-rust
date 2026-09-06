//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt7986/mt7986-afe-common.h
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


// SPDX-License-Identifier: GPL-2.0
//
// mt7986-afe-common.h  --  MediaTek 7986 audio driver definitions
//
// Copyright (c) 2023 MediaTek Inc.
// Authors: Vic Wu <vic.wu@mediatek.com>
// Maso Huang <maso.huang@mediatek.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7986_afe_private {
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub pm_runtime_bypass_reg_ctl: c_int,
// dai
    pub dai_priv: [*mut c_void; MT7986_DAI_NUM],
}

// dai register
extern "C" {
    pub fn mt7986_dai_etdm_register(afe: *mut mtk_base_afe) -> c_int;
}
