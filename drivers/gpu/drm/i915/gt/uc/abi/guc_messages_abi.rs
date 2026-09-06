//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/abi/guc_messages_abi.h
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
// DOC: HXG Message
//
// All messages exchanged with GuC are defined using 32 bit dwords.
// First dword is treated as a message header. Remaining dwords are optional.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// |   |       |                                                              |
// | 0 |    31 | **ORIGIN** - originator of the message                       |
// |   |       |   - _`GUC_HXG_ORIGIN_HOST` = 0                               |
// |   |       |   - _`GUC_HXG_ORIGIN_GUC` = 1                                |
// |   |       |                                                              |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | **TYPE** - message type                                      |
// |   |       |   - _`GUC_HXG_TYPE_REQUEST` = 0                              |
// |   |       |   - _`GUC_HXG_TYPE_EVENT` = 1                                |
// |   |       |   - _`GUC_HXG_TYPE_FAST_REQUEST` = 2                         |
// |   |       |   - _`GUC_HXG_TYPE_NO_RESPONSE_BUSY` = 3                     |
// |   |       |   - _`GUC_HXG_TYPE_NO_RESPONSE_RETRY` = 5                    |
// |   |       |   - _`GUC_HXG_TYPE_RESPONSE_FAILURE` = 6                     |
// |   |       |   - _`GUC_HXG_TYPE_RESPONSE_SUCCESS` = 7                     |
// |   +-------+--------------------------------------------------------------+
// |   |  27:0 | **AUX** - auxiliary data (depends on TYPE)                   |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 |                                                              |
// +---+-------+                                                              |
// |...|       | **PAYLOAD** - optional payload (depends on TYPE)             |
// +---+-------+                                                              |
// | n |  31:0 |                                                              |
// +---+-------+--------------------------------------------------------------+
//

//
// DOC: HXG Request
//
// The `HXG Request`_ message should be used to initiate synchronous activity
// for which confirmation or return data is expected.
//
// The recipient of this message shall use `HXG Response`_, `HXG Failure`_
// or `HXG Retry`_ message as a definite reply, and may use `HXG Busy`_
// message as a intermediate reply.
//
// Format of @DATA0 and all @DATAn fields depends on the @ACTION code.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN                                                       |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_REQUEST_                                 |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | **DATA0** - request data (depends on ACTION)                 |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | **ACTION** - requested action code                           |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 |                                                              |
// +---+-------+                                                              |
// |...|       | **DATAn** - optional data (depends on ACTION)                |
// +---+-------+                                                              |
// | n |  31:0 |                                                              |
// +---+-------+--------------------------------------------------------------+
//

//
// DOC: HXG Fast Request
//
// The `HXG Request`_ message should be used to initiate asynchronous activity
// for which confirmation or return data is not expected.
//
// If confirmation is required then `HXG Request`_ shall be used instead.
//
// The recipient of this message may only use `HXG Failure`_ message if it was
// unable to accept this request (like invalid data).
//
// Format of `HXG Fast Request`_ message is same as `HXG Request`_ except @TYPE.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN - see `HXG Message`_                                  |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = `GUC_HXG_TYPE_FAST_REQUEST`_                          |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | DATA0 - see `HXG Request`_                                   |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | ACTION - see `HXG Request`_                                  |
// +---+-------+--------------------------------------------------------------+
// |...|       | DATAn - see `HXG Request`_                                   |
// +---+-------+--------------------------------------------------------------+
//
// DOC: HXG Event
//
// The `HXG Event`_ message should be used to initiate asynchronous activity
// that does not involves immediate confirmation nor data.
//
// Format of @DATA0 and all @DATAn fields depends on the @ACTION code.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN                                                       |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_EVENT_                                   |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | **DATA0** - event data (depends on ACTION)                   |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | **ACTION** - event action code                               |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 |                                                              |
// +---+-------+                                                              |
// |...|       | **DATAn** - optional event  data (depends on ACTION)         |
// +---+-------+                                                              |
// | n |  31:0 |                                                              |
// +---+-------+--------------------------------------------------------------+
//

//
// DOC: HXG Busy
//
// The `HXG Busy`_ message may be used to acknowledge reception of the `HXG Request`_
// message if the recipient expects that it processing will be longer than default
// timeout.
//
// The @COUNTER field may be used as a progress indicator.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN                                                       |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_NO_RESPONSE_BUSY_                        |
// |   +-------+--------------------------------------------------------------+
// |   |  27:0 | **COUNTER** - progress indicator                             |
// +---+-------+--------------------------------------------------------------+
//

//
// DOC: HXG Retry
//
// The `HXG Retry`_ message should be used by recipient to indicate that the
// `HXG Request`_ message was dropped and it should be resent again.
//
// The @REASON field may be used to provide additional information.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN                                                       |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_NO_RESPONSE_RETRY_                       |
// |   +-------+--------------------------------------------------------------+
// |   |  27:0 | **REASON** - reason for retry                                |
// |   |       |  - _`GUC_HXG_RETRY_REASON_UNSPECIFIED` = 0                   |
// +---+-------+--------------------------------------------------------------+
//

//
// DOC: HXG Failure
//
// The `HXG Failure`_ message shall be used as a reply to the `HXG Request`_
// message that could not be processed due to an error.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN                                                       |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_RESPONSE_FAILURE_                        |
// |   +-------+--------------------------------------------------------------+
// |   | 27:16 | **HINT** - additional error hint                             |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | **ERROR** - error/result code                                |
// +---+-------+--------------------------------------------------------------+
//

//
// DOC: HXG Response
//
// The `HXG Response`_ message shall be used as a reply to the `HXG Request`_
// message that was successfully processed without an error.
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 |    31 | ORIGIN                                                       |
// |   +-------+--------------------------------------------------------------+
// |   | 30:28 | TYPE = GUC_HXG_TYPE_RESPONSE_SUCCESS_                        |
// |   +-------+--------------------------------------------------------------+
// |   |  27:0 | **DATA0** - data (depends on ACTION from `HXG Request`_)     |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 |                                                              |
// +---+-------+                                                              |
// |...|       | **DATAn** - data (depends on ACTION from `HXG Request`_)     |
// +---+-------+                                                              |
// | n |  31:0 |                                                              |
// +---+-------+--------------------------------------------------------------+
//

// deprecated
pub const INTEL_GUC_MSG_TYPE_SHIFT: c_int = 28;

pub const INTEL_GUC_MSG_DATA_SHIFT: c_int = 16;

pub const INTEL_GUC_MSG_CODE_SHIFT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_guc_msg_type {
    INTEL_GUC_MSG_TYPE_REQUEST = 0x0,
    INTEL_GUC_MSG_TYPE_RESPONSE = 0xF,
}
