//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bnx2i/57xx_iscsi_constants.h
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


// 57xx_iscsi_constants.h: QLogic NetXtreme II iSCSI HSI
//
// Copyright (c) 2006 - 2013 Broadcom Corporation
// Copyright (c) 2014, QLogic Corporation
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// Written by: Anil Veerabhadrappa (anilgv@broadcom.com)
// Previously Maintained by: Eddie Wai (eddie.wai@broadcom.com)
// Maintained by: QLogic-Storage-Upstream@qlogic.com
//
// This file defines HSI constants for the iSCSI flows
//
// iSCSI request op codes

// iSCSI response/messages op codes

// iSCSI task types

// initial CQ sequence numbers

// KWQ (kernel work queue) layer codes

// KWQ (kernel work queue) request op codes

// KCQ (kernel completion queue) response op codes

// KCQ (kernel completion queue) completion status

// Response

// Data-In

// R2T

// TMF

// IP/TCP processing errors:

// iSCSI licensing errors
// general iSCSI license not installed

// additional LOM specific iSCSI license not installed

// SQ/RQ/CQ DB structure sizes

pub const ISCSI_SQN_TO_NOTIFY_NOT_VALID: c_uint = 0xFFFF;
// Page size codes (for flags field in connection offload request)

// Iscsi PDU related defines

pub const B577XX_ISCSI_CONNECTION_TYPE: c_int = 3;
