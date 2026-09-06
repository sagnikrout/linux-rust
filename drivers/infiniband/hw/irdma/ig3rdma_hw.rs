//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/ig3rdma_hw.h
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


// SPDX-License-Identifier: GPL-2.0 or Linux-OpenIB
// Copyright (c) 2021 - 2024 Intel Corporation
pub const IG3_MAX_APFS: c_int = 1;
pub const IG3_MAX_AVFS: c_int = 0;
pub const IG3_PF_RDMA_REGION_OFFSET: c_uint = 0xBC00000;
pub const IG3_PF_RDMA_REGION_LEN: c_uint = 0x401000;
pub const IG3_VF_RDMA_REGION_OFFSET: c_uint = 0x8C00;
pub const IG3_VF_RDMA_REGION_LEN: c_uint = 0x8400;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ig3rdma_device_caps_const {
    IG3RDMA_MAX_WQ_FRAGMENT_COUNT		= 14,
    IG3RDMA_MAX_SGE_RD			= 14,

    IG3RDMA_MAX_STATS_COUNT			= 128,

    IG3RDMA_MAX_IRD_SIZE			= 64,
    IG3RDMA_MAX_ORD_SIZE			= 64,
    IG3RDMA_MIN_WQ_SIZE			= 16 /* WQEs */,
    IG3RDMA_MAX_INLINE_DATA_SIZE		= 216,
    IG3RDMA_MAX_PF_PUSH_PAGE_COUNT		= 8192,
    IG3RDMA_MAX_VF_PUSH_PAGE_COUNT		= 16,
}
