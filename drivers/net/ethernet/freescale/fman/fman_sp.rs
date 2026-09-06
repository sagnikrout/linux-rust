//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/fman/fman_sp.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0-or-later
//
// Copyright 2008 - 2015 Freescale Semiconductor Inc.
//

// defaults
pub const DFLT_FM_SP_BUFFER_PREFIX_CONTEXT_DATA_ALIGN: c_int = 64;
// Registers bit fields
pub const FMAN_SP_EXT_BUF_POOL_EN_COUNTER: c_uint = 0x40000000;
pub const FMAN_SP_EXT_BUF_POOL_VALID: c_uint = 0x80000000;
pub const FMAN_SP_EXT_BUF_POOL_BACKUP: c_uint = 0x20000000;
pub const FMAN_SP_DMA_ATTR_WRITE_OPTIMIZE: c_uint = 0x00100000;
pub const FMAN_SP_SG_DISABLE: c_uint = 0x80000000;
// shifts
pub const FMAN_SP_EXT_BUF_MARG_START_SHIFT: c_int = 16;
pub const FMAN_SP_DMA_ATTR_SWP_SHIFT: c_int = 30;
pub const FMAN_SP_IC_TO_EXT_SHIFT: c_int = 16;
pub const FMAN_SP_IC_FROM_INT_SHIFT: c_int = 8;
// structure for defining internal context copying
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fman_sp_int_context_data_copy {
// < Offset in External buffer to which internal
// context is copied to (Rx) or taken from (Tx, Op).
//
    pub ext_buf_offset: u16,
// Offset within internal context to copy from
// (Rx) or to copy to (Tx, Op).
//
    pub int_context_offset: u8,
// Internal offset size to be copied
    pub size: u16,
}

// struct for defining external buffer margins
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fman_sp_buf_margins {
// Number of bytes to be left at the beginning
// of the external buffer (must be divisible by 16)
//
    pub start_margins: u16,
// number of bytes to be left at the end
// of the external buffer(must be divisible by 16)
//
    pub end_margins: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fman_sp_buffer_offsets {
    pub data_offset: u32,
    pub prs_result_offset: u32,
    pub time_stamp_offset: u32,
    pub hash_result_offset: u32,
}

// int_context_data_copy,
// buffer_prefix_content,
// buffer_offsets,
// fm_ext_pools,
