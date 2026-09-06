//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/dp_rx.h
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
// Copyright (c) 2018-2021 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const DP_MAX_NWIFI_HDR_LEN: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_reoq_buf {
    pub vaddr: *mut c_void,
    pub paddr_aligned: dma_addr_t,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_rx_tid {
    pub tid: u8,
    pub ba_win_sz: u32,
    pub qbuf: ath12k_reoq_buf,
// Info related to rx fragments
    pub cur_sn: u32,
    pub last_frag_no: u16,
    pub rx_frag_bitmap: u16,
    pub rx_frags: sk_buff_head,
    pub dst_ring_desc: *mut hal_reo_dest_ring,
// Timer info related to fragments
    pub frag_timer: timer_list,
    pub dp: *mut ath12k_dp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_rx_tid_rxq {
    pub tid: u8,
    pub active: bool,
    pub qbuf: ath12k_reoq_buf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_rx_reo_cache_flush_elem {
    pub list: list_head,
    pub data: ath12k_dp_rx_tid_rxq,
    pub ts: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_reo_update_rx_queue_elem {
    pub list: list_head,
    pub rx_tid: ath12k_dp_rx_tid_rxq,
    pub peer_id: c_int,
    pub is_ml_peer: bool,
    pub ml_peer_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_rx_reo_cmd {
    pub list: list_head,
    pub data: ath12k_dp_rx_tid_rxq,
    pub cmd_num: c_int,
    pub status): hal_reo_cmd_status,
}

pub const ATH12K_DP_RX_REO_DESC_FREE_THRES: c_int = 64;
pub const ATH12K_DP_RX_REO_DESC_FREE_TIMEOUT_MS: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_dp_rx_decap_type {
    DP_RX_DECAP_TYPE_RAW,
    DP_RX_DECAP_TYPE_NATIVE_WIFI,
    DP_RX_DECAP_TYPE_ETHERNET2_DIX,
    DP_RX_DECAP_TYPE_8023,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_dp_rx_rfc1042_hdr {
    pub llc_dsap: u8,
    pub llc_ssap: u8,
    pub llc_ctrl: u8,
    pub snap_oui: [u8; 3],
    pub snap_type: __be16,
    pub __packed: },
    pub 0: u32 ret =,
    pub NL80211_RATE_INFO_HE_GI_0_8: ret =,
    pub NL80211_RATE_INFO_HE_GI_1_6: ret =,
    pub NL80211_RATE_INFO_HE_GI_3_2: ret =,
    pub NL80211_RATE_INFO_HE_GI_0_8: ret =,
    pub ret: return,
    pub hdr: *mut ieee80211_hdr,
    pub hal->hal_desc_sz): *mut *mut hdr = (struct ieee80211_hdr )(skb->data +,
    pub ieee80211_has_morefrags(hdr->frame_control): return,
    pub hdr: *mut ieee80211_hdr,
    pub hal->hal_desc_sz): *mut *mut hdr = (struct ieee80211_hdr )(skb->data +,
    pub IEEE80211_SCTL_FRAG: return le16_to_cpu(hdr->seq_ctrl) &,
    pub ab->hal.ops->rx_desc_get_l3_pad_bytes(desc): return,
    pub ldesc): hal->ops->rx_desc_copy_end_tlv(fdesc,,
    pub len): hal->ops->rx_desc_set_msdu_len(desc,,
    pub ab->hal.ops->rx_desc_get_mpdu_ppdu_id(rx_desc): return,
    pub hdr): hal->ops->rx_desc_get_dot11_hdr(desc,,
    pub enctype): hal->ops->rx_desc_get_crypto_header(desc, crypto_hdr,,
    pub hal->ops->rx_desc_get_msdu_src_link_id(desc): return,
    pub skb: *mut sk_buff,
    pub ldesc): hal->ops->extract_rx_desc_data(rx_info, rx_desc,,
    pub peer): *mut ath12k_dp_peer,
    pub rx_info): *mut hal_rx_desc_data,
    pub rx_info): *mut hal_rx_desc_data,
    pub skb): *mut *mut u64 ath12k_dp_rx_h_get_pn(struct ath12k_dp dp, struct sk_buff,
    pub cur_frag): *mut sk_buff,
    pub flags): hal_encrypt_type enctype, u32,
    pub link_id): u8,
    pub link_id): u8,
    pub key): *mut ieee80211_key_conf,
    pub peer): *mut *mut void ath12k_dp_rx_peer_tid_cleanup(struct ath12k ar, struct ath12k_dp_link_peer,
    pub tid): *mut *mut ath12k_dp_link_peer peer, u8,
    pub pn_type): hal_pn_type,
    pub ab): *mut int ath12k_dp_rx_pdev_reo_setup(struct ath12k_base,
    pub ab): *mut void ath12k_dp_rx_pdev_reo_cleanup(struct ath12k_base,
    pub ab): *mut int ath12k_dp_rx_htt_setup(struct ath12k_base,
    pub ab): *mut int ath12k_dp_rx_alloc(struct ath12k_base,
    pub ab): *mut void ath12k_dp_rx_free(struct ath12k_base,
    pub pdev_idx): *mut *mut int ath12k_dp_rx_pdev_alloc(struct ath12k_base ab, int,
    pub pdev_idx): *mut *mut void ath12k_dp_rx_pdev_free(struct ath12k_base ab, int,
    pub ab): *mut void ath12k_dp_rx_reo_cmd_list_cleanup(struct ath12k_base,
    pub req_entries): c_int,
    pub ar): *mut int ath12k_dp_rx_pdev_mon_attach(struct ath12k,
    pub vdev_id): *const *const *const int ath12k_dp_rx_peer_frag_setup(struct ath12k ar, u8 peer_mac, int,
    pub desc): *mut hal_rx_desc,
    pub rx_info): *mut hal_rx_desc_data,
    pub desc): *mut hal_rx_desc,
    pub desc): *mut hal_rx_desc,
    pub enctype): *mut *mut int ath12k_dp_rx_crypto_mic_len(struct ath12k_dp dp, enum hal_encrypt_type,
    pub rx_desc): *mut hal_rx_desc,
    pub rx_info): *mut hal_rx_desc_data,
    pub first): *mut sk_buff,
    pub status): hal_reo_cmd_status,
    pub status): hal_reo_cmd_status,
    pub dp): *mut void ath12k_dp_rx_process_reo_cmd_update_rx_queue_list(struct ath12k_dp,
    pub active): bool,
    pub tid): *mut *mut void ath12k_dp_mark_tid_as_inactive(struct ath12k_dp dp, int peer_id, u8,
