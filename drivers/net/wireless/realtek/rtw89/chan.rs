//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/chan.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright(c) 2020-2022  Realtek Corporation
//

// The dwell time in TU before doing rtw89_chanctx_work().
pub const RTW89_CHANCTX_TIME_MCC_PREPARE: c_int = 100;
pub const RTW89_CHANCTX_TIME_MCC: c_int = 100;
// various MCC setting time in TU
pub const RTW89_MCC_LONG_TRIGGER_TIME: c_int = 300;
pub const RTW89_MCC_SHORT_TRIGGER_TIME: c_int = 100;
pub const RTW89_MCC_EARLY_TX_BCN_TIME: c_int = 10;
pub const RTW89_MCC_EARLY_RX_BCN_TIME: c_int = 5;
pub const RTW89_MCC_MIN_RX_BCN_TIME: c_int = 10;
pub const RTW89_MCC_DFLT_BCN_OFST_TIME: c_int = 40;
pub const RTW89_MCC_SWITCH_CH_TIME: c_int = 3;
pub const RTW89_MCC_PROBE_TIMEOUT: c_int = 100;
pub const RTW89_MCC_PROBE_MAX_TRIES: c_int = 3;
pub const RTW89_MCC_DETECT_BCN_MAX_TRIES: c_int = 2;

pub const RTW89_MCC_DFLT_GROUP: c_int = 0;

pub const RTW89_MCC_DFLT_TX_NULL_EARLY: c_int = 7;
pub const RTW89_MCC_DFLT_COURTESY_SLOT: c_int = 3;
pub const RTW89_MCC_REQ_COURTESY_TIME: c_int = 5;

pub const NUM_OF_RTW89_MCC_ROLES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mr_wtype {
    RTW89_MR_WTYPE_NONE,
    RTW89_MR_WTYPE_NONMLD,
    RTW89_MR_WTYPE_MLD1L1R,
    RTW89_MR_WTYPE_MLD2L1R,
    RTW89_MR_WTYPE_MLD2L2R,
    RTW89_MR_WTYPE_NONMLD_NONMLD,
    RTW89_MR_WTYPE_MLD1L1R_NONMLD,
    RTW89_MR_WTYPE_MLD2L1R_NONMLD,
    RTW89_MR_WTYPE_MLD2L2R_NONMLD,
    RTW89_MR_WTYPE_UNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mr_wmode {
    RTW89_MR_WMODE_NONE,
    RTW89_MR_WMODE_1CLIENT,
    RTW89_MR_WMODE_1AP,
    RTW89_MR_WMODE_1AP_1CLIENT,
    RTW89_MR_WMODE_2CLIENTS,
    RTW89_MR_WMODE_2APS,
    RTW89_MR_WMODE_UNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mr_ctxtype {
    RTW89_MR_CTX_NONE,
    RTW89_MR_CTX1_2GHZ,
    RTW89_MR_CTX1_5GHZ,
    RTW89_MR_CTX1_6GHZ,
    RTW89_MR_CTX2_2GHZ,
    RTW89_MR_CTX2_5GHZ,
    RTW89_MR_CTX2_6GHZ,
    RTW89_MR_CTX2_2GHZ_5GHZ,
    RTW89_MR_CTX2_2GHZ_6GHZ,
    RTW89_MR_CTX2_5GHZ_6GHZ,
    RTW89_MR_CTX_UNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mr_chanctx_info {
    pub wtype: rtw89_mr_wtype,
    pub wmode: rtw89_mr_wmode,
    pub ctxtype: rtw89_mr_ctxtype,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_chanctx_pause_reasons {
    RTW89_CHANCTX_PAUSE_REASON_HW_SCAN,
    RTW89_CHANCTX_PAUSE_REASON_ROC,
    RTW89_CHANCTX_PAUSE_REASON_GC_BCN_LOSS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_chanctx_pause_parm {
    pub trigger: *const rtw89_vif_link,
    pub rsn: rtw89_chanctx_pause_reasons,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_chanctx_cb_parm {
    pub data): *mut *mut *mut int (cb)(struct rtw89_dev rtwdev, void,
    pub data: *mut c_void,
    pub caller: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_entity_weight {
    pub registered_chanctxs: c_uint,
    pub active_chanctxs: c_uint,
    pub active_roles: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_entity_conf {
    pub is_mld: bool,
    pub en_emlsr: bool,
    pub __RTW89_MLD_MAX_LINK_NUM): DECLARE_BITMAP(hw_bitmap,,
    pub chans: [*const rtw89_chan; __RTW89_MLD_MAX_LINK_NUM],
}

extern "C" {
    pub fn READ_ONCE(_arg: hal->entity_active[phy_idx]) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: hal->entity_mode) -> return;
}
extern "C" {
    pub fn rtw89_entity_init(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_entity_recalc(rtwdev: *mut rtw89_dev) -> rtw89_entity_mode;
}
extern "C" {
    pub fn rtw89_entity_check_hw(rtwdev: *mut rtw89_dev, phy_idx: rtw89_phy_idx) -> bool;
}
extern "C" {
    pub fn rtw89_entity_force_hw(rtwdev: *mut rtw89_dev, phy_idx: rtw89_phy_idx);
}
extern "C" {
    pub fn rtw89_entity_get_conf(rtwdev: *mut rtw89_dev, conf: *mut rtw89_entity_conf);
}
extern "C" {
    pub fn rtw89_chanctx_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn rtw89_queue_chanctx_work(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_chanctx_track(rtwdev: *mut rtw89_dev);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mcc_links_info {
    pub links: [*mut rtw89_vif_link; NUM_OF_RTW89_MCC_ROLES],
}

extern "C" {
    pub fn rtw89_mcc_get_links(rtwdev: *mut rtw89_dev, info: *mut rtw89_mcc_links_info);
}
extern "C" {
    pub fn rtw89_mcc_prepare_done_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn rtw89_mcc_gc_detect_beacon_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
