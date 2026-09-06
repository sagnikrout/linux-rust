//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/fsl/dpaa2-global.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Copyright 2014-2016 Freescale Semiconductor Inc.
// Copyright 2016 NXP
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_dq {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct common {
    pub verb: u8,
    pub reserved: [u8; 63],
    pub common: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dq {
    pub verb: u8,
    pub stat: u8,
    pub seqnum: __le16,
    pub oprid: __le16,
    pub reserved: u8,
    pub tok: u8,
    pub fqid: __le32,
    pub reserved2: u32,
    pub fq_byte_cnt: __le32,
    pub fq_frm_cnt: __le32,
    pub fqd_ctx: __le64,
    pub fd: [u8; 32],
    pub dq: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scn {
    pub verb: u8,
    pub stat: u8,
    pub state: u8,
    pub reserved: u8,
    pub rid_tok: __le32,
    pub ctx: __le64,
    pub scn: },
}

// Parsing frame dequeue results
// FQ empty
pub const DPAA2_DQ_STAT_FQEMPTY: c_uint = 0x80;
// FQ held active
pub const DPAA2_DQ_STAT_HELDACTIVE: c_uint = 0x40;
// FQ force eligible
pub const DPAA2_DQ_STAT_FORCEELIGIBLE: c_uint = 0x20;
// valid frame
pub const DPAA2_DQ_STAT_VALIDFRAME: c_uint = 0x10;
// FQ ODP enable
pub const DPAA2_DQ_STAT_ODPVALID: c_uint = 0x04;
// volatile dequeue
pub const DPAA2_DQ_STAT_VOLATILE: c_uint = 0x02;
// volatile dequeue command is expired
pub const DPAA2_DQ_STAT_EXPIRED: c_uint = 0x01;
pub const DQ_FQID_MASK: c_uint = 0x00FFFFFF;
pub const DQ_FRAME_COUNT_MASK: c_uint = 0x00FFFFFF;
//
// dpaa2_dq_flags() - Get the stat field of dequeue response
// @dq: the dequeue result.
//
// dpaa2_dq_is_pull() - Check whether the dq response is from a pull
// command.
// @dq: the dequeue result
//
// Return 1 for volatile(pull) dequeue, 0 for static dequeue.
//
// dpaa2_dq_is_pull_complete() - Check whether the pull command is completed.
// @dq: the dequeue result
//
// Return boolean.
//
// dpaa2_dq_seqnum() - Get the seqnum field in dequeue response
// @dq: the dequeue result
//
// seqnum is valid only if VALIDFRAME flag is TRUE
//
// Return seqnum.
//
extern "C" {
    pub fn le16_to_cpu(_arg: dq->dq.seqnum) -> return;
}
//
// dpaa2_dq_odpid() - Get the odpid field in dequeue response
// @dq: the dequeue result
//
// odpid is valid only if ODPVALID flag is TRUE.
//
// Return odpid.
//
extern "C" {
    pub fn le16_to_cpu(_arg: dq->dq.oprid) -> return;
}
//
// dpaa2_dq_fqid() - Get the fqid in dequeue response
// @dq: the dequeue result
//
// Return fqid.
//
// dpaa2_dq_byte_count() - Get the byte count in dequeue response
// @dq: the dequeue result
//
// Return the byte count remaining in the FQ.
//
extern "C" {
    pub fn le32_to_cpu(_arg: dq->dq.fq_byte_cnt) -> return;
}
//
// dpaa2_dq_frame_count() - Get the frame count in dequeue response
// @dq: the dequeue result
//
// Return the frame count remaining in the FQ.
//
// dpaa2_dq_fd_ctx() - Get the frame queue context in dequeue response
// @dq: the dequeue result
//
// Return the frame queue context.
//
extern "C" {
    pub fn le64_to_cpu(_arg: dq->dq.fqd_ctx) -> return;
}
//
// dpaa2_dq_fd() - Get the frame descriptor in dequeue response
// @dq: the dequeue result
//
// Return the frame descriptor.
//

pub const DPAA2_CSCN_ALIGN: c_int = 16;

//
// dpaa2_cscn_state_congested() - Check congestion state
// @cscn: congestion SCN (delivered to WQ or memory)
//
