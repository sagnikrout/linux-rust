//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt8196/mt8196-afe-clk.h
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
// mt8196-afe-clk.h  --  Mediatek MT8196 AFE Clock Control definitions
//
// Copyright (c) 2025 MediaTek Inc.
// Author: Darren Ye <darren.ye@mediatek.com>
//
pub const MT8196_AFE_26M: c_int = 26000000;
pub const MT8196_AUD_ENG1_CLK: c_int = 45158400;
pub const MT8196_AUD_ENG2_CLK: c_int = 49152000;
// APLL

// vlp clk
// pll
// divider
// mux
extern "C" {
    pub fn mt8196_mck_enable(afe: *mut mtk_base_afe, mck_id: c_int, rate: c_int) -> c_int;
}
extern "C" {
    pub fn mt8196_mck_disable(afe: *mut mtk_base_afe, mck_id: c_int) -> c_int;
}
extern "C" {
    pub fn mt8196_get_apll_rate(afe: *mut mtk_base_afe, apll: c_int) -> c_int;
}
extern "C" {
    pub fn mt8196_get_apll_by_rate(afe: *mut mtk_base_afe, rate: c_int) -> c_int;
}
extern "C" {
    pub fn mt8196_get_apll_by_name(afe: *mut mtk_base_afe, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn mt8196_init_clock(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8196_afe_enable_clk(afe: *mut mtk_base_afe, clk: *mut clk) -> c_int;
}
extern "C" {
    pub fn mt8196_afe_disable_clk(afe: *mut mtk_base_afe, clk: *mut clk);
}
extern "C" {
    pub fn mt8196_apll1_enable(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8196_apll1_disable(afe: *mut mtk_base_afe);
}
extern "C" {
    pub fn mt8196_apll2_enable(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8196_apll2_disable(afe: *mut mtk_base_afe);
}
extern "C" {
    pub fn mt8196_afe_enable_main_clock(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8196_afe_disable_main_clock(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8196_afe_enable_reg_rw_clk(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8196_afe_disable_reg_rw_clk(afe: *mut mtk_base_afe) -> c_int;
}
