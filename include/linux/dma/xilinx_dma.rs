//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dma/xilinx_dma.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Xilinx DMA Engine drivers support header file
//
// Copyright (C) 2010-2014 Xilinx, Inc. All rights reserved.
//

//
// struct xilinx_vdma_config - VDMA Configuration structure
// @frm_dly: Frame delay
// @gen_lock: Whether in gen-lock mode
// @master: Master that it syncs to
// @frm_cnt_en: Enable frame count enable
// @park: Whether wants to park
// @park_frm: Frame to park on
// @coalesc: Interrupt coalescing threshold
// @delay: Delay counter
// @reset: Reset Channel
// @ext_fsync: External Frame Sync source
// @vflip_en:  Vertical Flip enable
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilinx_vdma_config {
    pub frm_dly: c_int,
    pub gen_lock: c_int,
    pub master: c_int,
    pub frm_cnt_en: c_int,
    pub park: c_int,
    pub park_frm: c_int,
    pub coalesc: c_int,
    pub delay: c_int,
    pub reset: c_int,
    pub ext_fsync: c_int,
    pub vflip_en: bool,
}
