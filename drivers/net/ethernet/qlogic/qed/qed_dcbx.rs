//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_dcbx.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

pub const DCBX_CONFIG_MAX_APP_PROTOCOL: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_mib_read_type {
    QED_DCBX_OPERATIONAL_MIB,
    QED_DCBX_REMOTE_MIB,
    QED_DCBX_LOCAL_MIB,
    QED_DCBX_REMOTE_LLDP_MIB,
    QED_DCBX_LOCAL_LLDP_MIB
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dcbx_app_data {
    pub /: *mut *mut bool enable; / DCB enabled,
    pub /: *mut *mut u8 update; / Update indication,
    pub /: *mut *mut u8 priority; / Priority,
    pub /: *mut *mut u8 tc; / Traffic Class,
    pub /: *mut *mut bool dont_add_vlan0; / Do not insert a vlan tag with id 0,
}

pub const QED_DCBX_VERSION_DISABLED: c_int = 0;
pub const QED_DCBX_VERSION_IEEE: c_int = 1;
pub const QED_DCBX_VERSION_CEE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dcbx_set {

    pub override_flags: u32,
    pub enabled: bool,
    pub config: qed_dcbx_admin_params,
    pub ver_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dcbx_results {
    pub dcbx_enabled: bool,
    pub pf_id: u8,
    pub arr: [qed_dcbx_app_data; DCBX_MAX_PROTOCOL_TYPE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dcbx_app_metadata {
    pub id: dcbx_protocol_type,
    pub name: *mut c_char,
    pub personality: qed_pci_personality,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dcbx_info {
    pub lldp_remote: [lldp_status_params_s; LLDP_MAX_LLDP_AGENTS],
    pub lldp_local: [lldp_config_params_s; LLDP_MAX_LLDP_AGENTS],
    pub local_admin: dcbx_local_params,
    pub results: qed_dcbx_results,
    pub operational: dcbx_mib,
    pub remote: dcbx_mib,
    pub set: qed_dcbx_set,
    pub get: qed_dcbx_get,
    pub dcbx_cap: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dcbx_mib_meta_data {
    pub lldp_local: *mut lldp_config_params_s,
    pub lldp_remote: *mut lldp_status_params_s,
    pub local_admin: *mut dcbx_local_params,
    pub mib: *mut dcbx_mib,
    pub size: usize,
    pub addr: u32,
}

// QED local interface routines
extern "C" {
    pub fn qed_dcbx_info_alloc(p_hwfn: *mut qed_hwfn) -> c_int;
}
extern "C" {
    pub fn qed_dcbx_info_free(p_hwfn: *mut qed_hwfn);
}
pub const QED_DCBX_DEFAULT_TC: c_int = 0;
extern "C" {
    pub fn qed_dcbx_get_priority_tc(p_hwfn: *mut qed_hwfn, pri: u8) -> u8;
}
