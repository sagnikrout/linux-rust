//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_pfvf_msg.h
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2015 - 2021 Intel Corporation

//
// PF<->VF Gen2 Messaging format
//
// The PF has an array of 32-bit PF2VF registers, one for each VF. The
// PF can access all these registers while each VF can access only the one
// register associated with that particular VF.
//
// The register functionally is split into two parts:
// The bottom half is for PF->VF messages. In particular when the first
// bit of this register (bit 0) gets set an interrupt will be triggered
// in the respective VF.
// The top half is for VF->PF messages. In particular when the first bit
// of this half of register (bit 16) gets set an interrupt will be triggered
// in the PF.
//
// The remaining bits within this register are available to encode messages.
// and implement a collision control mechanism to prevent concurrent use of
// the PF2VF register by both the PF and VF.
//
// 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16
// _______________________________________________
// |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
// +-----------------------------------------------+
// \___________________________/ \_________/ ^   ^
// ^                    ^      |   |
// |                    |      |   VF2PF Int
// |                    |      Message Origin
// |                    Message Type
// Message-specific Data/Reserved
//
// 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
// _______________________________________________
// |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
// +-----------------------------------------------+
// \___________________________/ \_________/ ^   ^
// ^                    ^      |   |
// |                    |      |   PF2VF Int
// |                    |      Message Origin
// |                    Message Type
// Message-specific Data/Reserved
//
// Message Origin (Should always be 1)
// A legacy out-of-tree QAT driver allowed for a set of messages not supported
// by this driver; these had a Msg Origin of 0 and are ignored by this driver.
//
// When a PF or VF attempts to send a message in the lower or upper 16 bits,
// respectively, the other 16 bits are written to first with a defined
// IN_USE_BY pattern as part of a collision control scheme (see function
// adf_gen2_pfvf_send() in adf_pf2vf_msg.c).
//
// PF<->VF Gen4 Messaging format
//
// Similarly to the gen2 messaging format, 32-bit long registers are used for
// communication between PF and VFs. However, each VF and PF share a pair of
// 32-bits register to avoid collisions: one for PV to VF messages and one
// for VF to PF messages.
//
// Both the Interrupt bit and the Message Origin bit retain the same position
// and meaning, although non-system messages are now deprecated and not
// expected.
//
// 31 30              9  8  7  6  5  4  3  2  1  0
// _______________________________________________
// |  |  |   . . .   |  |  |  |  |  |  |  |  |  |  |
// +-----------------------------------------------+
// \_____________________/ \_______________/  ^  ^
// ^                     ^         |  |
// |                     |         |  PF/VF Int
// |                     |         Message Origin
// |                     Message Type
// Message-specific Data/Reserved
//
// For both formats, the message reception is acknowledged by lowering the
// interrupt bit on the register where the message was sent.
//
// PFVF message common bits

// Different generations have different CSR layouts, use this struct
// to abstract these differences away
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfvf_message {
    pub type: u8,
    pub data: u32,
}

// PF->VF messages
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pf2vf_msgtype {
    ADF_PF2VF_MSGTYPE_RESTARTING		= 0x01,
    ADF_PF2VF_MSGTYPE_VERSION_RESP		= 0x02,
    ADF_PF2VF_MSGTYPE_BLKMSG_RESP		= 0x03,
    ADF_PF2VF_MSGTYPE_FATAL_ERROR		= 0x04,
    ADF_PF2VF_MSGTYPE_RESTARTED		= 0x05,
// Values from 0x10 are Gen4 specific, message type is only 4 bits in Gen2 devices.
    ADF_PF2VF_MSGTYPE_RP_RESET_RESP		= 0x10,
}

// VF->PF messages
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vf2pf_msgtype {
    ADF_VF2PF_MSGTYPE_INIT			= 0x03,
    ADF_VF2PF_MSGTYPE_SHUTDOWN		= 0x04,
    ADF_VF2PF_MSGTYPE_VERSION_REQ		= 0x05,
    ADF_VF2PF_MSGTYPE_COMPAT_VER_REQ	= 0x06,
    ADF_VF2PF_MSGTYPE_LARGE_BLOCK_REQ	= 0x07,
    ADF_VF2PF_MSGTYPE_MEDIUM_BLOCK_REQ	= 0x08,
    ADF_VF2PF_MSGTYPE_SMALL_BLOCK_REQ	= 0x09,
    ADF_VF2PF_MSGTYPE_RESTARTING_COMPLETE	= 0x0a,
// Values from 0x10 are Gen4 specific, message type is only 4 bits in Gen2 devices.
    ADF_VF2PF_MSGTYPE_RP_RESET		= 0x10,
}

