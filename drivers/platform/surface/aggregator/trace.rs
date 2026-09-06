//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/surface/aggregator/trace.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Trace points for SSAM/SSH.
//
// Copyright (C) 2020-2022 Maximilian Luz <luzmaximilian@gmail.com>
//

pub const SSAM_PTR_UID_LEN: c_int = 9;

pub const SSAM_SSH_TC_NOT_APPLICABLE: c_int = 0;

//
// ssam_trace_ptr_uid() - Convert the pointer to a non-pointer UID string.
// @ptr: The pointer to convert.
// @uid_str: A buffer of length SSAM_PTR_UID_LEN where the UID will be stored.
//
// Converts the given pointer into a UID string that is safe to be shared
// with userspace and logs, i.e. doesn't give away the real memory location.
//
// ssam_trace_get_packet_seq() - Read the packet's sequence ID.
// @p: The packet.
//
// Return: Returns the packet's sequence ID (SEQ) field if present, or
// %SSAM_SEQ_NOT_APPLICABLE if not (e.g. flush packet).
//
// ssam_trace_get_request_id() - Read the packet's request ID.
// @p: The packet.
//
// Return: Returns the packet's request ID (RQID) field if the packet
// represents a request with command data, or %SSAM_RQID_NOT_APPLICABLE if not
// (e.g. flush request, control packet).
//
extern "C" {
    pub fn get_unaligned_le16(_arg: &p->data.ptr[SSH_MSGOFFSET_COMMAND(rqid)]) -> return;
}
//
// ssam_trace_get_request_tid() - Read the packet's request target ID.
// @p: The packet.
//
// Return: Returns the packet's request target ID (TID) field if the packet
// represents a request with command data, or %SSAM_SSH_TID_NOT_APPLICABLE
// if not (e.g. flush request, control packet).
//
extern "C" {
    pub fn get_unaligned_le16(_arg: &p->data.ptr[SSH_MSGOFFSET_COMMAND(tid)]) -> return;
}
//
// ssam_trace_get_request_sid() - Read the packet's request source ID.
// @p: The packet.
//
// Return: Returns the packet's request source ID (SID) field if the packet
// represents a request with command data, or %SSAM_SSH_TID_NOT_APPLICABLE
// if not (e.g. flush request, control packet).
//
extern "C" {
    pub fn get_unaligned_le16(_arg: &p->data.ptr[SSH_MSGOFFSET_COMMAND(sid)]) -> return;
}
//
// ssam_trace_get_request_tc() - Read the packet's request target category.
// @p: The packet.
//
// Return: Returns the packet's request target category (TC) field if the
// packet represents a request with command data, or %SSAM_SSH_TC_NOT_APPLICABLE
// if not (e.g. flush request, control packet).
//
extern "C" {
    pub fn get_unaligned_le16(_arg: &p->data.ptr[SSH_MSGOFFSET_COMMAND(tc)]) -> return;
}

// Use packet for UID so we can match requests to packets.

// Use packet for UID so we can match requests to packets.

// This part must be outside protection

