//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/trace/zcrypt.h
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
// Tracepoint definitions for the s390 zcrypt device driver
//
// Copyright IBM Corp. 2016,2025
// Author(s): Harald Freudenberger <freude@de.ibm.com>
//
// Currently there are two tracepoint events defined here.
// An s390_zcrypt_req request event occurs as soon as the request is
// recognized by the zcrypt ioctl function. This event may act as some kind
// of request-processing-starts-now indication.
// As late as possible within the zcrypt ioctl function there occurs the
// s390_zcrypt_rep event which may act as the point in time where the
// request has been processed by the kernel and the result is about to be
// transferred back to userspace.
// The glue which binds together request and reply event is the ptr
// parameter, which is the local buffer address where the request from
// userspace has been stored by the ioctl function.
//
// The main purpose of this zcrypt tracepoint api is to get some data for
// performance measurements together with information about on which card
// and queue the request has been processed. It is not an ffdc interface as
// there is already code in the zcrypt device driver to serve the s390
// debug feature interface.
//

pub const TP_ICARSAMODEXPO: c_uint = 0x0001;
pub const TP_ICARSACRT: c_uint = 0x0002;
pub const TB_ZSECSENDCPRB: c_uint = 0x0003;
pub const TP_ZSENDEP11CPRB: c_uint = 0x0004;
pub const TP_HWRNGCPRB: c_uint = 0x0005;

//
// trace_s390_zcrypt_req - zcrypt request tracepoint function
// @ptr:  Address of the local buffer where the request from userspace
// is stored. Can be used as a unique id to relate together
// request and reply.
// @type: One of the TP_ defines above.
//
// Called when a request from userspace is recognised within the ioctl
// function of the zcrypt device driver and may act as an entry
// timestamp.
//
// trace_s390_zcrypt_rep - zcrypt reply tracepoint function
// @ptr:   Address of the local buffer where the request from userspace
// is stored. Can be used as a unique id to match together
// request and reply.
// @fc:    Function code.
// @rc:    The bare returncode as returned by the device driver ioctl
// function.
// @card:  The adapter nr where this request was actually processed.
// @dom:   Domain id of the device where this request was processed.
// @psmid: Unique id identifying this request/reply.
//
// Called upon recognising the reply from the crypto adapter. This
// message may act as the exit timestamp for the request but also
// carries some info about on which adapter the request was processed
// and the returncode from the device driver.
//

// This part must be outside protection

