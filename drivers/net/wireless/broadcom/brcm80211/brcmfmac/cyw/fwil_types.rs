//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/cyw/fwil_types.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_event_msgs_ext_command {
    CYW_EVENTMSGS_NONE	= 0,
    CYW_EVENTMSGS_SET_BIT	= 1,
    CYW_EVENTMSGS_RESET_BIT	= 2,
    CYW_EVENTMSGS_SET_MASK	= 3,
}

pub const EVENTMSGS_VER: c_int = 1;

//
// struct brcmf_eventmsgs_ext - structure used with "eventmsgs_ext" iovar.
//
// @ver: version.
// @command: requested operation (see &enum event_msgs_ext_command).
// @len: length of the @mask array.
// @maxgetsize: indicates maximum mask size that may be returned by firmware
// upon iovar GET.
// @mask: array where each bit represents firmware event.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_eventmsgs_ext {
    pub ver: u8,
    pub command: u8,
    pub len: u8,
    pub maxgetsize: u8,
    pub __counted_by(len): u8 mask[],
}

pub const BRCMF_EXTAUTH_START: c_int = 1;
pub const BRCMF_EXTAUTH_ABORT: c_int = 2;
pub const BRCMF_EXTAUTH_FAIL: c_int = 3;
pub const BRCMF_EXTAUTH_SUCCESS: c_int = 4;
//
// struct brcmf_auth_req_status_le - external auth request and status update
//
// @flags: flags for external auth status
// @peer_mac: peer MAC address
// @ssid_len: length of ssid
// @ssid: ssid characters
// @pmkid: PMKSA identifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_auth_req_status_le {
    pub flags: __le16,
    pub peer_mac: [u8; ETH_ALEN],
    pub ssid_len: __le32,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub pmkid: [u8; WLAN_PMKID_LEN],
}

//
// struct brcmf_mf_params_le - management frame parameters for mgmt_frame iovar
//
// @version: version of the iovar
// @dwell_time: dwell duration in ms
// @len: length of frame data
// @frame_control: frame control
// @channel: channel
// @da: peer MAC address
// @bssid: BSS network identifier
// @packet_id: packet identifier
// @data: frame data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_mf_params_le {
    pub version: __le32,
    pub dwell_time: __le32,
    pub len: __le16,
    pub frame_control: __le16,
    pub channel: __le16,
    pub da: [u8; ETH_ALEN],
    pub bssid: [u8; ETH_ALEN],
    pub packet_id: __le32,
    pub __counted_by_le(len): u8 data[],
}
