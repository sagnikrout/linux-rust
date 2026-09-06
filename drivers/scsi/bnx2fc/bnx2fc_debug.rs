//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bnx2fc/bnx2fc_debug.h
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


// bnx2fc_debug.h: QLogic Linux FCoE offload driver.
// Handles operations such as session offload/upload etc, and manages
// session resources such as connection id and qp resources.
//
// Copyright (c) 2008-2013 Broadcom Corporation
// Copyright (c) 2014-2016 QLogic Corporation
// Copyright (c) 2016-2017 Cavium Inc.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// Log level bit mask
pub const LOG_IO: c_uint = 0x01	/* scsi cmd error, cleanup */;
pub const LOG_TGT: c_uint = 0x02	/* Session setup, cleanup, etc' */;
pub const LOG_HBA: c_uint = 0x04	/* lport events, link, mtu, etc' */;
pub const LOG_ELS: c_uint = 0x08	/* ELS logs */;
pub const LOG_MISC: c_uint = 0x10	/* fcoe L2 frame related logs*/;
pub const LOG_ALL: c_uint = 0xff	/* LOG all messages */;

extern "C" {
    pub fn BNX2FC_IO_DBG(io_req: *const bnx2fc_cmd, fmt: *const c_char, ...);
}
extern "C" {
    pub fn BNX2FC_TGT_DBG(tgt: *const bnx2fc_rport, fmt: *const c_char, ...);
}
extern "C" {
    pub fn BNX2FC_HBA_DBG(lport: *const fc_lport, fmt: *const c_char, ...);
}
