//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/pno.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2016 Broadcom
//
pub const BRCMF_PNO_SCAN_COMPLETE: c_int = 1;
pub const BRCMF_PNO_MAX_PFN_COUNT: c_int = 16;
pub const BRCMF_PNO_SCHED_SCAN_MIN_PERIOD: c_int = 10;
pub const BRCMF_PNO_SCHED_SCAN_MAX_PERIOD: c_int = 508;
// forward declaration
//
// brcmf_pno_start_sched_scan - initiate scheduled scan on device.
//
// @ifp: interface object used.
// @req: configuration parameters for scheduled scan.
//
// brcmf_pno_stop_sched_scan - terminate scheduled scan on device.
//
// @ifp: interface object used.
// @reqid: unique identifier of scan to be stopped.
//
extern "C" {
    pub fn brcmf_pno_stop_sched_scan(ifp: *mut brcmf_if, reqid: u64) -> c_int;
}
//
// brcmf_pno_wiphy_params - fill scheduled scan parameters in wiphy instance.
//
// @wiphy: wiphy instance to be used.
// @gscan: indicates whether the device has support for g-scan feature.
//
extern "C" {
    pub fn brcmf_pno_wiphy_params(wiphy: *mut wiphy, gscan: bool);
}
//
// brcmf_pno_attach - allocate and attach module information.
//
// @cfg: cfg80211 context used.
//
extern "C" {
    pub fn brcmf_pno_attach(cfg: *mut brcmf_cfg80211_info) -> c_int;
}
//
// brcmf_pno_detach - detach and free module information.
//
// @cfg: cfg80211 context used.
//
extern "C" {
    pub fn brcmf_pno_detach(cfg: *mut brcmf_cfg80211_info);
}
//
// brcmf_pno_find_reqid_by_bucket - find request id for given bucket index.
//
// @pi: pno instance used.
// @bucket: index of firmware bucket.
//
extern "C" {
    pub fn brcmf_pno_find_reqid_by_bucket(pi: *mut brcmf_pno_info, bucket: u32) -> u64;
}
//
// brcmf_pno_get_bucket_map - determine bucket map for given netinfo.
//
// @pi: pno instance used.
// @netinfo: netinfo to compare with bucket configuration.
//
