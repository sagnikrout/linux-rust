//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/fc_frame.h
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

// some helpful macros

//
// The fc_frame interface is used to pass frame data between functions.
// The frame includes the data buffer, length, and SOF / EOF delimiter types.
// A pointer to the port structure of the receiving port is also includeded.
//

// Max number of skb frags allowed, reserving one for fcoe_crc_eof page

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_frame {
    pub skb: sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_rcv_info {
    pub /: *mut *mut *mut fc_lport fr_dev; / transport layer private pointer,
    pub /: *mut *mut *mut fc_seq fr_seq; / for use with exchange manager,
    pub /: *mut *mut *mut fc_fcp_pkt fr_fsp; / for the corresponding fcp I/O,
    pub fr_crc: u32,
    pub /: *mut *mut u16 fr_max_payload; / max FC payload,
    pub /: *mut *mut u8 fr_sof; / start of frame delimiter,
    pub /: *mut *mut u8 fr_eof; / end of frame delimiter,
    pub /: *mut *mut u8 fr_flags; / flags - see below,
    pub /: *mut *mut u8 fr_encaps; / LLD encapsulation info (e.g. FIP),
    pub /: *mut *mut u8 granted_mac[ETH_ALEN]; / FCoE MAC address,
}

//
// Get fc_frame pointer for an skb that's already been imported.
//
// fr_flags.
//
pub const FCPHF_CRC_UNCHECKED: c_uint = 0x01	/* CRC not computed, still appended */;
//
// Initialize a frame.
// We don't do a complete memset here for performance reasons.
// The caller must set fr_free, fr_hdr, fr_len, fr_sof, and fr_eof eventually.
//
// Allocate fc_frame structure and buffer.  Set the initial length to
// payload_size + sizeof (struct fc_frame_header).
//
// Note: Since len will often be a constant multiple of 4,
// this check will usually be evaluated and eliminated at compile time.
//
// Free the fc_frame structure and buffer.
//
// Get frame header from message in fc_frame structure.
// This version doesn't do a length check.
//
// Get frame header from message in fc_frame structure.
// This hides a cast and provides a place to add some checking.
//
extern "C" {
    pub fn __fc_frame_header_get(_arg: fp) -> return;
}
//
// Get source FC_ID (S_ID) from frame header in message.
//
extern "C" {
    pub fn ntoh24(_arg: __fc_frame_header_get(fp)->fh_s_id) -> return;
}
//
// Get destination FC_ID (D_ID) from frame header in message.
//
extern "C" {
    pub fn ntoh24(_arg: __fc_frame_header_get(fp)->fh_d_id) -> return;
}
//
// Get frame payload from message in fc_frame structure.
// This hides a cast and provides a place to add some checking.
// The len parameter is the minimum length for the payload portion.
// Returns NULL if the frame is too short.
//
// This assumes the interesting part of the payload is in the first part
// of the buffer for received data.  This may not be appropriate to use for
// buffers being transmitted.
//
// Get frame payload opcode (first byte) from message in fc_frame structure.
// This hides a cast and provides a place to add some checking. Return 0
// if the frame has no payload.
//
// Get FC class from frame.
//
extern "C" {
    pub fn fc_sof_class(_arg: fr_sof(fp)) -> return;
}
//
// Check the CRC in a frame.
// The CRC immediately follows the last data item *AFTER* the length.
// The return value is zero if the CRC matches.
//
extern "C" {
    pub fn fc_frame_crc_check(: *mut fc_frame) -> u32;
}
//
// Check for leaks.
// Print the frame header of any currently allocated frame, assuming there
// should be none at this point.
//
extern "C" {
    pub fn fc_frame_leak_check();
}
//
// fill FC header fields in specified fc_frame
//
