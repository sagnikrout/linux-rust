//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/abi/guc_relay_communication_abi.h
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
// Copyright © 2023 Intel Corporation
//

//
// DOC: GuC Relay Communication
//
// The communication between Virtual Function (VF) drivers and Physical Function
// (PF) drivers is based on the GuC firmware acting as a proxy (relay) agent.
//
// To communicate with the PF driver, VF's drivers use `VF2GUC_RELAY_TO_PF`_
// action that takes the `Relay Message`_ as opaque payload and requires the
// relay message identifier (RID) as additional parameter.
//
// This identifier is used by the drivers to match related messages.
//
// The GuC forwards this `Relay Message`_ and its identifier to the PF driver
// in `GUC2PF_RELAY_FROM_VF`_ action. This event message additionally contains
// the identifier of the origin VF (VFID).
//
// Likewise, to communicate with the VF drivers, PF driver use
// `VF2GUC_RELAY_TO_PF`_ action that in addition to the `Relay Message`_
// and the relay message identifier (RID) also takes the target VF identifier.
//
// The GuC uses this target VFID from the message to select where to send the
// `GUC2VF_RELAY_FROM_PF`_ with the embedded `Relay Message`_ with response::
//
// VF                             GuC                              PF
// |                               |                               |
// [ ] VF2GUC_RELAY_TO_PF           |                               |
// [ ]---------------------------> [ ]                              |
// [ ] { rid, msg }                [ ]                              |
// [ ]                             [ ] GUC2PF_RELAY_FROM_VF         |
// [ ]                             [ ]---------------------------> [ ]
// [ ]                              |  { VFID, rid, msg }          [ ]
// [ ]                              |                              [ ]
// [ ]                              |           PF2GUC_RELAY_TO_VF [ ]
// [ ]                             [ ] <---------------------------[ ]
// [ ]                             [ ]        { VFID, rid, reply }  |
// [ ]        GUC2VF_RELAY_FROM_PF [ ]                              |
// [ ] <---------------------------[ ]                              |
// |               { rid, reply }  |                               |
// |                               |                               |
//
// It is also possible that PF driver will initiate communication with the
// selected VF driver. The same GuC action messages will be used::
//
// VF                             GuC                              PF
// |                               |                               |
// |                               |           PF2GUC_RELAY_TO_VF [ ]
// |                              [ ] <---------------------------[ ]
// |                              [ ]          { VFID, rid, msg } [ ]
// |         GUC2VF_RELAY_FROM_PF [ ]                             [ ]
// [ ] <---------------------------[ ]                             [ ]
// [ ]                { rid, msg }  |                              [ ]
// [ ]                              |                              [ ]
// [ ] VF2GUC_RELAY_TO_PF           |                              [ ]
// [ ]---------------------------> [ ]                             [ ]
// |  { rid, reply }              [ ]                             [ ]
// |                              [ ] GUC2PF_RELAY_FROM_VF        [ ]
// |                              [ ]---------------------------> [ ]
// |                               | { VFID, rid, reply }          |
// |                               |                               |
//
// DOC: Relay Message
//
// The `Relay Message`_ is used by Physical Function (PF) driver and Virtual
// Function (VF) drivers to communicate using `GuC Relay Communication`_.
//
// Format of the `Relay Message`_ follows format of the generic `HXG Message`_.
//
// +--------------------------------------------------------------------------+
// |  `Relay Message`_                                                        |
// +==========================================================================+
// |  `HXG Message`_                                                          |
// +--------------------------------------------------------------------------+
//
// Maximum length of the `Relay Message`_ is limited by the maximum length of
// the `CTB HXG Message`_ and format of the `GUC2PF_RELAY_FROM_VF`_ message.
//

//
// DOC: Relay Error Codes
//
// The `GuC Relay Communication`_ can be used to pass `Relay Message`_ between
// drivers that run on different Operating Systems. To help in troubleshooting,
// `GuC Relay Communication`_ uses error codes that mostly match errno values.
//
pub const GUC_RELAY_ERROR_UNDISCLOSED: c_int = 0;

