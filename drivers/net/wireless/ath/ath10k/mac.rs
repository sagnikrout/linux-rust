//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/mac.h
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
// Copyright (c) 2005-2011 Atheros Communications Inc.
// Copyright (c) 2011-2017 Qualcomm Atheros, Inc.
//

pub const WEP_KEYID_SHIFT: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_generic_iter {
    pub ar: *mut ath10k,
    pub ret: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfc1042_hdr {
    pub llc_dsap: u8,
    pub llc_ssap: u8,
    pub llc_ctrl: u8,
    pub snap_oui: [u8; 3],
    pub snap_type: __be16,
    pub __packed: },
    pub priv_size): *mut *mut ath10k ath10k_mac_create(size_t,
    pub ar): *mut void ath10k_mac_destroy(struct ath10k,
    pub ar): *mut int ath10k_mac_register(struct ath10k,
    pub ar): *mut void ath10k_mac_unregister(struct ath10k,
    pub vdev_id): *mut *mut *mut ath10k_vif ath10k_get_arvif(ath10k ar, u32,
    pub ar): *mut void __ath10k_scan_finish(struct ath10k,
    pub ar): *mut void ath10k_scan_finish(struct ath10k,
    pub work): *mut void ath10k_scan_timeout_work(struct work_struct,
    pub ar): *mut void ath10k_offchan_tx_purge(struct ath10k,
    pub work): *mut void ath10k_offchan_tx_work(struct work_struct,
    pub ar): *mut void ath10k_mgmt_over_wmi_tx_purge(struct ath10k,
    pub work): *mut void ath10k_mgmt_over_wmi_tx_work(struct work_struct,
    pub ar): *mut void ath10k_halt(struct ath10k,
    pub arvif): *mut void ath10k_mac_vif_beacon_free(struct ath10k_vif,
    pub ar): *mut void ath10k_drain_tx(struct ath10k,
    pub keyidx): u8,
    pub def): *mut cfg80211_chan_def,
    pub skb): *mut *mut void ath10k_mac_handle_beacon(struct ath10k ar, struct sk_buff,
    pub vdev_id): *mut *mut void ath10k_mac_handle_beacon_miss(struct ath10k ar, u32,
    pub action): wmi_tlv_tx_pause_action,
    pub cck): u8 hw_rate, bool,
    pub bitrate): u32,
    pub reason): *mut *mut void ath10k_mac_tx_lock(struct ath10k ar, int,
    pub reason): *mut *mut void ath10k_mac_tx_unlock(struct ath10k ar, int,
    pub reason): *mut *mut void ath10k_mac_vif_tx_lock(struct ath10k_vif arvif, int,
    pub reason): *mut *mut void ath10k_mac_vif_tx_unlock(struct ath10k_vif arvif, int,
    pub ar): *mut bool ath10k_mac_tx_frm_has_freq(struct ath10k,
    pub ar): *mut void ath10k_mac_tx_push_pending(struct ath10k,
    pub txq): *mut ieee80211_txq,
    pub tid): u8,
    pub val): *mut *mut int ath10k_mac_ext_resource_config(struct ath10k ar, u32,
    pub ar): *mut void ath10k_mac_wait_tx_complete(struct ath10k,
    pub enable): *mut *mut int ath10k_mac_rfkill_enable_radio(struct ath10k ar, bool,
    pub IEEE80211_SKB_CB(skb): *mut *mut ieee80211_tx_info info =,
    pub )skb->data: *mut *mut ieee80211_hdr hdr = (ieee80211_hdr,
    pub )vif->drv_priv: *mut *mut ath10k_vif arvif = (void,
    pub 0x1000: arvif->tx_seq_no =,
    pub 0x10: arvif->tx_seq_no +=,
    pub cpu_to_le16(IEEE80211_SCTL_FRAG): hdr->seq_ctrl &=,
    pub cpu_to_le16(arvif->tx_seq_no): hdr->seq_ctrl |=,
