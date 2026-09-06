//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/p2p.h
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
// Copyright (c) 2012 Broadcom Corporation
//

//
// enum p2p_bss_type - different type of BSS configurations.
//
// @P2PAPI_BSSCFG_PRIMARY: maps to driver's primary bsscfg.
// @P2PAPI_BSSCFG_DEVICE: maps to driver's P2P device discovery bsscfg.
// @P2PAPI_BSSCFG_CONNECTION: maps to driver's 1st P2P connection bsscfg.
// @P2PAPI_BSSCFG_CONNECTION2: maps to driver's 2nd P2P connection bsscfg.
// @P2PAPI_BSSCFG_MAX: used for range checking.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p2p_bss_type {
    P2PAPI_BSSCFG_PRIMARY, /* maps to driver's primary bsscfg */
    P2PAPI_BSSCFG_DEVICE, /* maps to driver's P2P device discovery bsscfg */
    P2PAPI_BSSCFG_CONNECTION, /* driver's 1st P2P connection bsscfg */
    P2PAPI_BSSCFG_CONNECTION2, /* driver's 2nd P2P connection bsscfg */
    P2PAPI_BSSCFG_MAX
}

//
// struct p2p_bss - peer-to-peer bss related information.
//
// @vif: virtual interface of this P2P bss.
// @private_data: TBD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p2p_bss {
    pub vif: *mut brcmf_cfg80211_vif,
    pub private_data: *mut c_void,
}

//
// enum brcmf_p2p_status - P2P specific dongle status.
//
// @BRCMF_P2P_STATUS_IF_ADD: peer-to-peer vif add sent to dongle.
// @BRCMF_P2P_STATUS_IF_DEL: NOT-USED?
// @BRCMF_P2P_STATUS_IF_DELETING: peer-to-peer vif delete sent to dongle.
// @BRCMF_P2P_STATUS_IF_CHANGING: peer-to-peer vif change sent to dongle.
// @BRCMF_P2P_STATUS_IF_CHANGED: peer-to-peer vif change completed on dongle.
// @BRCMF_P2P_STATUS_ACTION_TX_COMPLETED: action frame tx completed.
// @BRCMF_P2P_STATUS_ACTION_TX_NOACK: action frame tx not acked.
// @BRCMF_P2P_STATUS_GO_NEG_PHASE: P2P GO negotiation ongoing.
// @BRCMF_P2P_STATUS_DISCOVER_LISTEN: P2P listen, remaining on channel.
// @BRCMF_P2P_STATUS_SENDING_ACT_FRAME: In the process of sending action frame.
// @BRCMF_P2P_STATUS_WAITING_NEXT_AF_LISTEN: extra listen time for af tx.
// @BRCMF_P2P_STATUS_WAITING_NEXT_ACT_FRAME: waiting for action frame response.
// @BRCMF_P2P_STATUS_FINDING_COMMON_CHANNEL: search channel for AF active.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_p2p_status {
    BRCMF_P2P_STATUS_ENABLED,
    BRCMF_P2P_STATUS_IF_ADD,
    BRCMF_P2P_STATUS_IF_DEL,
    BRCMF_P2P_STATUS_IF_DELETING,
    BRCMF_P2P_STATUS_IF_CHANGING,
    BRCMF_P2P_STATUS_IF_CHANGED,
    BRCMF_P2P_STATUS_ACTION_TX_COMPLETED,
    BRCMF_P2P_STATUS_ACTION_TX_NOACK,
    BRCMF_P2P_STATUS_GO_NEG_PHASE,
    BRCMF_P2P_STATUS_DISCOVER_LISTEN,
    BRCMF_P2P_STATUS_SENDING_ACT_FRAME,
    BRCMF_P2P_STATUS_WAITING_NEXT_AF_LISTEN,
    BRCMF_P2P_STATUS_WAITING_NEXT_ACT_FRAME,
    BRCMF_P2P_STATUS_FINDING_COMMON_CHANNEL
}

