//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnx2x/bnx2x_dcb.h
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


// bnx2x_dcb.h: QLogic Everest network driver.
//
// Copyright 2009-2013 Broadcom Corporation
// Copyright 2014 QLogic Corporation
// All rights reserved
//
// Unless you and QLogic execute a separate written software license
// agreement governing use of this software, this software is licensed to you
// under the terms of the GNU General Public License version 2, available
// at http://www.gnu.org/licenses/old-licenses/gpl-2.0.html (the "GPL").
//
// Notwithstanding the above, under no circumstances may you combine this
// software in any way with any other QLogic software provided under a
// license other than the GPL, without QLogic's express prior written
// consent.
//
// Maintained by: Ariel Elior <ariel.elior@qlogic.com>
// Written by: Dmitry Kravkov
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_dcbx_app_params {
    pub enabled: u32,
    pub traffic_type_priority: [u32; LLFC_DRIVER_TRAFFIC_TYPE_MAX],
}

// bnx2x currently limits numbers of supported COSes to 3 to be extended to 6
pub const BNX2X_MAX_COS_SUPPORT: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_dcbx_cos_params {
    pub bw_tbl: u32,
    pub pri_bitmask: u32,
//
// strict priority: valid values are 0..5; 0 is highest priority.
// There can't be two COSes with the same priority.
//
    pub strict: u8,

pub const BNX2X_DCBX_STRICT_COS_HIGHEST: c_int = 0;

    pub pauseable: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_dcbx_pg_params {
    pub enabled: u32,
    pub /: *mut *mut u8 num_of_cos; / valid COS entries,
    pub cos_params: [bnx2x_dcbx_cos_params; DCBX_COS_MAX_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_dcbx_pfc_params {
    pub enabled: u32,
    pub priority_non_pauseable_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_dcbx_port_params {
    pub pfc: bnx2x_dcbx_pfc_params,
    pub ets: bnx2x_dcbx_pg_params,
    pub app: bnx2x_dcbx_app_params,
}

pub const BNX2X_DCBX_OVERWRITE_SETTINGS_DISABLE: c_int = 0;
pub const BNX2X_DCBX_OVERWRITE_SETTINGS_ENABLE: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_config_lldp_params {
    pub overwrite_settings: u32,
    pub msg_tx_hold: u32,
    pub msg_fast_tx: u32,
    pub tx_credit_max: u32,
    pub msg_tx_interval: u32,
    pub tx_fast: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_admin_priority_app_table {
    pub valid: u32,
    pub priority: u32,

    pub traffic_type: u32,
pub const TRAFFIC_TYPE_ETH: c_int = 0;
pub const TRAFFIC_TYPE_PORT: c_int = 1;
    pub app_id: u32,
}

pub const DCBX_CONFIG_MAX_APP_PROTOCOL: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_config_dcbx_params {
    pub overwrite_settings: u32,
    pub admin_dcbx_version: u32,
    pub admin_ets_enable: u32,
    pub admin_pfc_enable: u32,
    pub admin_tc_supported_tx_enable: u32,
    pub admin_ets_configuration_tx_enable: u32,
    pub admin_ets_recommendation_tx_enable: u32,
    pub admin_pfc_tx_enable: u32,
    pub admin_application_priority_tx_enable: u32,
    pub admin_ets_willing: u32,
    pub admin_ets_reco_valid: u32,
    pub admin_pfc_willing: u32,
    pub admin_app_priority_willing: u32,
    pub admin_configuration_bw_precentage: [u32; 8],
    pub admin_configuration_ets_pg: [u32; 8],
    pub admin_recommendation_bw_precentage: [u32; 8],
    pub admin_recommendation_ets_pg: [u32; 8],
    pub admin_pfc_bitmap: u32,
    pub admin_default_priority: u32,
}

pub const PFC_BRB1_REG_HIGH_LLFC_LOW_THRESHOLD: c_int = 130;
pub const PFC_BRB1_REG_HIGH_LLFC_HIGH_THRESHOLD: c_int = 170;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cos_entry_help_data {
    pub pri_join_mask: u32,
    pub cos_bw: u32,
    pub strict: u8,
    pub pausable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cos_help_data {
    pub data: [cos_entry_help_data; DCBX_COS_MAX_NUM],
    pub num_of_cos: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pg_entry_help_data {
    pub num_of_dif_pri: u8,
    pub pg: u8,
    pub pg_priority: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pg_help_data {
    pub data: [pg_entry_help_data; LLFC_DRIVER_TRAFFIC_TYPE_MAX],
    pub num_of_pg: u8,
}

// forward DCB/PFC related declarations
extern "C" {
    pub fn bnx2x_dcbx_update(work: *mut work_struct);
}
extern "C" {
    pub fn bnx2x_dcbx_init_params(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_dcbx_set_state(bp: *mut bnx2x, dcb_on: bool, dcbx_enabled: u32);
}
extern "C" {
    pub fn bnx2x_dcbx_set_params(bp: *mut bnx2x, state: u32);
}
extern "C" {
    pub fn bnx2x_dcbx_pmf_update(bp: *mut bnx2x);
}
// DCB netlink

extern "C" {
    pub fn bnx2x_dcbnl_update_applist(bp: *mut bnx2x, delall: bool) -> c_int;
}

extern "C" {
    pub fn bnx2x_dcbx_stop_hw_tx(bp: *mut bnx2x) -> c_int;
}
extern "C" {
    pub fn bnx2x_dcbx_resume_hw_tx(bp: *mut bnx2x) -> c_int;
}
