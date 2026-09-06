//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/fweh.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2012 Broadcom Corporation
//

// formward declarations

// list of firmware events

// firmware event codes sent by the dongle
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_fweh_event_code {
    BRCMF_FWEH_EVENT_ENUM_DEFLIST
}

// flags field values in struct brcmf_event_msg
pub const BRCMF_EVENT_MSG_LINK: c_uint = 0x01;
pub const BRCMF_EVENT_MSG_FLUSHTXQ: c_uint = 0x02;
pub const BRCMF_EVENT_MSG_GROUP: c_uint = 0x04;
// status field values in struct brcmf_event_msg
pub const BRCMF_E_STATUS_SUCCESS: c_int = 0;
pub const BRCMF_E_STATUS_FAIL: c_int = 1;
pub const BRCMF_E_STATUS_TIMEOUT: c_int = 2;
pub const BRCMF_E_STATUS_NO_NETWORKS: c_int = 3;
pub const BRCMF_E_STATUS_ABORT: c_int = 4;
pub const BRCMF_E_STATUS_NO_ACK: c_int = 5;
pub const BRCMF_E_STATUS_UNSOLICITED: c_int = 6;
pub const BRCMF_E_STATUS_ATTEMPT: c_int = 7;
pub const BRCMF_E_STATUS_PARTIAL: c_int = 8;
pub const BRCMF_E_STATUS_NEWSCAN: c_int = 9;
pub const BRCMF_E_STATUS_NEWASSOC: c_int = 10;
pub const BRCMF_E_STATUS_11HQUIET: c_int = 11;
pub const BRCMF_E_STATUS_SUPPRESS: c_int = 12;
pub const BRCMF_E_STATUS_NOCHANS: c_int = 13;
pub const BRCMF_E_STATUS_CS_ABORT: c_int = 15;
pub const BRCMF_E_STATUS_ERROR: c_int = 16;
// status field values for PSK_SUP event
pub const BRCMF_E_STATUS_FWSUP_WAIT_M1: c_int = 4;
pub const BRCMF_E_STATUS_FWSUP_PREP_M2: c_int = 5;
pub const BRCMF_E_STATUS_FWSUP_COMPLETED: c_int = 6;
pub const BRCMF_E_STATUS_FWSUP_TIMEOUT: c_int = 7;
pub const BRCMF_E_STATUS_FWSUP_WAIT_M3: c_int = 8;
pub const BRCMF_E_STATUS_FWSUP_PREP_M4: c_int = 9;
pub const BRCMF_E_STATUS_FWSUP_WAIT_G1: c_int = 10;
pub const BRCMF_E_STATUS_FWSUP_PREP_G2: c_int = 11;
// reason field values in struct brcmf_event_msg
pub const BRCMF_E_REASON_INITIAL_ASSOC: c_int = 0;
pub const BRCMF_E_REASON_LOW_RSSI: c_int = 1;
pub const BRCMF_E_REASON_DEAUTH: c_int = 2;
pub const BRCMF_E_REASON_DISASSOC: c_int = 3;
pub const BRCMF_E_REASON_BCNS_LOST: c_int = 4;
pub const BRCMF_E_REASON_MINTXRATE: c_int = 9;
pub const BRCMF_E_REASON_TXFAIL: c_int = 10;
pub const BRCMF_E_REASON_LINK_BSSCFG_DIS: c_int = 4;
pub const BRCMF_E_REASON_FAST_ROAM_FAILED: c_int = 5;
pub const BRCMF_E_REASON_DIRECTED_ROAM: c_int = 6;
pub const BRCMF_E_REASON_TSPEC_REJECTED: c_int = 7;
pub const BRCMF_E_REASON_BETTER_AP: c_int = 8;
pub const BRCMF_E_REASON_TDLS_PEER_DISCOVERED: c_int = 0;
pub const BRCMF_E_REASON_TDLS_PEER_CONNECTED: c_int = 1;
pub const BRCMF_E_REASON_TDLS_PEER_DISCONNECTED: c_int = 2;
// reason field values for PSK_SUP event
pub const BRCMF_E_REASON_FWSUP_OTHER: c_int = 0;
pub const BRCMF_E_REASON_FWSUP_DECRYPT_KEY_DATA: c_int = 1;
pub const BRCMF_E_REASON_FWSUP_BAD_UCAST_WEP128: c_int = 2;
pub const BRCMF_E_REASON_FWSUP_BAD_UCAST_WEP40: c_int = 3;
pub const BRCMF_E_REASON_FWSUP_UNSUP_KEY_LEN: c_int = 4;
pub const BRCMF_E_REASON_FWSUP_PW_KEY_CIPHER: c_int = 5;
pub const BRCMF_E_REASON_FWSUP_MSG3_TOO_MANY_IE: c_int = 6;
pub const BRCMF_E_REASON_FWSUP_MSG3_IE_MISMATCH: c_int = 7;
pub const BRCMF_E_REASON_FWSUP_NO_INSTALL_FLAG: c_int = 8;
pub const BRCMF_E_REASON_FWSUP_MSG3_NO_GTK: c_int = 9;
pub const BRCMF_E_REASON_FWSUP_GRP_KEY_CIPHER: c_int = 10;
pub const BRCMF_E_REASON_FWSUP_GRP_MSG1_NO_GTK: c_int = 11;
pub const BRCMF_E_REASON_FWSUP_GTK_DECRYPT_FAIL: c_int = 12;
pub const BRCMF_E_REASON_FWSUP_SEND_FAIL: c_int = 13;
pub const BRCMF_E_REASON_FWSUP_DEAUTH: c_int = 14;
pub const BRCMF_E_REASON_FWSUP_WPA_PSK_TMO: c_int = 15;
pub const BRCMF_E_REASON_FWSUP_WPA_PSK_M1_TMO: c_int = 16;
pub const BRCMF_E_REASON_FWSUP_WPA_PSK_M3_TMO: c_int = 17;
// action field values for brcmf_ifevent
pub const BRCMF_E_IF_ADD: c_int = 1;
pub const BRCMF_E_IF_DEL: c_int = 2;
pub const BRCMF_E_IF_CHANGE: c_int = 3;
// flag field values for brcmf_ifevent
pub const BRCMF_E_IF_FLAG_NOIF: c_int = 1;
// role field values for brcmf_ifevent
pub const BRCMF_E_IF_ROLE_STA: c_int = 0;
pub const BRCMF_E_IF_ROLE_AP: c_int = 1;
pub const BRCMF_E_IF_ROLE_WDS: c_int = 2;
pub const BRCMF_E_IF_ROLE_P2P_GO: c_int = 3;
pub const BRCMF_E_IF_ROLE_P2P_CLIENT: c_int = 4;
//
// definitions for event packet validation.
//

