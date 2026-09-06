//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/mediatek/clk-mux.h
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
// Copyright (c) 2018 MediaTek Inc.
// Author: Owen Chen <owen.chen@mediatek.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mux {
    pub id: c_int,
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub parent_index: *const u8,
    pub flags: c_uint,
    pub mux_ofs: u32,
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub upd_ofs: u32,
    pub hwv_set_ofs: u32,
    pub hwv_clr_ofs: u32,
    pub hwv_sta_ofs: u32,
    pub fenc_sta_mon_ofs: u32,
    pub mux_shift: u8,
    pub mux_width: u8,
    pub gate_shift: u8,
    pub upd_shift: i8,
    pub fenc_shift: u8,
    pub ops: *const clk_ops,
    pub num_parents: signed char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mux_nb {
    pub nb: notifier_block,
    pub ops: *const clk_ops,
    pub /: *mut *mut u8 bypass_index; / Which parent to temporarily use,
    pub /: *mut *mut u8 original_index; / Set by notifier callback,
}

