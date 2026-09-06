//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ieee80211-s1g.h
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
// IEEE 802.11 S1G definitions
//
// Copyright (c) 2001-2002, SSH Communications Security Corp and Jouni Malinen
// <jkmaline@cc.hut.fi>
// Copyright (c) 2002-2003, Jouni Malinen <jkmaline@cc.hut.fi>
// Copyright (c) 2005, Devicescape Software, Inc.
// Copyright (c) 2006, Michael Wu <flamingice@sourmilk.net>
// Copyright (c) 2013 - 2014 Intel Mobile Communications GmbH
// Copyright (c) 2016 - 2017 Intel Deutschland GmbH
// Copyright (c) 2018 - 2025 Intel Corporation
//

// bits unique to S1G beacon frame control
pub const IEEE80211_S1G_BCN_NEXT_TBTT: c_uint = 0x100;
pub const IEEE80211_S1G_BCN_CSSID: c_uint = 0x200;
pub const IEEE80211_S1G_BCN_ANO: c_uint = 0x400;
// see 802.11ah-2016 9.9 NDP CMAC frames
pub const IEEE80211_S1G_1MHZ_NDP_BITS: c_int = 25;
pub const IEEE80211_S1G_1MHZ_NDP_BYTES: c_int = 4;
pub const IEEE80211_S1G_2MHZ_NDP_BITS: c_int = 37;
pub const IEEE80211_S1G_2MHZ_NDP_BYTES: c_int = 5;
//
// ieee80211_is_s1g_beacon - check if IEEE80211_FTYPE_EXT &&
// IEEE80211_STYPE_S1G_BEACON
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is an S1G beacon
//
// ieee80211_s1g_has_next_tbtt - check if IEEE80211_S1G_BCN_NEXT_TBTT
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame contains the variable-length
// next TBTT field
//
// ieee80211_s1g_has_ano - check if IEEE80211_S1G_BCN_ANO
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame contains the variable-length
// ANO field
//
// ieee80211_s1g_has_cssid - check if IEEE80211_S1G_BCN_CSSID
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame contains the variable-length
// compressed SSID field
//
// enum ieee80211_s1g_chanwidth - S1G channel widths
// These are defined in IEEE802.11-2016ah Table 10-20
// as BSS Channel Width
//
// @IEEE80211_S1G_CHANWIDTH_1MHZ: 1MHz operating channel
// @IEEE80211_S1G_CHANWIDTH_2MHZ: 2MHz operating channel
// @IEEE80211_S1G_CHANWIDTH_4MHZ: 4MHz operating channel
// @IEEE80211_S1G_CHANWIDTH_8MHZ: 8MHz operating channel
// @IEEE80211_S1G_CHANWIDTH_16MHZ: 16MHz operating channel
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_s1g_chanwidth {
    IEEE80211_S1G_CHANWIDTH_1MHZ = 0,
    IEEE80211_S1G_CHANWIDTH_2MHZ = 1,
    IEEE80211_S1G_CHANWIDTH_4MHZ = 3,
    IEEE80211_S1G_CHANWIDTH_8MHZ = 7,
    IEEE80211_S1G_CHANWIDTH_16MHZ = 15,
}

//
// enum ieee80211_s1g_pri_chanwidth - S1G primary channel widths
// described in IEEE80211-2024 Table 10-39.
//
// @IEEE80211_S1G_PRI_CHANWIDTH_2MHZ: 2MHz primary channel
// @IEEE80211_S1G_PRI_CHANWIDTH_1MHZ: 1MHz primary channel
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_s1g_pri_chanwidth {
    IEEE80211_S1G_PRI_CHANWIDTH_2MHZ = 0,
    IEEE80211_S1G_PRI_CHANWIDTH_1MHZ = 1,
}

//
// struct ieee80211_s1g_bcn_compat_ie - S1G Beacon Compatibility element
// @compat_info: Compatibility Information
// @beacon_int: Beacon Interval
// @tsf_completion: TSF Completion
//
// This structure represents the payload of the "S1G Beacon
// Compatibility element" as described in IEEE Std 802.11-2020 section
// 9.4.2.196.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_s1g_bcn_compat_ie {
    pub compat_info: __le16,
    pub beacon_int: __le16,
    pub tsf_completion: __le32,
    pub __packed: },
