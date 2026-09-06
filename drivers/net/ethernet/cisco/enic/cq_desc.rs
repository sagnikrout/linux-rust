//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cisco/enic/cq_desc.h
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
// Copyright 2008-2010 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//
// Completion queue descriptor types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cq_desc_types {
    CQ_DESC_TYPE_WQ_ENET = 0,
    CQ_DESC_TYPE_DESC_COPY = 1,
    CQ_DESC_TYPE_WQ_EXCH = 2,
    CQ_DESC_TYPE_RQ_ENET = 3,
    CQ_DESC_TYPE_RQ_FCP = 4,
}

// Completion queue descriptor: 16B
//
// All completion queues have this basic layout.  The
// type_specfic area is unique for each completion
// queue type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cq_desc {
    pub completed_index: __le16,
    pub q_number: __le16,
    pub type_specfic: [u8; 11],
    pub type_color: u8,
}

pub const CQ_DESC_TYPE_BITS: c_int = 4;

pub const CQ_DESC_COLOR_MASK: c_int = 1;
pub const CQ_DESC_COLOR_SHIFT: c_int = 7;
pub const CQ_DESC_Q_NUM_BITS: c_int = 10;

pub const CQ_DESC_COMP_NDX_BITS: c_int = 12;