pub const BCMILCP_BCM_SUBTYPE_EVENT: c_int = 1;
pub const BCMILCP_SUBTYPE_VENDOR_LONG: c_int = 32769;
//
// struct brcm_ethhdr - broadcom specific ether header.
//
// @subtype: subtype for this packet.
// @length: TODO: length of appended data.
// @version: version indication.
// @oui: OUI of this packet.
// @usr_subtype: subtype for this OUI.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcm_ethhdr {
    pub subtype: __be16,
    pub length: __be16,
    pub version: u8,
    pub oui: [u8; 3],
    pub usr_subtype: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_event_msg_be {
    pub version: __be16,
    pub flags: __be16,
    pub event_type: __be32,
    pub status: __be32,
    pub reason: __be32,
    pub auth_type: __be32,
    pub datalen: __be32,
    pub addr: [u8; ETH_ALEN],
    pub ifname: [c_char; IFNAMSIZ],
    pub ifidx: u8,
    pub bsscfgidx: u8,
    pub __packed: },
//
// struct brcmf_event - contents of broadcom event packet.
//
// @eth: standard ether header.
// @hdr: broadcom specific ether header.
// @msg: common part of the actual event message.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_event {
    pub eth: ethhdr,
    pub hdr: brcm_ethhdr,
    pub msg: brcmf_event_msg_be,
    pub __packed: },
