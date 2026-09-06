//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/fc/fc_fc2.h
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
// Copyright(c) 2007 Intel Corporation. All rights reserved.
//
// Maintained at www.Open-FCoE.org
//
// Fibre Channel Exchanges and Sequences.
//

//
// Sequence Status Block.
// This format is set by the FC-FS standard and is sent over the wire.
// Note that the fields aren't all naturally aligned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ssb {
    pub /: *mut *mut __u8 ssb_seq_id; / sequence ID,
    pub _ssb_resvd: __u8,
    pub /: *mut *mut __be16 ssb_low_seq_cnt; / lowest SEQ_CNT,
    pub /: *mut *mut __be16 ssb_high_seq_cnt; / highest SEQ_CNT,
    pub /: *mut *mut __be16 ssb_s_stat; / sequence status flags,
    pub /: *mut *mut __be16 ssb_err_seq_cnt; / error SEQ_CNT,
    pub /: *mut *mut __u8 ssb_fh_cs_ctl; / frame header CS_CTL,
    pub /: *mut *mut __be16 ssb_fh_ox_id; / frame header OX_ID,
    pub /: *mut *mut __be16 ssb_rx_id; / responder's exchange ID,
    pub _ssb_resvd2: [__u8; 2],
    pub PACKED: },
//
// The SSB should be 17 bytes.  Since it's layout is somewhat strange,
// we define the size here so that code can ASSERT that the size comes out
// correct.
//

//
// ssb_s_stat - flags from FC-FS-2 T11/1619-D Rev 0.90.
//

//
// Exchange Status Block.
// This format is set by the FC-FS standard and is sent over the wire.
// Note that the fields aren't all naturally aligned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_esb {
    pub /: *mut *mut __u8 esb_cs_ctl; / CS_CTL for frame header,
    pub /: *mut *mut __be16 esb_ox_id; / originator exchange ID,
    pub /: *mut *mut __be16 esb_rx_id; / responder exchange ID,
    pub /: *mut *mut __be32 esb_orig_fid; / fabric ID of originator,
    pub /: *mut *mut __be32 esb_resp_fid; / fabric ID of responder,
    pub /: *mut *mut __be32 esb_e_stat; / status,
    pub _esb_resvd: [__u8; 4],
    pub /: *mut *mut __u8 esb_service_params[112]; / TBD,
    pub /: *mut *mut __u8 esb_seq_status[8]; / sequence statuses, 8 bytes each,
    pub __attribute__((packed)): },
//
// Define expected size for ASSERTs.
// See comments on FC_SSB_SIZE.
//

//
// esb_e_stat - flags from FC-FS-2 T11/1619-D Rev 0.90.
//

