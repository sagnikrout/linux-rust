//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/cxgbi/cxgb3i/cxgb3i.h
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
// cxgb3i.h: Chelsio S3xx iSCSI driver.
//
// Copyright (c) 2008-2015 Chelsio Communications, Inc.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// Written by: Karen Xie (kxie@chelsio.com)
//
pub const CXGB3I_SCSI_HOST_QDEPTH: c_int = 1024;
pub const CXGB3I_MAX_LUN: c_int = 512;

// for TX: a skb must have a headroom of at least TX_HEADER_LEN bytes

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_iscsi_hdr_norss {
    pub ot: opcode_tid,
    pub pdu_len_ddp: u16,
    pub len: u16,
    pub seq: u32,
    pub urg: u16,
    pub rsvd: u8,
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpl_rx_data_ddp_norss {
    pub ot: opcode_tid,
    pub urg: u16,
    pub len: u16,
    pub seq: u32,
    pub nxt_seq: u32,
    pub ulp_crc: u32,
    pub ddp_status: u32,
}
