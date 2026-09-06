//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ieee802154_netdev.h
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
// An interface between IEEE802.15.4 device and rest of the kernel.
//
// Copyright (C) 2007-2012 Siemens AG
//
// Written by:
// Pavel Smolenskiy <pavel.smolenskiy@gmail.com>
// Maxim Gorbachyov <maxim.gorbachev@siemens.com>
// Maxim Osipov <maxim.osipov@siemens.com>
// Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>
// Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_beacon_hdr {

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_mac_cmd_pl {
    pub cmd_id: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_sechdr {

    pub key_id: u8,
    pub frame_counter: __le32,
    pub short_src: __le32,
    pub extended_src: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_hdr_fc {

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_assoc_req_pl {

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_assoc_resp_pl {
    pub short_addr: __le16,
    pub status: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee802154_frame_version {
    IEEE802154_2003_STD,
    IEEE802154_2006_STD,
    IEEE802154_STD,
    IEEE802154_RESERVED_STD,
    IEEE802154_MULTIPURPOSE_STD = IEEE802154_2003_STD,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee802154_addressing_mode {
    IEEE802154_NO_ADDRESSING,
    IEEE802154_RESERVED,
    IEEE802154_SHORT_ADDRESSING,
    IEEE802154_EXTENDED_ADDRESSING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee802154_association_status {
    IEEE802154_ASSOCIATION_SUCCESSFUL = 0x00,
    IEEE802154_PAN_AT_CAPACITY = 0x01,
    IEEE802154_PAN_ACCESS_DENIED = 0x02,
    IEEE802154_HOPPING_SEQUENCE_OFFSET_DUP = 0x03,
    IEEE802154_FAST_ASSOCIATION_SUCCESSFUL = 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee802154_disassociation_reason {
    IEEE802154_COORD_WISHES_DEVICE_TO_LEAVE = 0x1,
    IEEE802154_DEVICE_WISHES_TO_LEAVE = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_hdr {
    pub fc: ieee802154_hdr_fc,
    pub seq: u8,
    pub source: ieee802154_addr,
    pub dest: ieee802154_addr,
    pub sec: ieee802154_sechdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_beacon_frame {
    pub mhr: ieee802154_hdr,
    pub mac_pl: ieee802154_beacon_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_mac_cmd_frame {
    pub mhr: ieee802154_hdr,
    pub mac_pl: ieee802154_mac_cmd_pl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_beacon_req_frame {
    pub mhr: ieee802154_hdr,
    pub mac_pl: ieee802154_mac_cmd_pl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_association_req_frame {
    pub mhr: ieee802154_hdr,
    pub mac_pl: ieee802154_mac_cmd_pl,
    pub assoc_req_pl: ieee802154_assoc_req_pl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_association_resp_frame {
    pub mhr: ieee802154_hdr,
    pub mac_pl: ieee802154_mac_cmd_pl,
    pub assoc_resp_pl: ieee802154_assoc_resp_pl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_disassociation_notif_frame {
    pub mhr: ieee802154_hdr,
    pub mac_pl: ieee802154_mac_cmd_pl,
    pub disassoc_pl: u8,
}

// pushes hdr onto the skb. fields of hdr->fc that can be calculated from
// the contents of hdr will be, and the actual value of those bits in
// hdr->fc will be ignored. this includes the INTRA_PAN bit and the frame
// version, if SECEN is set.
//
extern "C" {
    pub fn ieee802154_hdr_push(skb: *mut sk_buff, hdr: *mut ieee802154_hdr) -> c_int;
}
// pulls the entire 802.15.4 header off of the skb, including the security
// header, and performs pan id decompression
//
extern "C" {
    pub fn ieee802154_hdr_pull(skb: *mut sk_buff, hdr: *mut ieee802154_hdr) -> c_int;
}
// parses the frame control, sequence number of address fields in a given skb
// and stores them into hdr, performing pan id decompression and length checks
// to be suitable for use in header_ops.parse
//
// parses the full 802.15.4 header a given skb and stores them into hdr,
// performing pan id decompression and length checks to be suitable for use in
// header_ops.parse
//
extern "C" {
    pub fn ieee802154_hdr_peek(skb: *const sk_buff, hdr: *mut ieee802154_hdr) -> c_int;
}
// pushes/pulls various frame types into/from an skb
extern "C" {
    pub fn ieee802154_max_payload(hdr: *const ieee802154_hdr) -> c_int;
}
//
// A control block of skb passed between the ARPHRD_IEEE802154 device
// and other stack parts.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_mac_cb {
    pub lqi: u8,
    pub type: u8,
    pub ackreq: bool,
    pub secen: bool,
    pub secen_override: bool,
    pub seclevel: u8,
    pub seclevel_override: bool,
    pub source: ieee802154_addr,
    pub dest: ieee802154_addr,
}

extern "C" {
    pub fn mac_cb(_arg: skb) -> return;
}
pub const IEEE802154_MAC_SCAN_ED: c_int = 0;
pub const IEEE802154_MAC_SCAN_ACTIVE: c_int = 1;
pub const IEEE802154_MAC_SCAN_PASSIVE: c_int = 2;
pub const IEEE802154_MAC_SCAN_ORPHAN: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_mac_params {
    pub transmit_power: i8,
    pub min_be: u8,
    pub max_be: u8,
    pub csma_retries: u8,
    pub frame_retries: i8,
    pub lbt: bool,
    pub cca: wpan_phy_cca,
    pub cca_ed_level: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_llsec_ops {
    pub params): *mut ieee802154_llsec_params,
    pub changed): c_int,
    pub key): *const ieee802154_llsec_key,
    pub id): *const ieee802154_llsec_key_id,
    pub llsec_dev): *const ieee802154_llsec_device,
    pub dev_addr): *mut *mut *mut int (del_dev)(struct net_device dev, __le64,
    pub key): *const ieee802154_llsec_device_key,
    pub key): *const ieee802154_llsec_device_key,
    pub sl): *const ieee802154_llsec_seclevel,
    pub sl): *const ieee802154_llsec_seclevel,
    pub dev): *mut *mut void (lock_table)(struct net_device,
    pub t): *mut ieee802154_llsec_table,
    pub dev): *mut *mut void (unlock_table)(struct net_device,
}

//
// This should be located at net_device->ml_priv
//
// get_phy should increment the reference counting on returned phy.
// Use wpan_wpy_put to put that reference.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_mlme_ops {
// The following fields are optional (can be NULL).
    pub cap): u8 channel, u8 page, u8,
    pub status): __le16 short_addr, u8,
    pub reason): u8,
    pub coord_realign): u8 pan_coord, u8 blx, u8,
    pub duration): u8 type, u32 channels, u8 page, u8,
    pub params): *const ieee802154_mac_params,
    pub params): *mut ieee802154_mac_params,
    pub llsec: *const ieee802154_llsec_ops,
}
