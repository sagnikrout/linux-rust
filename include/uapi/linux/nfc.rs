//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nfc.h
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


//
// Copyright (C) 2011 Instituto Nokia de Tecnologia
//
// Authors:
// Lauro Ramos Venancio <lauro.venancio@openbossa.org>
// Aloisio Almeida Jr <aloisio.almeida@openbossa.org>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

pub const NFC_GENL_VERSION: c_int = 1;

//
// enum nfc_commands - supported nfc commands
//
// @NFC_CMD_UNSPEC: unspecified command
//
// @NFC_CMD_GET_DEVICE: request information about a device (requires
// %NFC_ATTR_DEVICE_INDEX) or dump request to get a list of all nfc devices
// @NFC_CMD_DEV_UP: turn on the nfc device
// (requires %NFC_ATTR_DEVICE_INDEX)
// @NFC_CMD_DEV_DOWN: turn off the nfc device
// (requires %NFC_ATTR_DEVICE_INDEX)
// @NFC_CMD_START_POLL: start polling for targets using the given protocols
// (requires %NFC_ATTR_DEVICE_INDEX and %NFC_ATTR_PROTOCOLS)
// @NFC_CMD_STOP_POLL: stop polling for targets (requires
// %NFC_ATTR_DEVICE_INDEX)
// @NFC_CMD_GET_TARGET: dump all targets found by the previous poll (requires
// %NFC_ATTR_DEVICE_INDEX)
// @NFC_EVENT_TARGETS_FOUND: event emitted when a new target is found
// (it sends %NFC_ATTR_DEVICE_INDEX)
// @NFC_EVENT_DEVICE_ADDED: event emitted when a new device is registred
// (it sends %NFC_ATTR_DEVICE_NAME, %NFC_ATTR_DEVICE_INDEX and
// %NFC_ATTR_PROTOCOLS)
// @NFC_EVENT_DEVICE_REMOVED: event emitted when a device is removed
// (it sends %NFC_ATTR_DEVICE_INDEX)
// @NFC_EVENT_TM_ACTIVATED: event emitted when the adapter is activated in
// target mode.
// @NFC_EVENT_TM_DEACTIVATED: event emitted when the adapter is deactivated
// from target mode.
// @NFC_CMD_LLC_GET_PARAMS: request LTO, RW, and MIUX parameters for a device
// @NFC_CMD_LLC_SET_PARAMS: set one or more of LTO, RW, and MIUX parameters for
// a device. LTO must be set before the link is up otherwise -EINPROGRESS
// is returned. RW and MIUX can be set at anytime and will be passed in
// subsequent CONNECT and CC messages.
// If one of the passed parameters is wrong none is set and -EINVAL is
// returned.
// @NFC_CMD_ENABLE_SE: Enable the physical link to a specific secure element.
// Once enabled a secure element will handle card emulation mode, i.e.
// starting a poll from a device which has a secure element enabled means
// we want to do SE based card emulation.
// @NFC_CMD_DISABLE_SE: Disable the physical link to a specific secure element.
// @NFC_CMD_FW_DOWNLOAD: Request to Load/flash firmware, or event to inform
// that some firmware was loaded
// @NFC_EVENT_SE_ADDED: Event emitted when a new secure element is discovered.
// This typically will be sent whenever a new NFC controller with either
// an embedded SE or an UICC one connected to it through SWP.
// @NFC_EVENT_SE_REMOVED: Event emitted when a secure element is removed from
// the system, as a consequence of e.g. an NFC controller being unplugged.
// @NFC_EVENT_SE_CONNECTIVITY: This event is emitted whenever a secure element
// is requesting connectivity access. For example a UICC SE may need to
// talk with a sleeping modem and will notify this need by sending this
// event. It is then up to userspace to decide if it will wake the modem
// up or not.
// @NFC_EVENT_SE_TRANSACTION: This event is sent when an application running on
// a specific SE notifies us about the end of a transaction. The parameter
// for this event is the application ID (AID).
// @NFC_CMD_GET_SE: Dump all discovered secure elements from an NFC controller.
// @NFC_CMD_SE_IO: Send/Receive APDUs to/from the selected secure element.
// @NFC_CMD_ACTIVATE_TARGET: Request NFC controller to reactivate target.
// @NFC_CMD_VENDOR: Vendor specific command, to be implemented directly
// from the driver in order to support hardware specific operations.
// @NFC_CMD_DEACTIVATE_TARGET: Request NFC controller to deactivate target.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfc_commands {
    NFC_CMD_UNSPEC,
    NFC_CMD_GET_DEVICE,
    NFC_CMD_DEV_UP,
    NFC_CMD_DEV_DOWN,
    NFC_CMD_DEP_LINK_UP,
    NFC_CMD_DEP_LINK_DOWN,
    NFC_CMD_START_POLL,
    NFC_CMD_STOP_POLL,
    NFC_CMD_GET_TARGET,
    NFC_EVENT_TARGETS_FOUND,
    NFC_EVENT_DEVICE_ADDED,
    NFC_EVENT_DEVICE_REMOVED,
    NFC_EVENT_TARGET_LOST,
    NFC_EVENT_TM_ACTIVATED,
    NFC_EVENT_TM_DEACTIVATED,
    NFC_CMD_LLC_GET_PARAMS,
    NFC_CMD_LLC_SET_PARAMS,
    NFC_CMD_ENABLE_SE,
    NFC_CMD_DISABLE_SE,
    NFC_CMD_LLC_SDREQ,
    NFC_EVENT_LLC_SDRES,
    NFC_CMD_FW_DOWNLOAD,
    NFC_EVENT_SE_ADDED,
    NFC_EVENT_SE_REMOVED,
    NFC_EVENT_SE_CONNECTIVITY,
    NFC_EVENT_SE_TRANSACTION,
    NFC_CMD_GET_SE,
    NFC_CMD_SE_IO,
    NFC_CMD_ACTIVATE_TARGET,
    NFC_CMD_VENDOR,
    NFC_CMD_DEACTIVATE_TARGET,