//
// struct brcmf_event_msg - firmware event message.
//
// @version: version information.
// @flags: event flags.
// @event_code: firmware event code.
// @status: status information.
// @reason: reason code.
// @auth_type: authentication type.
// @datalen: length of event data buffer.
// @addr: ether address.
// @ifname: interface name.
// @ifidx: interface index.
// @bsscfgidx: bsscfg index.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_event_msg {
    pub version: u16,
    pub flags: u16,
    pub event_code: u32,
    pub status: u32,
    pub reason: u32,
    pub auth_type: i32,
    pub datalen: u32,
    pub addr: [u8; ETH_ALEN],
    pub ifname: [c_char; IFNAMSIZ],
    pub ifidx: u8,
    pub bsscfgidx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_if_event {
    pub ifidx: u8,
    pub action: u8,
    pub flags: u8,
    pub bsscfgidx: u8,
    pub role: u8,
}

//
// struct brcmf_fweh_event_map_item - fweh event and firmware event pair.
//
// @code: fweh event code as used by higher layers.
// @fwevt_code: firmware event code as used by firmware.
//
// This mapping is needed when a functionally identical event has a
// different numerical definition between vendors. When such mapping
// is needed the higher layer event code should not collide with the
// firmware event.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_fweh_event_map_item {
    pub code: brcmf_fweh_event_code,
    pub fwevt_code: u32,
}

//
// struct brcmf_fweh_event_map - mapping between firmware event and fweh event.
//
// @n_items: number of mapping items.
// @items: array of fweh event and firmware event pairs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_fweh_event_map {
    pub n_items: u32,
    pub __counted_by(n_items): brcmf_fweh_event_map_item items[],
}

//
// struct brcmf_fweh_info - firmware event handling information.
//
// @p2pdev_setup_ongoing: P2P device creation in progress.
// @event_work: event worker.
// @evt_q_lock: lock for event queue protection.
// @event_q: event queue.
// @event_mask_len: length of @event_mask used to enable firmware events.
// @event_mask: byte array used in 'event_msgs' iovar command.
// @event_map: mapping between fweh event and firmware event which
// may be provided by vendor-specific module for events that need
// mapping.
// @num_event_codes: number of firmware events supported by firmware which
// does a minimum length check for the @event_mask. This value is to
// be provided by vendor-specific module determining @event_mask_len
// and consequently the allocation size for @event_mask.
// @evt_handler: event handler registry indexed by firmware event code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_fweh_info {
    pub drvr: *mut brcmf_pub,
    pub p2pdev_setup_ongoing: bool,
    pub event_work: work_struct,
    pub evt_q_lock: spinlock_t,
    pub event_q: list_head,
    pub event_mask_len: c_uint,
    pub event_mask: *mut u8,
    pub event_map: *const brcmf_fweh_event_map,
    pub num_event_codes: c_uint,
    pub __counted_by(num_event_codes): brcmf_fweh_handler_t evt_handler[],
}

extern "C" {
    pub fn brcmf_fweh_attach(drvr: *mut brcmf_pub) -> c_int;
}
extern "C" {
    pub fn brcmf_fweh_detach(drvr: *mut brcmf_pub);
}
extern "C" {
    pub fn brcmf_fweh_activate_events(ifp: *mut brcmf_if) -> c_int;
}
extern "C" {
    pub fn brcmf_fweh_p2pdev_setup(ifp: *mut brcmf_if, ongoing: bool);
}
// only process events when protocol matches
// check subtype if needed
// check for BRCM oui match
// final match on usr_subtype
