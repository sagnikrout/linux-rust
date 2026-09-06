//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qed/roce_common.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//
// ROCE FW CONSTANTS
//

// Affiliated asynchronous events / errors enumeration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum roce_async_events_type {
    ROCE_ASYNC_EVENT_NONE = 0,
    ROCE_ASYNC_EVENT_COMM_EST = 1,
    ROCE_ASYNC_EVENT_SQ_DRAINED,
    ROCE_ASYNC_EVENT_SRQ_LIMIT,
    ROCE_ASYNC_EVENT_LAST_WQE_REACHED,
    ROCE_ASYNC_EVENT_CQ_ERR,
    ROCE_ASYNC_EVENT_LOCAL_INVALID_REQUEST_ERR,
    ROCE_ASYNC_EVENT_LOCAL_CATASTROPHIC_ERR,
    ROCE_ASYNC_EVENT_LOCAL_ACCESS_ERR,
    ROCE_ASYNC_EVENT_QP_CATASTROPHIC_ERR,
    ROCE_ASYNC_EVENT_CQ_OVERFLOW_ERR,
    ROCE_ASYNC_EVENT_SRQ_EMPTY,
    ROCE_ASYNC_EVENT_DESTROY_QP_DONE,
    ROCE_ASYNC_EVENT_XRC_DOMAIN_ERR,
    ROCE_ASYNC_EVENT_INVALID_XRCETH_ERR,
    ROCE_ASYNC_EVENT_XRC_SRQ_CATASTROPHIC_ERR,
    MAX_ROCE_ASYNC_EVENTS_TYPE
}
