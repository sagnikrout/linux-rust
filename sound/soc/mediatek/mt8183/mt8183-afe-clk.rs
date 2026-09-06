//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt8183/mt8183-afe-clk.h
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
// mt8183-afe-clk.h  --  Mediatek 8183 afe clock ctrl definition
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>
//
// APLL

extern "C" {
    pub fn mt8183_init_clock(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8183_afe_enable_clock(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8183_afe_disable_clock(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8183_apll1_enable(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8183_apll1_disable(afe: *mut mtk_base_afe);
}
extern "C" {
    pub fn mt8183_apll2_enable(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8183_apll2_disable(afe: *mut mtk_base_afe);
}
extern "C" {
    pub fn mt8183_get_apll_rate(afe: *mut mtk_base_afe, apll: c_int) -> c_int;
}
extern "C" {
    pub fn mt8183_get_apll_by_rate(afe: *mut mtk_base_afe, rate: c_int) -> c_int;
}
extern "C" {
    pub fn mt8183_get_apll_by_name(afe: *mut mtk_base_afe, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn mt8183_mck_enable(afe: *mut mtk_base_afe, mck_id: c_int, rate: c_int) -> c_int;
}
extern "C" {
    pub fn mt8183_mck_disable(afe: *mut mtk_base_afe, mck_id: c_int);
}
