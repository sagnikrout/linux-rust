//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/mac.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2018-2019 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_generic_iter {
    pub ar: *mut ath11k,
    pub ret: c_int,
}

// number of failed packets (20 packets with 16 sw reties each)

// Use insanely high numbers to make sure that the firmware implementation
// won't start, we have the same functionality already in hostapd. Unit
// is seconds.
//
pub const ATH11K_KEEPALIVE_MIN_IDLE: c_int = 3747;
pub const ATH11K_KEEPALIVE_MAX_IDLE: c_int = 3895;
pub const ATH11K_KEEPALIVE_MAX_UNRESPONSIVE: c_int = 3900;
pub const WMI_HOST_RC_DS_FLAG: c_uint = 0x01;
pub const WMI_HOST_RC_CW40_FLAG: c_uint = 0x02;
pub const WMI_HOST_RC_SGI_FLAG: c_uint = 0x04;
pub const WMI_HOST_RC_HT_FLAG: c_uint = 0x08;
pub const WMI_HOST_RC_RTSCTS_FLAG: c_uint = 0x10;
pub const WMI_HOST_RC_TX_STBC_FLAG: c_uint = 0x20;
pub const WMI_HOST_RC_RX_STBC_FLAG: c_uint = 0xC0;
pub const WMI_HOST_RC_RX_STBC_FLAG_S: c_int = 6;
pub const WMI_HOST_RC_WEP_TKIP_FLAG: c_uint = 0x100;
pub const WMI_HOST_RC_TS_FLAG: c_uint = 0x200;
pub const WMI_HOST_RC_UAPSD_FLAG: c_uint = 0x400;
pub const WMI_HT_CAP_ENABLED: c_uint = 0x0001;
pub const WMI_HT_CAP_HT20_SGI: c_uint = 0x0002;
pub const WMI_HT_CAP_DYNAMIC_SMPS: c_uint = 0x0004;
pub const WMI_HT_CAP_TX_STBC: c_uint = 0x0008;
pub const WMI_HT_CAP_TX_STBC_MASK_SHIFT: c_int = 3;
pub const WMI_HT_CAP_RX_STBC: c_uint = 0x0030;
pub const WMI_HT_CAP_RX_STBC_MASK_SHIFT: c_int = 4;
pub const WMI_HT_CAP_LDPC: c_uint = 0x0040;
pub const WMI_HT_CAP_L_SIG_TXOP_PROT: c_uint = 0x0080;
pub const WMI_HT_CAP_MPDU_DENSITY: c_uint = 0x0700;
pub const WMI_HT_CAP_MPDU_DENSITY_MASK_SHIFT: c_int = 8;
pub const WMI_HT_CAP_HT40_SGI: c_uint = 0x0800;
pub const WMI_HT_CAP_RX_LDPC: c_uint = 0x1000;
pub const WMI_HT_CAP_TX_LDPC: c_uint = 0x2000;
pub const WMI_HT_CAP_IBF_BFER: c_uint = 0x4000;
// These macros should be used when we wish to advertise STBC support for
// only 1SS or 2SS or 3SS.
//
pub const WMI_HT_CAP_RX_STBC_1SS: c_uint = 0x0010;
pub const WMI_HT_CAP_RX_STBC_2SS: c_uint = 0x0020;
pub const WMI_HT_CAP_RX_STBC_3SS: c_uint = 0x0030;

