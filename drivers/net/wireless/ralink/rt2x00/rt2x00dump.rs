//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ralink/rt2x00/rt2x00dump.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// DOC: Introduction
//
// This header is intended to be exported to userspace,
// to make the structures and enumerations available to userspace
// applications. This means that all data types should be exportable.
//
// When rt2x00 is compiled with debugfs support enabled,
// it is possible to capture all data coming in and out of the device
// by reading the frame dump file. This file can have only a single reader.
// The following frames will be reported:
// - All incoming frames (rx)
// - All outgoing frames (tx, including beacon and atim)
// - All completed frames (txdone including atim)
//
// The data is send to the file using the following format:
//
// [rt2x00dump header][hardware descriptor][ieee802.11 frame]
//
// rt2x00dump header: The description of the dumped frame, as well as
// additional information useful for debugging. See &rt2x00dump_hdr.
// hardware descriptor: Descriptor that was used to receive or transmit
// the frame.
// ieee802.11 frame: The actual frame that was received or transmitted.
//
// enum rt2x00_dump_type - Frame type
//
// These values are used for the @type member of &rt2x00dump_hdr.
// @DUMP_FRAME_RXDONE: This frame has been received by the hardware.
// @DUMP_FRAME_TX: This frame is queued for transmission to the hardware.
// @DUMP_FRAME_TXDONE: This frame indicates the device has handled
// the tx event which has either succeeded or failed. A frame
// with this type should also have been reported with as a
// %DUMP_FRAME_TX frame.
// @DUMP_FRAME_BEACON: This beacon frame is queued for transmission to the
// hardware.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt2x00_dump_type {
    DUMP_FRAME_RXDONE = 1,
    DUMP_FRAME_TX = 2,
    DUMP_FRAME_TXDONE = 3,
    DUMP_FRAME_BEACON = 4,
}

//
// struct rt2x00dump_hdr - Dump frame header
//
// Each frame dumped to the debugfs file starts with this header
// attached. This header contains the description of the actual
// frame which was dumped.
//
// New fields inside the structure must be appended to the end of
// the structure. This way userspace tools compiled for earlier
// header versions can still correctly handle the frame dump
// (although they will not handle all data passed to them in the dump).
//
// @version: Header version should always be set to %DUMP_HEADER_VERSION.
// This field must be checked by userspace to determine if it can
// handle this frame.
// @header_length: The length of the &rt2x00dump_hdr structure. This is
// used for compatibility reasons so userspace can easily determine
// the location of the next field in the dump.
// @desc_length: The length of the device descriptor.
// @data_length: The length of the frame data (including the ieee802.11 header.
// @chip_rt: RT chipset
// @chip_rf: RF chipset
// @chip_rev: Chipset revision
// @type: The frame type (&rt2x00_dump_type)
// @queue_index: The index number of the data queue.
// @entry_index: The index number of the entry inside the data queue.
// @timestamp_sec: Timestamp - seconds
// @timestamp_usec: Timestamp - microseconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2x00dump_hdr {
    pub version: __le32,
pub const DUMP_HEADER_VERSION: c_int = 3;
    pub header_length: __le32,
    pub desc_length: __le32,
    pub data_length: __le32,
    pub chip_rt: __le16,
    pub chip_rf: __le16,
    pub chip_rev: __le16,
    pub type: __le16,
    pub queue_index: __u8,
    pub entry_index: __u8,
    pub timestamp_sec: __le32,
    pub timestamp_usec: __le32,
}
