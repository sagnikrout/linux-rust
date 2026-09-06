//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnge/bnge_hwrm_lib.h
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
// Copyright (c) 2025 Broadcom

extern "C" {
    pub fn bnge_hwrm_ver_get(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_func_reset(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_fw_set_time(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_func_drv_rgtr(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_func_drv_unrgtr(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_vnic_qcaps(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_func_backing_store_qcaps(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_func_qcaps(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_vnic_qcaps(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_func_qcfg(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_func_resc_qcaps(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_queue_qportcfg(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_vnic_set_hds(bn: *mut bnge_net, vnic: *mut bnge_vnic_info) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_vnic_cfg(bn: *mut bnge_net, vnic: *mut bnge_vnic_info) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_update_rss_hash_cfg(bn: *mut bnge_net);
}
extern "C" {
    pub fn bnge_hwrm_vnic_free_one(bd: *mut bnge_dev, vnic: *mut bnge_vnic_info);
}
extern "C" {
    pub fn bnge_hwrm_l2_filter_free(bd: *mut bnge_dev, fltr: *mut bnge_l2_filter) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_l2_filter_alloc(bd: *mut bnge_dev, fltr: *mut bnge_l2_filter) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_stat_ctx_free(bn: *mut bnge_net);
}
extern "C" {
    pub fn bnge_hwrm_stat_ctx_alloc(bn: *mut bnge_net) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_set_async_event_cr(bd: *mut bnge_dev, idx: c_int) -> c_int;
}
extern "C" {
    pub fn bnge_update_link(bn: *mut bnge_net, chng_link_state: bool) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_phy_qcaps(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_set_link_setting(bn: *mut bnge_net, set_pause: bool) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_set_pause(bn: *mut bnge_net) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_shutdown_link(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_port_qstats(bd: *mut bnge_dev, flags: u8) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_port_qstats_ext(bd: *mut bnge_dev, flags: u8) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_func_qstat_ext(bd: *mut bnge_dev, stats: *mut bnge_stats_mem) -> c_int;
}
