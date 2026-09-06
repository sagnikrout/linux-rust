//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/hw_atl2/hw_atl2_internal.h
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
// Atlantic Network Driver
// Copyright (C) 2020 Marvell International Ltd.
//

// interrupts

// hw_atl2 on-chip RX packet buffer available for data TCs

pub const HW_ATL2_INTR_MODER_MAX: c_uint = 0x1FF;
pub const HW_ATL2_INTR_MODER_MIN: c_uint = 0xFF;

pub const HW_ATL2_FW_SM_ACT_RSLVR: c_uint = 0x3U;
pub const HW_ATL2_RPF_TAG_UC_OFFSET: c_uint = 0x0;
pub const HW_ATL2_RPF_TAG_ALLMC_OFFSET: c_uint = 0x6;
pub const HW_ATL2_RPF_TAG_ET_OFFSET: c_uint = 0x7;
pub const HW_ATL2_RPF_TAG_VLAN_OFFSET: c_uint = 0xA;
pub const HW_ATL2_RPF_TAG_UNTAG_OFFSET: c_uint = 0xE;
pub const HW_ATL2_RPF_TAG_L3_V4_OFFSET: c_uint = 0xF;
pub const HW_ATL2_RPF_TAG_L3_V6_OFFSET: c_uint = 0x12;
pub const HW_ATL2_RPF_TAG_L4_OFFSET: c_uint = 0x15;
pub const HW_ATL2_RPF_TAG_L4_FLEX_OFFSET: c_uint = 0x18;
pub const HW_ATL2_RPF_TAG_FLEX_OFFSET: c_uint = 0x1B;
pub const HW_ATL2_RPF_TAG_PCP_OFFSET: c_uint = 0x1D;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HW_ATL2_RPF_ART_INDEX {
    HW_ATL2_RPF_L2_PROMISC_OFF_INDEX,
    HW_ATL2_RPF_VLAN_PROMISC_OFF_INDEX,
    HW_ATL2_RPF_L3L4_USER_INDEX	= 8,
    HW_ATL2_RPF_ET_PCP_USER_INDEX	= HW_ATL2_RPF_L3L4_USER_INDEX + 16,
    HW_ATL2_RPF_VLAN_USER_INDEX	= HW_ATL2_RPF_ET_PCP_USER_INDEX + 16,
    HW_ATL2_RPF_PCP_TO_TC_INDEX	= HW_ATL2_RPF_VLAN_USER_INDEX +
    HW_ATL_VLAN_MAX_FILTERS,
}

pub const HW_ATL2_ART_TOTAL_ENTRIES: c_int = 128;

pub const HW_ATL2_RPF_L3L4_FILTERS: c_int = 8;
pub const HW_ATL2_RPF_L3V4_FILTERS: c_int = 8;
pub const HW_ATL2_RPF_L3V6_FILTERS: c_int = 6;
pub const HW_ATL2_RPF_L4_FILTERS: c_int = 8;
pub const HW_ATL2_RPF_VLAN_FILTERS: c_int = 16;
pub const HW_ATL2_RPF_ETYPE_FILTERS: c_int = 16;
pub const HW_ATL2_RPF_ETYPE_TAGS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HW_ATL2_RPF_RSS_HASH_TYPE {
    HW_ATL2_RPF_RSS_HASH_TYPE_NONE = 0,
    HW_ATL2_RPF_RSS_HASH_TYPE_IPV4 = BIT(0),
    HW_ATL2_RPF_RSS_HASH_TYPE_IPV4_TCP = BIT(1),
    HW_ATL2_RPF_RSS_HASH_TYPE_IPV4_UDP = BIT(2),
    HW_ATL2_RPF_RSS_HASH_TYPE_IPV6 = BIT(3),
    HW_ATL2_RPF_RSS_HASH_TYPE_IPV6_TCP = BIT(4),
    HW_ATL2_RPF_RSS_HASH_TYPE_IPV6_UDP = BIT(5),
    HW_ATL2_RPF_RSS_HASH_TYPE_IPV6_EX = BIT(6),
    HW_ATL2_RPF_RSS_HASH_TYPE_IPV6_EX_TCP = BIT(7),
    HW_ATL2_RPF_RSS_HASH_TYPE_IPV6_EX_UDP = BIT(8),
    HW_ATL2_RPF_RSS_HASH_TYPE_ALL = HW_ATL2_RPF_RSS_HASH_TYPE_IPV4 |
    HW_ATL2_RPF_RSS_HASH_TYPE_IPV4_TCP |
    HW_ATL2_RPF_RSS_HASH_TYPE_IPV4_UDP |
    HW_ATL2_RPF_RSS_HASH_TYPE_IPV6 |
    HW_ATL2_RPF_RSS_HASH_TYPE_IPV6_TCP |
    HW_ATL2_RPF_RSS_HASH_TYPE_IPV6_UDP |
    HW_ATL2_RPF_RSS_HASH_TYPE_IPV6_EX |
    HW_ATL2_RPF_RSS_HASH_TYPE_IPV6_EX_TCP |
    HW_ATL2_RPF_RSS_HASH_TYPE_IPV6_EX_UDP,
}

pub const HW_ATL_MCAST_FLT_ANY_TO_HOST: c_uint = 0x00010FFFU;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_atl2_l3_filter {
    pub proto: u8,
    pub usage: u8,
    pub cmd: u32,
    pub srcip: [u32; 4],
    pub dstip: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_atl2_l4_filter {
    pub usage: u8,
    pub cmd: u32,
    pub sport: u16,
    pub dport: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_atl2_l3l4_filter {
    pub l3_index: i8,
    pub l4_index: i8,
    pub ipv6: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_atl2_tag_policy {
    pub action: u16,
    pub usage: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_atl2_priv {
    pub l3_v4_filters: [hw_atl2_l3_filter; HW_ATL2_RPF_L3L4_FILTERS],
    pub l3_v6_filters: [hw_atl2_l3_filter; HW_ATL2_RPF_L3L4_FILTERS],
    pub l4_filters: [hw_atl2_l4_filter; HW_ATL2_RPF_L3L4_FILTERS],
    pub l3l4_filters: [hw_atl2_l3l4_filter; HW_ATL2_RPF_L3L4_FILTERS],
    pub etype_policy: [hw_atl2_tag_policy; HW_ATL2_RPF_ETYPE_FILTERS],
    pub last_stats: statistics_s,
    pub art_base_index: c_uint,
    pub art_count: c_uint,
    pub l2_filters_base_index: c_uint,
    pub l2_filter_count: c_uint,
    pub etype_filter_base_index: c_uint,
    pub etype_filter_count: c_uint,
    pub etype_filter_tag_top: c_uint,
    pub vlan_filter_base_index: c_uint,
    pub vlan_filter_count: c_uint,
    pub l3_v4_filter_base_index: c_uint,
    pub l3_v4_filter_count: c_uint,
    pub l3_v6_filter_base_index: c_uint,
    pub l3_v6_filter_count: c_uint,
    pub l4_filter_base_index: c_uint,
    pub l4_filter_count: c_uint,
}