//
// struct ieee80211_s1g_oper_ie - S1G Operation element
// @ch_width: S1G Operation Information Channel Width
// @oper_class: S1G Operation Information Operating Class
// @primary_ch: S1G Operation Information Primary Channel Number
// @oper_ch: S1G Operation Information  Channel Center Frequency
// @basic_mcs_nss: Basic S1G-MCS and NSS Set
//
// This structure represents the payload of the "S1G Operation
// element" as described in IEEE Std 802.11-2020 section 9.4.2.212.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_s1g_oper_ie {
    pub ch_width: u8,
    pub oper_class: u8,
    pub primary_ch: u8,
    pub oper_ch: u8,
    pub basic_mcs_nss: __le16,
    pub __packed: },
//
// struct ieee80211_aid_response_ie - AID Response element
// @aid: AID/Group AID
// @switch_count: AID Switch Count
// @response_int: AID Response Interval
//
// This structure represents the payload of the "AID Response element"
// as described in IEEE Std 802.11-2020 section 9.4.2.194.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_aid_response_ie {
    pub aid: __le16,
    pub switch_count: u8,
    pub response_int: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_s1g_cap {
    pub capab_info: [u8; 10],
    pub supp_mcs_nss: [u8; 5],
    pub __packed: },
//
// ieee80211_s1g_optional_len - determine length of optional S1G beacon fields
// @fc: frame control bytes in little-endian byteorder
// Return: total length in bytes of the optional fixed-length fields
//
// S1G beacons may contain up to three optional fixed-length fields that
// precede the variable-length elements. Whether these fields are present
// is indicated by flags in the frame control field.
//
// From IEEE 802.11-2024 section 9.3.4.3:
// - Next TBTT field may be 0 or 3 bytes
// - Short SSID field may be 0 or 4 bytes
// - Access Network Options (ANO) field may be 0 or 1 byte
//
    pub 0: size_t len =,
    pub 3: len +=,
    pub 4: len +=,
    pub 1: len +=,
    pub len: return,
// S1G Capabilities Information field
pub const IEEE80211_S1G_CAPABILITY_LEN: c_int = 15;

pub const S1G_SUPP_CH_WIDTH_2: c_int = 0;
pub const S1G_SUPP_CH_WIDTH_4: c_int = 1;
pub const S1G_SUPP_CH_WIDTH_8: c_int = 2;
pub const S1G_SUPP_CH_WIDTH_16: c_int = 3;

pub const S1G_2M_PRIMARY_LOCATION_LOWER: c_int = 0;
pub const S1G_2M_PRIMARY_LOCATION_UPPER: c_int = 1;

// S1G encoding types
pub const IEEE80211_S1G_TIM_ENC_MODE_BLOCK: c_int = 0;
pub const IEEE80211_S1G_TIM_ENC_MODE_SINGLE: c_int = 1;
pub const IEEE80211_S1G_TIM_ENC_MODE_OLB: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_s1g_actioncode {
    WLAN_S1G_AID_SWITCH_REQUEST,
    WLAN_S1G_AID_SWITCH_RESPONSE,
    WLAN_S1G_SYNC_CONTROL,
    WLAN_S1G_STA_INFO_ANNOUNCE,
    WLAN_S1G_EDCA_PARAM_SET,
    WLAN_S1G_EL_OPERATION,
    WLAN_S1G_TWT_SETUP,
    WLAN_S1G_TWT_TEARDOWN,
    WLAN_S1G_SECT_GROUP_ID_LIST,
    WLAN_S1G_SECT_ID_FEEDBACK,
    WLAN_S1G_TWT_INFORMATION = 11,
}

