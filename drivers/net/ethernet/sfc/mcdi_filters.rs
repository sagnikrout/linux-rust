//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/mcdi_filters.h
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
// Driver for Solarflare network controllers and boards
// Copyright 2019 Solarflare Communications Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//

pub const EFX_EF10_FILTER_DEV_UC_MAX: c_int = 32;
pub const EFX_EF10_FILTER_DEV_MC_MAX: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efx_mcdi_filter_default_filters {
    EFX_EF10_BCAST,
    EFX_EF10_UCDEF,
    EFX_EF10_MCDEF,
    EFX_EF10_VXLAN4_UCDEF,
    EFX_EF10_VXLAN4_MCDEF,
    EFX_EF10_VXLAN6_UCDEF,
    EFX_EF10_VXLAN6_MCDEF,
    EFX_EF10_NVGRE4_UCDEF,
    EFX_EF10_NVGRE4_MCDEF,
    EFX_EF10_NVGRE6_UCDEF,
    EFX_EF10_NVGRE6_MCDEF,
    EFX_EF10_GENEVE4_UCDEF,
    EFX_EF10_GENEVE4_MCDEF,
    EFX_EF10_GENEVE6_UCDEF,
    EFX_EF10_GENEVE6_MCDEF,

    EFX_EF10_NUM_DEFAULT_FILTERS
}

// Per-VLAN filters information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_mcdi_filter_vlan {
    pub list: list_head,
    pub vid: u16,
    pub uc: [u16; EFX_EF10_FILTER_DEV_UC_MAX],
    pub mc: [u16; EFX_EF10_FILTER_DEV_MC_MAX],
    pub default_filters: [u16; EFX_EF10_NUM_DEFAULT_FILTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_mcdi_dev_addr {
    pub addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_mcdi_filter_table {
// The MCDI match masks supported by this fw & hw, in order of priority
    pub 2]: *mut *mut MC_CMD_GET_PARSER_DISP_INFO_OUT_SUPPORTED_MATCHES_MAXNUM,
    pub rx_match_count: c_uint,
// Our RSS context is exclusive (as opposed to shared)
    pub rx_rss_context_exclusive: bool,
    pub /: *mut *mut rw_semaphore lock; / Protects entries,
    pub /: *mut *mut unsigned long spec; / pointer to spec plus flag bits,
// AUTO_OLD is used to mark and sweep MAC filters for the device address lists.
// unused flag	1UL

    pub /: *mut *mut u64 handle; / firmware handle,
    pub entry: *mut },
// Shadow of net_device address lists, guarded by mac_lock
    pub dev_uc_list: [efx_mcdi_dev_addr; EFX_EF10_FILTER_DEV_UC_MAX],
    pub dev_mc_list: [efx_mcdi_dev_addr; EFX_EF10_FILTER_DEV_MC_MAX],
    pub dev_uc_count: c_int,
    pub dev_mc_count: c_int,
    pub uc_promisc: bool,
    pub mc_promisc: bool,
// Whether in multicast promiscuous mode when last changed
    pub mc_promisc_last: bool,
    pub /: *mut *mut bool mc_overflow; / Too many MC addrs; should always imply mc_promisc,
// RSS contexts have yet to be restored after MC reboot
    pub must_restore_rss_contexts: bool,
// filters have yet to be restored after MC reboot
    pub must_restore_filters: bool,
// Multicast filter chaining allows less-specific filters to receive
// multicast packets that matched more-specific filters.  Early EF10
// firmware didn't support this (SF bug 26807); if mc_chaining == false
// then we still subscribe the dev_mc_list even when mc_promisc to
// prevent another VI stealing the traffic.
//
    pub mc_chaining: bool,
    pub vlan_filter: bool,
// Entries on the vlan_list are added/removed under filter_sem
    pub vlan_list: list_head,
}

extern "C" {
    pub fn efx_mcdi_filter_table_probe(efx: *mut efx_nic, multicast_chaining: bool) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_filter_table_down(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_mcdi_filter_table_remove(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_mcdi_filter_table_restore(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_mcdi_filter_table_reset_mc_allocations(efx: *mut efx_nic);
}
//
// The filter table(s) are managed by firmware and we have write-only
// access.  When removing filters we must identify them to the
// firmware by a 64-bit handle, but this is too wide for Linux kernel
// interfaces (32-bit for RX NFC, 16-bit for RFS).  Also, we need to
// be able to tell in advance whether a requested insertion will
// replace an existing filter.  Therefore we maintain a software hash
// table, which should be at least as large as the hardware hash
// table.
//
// Huntington has a single 8K filter table shared between all filter
// types and both ports.
//
pub const EFX_MCDI_FILTER_TBL_ROWS: c_int = 8192;
extern "C" {
    pub fn efx_mcdi_filter_sync_rx_mode(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_mcdi_filter_get_rx_id_limit(efx: *mut efx_nic) -> u32;
}
extern "C" {
    pub fn efx_mcdi_filter_cleanup_vlans(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_mcdi_filter_add_vlan(efx: *mut efx_nic, vid: u16) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_filter_del_vlan(efx: *mut efx_nic, vid: u16);
}
extern "C" {
    pub fn efx_mcdi_rx_free_indir_table(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_mcdi_rx_pull_rss_config(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_rx_restore_rss_contexts(efx: *mut efx_nic);
}
// no need to do anything here
