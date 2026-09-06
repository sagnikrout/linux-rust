//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/gce/mt8173-gce.h
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
// Copyright (c) 2018 MediaTek Inc.
// Author: Houlong Wei <houlong.wei@mediatek.com>
//
// GCE HW thread priority
pub const CMDQ_THR_PRIO_LOWEST: c_int = 0;
pub const CMDQ_THR_PRIO_HIGHEST: c_int = 1;
// GCE SUBSYS
pub const SUBSYS_1400XXXX: c_int = 1;
pub const SUBSYS_1401XXXX: c_int = 2;
pub const SUBSYS_1402XXXX: c_int = 3;
// GCE HW EVENT
pub const CMDQ_EVENT_DISP_OVL0_SOF: c_int = 11;
pub const CMDQ_EVENT_DISP_OVL1_SOF: c_int = 12;
pub const CMDQ_EVENT_DISP_RDMA0_SOF: c_int = 13;
pub const CMDQ_EVENT_DISP_RDMA1_SOF: c_int = 14;
pub const CMDQ_EVENT_DISP_RDMA2_SOF: c_int = 15;
pub const CMDQ_EVENT_DISP_WDMA0_SOF: c_int = 16;
pub const CMDQ_EVENT_DISP_WDMA1_SOF: c_int = 17;
pub const CMDQ_EVENT_DISP_OVL0_EOF: c_int = 39;
pub const CMDQ_EVENT_DISP_OVL1_EOF: c_int = 40;
pub const CMDQ_EVENT_DISP_RDMA0_EOF: c_int = 41;
pub const CMDQ_EVENT_DISP_RDMA1_EOF: c_int = 42;
pub const CMDQ_EVENT_DISP_RDMA2_EOF: c_int = 43;
pub const CMDQ_EVENT_DISP_WDMA0_EOF: c_int = 44;
pub const CMDQ_EVENT_DISP_WDMA1_EOF: c_int = 45;
pub const CMDQ_EVENT_MUTEX0_STREAM_EOF: c_int = 53;
pub const CMDQ_EVENT_MUTEX1_STREAM_EOF: c_int = 54;
pub const CMDQ_EVENT_MUTEX2_STREAM_EOF: c_int = 55;
pub const CMDQ_EVENT_MUTEX3_STREAM_EOF: c_int = 56;
pub const CMDQ_EVENT_MUTEX4_STREAM_EOF: c_int = 57;
pub const CMDQ_EVENT_DISP_RDMA0_UNDERRUN: c_int = 63;
pub const CMDQ_EVENT_DISP_RDMA1_UNDERRUN: c_int = 64;
pub const CMDQ_EVENT_DISP_RDMA2_UNDERRUN: c_int = 65;
