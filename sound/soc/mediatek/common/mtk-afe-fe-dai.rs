//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/common/mtk-afe-fe-dai.h
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
// mtk-afe-fe-dais.h  --  Mediatek afe fe dai operator definition
//
// Copyright (c) 2016 MediaTek Inc.
// Author: Garlic Tseng <garlic.tseng@mediatek.com>
//
extern "C" {
    pub fn mtk_dynamic_irq_acquire(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mtk_dynamic_irq_release(afe: *mut mtk_base_afe, irq_id: c_int) -> c_int;
}
extern "C" {
    pub fn mtk_afe_suspend(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn mtk_afe_resume(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn mtk_memif_set_enable(afe: *mut mtk_base_afe, id: c_int) -> c_int;
}
extern "C" {
    pub fn mtk_memif_set_disable(afe: *mut mtk_base_afe, id: c_int) -> c_int;
}
