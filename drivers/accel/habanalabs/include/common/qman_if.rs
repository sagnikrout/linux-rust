//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/common/qman_if.h
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
// Copyright 2016-2018 HabanaLabs, Ltd.
// All Rights Reserved.
//

//
// PRIMARY QUEUE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_bd {
    pub ptr: __le64,
    pub len: __le32,
    pub ctl: __le32,
}

//
// S/W CTL FIELDS.
//
// BD_CTL_REPEAT_VALID tells the CP whether the repeat field in the BD CTL is
// valid. 1 means the repeat field is valid, 0 means not-valid,
// i.e. repeat == 1
//
pub const BD_CTL_REPEAT_VALID_SHIFT: c_int = 24;
pub const BD_CTL_REPEAT_VALID_MASK: c_uint = 0x01000000;
pub const BD_CTL_SHADOW_INDEX_SHIFT: c_int = 0;
pub const BD_CTL_SHADOW_INDEX_MASK: c_uint = 0x00000FFF;
//
// H/W CTL FIELDS
//
pub const BD_CTL_COMP_OFFSET_SHIFT: c_int = 16;
pub const BD_CTL_COMP_OFFSET_MASK: c_uint = 0x0FFF0000;
pub const BD_CTL_COMP_DATA_SHIFT: c_int = 0;
pub const BD_CTL_COMP_DATA_MASK: c_uint = 0x0000FFFF;
//
// COMPLETION QUEUE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_cq_entry {
    pub data: __le32,
}

pub const CQ_ENTRY_READY_SHIFT: c_int = 31;
pub const CQ_ENTRY_READY_MASK: c_uint = 0x80000000;
pub const CQ_ENTRY_SHADOW_INDEX_VALID_SHIFT: c_int = 30;
pub const CQ_ENTRY_SHADOW_INDEX_VALID_MASK: c_uint = 0x40000000;

