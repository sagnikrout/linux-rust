//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/common/mtk-soundcard-driver.h
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
// mtk-soundcard-driver.h  --  MediaTek soundcard driver common definition
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Trevor Wu <trevor.wu@mediatek.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_pcm_constraint_type {
    MTK_CONSTRAINT_PLAYBACK,
    MTK_CONSTRAINT_CAPTURE,
    MTK_CONSTRAINT_HDMIDP,
    MTK_CONSTRAINT_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pcm_constraints_data {
    pub channels: *const snd_pcm_hw_constraint_list,
    pub rates: *const snd_pcm_hw_constraint_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_platform_card_data {
    pub card: *mut snd_soc_card,
    pub jacks: *mut snd_soc_jack,
    pub pcm_constraints: *const mtk_pcm_constraints_data,
    pub num_jacks: u8,
    pub num_pcm_constraints: u8,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_soundcard_pdata {
    pub card_name: *const c_char,
    pub card_data: *mut mtk_platform_card_data,
    pub sof_priv: *const mtk_sof_priv,
    pub legacy): *mut *mut *mut int (soc_probe)(struct mtk_soc_card_data card_data, bool,
}

// Common playback/capture card startup ops
// Exported for custom/extended soundcard startup ops
extern "C" {
    pub fn parse_dai_link_info(card: *mut snd_soc_card) -> c_int;
}
extern "C" {
    pub fn clean_card_reference(card: *mut snd_soc_card);
}
extern "C" {
    pub fn mtk_soundcard_common_probe(pdev: *mut platform_device) -> c_int;
}
