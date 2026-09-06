//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/abi/guc_communication_ctb_abi.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2014-2021 Intel Corporation
//

//
// DOC: CT Buffer
//
// Circular buffer used to send `CTB Message`_
//
// DOC: CTB Descriptor
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |  31:0 | **HEAD** - offset (in dwords) to the last dword that was     |
// |   |       | read from the `CT Buffer`_.                                  |
// |   |       | It can only be updated by the receiver.                      |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 | **TAIL** - offset (in dwords) to the last dword that was     |
// |   |       | written to the `CT Buffer`_.                                 |
// |   |       | It can only be updated by the sender.                        |
// +---+-------+--------------------------------------------------------------+
// | 2 |  31:0 | **STATUS** - status of the CTB                               |
// |   |       |                                                              |
// |   |       |   - _`GUC_CTB_STATUS_NO_ERROR` = 0 (normal operation)        |
// |   |       |   - _`GUC_CTB_STATUS_OVERFLOW` = 1 (head/tail too large)     |
// |   |       |   - _`GUC_CTB_STATUS_UNDERFLOW` = 2 (truncated message)      |
// |   |       |   - _`GUC_CTB_STATUS_MISMATCH` = 4 (head/tail modified)      |
// |   |       |   - _`GUC_CTB_STATUS_UNUSED` = 8 (CTB is not in use)         |
// +---+-------+--------------------------------------------------------------+
// |...|       | RESERVED = MBZ                                               |
// +---+-------+--------------------------------------------------------------+
// | 15|  31:0 | RESERVED = MBZ                                               |
// +---+-------+--------------------------------------------------------------+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_ct_buffer_desc {
    pub head: u32,
    pub tail: u32,
    pub status: u32,
pub const GUC_CTB_STATUS_NO_ERROR: c_int = 0;
    pub reserved: [u32; 13],
    pub __packed: },
    pub 64): static_assert(sizeof(struct guc_ct_buffer_desc) ==,
//
// DOC: CTB Message
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 | 31:16 | **FENCE** - message identifier                               |
// |   +-------+--------------------------------------------------------------+
// |   | 15:12 | **FORMAT** - format of the CTB message                       |
// |   |       |  - _`GUC_CTB_FORMAT_HXG` = 0 - see `CTB HXG Message`_        |
// |   +-------+--------------------------------------------------------------+
// |   |  11:8 | **RESERVED**                                                 |
// |   +-------+--------------------------------------------------------------+
// |   |   7:0 | **NUM_DWORDS** - length of the CTB message (w/o header)      |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 | optional (depends on FORMAT)                                 |
// +---+-------+                                                              |
// |...|       |                                                              |
// +---+-------+                                                              |
// | n |  31:0 |                                                              |
// +---+-------+--------------------------------------------------------------+
//

//
// DOC: CTB HXG Message
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 | 31:16 | FENCE                                                        |
// |   +-------+--------------------------------------------------------------+
// |   | 15:12 | FORMAT = GUC_CTB_FORMAT_HXG_                                 |
// |   +-------+--------------------------------------------------------------+
// |   |  11:8 | RESERVED = MBZ                                               |
// |   +-------+--------------------------------------------------------------+
// |   |   7:0 | NUM_DWORDS = length (in dwords) of the embedded HXG message  |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 |                                                              |
// +---+-------+                                                              |
// |...|       | [Embedded `HXG Message`_]                                    |
// +---+-------+                                                              |
// | n |  31:0 |                                                              |
// +---+-------+--------------------------------------------------------------+
//

//
// DOC: CTB based communication
//
// The CTB (command transport buffer) communication between Host and GuC
// is based on u32 data stream written to the shared buffer. One buffer can
// be used to transmit data only in one direction (one-directional channel).
//
// Current status of the each buffer is stored in the buffer descriptor.
// Buffer descriptor holds tail and head fields that represents active data
// stream. The tail field is updated by the data producer (sender), and head
// field is updated by the data consumer (receiver)::
//
// +------------+
// | DESCRIPTOR |          +=================+============+========+
// +============+          |                 | MESSAGE(s) |        |
// | address    |--------->+=================+============+========+
// +------------+
// | head       |          ^-----head--------^
// +------------+
// | tail       |          ^---------tail-----------------^
// +------------+
// | size       |          ^---------------size--------------------^
// +------------+
//
// Each message in data stream starts with the single u32 treated as a header,
// followed by optional set of u32 data that makes message specific payload::
//
// +------------+---------+---------+---------+
// |         MESSAGE                          |
// +------------+---------+---------+---------+
// |   msg[0]   |   [1]   |   ...   |  [n-1]  |
// +------------+---------+---------+---------+
// |   MESSAGE  |       MESSAGE PAYLOAD       |
// +   HEADER   +---------+---------+---------+
// |            |    0    |   ...   |    n    |
// +======+=====+=========+=========+=========+
// | 31:16| code|         |         |         |
// +------+-----+         |         |         |
// |  15:5|flags|         |         |         |
// +------+-----+         |         |         |
// |   4:0|  len|         |         |         |
// +------+-----+---------+---------+---------+
//
// ^-------------len-------------^
//
// The message header consists of:
//
// - **len**, indicates length of the message payload (in u32)
// - **code**, indicates message code
// - **flags**, holds various bits to control message handling
//