//
// ieee80211_is_s1g_short_beacon - check if frame is an S1G short beacon
// @fc: frame control bytes in little-endian byteorder
// @variable: pointer to the beacon frame elements
// @variable_len: length of the frame elements
// Return: whether or not the frame is an S1G short beacon. As per
// IEEE80211-2024 11.1.3.10.1, The S1G beacon compatibility element shall
// always be present as the first element in beacon frames generated at a
// TBTT (Target Beacon Transmission Time), so any frame not containing
// this element must have been generated at a TSBTT (Target Short Beacon
// Transmission Time) that is not a TBTT. Additionally, short beacons are
// prohibited from containing the S1G beacon compatibility element as per
// IEEE80211-2024 9.3.4.3 Table 9-76, so if we have an S1G beacon with
// either no elements or the first element is not the beacon compatibility
// element, we have a short beacon.
//
    pub false: return,
//
// If the frame does not contain at least 1 element (this is perfectly
// valid in a short beacon) and is an S1G beacon, we have a short
// beacon.
//
    pub true: return,
    pub WLAN_EID_S1G_BCN_COMPAT: return variable[0] !=,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s1g_tim_aid {
    pub aid: u16,
    pub /: *mut *mut u8 target_blk; / Target block index,
    pub /: *mut *mut u8 target_subblk; / Target subblock index,
    pub /: *mut *mut u8 target_subblk_bit; / Target subblock bit,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s1g_tim_enc_block {
    pub enc_mode: u8,
    pub inverse: bool,
    pub ptr: *const u8,
    pub len: u8,
//
// For an OLB encoded block that spans multiple blocks, this
// is the offset into the span described by that encoded block.
//
    pub olb_blk_offset: u8,
}

//
// Helper routines to quickly extract the length of an encoded block. Validation
// is also performed to ensure the length extracted lies within the TIM.
//
// Enumerate all encoded blocks until we find the encoded block that describes
// our target AID. OLB is a special case as a single encoded block can describe
// multiple blocks as a single encoded block.
//
// need at least block-control octet
//
// An OLB encoded block can describe more then one
// block, meaning an encoded OLB block can span more
// then a single block.
//
// Minus one for the length octet
//
// Check if our target block lies within the
// block span described by this encoded block.
//
// If our block bitmap does not contain a set bit that corresponds
// to our AID, it could mean a variety of things depending on if
// the encoding mode is inverted or not.
//
// 1. If inverted, it means the entire subblock is present and hence
// our AID has been set.
// 2. If not inverted, it means our subblock is not present and hence
// it is all zero meaning our AID is not set.
//
// Increment ptr by the number of set subblocks that appear before our
// target subblock. If our target subblock is 0, do nothing as ptr
// already points to our target subblock.
//
// Single AID mode describes, as the name suggests, a single AID
// within the block described by the encoded block. The octet
// contains the 6 LSBs of the AID described in the block. The other
// 2 bits are reserved. When inversed, every single AID described
// by the current block have buffered traffic except for the AID
// described in the single AID octet.
//
// Given an OLB encoded block that describes multiple blocks,
// calculate the offset into the span. Then calculate the
// subblock location normally.
//
// An S1G PVB has 3 non optional encoding types, each that can be inverted.
// An S1G PVB is constructed with zero or more encoded block subfields. Each
// encoded block represents a single "block" of AIDs (64), and each encoded
// block can contain one of the 3 encoding types alongside a single bit for
// whether the bits should be inverted.
//
// As the standard makes no guarantee about the ordering of encoded blocks,
// we must parse every encoded block in the worst case scenario given an
// AID that lies within the last block.
//
// Find our AIDs target encoded block and fill &enc_blk with the
// encoded blocks information. If no entry is found or an error
// occurs return false.
//
extern "C" {
    pub fn ieee80211_s1g_parse_bitmap(_arg: &enc_blk, _arg: &target_aid) -> return;
}
extern "C" {
    pub fn ieee80211_s1g_parse_single(_arg: &enc_blk, _arg: &target_aid) -> return;
}
extern "C" {
    pub fn ieee80211_s1g_parse_olb(_arg: &enc_blk, _arg: &target_aid) -> return;
}
