//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/feature.h
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
// Copyright (c) 2014 Broadcom Corporation
//
// Features:
//
// MBSS: multiple BSSID support (eg. guest network in AP mode).
// MCHAN: multi-channel for concurrent P2P.
// PNO: preferred network offload.
// WOWL: Wake-On-WLAN.
// P2P: peer-to-peer
// RSDB: Real Simultaneous Dual Band
// TDLS: Tunneled Direct Link Setup
// SCAN_RANDOM_MAC: Random MAC during (net detect) scheduled scan.
// WOWL_ND: WOWL net detect (PNO)
// WOWL_GTK: (WOWL) GTK rekeying offload
// WOWL_ARP_ND: ARP and Neighbor Discovery offload support during WOWL.
// MFP: 802.11w Management Frame Protection.
// GSCAN: enhanced scan offload feature.
// FWSUP: Firmware supplicant.
// MONITOR: firmware can pass monitor packets to host.
// MONITOR_FLAG: firmware flags monitor packets.
// MONITOR_FMT_RADIOTAP: firmware provides monitor packets with radiotap header
// MONITOR_FMT_HW_RX_HDR: firmware provides monitor packets with hw/ucode header
// DOT11H: firmware supports 802.11h
// SAE: simultaneous authentication of equals
// FWAUTH: Firmware authenticator
// DUMP_OBSS: Firmware has capable to dump obss info to support ACS
// SCAN_V2: Version 2 scan params
// SAE_EXT: SAE authentication handled by user-space supplicant
//

//
// Quirks:
//
// AUTO_AUTH: workaround needed for automatic authentication type.
// NEED_MPC: driver needs to disable MPC during scanning operation.
//

//
// expand feature list to enumeration.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_feat_id {
    BRCMF_FEAT_LIST
    BRCMF_FEAT_LAST
}

//
// expand quirk list to enumeration.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_feat_quirk {
    BRCMF_QUIRK_LIST
    BRCMF_FEAT_QUIRK_LAST
}

//
// brcmf_feat_attach() - determine features and quirks.
//
// @drvr: driver instance.
//
extern "C" {
    pub fn brcmf_feat_attach(drvr: *mut brcmf_pub);
}
//
// brcmf_feat_debugfs_create() - create debugfs entries.
//
// @drvr: driver instance.
//
extern "C" {
    pub fn brcmf_feat_debugfs_create(drvr: *mut brcmf_pub);
}
//
// brcmf_feat_is_enabled() - query feature.
//
// @ifp: interface instance.
// @id: feature id to check.
//
// Return: true is feature is enabled; otherwise false.
//
extern "C" {
    pub fn brcmf_feat_is_enabled(ifp: *mut brcmf_if, id: brcmf_feat_id) -> bool;
}
//
// brcmf_feat_is_quirk_enabled() - query chip quirk.
//
// @ifp: interface instance.
// @quirk: quirk id to check.
//
// Return: true is quirk is enabled; otherwise false.
//