//
// struct afx_hdl - action frame off channel storage.
//
// @afx_work: worker thread for searching channel
// @act_frm_scan: thread synchronizing struct.
// @is_active: channel searching active.
// @peer_chan: current channel.
// @is_listen: sets mode for afx worker.
// @my_listen_chan: this peers listen channel.
// @peer_listen_chan: remote peers listen channel.
// @tx_dst_addr: mac address where tx af should be sent to.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afx_hdl {
    pub afx_work: work_struct,
    pub act_frm_scan: completion,
    pub is_active: bool,
    pub peer_chan: i32,
    pub is_listen: bool,
    pub my_listen_chan: u16,
    pub peer_listen_chan: u16,
    pub tx_dst_addr: [u8; ETH_ALEN],
}

//
// struct brcmf_p2p_info - p2p specific driver information.
//
// @cfg: driver private data for cfg80211 interface.
// @status: status of P2P (see enum brcmf_p2p_status).
// @dev_addr: P2P device address.
// @int_addr: P2P interface address.
// @bss_idx: informate for P2P bss types.
// @listen_timer: timer for @WL_P2P_DISC_ST_LISTEN discover state.
// @listen_channel: channel for @WL_P2P_DISC_ST_LISTEN discover state.
// @remain_on_channel: contains copy of struct used by cfg80211.
// @remain_on_channel_cookie: cookie counter for remain on channel cmd
// @next_af_subtype: expected action frame subtype.
// @send_af_done: indication that action frame tx is complete.
// @afx_hdl: action frame search handler info.
// @af_sent_channel: channel action frame is sent.
// @af_tx_sent_jiffies: jiffies time when af tx was transmitted.
// @wait_next_af: thread synchronizing struct.
// @gon_req_action: about to send go negotiation requets frame.
// @block_gon_req_tx: drop tx go negotiation requets frame.
// @p2pdev_dynamically: is p2p device if created by module param or supplicant.
// @wait_for_offchan_complete: wait for off-channel tx completion event.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_p2p_info {
    pub cfg: *mut brcmf_cfg80211_info,
    pub status: c_ulong,
    pub dev_addr: [u8; ETH_ALEN],
    pub conn_int_addr: [u8; ETH_ALEN],
    pub conn2_int_addr: [u8; ETH_ALEN],
    pub bss_idx: [p2p_bss; P2PAPI_BSSCFG_MAX],
    pub listen_timer: timer_list,
    pub listen_channel: u8,
    pub remain_on_channel: ieee80211_channel,
    pub remain_on_channel_cookie: u32,
    pub next_af_subtype: u8,
    pub send_af_done: completion,
    pub afx_hdl: afx_hdl,
    pub af_sent_channel: u32,
    pub af_tx_sent_jiffies: c_ulong,
    pub wait_next_af: completion,
    pub gon_req_action: bool,
    pub block_gon_req_tx: bool,
    pub p2pdev_dynamically: bool,
    pub wait_for_offchan_complete: bool,
}

extern "C" {
    pub fn brcmf_p2p_attach(cfg: *mut brcmf_cfg80211_info, p2pdev_forced: bool) -> i32;
}
extern "C" {
    pub fn brcmf_p2p_detach(p2p: *mut brcmf_p2p_info);
}
extern "C" {
    pub fn brcmf_p2p_del_vif(wiphy: *mut wiphy, wdev: *mut wireless_dev) -> c_int;
}
extern "C" {
    pub fn brcmf_p2p_ifp_removed(ifp: *mut brcmf_if, rtnl_locked: bool);
}
extern "C" {
    pub fn brcmf_p2p_start_device(wiphy: *mut wiphy, wdev: *mut wireless_dev) -> c_int;
}
extern "C" {
    pub fn brcmf_p2p_stop_device(wiphy: *mut wiphy, wdev: *mut wireless_dev);
}
extern "C" {
    pub fn brcmf_p2p_cancel_remain_on_channel(ifp: *mut brcmf_if);
}