pub const WMI_VHT_CAP_MAX_MPDU_LEN_MASK: c_uint = 0x00000003;
pub const WMI_VHT_CAP_RX_LDPC: c_uint = 0x00000010;
pub const WMI_VHT_CAP_SGI_80MHZ: c_uint = 0x00000020;
pub const WMI_VHT_CAP_SGI_160MHZ: c_uint = 0x00000040;
pub const WMI_VHT_CAP_TX_STBC: c_uint = 0x00000080;
pub const WMI_VHT_CAP_RX_STBC_MASK: c_uint = 0x00000300;
pub const WMI_VHT_CAP_RX_STBC_MASK_SHIFT: c_int = 8;
pub const WMI_VHT_CAP_SU_BFER: c_uint = 0x00000800;
pub const WMI_VHT_CAP_SU_BFEE: c_uint = 0x00001000;
pub const WMI_VHT_CAP_MAX_CS_ANT_MASK: c_uint = 0x0000E000;
pub const WMI_VHT_CAP_MAX_CS_ANT_MASK_SHIFT: c_int = 13;
pub const WMI_VHT_CAP_MAX_SND_DIM_MASK: c_uint = 0x00070000;
pub const WMI_VHT_CAP_MAX_SND_DIM_MASK_SHIFT: c_int = 16;
pub const WMI_VHT_CAP_MU_BFER: c_uint = 0x00080000;
pub const WMI_VHT_CAP_MU_BFEE: c_uint = 0x00100000;
pub const WMI_VHT_CAP_MAX_AMPDU_LEN_EXP: c_uint = 0x03800000;
pub const WMI_VHT_CAP_MAX_AMPDU_LEN_EXP_SHIT: c_int = 23;
pub const WMI_VHT_CAP_RX_FIXED_ANT: c_uint = 0x10000000;
pub const WMI_VHT_CAP_TX_FIXED_ANT: c_uint = 0x20000000;
pub const WMI_VHT_CAP_MAX_MPDU_LEN_11454: c_uint = 0x00000002;
// These macros should be used when we wish to advertise STBC support for
// only 1SS or 2SS or 3SS.
//
pub const WMI_VHT_CAP_RX_STBC_1SS: c_uint = 0x00000100;
pub const WMI_VHT_CAP_RX_STBC_2SS: c_uint = 0x00000200;
pub const WMI_VHT_CAP_RX_STBC_3SS: c_uint = 0x00000300;

// FIXME: should these be in ieee80211.h?

pub const WMI_MAX_SPATIAL_STREAM: c_int = 3;
pub const ATH11K_CHAN_WIDTH_NUM: c_int = 8;

pub const ATH11K_SCAN_11D_INTERVAL: c_int = 600000;
pub const ATH11K_11D_INVALID_VDEV_ID: c_uint = 0xFFFF;
extern "C" {
    pub fn ath11k_mac_11d_scan_start(ar: *mut ath11k, vdev_id: u32);
}
extern "C" {
    pub fn ath11k_mac_11d_scan_stop(ar: *mut ath11k);
}
extern "C" {
    pub fn ath11k_mac_11d_scan_stop_all(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_mac_destroy(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_mac_unregister(ab: *mut ath11k_base);
}
extern "C" {
    pub fn ath11k_mac_register(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_mac_allocate(ab: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn __ath11k_mac_scan_finish(ar: *mut ath11k);
}
extern "C" {
    pub fn ath11k_mac_scan_finish(ar: *mut ath11k);
}
extern "C" {
    pub fn ath11k_mac_get_target_pdev_id(ar: *mut ath11k) -> u8;
}
extern "C" {
    pub fn ath11k_mac_get_target_pdev_id_from_vif(arvif: *mut ath11k_vif) -> u8;
}
extern "C" {
    pub fn ath11k_mac_drain_tx(ar: *mut ath11k);
}
extern "C" {
    pub fn ath11k_mac_peer_cleanup_all(ar: *mut ath11k);
}
extern "C" {
    pub fn ath11k_mac_tx_mgmt_pending_free(buf_id: c_int, skb: *mut c_void, ctx: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ath11k_mac_bw_to_mac80211_bw(bw: u8) -> u8;
}
extern "C" {
    pub fn ath11k_mac_he_gi_to_nl80211_he_gi(sgi: u8) -> nl80211_he_gi;
}
extern "C" {
    pub fn ath11k_mac_phy_he_ru_to_nl80211_he_ru_alloc(ru_phy: u16) -> nl80211_he_ru_alloc;
}
extern "C" {
    pub fn ath11k_mac_he_ru_tones_to_nl80211_he_ru_alloc(ru_tones: u16) -> nl80211_he_ru_alloc;
}
extern "C" {
    pub fn ath11k_mac_mac80211_bw_to_ath11k_bw(bw: rate_info_bw) -> ath11k_supported_bw;
}
extern "C" {
    pub fn ath11k_dp_tx_get_encrypt_type(cipher: u32) -> hal_encrypt_type;
}
extern "C" {
    pub fn ath11k_mac_handle_beacon(ar: *mut ath11k, skb: *mut sk_buff);
}
extern "C" {
    pub fn ath11k_mac_handle_beacon_miss(ar: *mut ath11k, vdev_id: u32);
}
extern "C" {
    pub fn ath11k_mac_bcn_tx_event(arvif: *mut ath11k_vif);
}
extern "C" {
    pub fn ath11k_mac_wait_tx_complete(ar: *mut ath11k) -> c_int;
}
