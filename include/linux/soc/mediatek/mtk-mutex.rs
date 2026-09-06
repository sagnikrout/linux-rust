//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/mediatek/mtk-mutex.h
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
// Copyright (c) 2015 MediaTek Inc.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_mutex_mod_index {
// MDP table index
    MUTEX_MOD_IDX_MDP_RDMA0,
    MUTEX_MOD_IDX_MDP_RSZ0,
    MUTEX_MOD_IDX_MDP_RSZ1,
    MUTEX_MOD_IDX_MDP_TDSHP0,
    MUTEX_MOD_IDX_MDP_WROT0,
    MUTEX_MOD_IDX_MDP_WDMA,
    MUTEX_MOD_IDX_MDP_AAL0,
    MUTEX_MOD_IDX_MDP_CCORR0,
    MUTEX_MOD_IDX_MDP_HDR0,
    MUTEX_MOD_IDX_MDP_COLOR0,
    MUTEX_MOD_IDX_MDP_RDMA1,
    MUTEX_MOD_IDX_MDP_RDMA2,
    MUTEX_MOD_IDX_MDP_RDMA3,
    MUTEX_MOD_IDX_MDP_STITCH0,
    MUTEX_MOD_IDX_MDP_FG0,
    MUTEX_MOD_IDX_MDP_FG1,
    MUTEX_MOD_IDX_MDP_FG2,
    MUTEX_MOD_IDX_MDP_FG3,
    MUTEX_MOD_IDX_MDP_HDR1,
    MUTEX_MOD_IDX_MDP_HDR2,
    MUTEX_MOD_IDX_MDP_HDR3,
    MUTEX_MOD_IDX_MDP_AAL1,
    MUTEX_MOD_IDX_MDP_AAL2,
    MUTEX_MOD_IDX_MDP_AAL3,
    MUTEX_MOD_IDX_MDP_RSZ2,
    MUTEX_MOD_IDX_MDP_RSZ3,
    MUTEX_MOD_IDX_MDP_MERGE2,
    MUTEX_MOD_IDX_MDP_MERGE3,
    MUTEX_MOD_IDX_MDP_TDSHP1,
    MUTEX_MOD_IDX_MDP_TDSHP2,
    MUTEX_MOD_IDX_MDP_TDSHP3,
    MUTEX_MOD_IDX_MDP_COLOR1,
    MUTEX_MOD_IDX_MDP_COLOR2,
    MUTEX_MOD_IDX_MDP_COLOR3,
    MUTEX_MOD_IDX_MDP_OVL0,
    MUTEX_MOD_IDX_MDP_OVL1,
    MUTEX_MOD_IDX_MDP_PAD0,
    MUTEX_MOD_IDX_MDP_PAD1,
    MUTEX_MOD_IDX_MDP_PAD2,
    MUTEX_MOD_IDX_MDP_PAD3,
    MUTEX_MOD_IDX_MDP_TCC0,
    MUTEX_MOD_IDX_MDP_TCC1,
    MUTEX_MOD_IDX_MDP_WROT1,
    MUTEX_MOD_IDX_MDP_WROT2,
    MUTEX_MOD_IDX_MDP_WROT3,

    MUTEX_MOD_IDX_MAX		/* ALWAYS keep at the end */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_mutex_sof_index {
    MUTEX_SOF_IDX_SINGLE_MODE,

    MUTEX_SOF_IDX_MAX		/* ALWAYS keep at the end */
}

extern "C" {
    pub fn mtk_mutex_prepare(mutex: *mut mtk_mutex) -> c_int;
}
extern "C" {
    pub fn mtk_mutex_enable(mutex: *mut mtk_mutex);
}
extern "C" {
    pub fn mtk_mutex_disable(mutex: *mut mtk_mutex);
}
extern "C" {
    pub fn mtk_mutex_unprepare(mutex: *mut mtk_mutex);
}
extern "C" {
    pub fn mtk_mutex_put(mutex: *mut mtk_mutex);
}
extern "C" {
    pub fn mtk_mutex_acquire(mutex: *mut mtk_mutex);
}
extern "C" {
    pub fn mtk_mutex_release(mutex: *mut mtk_mutex);
}
