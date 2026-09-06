//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/cpsw_ale.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Texas Instruments N-Port Ethernet Switch Address Lookup Engine APIs
//
// Copyright (C) 2012 Texas Instruments
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpsw_ale_params {
    pub dev: *mut device,
    pub ale_regs: *mut void __iomem,
    pub /: *mut *mut unsigned long ale_ageout; / in secs,
    pub ale_entries: c_ulong,
    pub num_policers: c_ulong,
    pub ale_ports: c_ulong,
// NU Switch has specific handling as number of bits in ALE entries
// are different than other versions of ALE. Also there are specific
// registers for unknown vlan specific fields. So use nu_switch_ale
// to identify this hardware.
//
    pub nu_switch_ale: bool,
    pub reg_fields: *const reg_field,
    pub num_fields: c_int,
    pub dev_id: *const c_char,
    pub bus_freq: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ale_fields {
    MINOR_VER,
    MAJOR_VER,
    ALE_ENTRIES,
    ALE_POLICERS,
    POL_PORT_MEN,
    POL_TRUNK_ID,
    POL_PORT_NUM,
    POL_PRI_MEN,
    POL_PRI_VAL,
    POL_OUI_MEN,
    POL_OUI_INDEX,
    POL_DST_MEN,
    POL_DST_INDEX,
    POL_SRC_MEN,
    POL_SRC_INDEX,
    POL_OVLAN_MEN,
    POL_OVLAN_INDEX,
    POL_IVLAN_MEN,
    POL_IVLAN_INDEX,
    POL_ETHERTYPE_MEN,
    POL_ETHERTYPE_INDEX,
    POL_IPSRC_MEN,
    POL_IPSRC_INDEX,
    POL_IPDST_MEN,
    POL_IPDST_INDEX,
    POL_EN,
    POL_RED_DROP_EN,
    POL_YELLOW_DROP_EN,
    POL_YELLOW_THRESH,
    POL_POL_MATCH_MODE,
    POL_PRIORITY_THREAD_EN,
    POL_MAC_ONLY_DEF_DIS,
    POL_TEST_CLR,
    POL_TEST_CLR_RED,
    POL_TEST_CLR_YELLOW,
    POL_TEST_CLR_SELECTED,
    POL_TEST_ENTRY,
    POL_STATUS_HIT,
    POL_STATUS_HIT_RED,
    POL_STATUS_HIT_YELLOW,
    ALE_DEFAULT_THREAD_EN,
    ALE_DEFAULT_THREAD_VAL,
    ALE_THREAD_CLASS_INDEX,
    ALE_THREAD_ENABLE,
    ALE_THREAD_VALUE,
// terminator
    ALE_FIELDS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpsw_ale {
    pub params: cpsw_ale_params,
    pub timer: timer_list,
    pub regmap: *mut regmap,
    pub fields: [*mut regmap_field; ALE_FIELDS_MAX],
    pub ageout: c_ulong,
    pub version: u32,
    pub features: u32,
// These bits are different on NetCP NU Switch ALE
    pub port_mask_bits: u32,
    pub port_num_bits: u32,
    pub vlan_field_bits: u32,
    pub p0_untag_vid_mask: *mut c_ulong,
    pub vlan_entry_tbl: *const ale_entry_fld,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpsw_ale_control {
// global
    ALE_ENABLE,
    ALE_CLEAR,
    ALE_AGEOUT,
    ALE_P0_UNI_FLOOD,
    ALE_VLAN_NOLEARN,
    ALE_NO_PORT_VLAN,
    ALE_OUI_DENY,
    ALE_BYPASS,
    ALE_RATE_LIMIT_TX,
    ALE_VLAN_AWARE,
    ALE_AUTH_ENABLE,
    ALE_RATE_LIMIT,
// port controls
    ALE_PORT_STATE,
    ALE_PORT_DROP_UNTAGGED,
    ALE_PORT_DROP_UNKNOWN_VLAN,
    ALE_PORT_NOLEARN,
    ALE_PORT_NO_SA_UPDATE,
    ALE_PORT_UNKNOWN_VLAN_MEMBER,
    ALE_PORT_UNKNOWN_MCAST_FLOOD,
    ALE_PORT_UNKNOWN_REG_MCAST_FLOOD,
    ALE_PORT_UNTAGGED_EGRESS,
    ALE_PORT_MACONLY,
    ALE_PORT_MACONLY_CAF,
    ALE_PORT_BCAST_LIMIT,
    ALE_PORT_MCAST_LIMIT,
    ALE_DEFAULT_THREAD_ID,
    ALE_DEFAULT_THREAD_ENABLE,
    ALE_NUM_CONTROLS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpsw_ale_port_state {
    ALE_PORT_STATE_DISABLE	= 0x00,
    ALE_PORT_STATE_BLOCK	= 0x01,
    ALE_PORT_STATE_LEARN	= 0x02,
    ALE_PORT_STATE_FORWARD	= 0x03,
}

// ALE unicast entry flags - passed into cpsw_ale_add_ucast()

pub const ALE_MCAST_FWD: c_int = 0;
pub const ALE_MCAST_BLOCK_LEARN_FWD: c_int = 1;
pub const ALE_MCAST_FWD_LEARN: c_int = 2;
pub const ALE_MCAST_FWD_2: c_int = 3;
pub const ALE_ENTRY_BITS: c_int = 68;

extern "C" {
    pub fn cpsw_ale_start(ale: *mut cpsw_ale);
}
extern "C" {
    pub fn cpsw_ale_stop(ale: *mut cpsw_ale);
}
extern "C" {
    pub fn cpsw_ale_flush_multicast(ale: *mut cpsw_ale, port_mask: c_int, vid: c_int) -> c_int;
}
extern "C" {
    pub fn cpsw_ale_del_vlan(ale: *mut cpsw_ale, vid: u16, port: c_int) -> c_int;
}
extern "C" {
    pub fn cpsw_ale_set_allmulti(ale: *mut cpsw_ale, allmulti: c_int, port: c_int);
}
extern "C" {
    pub fn cpsw_ale_rx_ratelimit_bc(ale: *mut cpsw_ale, port: c_int, ratelimit_pps: c_uint) -> c_int;
}
extern "C" {
    pub fn cpsw_ale_rx_ratelimit_mc(ale: *mut cpsw_ale, port: c_int, ratelimit_pps: c_uint) -> c_int;
}
extern "C" {
    pub fn cpsw_ale_control_get(ale: *mut cpsw_ale, port: c_int, control: c_int) -> c_int;
}
extern "C" {
    pub fn cpsw_ale_dump(ale: *mut cpsw_ale, data: *mut u32);
}
extern "C" {
    pub fn cpsw_ale_restore(ale: *mut cpsw_ale, data: *mut u32);
}
extern "C" {
    pub fn cpsw_ale_get_num_entries(ale: *mut cpsw_ale) -> u32;
}
extern "C" {
    pub fn test_bit(_arg: vid, _arg: ale->p0_untag_vid_mask) -> return;
}
extern "C" {
    pub fn cpsw_ale_vlan_del_modify(ale: *mut cpsw_ale, vid: u16, port_mask: c_int) -> c_int;
}
extern "C" {
    pub fn cpsw_ale_classifier_setup_default(ale: *mut cpsw_ale, num_rx_ch: c_int);
}
