//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/enetc/enetc_mailbox.h
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
// Copyright 2025-2026 NXP
//
// The VSI-to-PSI message generic format:
//
// OFFSET  0                               16              24            31
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// 0x0   |       CRC16 (big-endian)      |    CLASS ID   |     CMD ID    |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// 0x4   |   PROTO VER   |      LEN      |    RESV       | COOKIE|  RESV |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// 0x8   |                              RESV                             |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// 0xc   |                              RESV                             |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// 0x10  |                                                               |
// 0x14  |                                                               |
// 0x18  |                          Message Body                         |
// 0x1c  |                                                               |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// 0x20  |                                                               |
// ~   |              Extended Message Body: LEN x 32B                 |
// 0x3e0 |                                                               |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Field Descriptions:
// CRC16 (16-bit): Big endian, CRC16 CCITT-FALSE algorithm, It provides the
// equivalent data integrity check functionality as the FCS for standard
// Ethernet frames.
//
// CLASS ID (8-bit) and CMD ID (8-bit): These are 8-bit fields identifying
// the command class and the class-specific operations supported. For more
// details, please refer to the definitions of the relevant class ID and
// cmd ID in this document.
//
// PROTO VER (8-bit): Supported VSI-PSI command protocol version. Currently
// only support version 0. To be incremented for future protocol extensions.
//
// LEN (8-bit): Extended message body length in increments of 32B. The upper
// limit is given by the physical implementation of the NETC VSI-PSI Messaging
// mechanism that supports message sizes of up to 1024B (including headers),
// that are multiple of 32B.
//
// COOKIE (4-bit): Optional parameter, which, if not 0, indicates that the
// command should be execute asynchronously on PSI side. If COOKIE is not 0
// and the command cannot be executed instantly on the PSI side (it would
// take longer time to complete), the PSI may enqueue the request in a command
// queue of up to 15 entries per VSI and, later after command execution, the
// PSI returns the COOKIE to VSI as part of an asynchronous notification
// message that indicates the command completion status. If COOKIE is 0 then
// the command is considered as blocking, the PSI will wait for the execution
// of the command to complete before updating the PSIMSGRR[MC] field with the
// corresponding return code.
//
// The PSI-to-VSI message generic format:
// 0               4               8               12          15
// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
// |       COOKIE      |   CLASS CODE  |          CLASS ID         |
// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//
// The PSI to VSI message format is mapped to the following PSI message
// registers/fields, depending on use case:
// 1) PSI_RX_control: PSIMSGRR[MC] - for VSI command return code messages
// (blocking requests), and
// 2) PSI_TX_control: PSIMSGSR[MC] - for PSI to VSI notification messages
// (async mode)
//
// Note that for some GET messages, there is no COOKIE field, and the CLASS
// CODE field is expanded to 8 bits.
//

pub const ENETC_CRC_INIT: c_uint = 0xffff;
pub const ENETC_MSG_ALIGN: c_int = 32;
// s indicates the size of the message

// l indicates the extended body len (LEN field) of the message

// The cookie filed of VSI-to-PSI message

// The fileds of PSI-to-VSI message, the message is only 16-bit

// Extend the class code to 8-bit for GET messages without COOKIE

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enetc_msg_class_id {
// Class ID for PSI-to-VSI messages
    ENETC_MSG_CLASS_ID_CMD_SUCCESS		= 1,
    ENETC_MSG_CLASS_ID_PERMISSION_DENY,
    ENETC_MSG_CLASS_ID_CMD_NOT_SUPPORT,
    ENETC_MSG_CLASS_ID_PSI_BUSY,
    ENETC_MSG_CLASS_ID_CRC_ERROR,
    ENETC_MSG_CLASS_ID_PROTO_NOT_SUPPORT,
    ENETC_MSG_CLASS_ID_INVALID_MSG_LEN,
    ENETC_MSG_CLASS_ID_CMD_TIMEOUT,
    ENETC_MSG_CLASS_ID_CMD_NOT_PERMITTED,
    ENETC_MSG_CLASS_ID_CMD_FAIL, /* Generic error code for failure */
    ENETC_MSG_CLASS_ID_CMD_DEFERRED		= 0xf,

// Common Class ID for PSI-to-VSI and VSI-to-PSI messages
    ENETC_MSG_CLASS_ID_MAC_FILTER		= 0x20,
    ENETC_MSG_CLASS_ID_IP_REVISION		= 0xf0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enetc_msg_mac_filter_cmd_id {
    ENETC_MSG_SET_PRIMARY_MAC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enetc_msg_ip_revision_cmd_id {
    ENETC_MSG_GET_IP_MN			= 1,
}

// Class-specific error return codes of MAC filter
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enetc_mac_filter_class_code {
    ENETC_MF_CLASS_CODE_INVALID_MAC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_msg_swbd {
    pub vaddr: *mut c_void,
    pub dma: dma_addr_t,
    pub size: c_int,
}

// The generic VSI-to-PSI message header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_msg_header {
    pub crc16: __be16,
    pub class_id: u8,
    pub cmd_id: u8,
    pub proto_ver: u8,
    pub len: u8,
    pub resv0: u8,
    pub cookie: u8,
    pub resv2: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_mac_addr {
    pub /: *mut *mut u8 addr[ETH_ALEN]; / Network byte order,
}

// Message format of class_id 0x20 for exact MAC filter.
// cmd_id 0x0: set primary MAC
// cmd_id 0x1: Add entries to MAC address filter table
// cmd_id 0x2: Delete entries from MAC address filter table
// Note that cmd_id 0x1 and 0x2 are not supported yet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_msg_mac_exact_filter {
    pub hdr: enetc_msg_header,
    pub /: *mut *mut u8 mac_cnt; / No need to set for cmd_id 0,
    pub resv: [u8; 3],
    pub mac: [enetc_mac_addr; ],
}

// The generic message format applies to the following messages:
// Get IP revision message, class_id 0xf0.
// cmd_id 1: get IP minor revision
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_msg_generic {
    pub hdr: enetc_msg_header,
    pub resv: [u8; 16],
}
