//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/cxgbi/cxgb4i/cxgb4i.h
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


//
// cxgb4i.h: Chelsio T4 iSCSI driver.
//
// Copyright (c) 2010-2015 Chelsio Communications, Inc.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// Written by: Karen Xie (kxie@chelsio.com)
// Written by: Rakesh Ranjan (rranjan@chelsio.com)
//
pub const CXGB4I_SCSI_HOST_QDEPTH: c_int = 1024;
pub const CXGB4I_MAX_CONN: c_int = 16384;

pub const CXGB4I_MAX_LUN: c_uint = 0x1000;
// for TX: a skb must have a headroom of at least TX_HEADER_LEN bytes

