//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/common/mtk-dsp-sof-common.h
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
// mtk-dsp-sof-common.h  --  MediaTek dsp sof common definition
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Chunxu Li <chunxu.li@mediatek.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_conn_stream {
    pub normal_link: *const c_char,
    pub sof_link: *const c_char,
    pub sof_dma: *const c_char,
    pub stream_dir: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_dai_link {
    pub name: *const c_char,
    pub params): *mut snd_pcm_hw_params,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_sof_priv {
    pub conn_streams: *const sof_conn_stream,
    pub num_streams: c_int,
    pub params): *mut snd_pcm_hw_params,
}

extern "C" {
    pub fn mtk_sof_card_probe(card: *mut snd_soc_card) -> c_int;
}
extern "C" {
    pub fn mtk_sof_card_late_probe(card: *mut snd_soc_card) -> c_int;
}
