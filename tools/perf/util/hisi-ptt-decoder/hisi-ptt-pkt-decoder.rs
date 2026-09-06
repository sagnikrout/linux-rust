//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/hisi-ptt-decoder/hisi-ptt-pkt-decoder.h
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
// HiSilicon PCIe Trace and Tuning (PTT) support
// Copyright (c) 2022 HiSilicon Technologies Co., Ltd.
//

// Macro flag: #define INCLUDE__HISI_PTT_PKT_DECODER_H__

pub const HISI_PTT_MAX_SPACE_LEN: c_int = 10;
pub const HISI_PTT_FIELD_LENGTH: c_int = 4;
// Header DW0 fields for 4DW format

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hisi_ptt_pkt_type {
    HISI_PTT_4DW_PKT,
    HISI_PTT_8DW_PKT,
    HISI_PTT_PKT_MAX
}

extern "C" {
    pub fn hisi_ptt_pkt_desc(buf: *const c_uchar, pos: c_int, type: hisi_ptt_pkt_type) -> c_int;
}
