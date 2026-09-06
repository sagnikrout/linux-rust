//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt8365/mt8365-afe-clk.h
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
// MediaTek 8365 AFE clock control definitions
//
// Copyright (c) 2024 MediaTek Inc.
// Authors: Jia Zeng <jia.zeng@mediatek.com>
// Alexandre Mergnat <amergnat@baylibre.com>
//
extern "C" {
    pub fn mt8365_afe_init_audio_clk(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8365_afe_disable_clk(afe: *mut mtk_base_afe, clk: *mut clk);
}
extern "C" {
    pub fn mt8365_afe_set_clk_rate(afe: *mut mtk_base_afe, clk: *mut clk, rate: c_uint) -> c_int;
}
extern "C" {
    pub fn mt8365_afe_set_clk_parent(afe: *mut mtk_base_afe, clk: *mut clk, parent: *mut clk) -> c_int;
}
extern "C" {
    pub fn mt8365_afe_enable_top_cg(afe: *mut mtk_base_afe, cg_type: c_uint) -> c_int;
}
extern "C" {
    pub fn mt8365_afe_disable_top_cg(afe: *mut mtk_base_afe, cg_type: c_uint) -> c_int;
}
extern "C" {
    pub fn mt8365_afe_enable_main_clk(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8365_afe_disable_main_clk(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8365_afe_emi_clk_on(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8365_afe_emi_clk_off(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8365_afe_enable_afe_on(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8365_afe_disable_afe_on(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8365_afe_enable_apll_tuner_cfg(afe: *mut mtk_base_afe, apll: c_uint) -> c_int;
}
extern "C" {
    pub fn mt8365_afe_disable_apll_tuner_cfg(afe: *mut mtk_base_afe, apll: c_uint) -> c_int;
}
extern "C" {
    pub fn mt8365_afe_enable_apll_associated_cfg(afe: *mut mtk_base_afe, apll: c_uint) -> c_int;
}
extern "C" {
    pub fn mt8365_afe_disable_apll_associated_cfg(afe: *mut mtk_base_afe, apll: c_uint) -> c_int;
}