// private: internal use only
    __NFC_CMD_AFTER_LAST
}

//
// enum nfc_attrs - supported nfc attributes
//
// @NFC_ATTR_UNSPEC: unspecified attribute
//
// @NFC_ATTR_DEVICE_INDEX: index of nfc device
// @NFC_ATTR_DEVICE_NAME: device name, max 8 chars
// @NFC_ATTR_PROTOCOLS: nfc protocols - bitwise or-ed combination from
// NFC_PROTO_*_MASK constants
// @NFC_ATTR_TARGET_INDEX: index of the nfc target
// @NFC_ATTR_TARGET_SENS_RES: NFC-A targets extra information such as NFCID
// @NFC_ATTR_TARGET_SEL_RES: NFC-A targets extra information (useful if the
// target is not NFC-Forum compliant)
// @NFC_ATTR_TARGET_NFCID1: NFC-A targets identifier, max 10 bytes
// @NFC_ATTR_TARGET_SENSB_RES: NFC-B targets extra information, max 12 bytes
// @NFC_ATTR_TARGET_SENSF_RES: NFC-F targets extra information, max 18 bytes
// @NFC_ATTR_COMM_MODE: Passive or active mode
// @NFC_ATTR_RF_MODE: Initiator or target
// @NFC_ATTR_IM_PROTOCOLS: Initiator mode protocols to poll for
// @NFC_ATTR_TM_PROTOCOLS: Target mode protocols to listen for
// @NFC_ATTR_LLC_PARAM_LTO: Link TimeOut parameter
// @NFC_ATTR_LLC_PARAM_RW: Receive Window size parameter
// @NFC_ATTR_LLC_PARAM_MIUX: MIU eXtension parameter
// @NFC_ATTR_SE: Available Secure Elements
// @NFC_ATTR_FIRMWARE_NAME: Free format firmware version
// @NFC_ATTR_SE_INDEX: Secure element index
// @NFC_ATTR_SE_TYPE: Secure element type (UICC or EMBEDDED)
// @NFC_ATTR_FIRMWARE_DOWNLOAD_STATUS: Firmware download operation status
// @NFC_ATTR_SE_APDU: Secure element APDU
// @NFC_ATTR_TARGET_ISO15693_DSFID: ISO 15693 Data Storage Format Identifier
// @NFC_ATTR_TARGET_ISO15693_UID: ISO 15693 Unique Identifier
// @NFC_ATTR_SE_PARAMS: Parameters data from an evt_transaction
// @NFC_ATTR_VENDOR_ID: NFC manufacturer unique ID, typically an OUI
// @NFC_ATTR_VENDOR_SUBCMD: Vendor specific sub command
// @NFC_ATTR_VENDOR_DATA: Vendor specific data, to be optionally passed
// to a vendor specific command implementation
// @NFC_ATTR_TARGET_ATS: ISO 14443 type A target Answer To Select
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfc_attrs {
    NFC_ATTR_UNSPEC,
    NFC_ATTR_DEVICE_INDEX,
    NFC_ATTR_DEVICE_NAME,
    NFC_ATTR_PROTOCOLS,
    NFC_ATTR_TARGET_INDEX,
    NFC_ATTR_TARGET_SENS_RES,
    NFC_ATTR_TARGET_SEL_RES,
    NFC_ATTR_TARGET_NFCID1,
    NFC_ATTR_TARGET_SENSB_RES,
    NFC_ATTR_TARGET_SENSF_RES,
    NFC_ATTR_COMM_MODE,
    NFC_ATTR_RF_MODE,
    NFC_ATTR_DEVICE_POWERED,
    NFC_ATTR_IM_PROTOCOLS,
    NFC_ATTR_TM_PROTOCOLS,
    NFC_ATTR_LLC_PARAM_LTO,
    NFC_ATTR_LLC_PARAM_RW,
    NFC_ATTR_LLC_PARAM_MIUX,
    NFC_ATTR_SE,
    NFC_ATTR_LLC_SDP,
    NFC_ATTR_FIRMWARE_NAME,
    NFC_ATTR_SE_INDEX,
    NFC_ATTR_SE_TYPE,
    NFC_ATTR_SE_AID,
    NFC_ATTR_FIRMWARE_DOWNLOAD_STATUS,
    NFC_ATTR_SE_APDU,
    NFC_ATTR_TARGET_ISO15693_DSFID,
    NFC_ATTR_TARGET_ISO15693_UID,
    NFC_ATTR_SE_PARAMS,
    NFC_ATTR_VENDOR_ID,
    NFC_ATTR_VENDOR_SUBCMD,
    NFC_ATTR_VENDOR_DATA,
    NFC_ATTR_TARGET_ATS,
// private: internal use only
    __NFC_ATTR_AFTER_LAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfc_sdp_attr {
    NFC_SDP_ATTR_UNSPEC,
    NFC_SDP_ATTR_URI,
    NFC_SDP_ATTR_SAP,
// private: internal use only
    __NFC_SDP_ATTR_AFTER_LAST
}

pub const NFC_DEVICE_NAME_MAXSIZE: c_int = 8;
pub const NFC_NFCID1_MAXSIZE: c_int = 10;
pub const NFC_NFCID2_MAXSIZE: c_int = 8;
pub const NFC_NFCID3_MAXSIZE: c_int = 10;
pub const NFC_SENSB_RES_MAXSIZE: c_int = 12;
pub const NFC_SENSF_RES_MAXSIZE: c_int = 18;
pub const NFC_ATR_REQ_MAXSIZE: c_int = 64;
pub const NFC_ATR_RES_MAXSIZE: c_int = 64;
pub const NFC_ATR_REQ_GB_MAXSIZE: c_int = 48;
pub const NFC_ATR_RES_GB_MAXSIZE: c_int = 47;
pub const NFC_GB_MAXSIZE: c_int = 48;
pub const NFC_FIRMWARE_NAME_MAXSIZE: c_int = 32;
pub const NFC_ISO15693_UID_MAXSIZE: c_int = 8;
pub const NFC_ATS_MAXSIZE: c_int = 20;
// NFC protocols
pub const NFC_PROTO_JEWEL: c_int = 1;
pub const NFC_PROTO_MIFARE: c_int = 2;
pub const NFC_PROTO_FELICA: c_int = 3;
pub const NFC_PROTO_ISO14443: c_int = 4;
pub const NFC_PROTO_NFC_DEP: c_int = 5;
pub const NFC_PROTO_ISO14443_B: c_int = 6;
pub const NFC_PROTO_ISO15693: c_int = 7;
pub const NFC_PROTO_MAX: c_int = 8;
// NFC communication modes
pub const NFC_COMM_ACTIVE: c_int = 0;
pub const NFC_COMM_PASSIVE: c_int = 1;
// NFC RF modes
pub const NFC_RF_INITIATOR: c_int = 0;
pub const NFC_RF_TARGET: c_int = 1;
pub const NFC_RF_NONE: c_int = 2;
// NFC protocols masks used in bitsets

// NFC Secure Elements
pub const NFC_SE_UICC: c_uint = 0x1;
pub const NFC_SE_EMBEDDED: c_uint = 0x2;
pub const NFC_SE_DISABLED: c_uint = 0x0;
pub const NFC_SE_ENABLED: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_nfc {
    pub sa_family: __kernel_sa_family_t,
    pub dev_idx: __u32,
    pub target_idx: __u32,
    pub nfc_protocol: __u32,
}

pub const NFC_LLCP_MAX_SERVICE_NAME: c_int = 63;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_nfc_llcp {
    pub sa_family: __kernel_sa_family_t,
    pub dev_idx: __u32,
    pub target_idx: __u32,
    pub nfc_protocol: __u32,
    pub /: *mut *mut __u8 dsap; / Destination SAP, if known,
    pub /: *mut *mut __u8 ssap; / Source SAP to be bound to,
    pub /: *mut *mut char service_name[NFC_LLCP_MAX_SERVICE_NAME]; / Service name URI,
    pub service_name_len: __kernel_size_t,
}

// NFC socket protocols
pub const NFC_SOCKPROTO_RAW: c_int = 0;
pub const NFC_SOCKPROTO_LLCP: c_int = 1;
pub const NFC_SOCKPROTO_MAX: c_int = 2;
pub const NFC_HEADER_SIZE: c_int = 1;
//
// Pseudo-header info for raw socket packets
// First byte is the adapter index
// Second byte contains flags
// - 0x01 - Direction (0=RX, 1=TX)
// - 0x02-0x04 - Payload type (000=LLCP, 001=NCI, 010=HCI, 011=Digital,
// 100=Proprietary)
// - 0x05-0x80 - Reserved
//
pub const NFC_RAW_HEADER_SIZE: c_int = 2;
pub const NFC_DIRECTION_RX: c_uint = 0x00;
pub const NFC_DIRECTION_TX: c_uint = 0x01;
pub const RAW_PAYLOAD_LLCP: c_int = 0;
pub const RAW_PAYLOAD_NCI: c_int = 1;
pub const RAW_PAYLOAD_HCI: c_int = 2;
pub const RAW_PAYLOAD_DIGITAL: c_int = 3;
pub const RAW_PAYLOAD_PROPRIETARY: c_int = 4;
// socket option names
pub const NFC_LLCP_RW: c_int = 0;
pub const NFC_LLCP_MIUX: c_int = 1;
pub const NFC_LLCP_REMOTE_MIU: c_int = 2;
pub const NFC_LLCP_REMOTE_LTO: c_int = 3;
pub const NFC_LLCP_REMOTE_RW: c_int = 4;