// VF/PF compatibility version.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pfvf_compatibility_version {
// Support for extended capabilities
    ADF_PFVF_COMPAT_CAPABILITIES		= 0x02,
// In-use pattern cleared by receiver
    ADF_PFVF_COMPAT_FAST_ACK		= 0x03,
// Ring to service mapping support for non-standard mappings
    ADF_PFVF_COMPAT_RING_TO_SVC_MAP		= 0x04,
// Fallback compat
    ADF_PFVF_COMPAT_FALLBACK		= 0x05,
// Reference to the latest version
    ADF_PFVF_COMPAT_THIS_VERSION		= 0x05,
}

// PF->VF Version Response

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pf2vf_compat_response {
    ADF_PF2VF_VF_COMPATIBLE			= 0x01,
    ADF_PF2VF_VF_INCOMPATIBLE		= 0x02,
    ADF_PF2VF_VF_COMPAT_UNKNOWN		= 0x03,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ring_reset_result {
    RPRESET_SUCCESS				= 0x00,
    RPRESET_NOT_SUPPORTED			= 0x01,
    RPRESET_INVAL_BANK			= 0x02,
    RPRESET_TIMEOUT				= 0x03,
}

// PF->VF Block Responses

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pf2vf_blkmsg_resp_type {
    ADF_PF2VF_BLKMSG_RESP_TYPE_DATA		= 0x00,
    ADF_PF2VF_BLKMSG_RESP_TYPE_CRC		= 0x01,
    ADF_PF2VF_BLKMSG_RESP_TYPE_ERROR	= 0x02,
}

// PF->VF Block Error Code
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pf2vf_blkmsg_error {
    ADF_PF2VF_INVALID_BLOCK_TYPE		= 0x00,
    ADF_PF2VF_INVALID_BYTE_NUM_REQ		= 0x01,
    ADF_PF2VF_PAYLOAD_TRUNCATED		= 0x02,
    ADF_PF2VF_UNSPECIFIED_ERROR		= 0x03,
}

// VF->PF Block Requests

// PF->VF Block Request Types
// 0..15 - 32 byte message
// 16..23 - 64 byte message
// 24..27 - 128 byte message
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vf2pf_blkmsg_req_type {
    ADF_VF2PF_BLKMSG_REQ_CAP_SUMMARY	= 0x02,
    ADF_VF2PF_BLKMSG_REQ_RING_SVC_MAP	= 0x03,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfvf_blkmsg_header {
    pub version: u8,
    pub payload_size: u8,
    pub __packed: },

pub const ADF_PFVF_BLKMSG_MSG_MAX_SIZE: c_int = 128;
// PF->VF Block message header bytes
pub const ADF_PFVF_BLKMSG_VER_BYTE: c_int = 0;
pub const ADF_PFVF_BLKMSG_LEN_BYTE: c_int = 1;
// PF/VF Capabilities message values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blkmsg_capabilities_versions {
    ADF_PFVF_CAPABILITIES_V1_VERSION	= 0x01,
    ADF_PFVF_CAPABILITIES_V2_VERSION	= 0x02,
    ADF_PFVF_CAPABILITIES_V3_VERSION	= 0x03,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct capabilities_v1 {
    pub hdr: pfvf_blkmsg_header,
    pub ext_dc_caps: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct capabilities_v2 {
    pub hdr: pfvf_blkmsg_header,
    pub ext_dc_caps: u32,
    pub capabilities: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct capabilities_v3 {
    pub hdr: pfvf_blkmsg_header,
    pub ext_dc_caps: u32,
    pub capabilities: u32,
    pub frequency: u32,
    pub __packed: },
// PF/VF Ring to service mapping values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blkmsg_ring_to_svc_versions {
    ADF_PFVF_RING_TO_SVC_VERSION		= 0x01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_to_svc_map_v1 {
    pub hdr: pfvf_blkmsg_header,
    pub map: u16,
    pub __packed: },
