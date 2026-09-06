//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt8195/mt8195-afe-clk.h
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
// mt8195-afe-clk.h  --  Mediatek 8195 afe clock ctrl definition
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Bicycle Tsai <bicycle.tsai@mediatek.com>
// Trevor Wu <trevor.wu@mediatek.com>
//
// xtal
// divider
// mux
// clock gate
extern "C" {
    pub fn mt8195_afe_get_mclk_source_clk_id(sel: c_int) -> c_int;
}
extern "C" {
    pub fn mt8195_afe_get_mclk_source_rate(afe: *mut mtk_base_afe, apll: c_int) -> c_int;
}
extern "C" {
    pub fn mt8195_afe_get_default_mclk_source_by_rate(rate: c_int) -> c_int;
}
extern "C" {
    pub fn mt8195_afe_init_clock(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8195_afe_enable_clk(afe: *mut mtk_base_afe, clk: *mut clk) -> c_int;
}
extern "C" {
    pub fn mt8195_afe_disable_clk(afe: *mut mtk_base_afe, clk: *mut clk);
}
extern "C" {
    pub fn mt8195_afe_prepare_clk(afe: *mut mtk_base_afe, clk: *mut clk) -> c_int;
}
extern "C" {
    pub fn mt8195_afe_unprepare_clk(afe: *mut mtk_base_afe, clk: *mut clk);
}
extern "C" {
    pub fn mt8195_afe_enable_clk_atomic(afe: *mut mtk_base_afe, clk: *mut clk) -> c_int;
}
extern "C" {
    pub fn mt8195_afe_disable_clk_atomic(afe: *mut mtk_base_afe, clk: *mut clk);
}
extern "C" {
    pub fn mt8195_afe_enable_main_clock(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8195_afe_disable_main_clock(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8195_afe_enable_reg_rw_clk(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8195_afe_disable_reg_rw_clk(afe: *mut mtk_base_afe) -> c_int;
}
