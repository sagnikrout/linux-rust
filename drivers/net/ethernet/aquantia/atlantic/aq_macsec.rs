//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/aq_macsec.h
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

pub const AQ_MACSEC_MAX_SC: c_int = 32;
pub const AQ_MACSEC_MAX_SA: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aq_macsec_sc_sa {
    aq_macsec_sa_sc_4sa_8sc,
    aq_macsec_sa_sc_not_used,
    aq_macsec_sa_sc_2sa_16sc,
    aq_macsec_sa_sc_1sa_32sc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_macsec_common_stats {
// Ingress Common Counters
    pub ctl_pkts: u64,
    pub tagged_miss_pkts: u64,
    pub untagged_miss_pkts: u64,
    pub notag_pkts: u64,
    pub untagged_pkts: u64,
    pub bad_tag_pkts: u64,
    pub no_sci_pkts: u64,
    pub unknown_sci_pkts: u64,
    pub ctrl_prt_pass_pkts: u64,
    pub unctrl_prt_pass_pkts: u64,
    pub ctrl_prt_fail_pkts: u64,
    pub unctrl_prt_fail_pkts: u64,
    pub too_long_pkts: u64,
    pub igpoc_ctl_pkts: u64,
    pub ecc_error_pkts: u64,
    pub unctrl_hit_drop_redir: u64,
    pub in: },
// Egress Common Counters
    pub ctl_pkts: u64,
    pub unknown_sa_pkts: u64,
    pub untagged_pkts: u64,
    pub too_long: u64,
    pub ecc_error_pkts: u64,
    pub unctrl_hit_drop_redir: u64,
    pub out: },
}

// Ingress SA Counters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_macsec_rx_sa_stats {
    pub untagged_hit_pkts: u64,
    pub ctrl_hit_drop_redir_pkts: u64,
    pub not_using_sa: u64,
    pub unused_sa: u64,
    pub not_valid_pkts: u64,
    pub invalid_pkts: u64,
    pub ok_pkts: u64,
    pub late_pkts: u64,
    pub delayed_pkts: u64,
    pub unchecked_pkts: u64,
    pub validated_octets: u64,
    pub decrypted_octets: u64,
}

// Egress SA Counters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_macsec_tx_sa_stats {
    pub sa_hit_drop_redirect: u64,
    pub sa_protected2_pkts: u64,
    pub sa_protected_pkts: u64,
    pub sa_encrypted_pkts: u64,
}

// Egress SC Counters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_macsec_tx_sc_stats {
    pub sc_protected_pkts: u64,
    pub sc_encrypted_pkts: u64,
    pub sc_protected_octets: u64,
    pub sc_encrypted_octets: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_macsec_txsc {
    pub hw_sc_idx: u32,
    pub tx_sa_idx_busy: c_ulong,
    pub sw_secy: *const macsec_secy,
    pub tx_sa_key: [u8; MACSEC_NUM_AN][MACSEC_MAX_KEY_LEN],
    pub stats: aq_macsec_tx_sc_stats,
    pub tx_sa_stats: [aq_macsec_tx_sa_stats; MACSEC_NUM_AN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_macsec_rxsc {
    pub hw_sc_idx: u32,
    pub rx_sa_idx_busy: c_ulong,
    pub sw_secy: *const macsec_secy,
    pub sw_rxsc: *const macsec_rx_sc,
    pub rx_sa_key: [u8; MACSEC_NUM_AN][MACSEC_MAX_KEY_LEN],
    pub rx_sa_stats: [aq_macsec_rx_sa_stats; MACSEC_NUM_AN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_macsec_cfg {
    pub sc_sa: aq_macsec_sc_sa,
// Egress channel configuration
    pub txsc_idx_busy: c_ulong,
    pub aq_txsc: [aq_macsec_txsc; AQ_MACSEC_MAX_SC],
// Ingress channel configuration
    pub rxsc_idx_busy: c_ulong,
    pub aq_rxsc: [aq_macsec_rxsc; AQ_MACSEC_MAX_SC],
// Statistics / counters
    pub stats: aq_macsec_common_stats,
}

extern "C" {
    pub fn aq_macsec_init(nic: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_macsec_free(nic: *mut aq_nic_s);
}
extern "C" {
    pub fn aq_macsec_enable(nic: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_macsec_work(nic: *mut aq_nic_s);
}
extern "C" {
    pub fn aq_macsec_rx_sa_cnt(nic: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_macsec_tx_sc_cnt(nic: *mut aq_nic_s) -> c_int;
}
extern "C" {
    pub fn aq_macsec_tx_sa_cnt(nic: *mut aq_nic_s) -> c_int;
}

